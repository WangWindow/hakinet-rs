use anyhow::{anyhow, Result};

use hakinet_common::{
    network::{parse_targets, PortRange, Protocol},
    output::print_scan_progress,
    types::{HostInfo, PortInfo, PortState, ScanConfig, ScanResults},
    utils::{current_timestamp_micros, shuffle},
};

use std::net::{IpAddr, SocketAddr};
use std::sync::Arc;
use std::time::Duration;
use tokio::net::{TcpSocket, UdpSocket};
use tokio::sync::Semaphore;
use tokio::time::timeout;

pub struct PortScanner {
    config: ScanConfig,
    service_detection: bool,
    os_detection: bool,
}

impl PortScanner {
    pub fn new() -> Self {
        PortScanner {
            config: ScanConfig::default(),
            service_detection: false,
            os_detection: false,
        }
    }

    pub fn with_max_parallel(mut self, max_parallel: usize) -> Self {
        self.config.max_parallel = max_parallel;
        self
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.config.timeout = timeout;
        self
    }

    pub fn with_randomize(mut self, randomize: bool) -> Self {
        self.config.randomize = randomize;
        self
    }

    pub fn with_service_detection(mut self, enabled: bool) -> Self {
        self.service_detection = enabled;
        self
    }

    pub fn with_os_detection(mut self, enabled: bool) -> Self {
        self.os_detection = enabled;
        self
    }

    pub async fn syn_scan(&self, targets: Vec<String>, ports: String) -> Result<ScanResults> {
        let hosts = self.parse_all_targets(targets).await?;
        let port_ranges = self.parse_ports(&ports)?;
        
        let mut results = ScanResults::new();
        let semaphore = Arc::new(Semaphore::new(self.config.max_parallel));
        
        for host in hosts {
            let mut host_info = HostInfo::new(host.addr).with_hostname(
                host.hostname.unwrap_or_else(|| host.addr.to_string())
            );
            
            // Check if host is up first
            if self.ping_host(&host.addr).await {
                host_info = host_info.set_up(true);
                
                let mut scan_ports = Vec::new();
                for range in &port_ranges {
                    scan_ports.extend(range.iter());
                }
                
                if self.config.randomize {
                    shuffle(&mut scan_ports);
                }

                let total_ports = scan_ports.len();
                let mut completed = 0;

                for port in scan_ports {
                    let permit = semaphore.clone().acquire_owned().await.unwrap();
                    let host_addr = host.addr;
                    let timeout_duration = self.config.timeout;
                    let service_detection = self.service_detection;

                    tokio::spawn(async move {
                        let _permit = permit;
                        let start_time = current_timestamp_micros();
                        
                        let port_info = match syn_scan_port(host_addr, port, timeout_duration).await {
                            Ok(state) => {
                                let response_time = current_timestamp_micros() - start_time;
                                let mut port_info = PortInfo::new(port, "tcp".to_string(), state)
                                    .with_response_time(response_time);
                                
                                if service_detection && state == PortState::Open {
                                    if let Some(service) = detect_service(host_addr, port).await {
                                        port_info = port_info.with_service(service);
                                    }
                                }
                                
                                port_info
                            },
                            Err(_) => PortInfo::new(port, "tcp".to_string(), PortState::Filtered),
                        };

                        port_info
                    });

                    completed += 1;
                    print_scan_progress(completed, total_ports, &host.addr.to_string());
                }
            }
            
            results.add_host(host_info);
        }

        results.finalize();
        Ok(results)
    }

    pub async fn connect_scan(&self, targets: Vec<String>, ports: String) -> Result<ScanResults> {
        let hosts = self.parse_all_targets(targets).await?;
        let port_ranges = self.parse_ports(&ports)?;
        
        let mut results = ScanResults::new();
        let semaphore = Arc::new(Semaphore::new(self.config.max_parallel));
        
        for host in hosts {
            let mut host_info = HostInfo::new(host.addr).with_hostname(
                host.hostname.unwrap_or_else(|| host.addr.to_string())
            );
            
            if self.ping_host(&host.addr).await {
                host_info = host_info.set_up(true);
                
                let mut scan_ports = Vec::new();
                for range in &port_ranges {
                    scan_ports.extend(range.iter());
                }
                
                if self.config.randomize {
                    shuffle(&mut scan_ports);
                }

                let total_ports = scan_ports.len();
                let mut port_results = Vec::new();

                for (i, port) in scan_ports.iter().enumerate() {
                    let permit = semaphore.clone().acquire_owned().await.unwrap();
                    let host_addr = host.addr;
                    let port = *port;
                    let timeout_duration = self.config.timeout;
                    let service_detection = self.service_detection;

                    let handle = tokio::spawn(async move {
                        let _permit = permit;
                        let start_time = current_timestamp_micros();
                        
                        let socket_addr = SocketAddr::new(host_addr, port);
                        let state = match timeout(timeout_duration, tokio::net::TcpStream::connect(socket_addr)).await {
                            Ok(Ok(_)) => PortState::Open,
                            Ok(Err(_)) => PortState::Closed,
                            Err(_) => PortState::Filtered,
                        };

                        let response_time = current_timestamp_micros() - start_time;
                        let mut port_info = PortInfo::new(port, "tcp".to_string(), state)
                            .with_response_time(response_time);
                        
                        if service_detection && state == PortState::Open {
                            if let Some(service) = detect_service(host_addr, port).await {
                                port_info = port_info.with_service(service);
                            }
                        }
                        
                        port_info
                    });

                    port_results.push(handle);
                    print_scan_progress(i + 1, total_ports, &host.addr.to_string());
                }

                // Collect results
                for handle in port_results {
                    if let Ok(port_info) = handle.await {
                        host_info = host_info.add_port(port_info);
                    }
                }
            }
            
            results.add_host(host_info);
        }

        println!(); // New line after progress
        results.finalize();
        Ok(results)
    }

    pub async fn udp_scan(&self, targets: Vec<String>, ports: String) -> Result<ScanResults> {
        let hosts = self.parse_all_targets(targets).await?;
        let port_ranges = self.parse_ports(&ports)?;
        
        let mut results = ScanResults::new();
        
        for host in hosts {
            let mut host_info = HostInfo::new(host.addr).with_hostname(
                host.hostname.unwrap_or_else(|| host.addr.to_string())
            );
            
            if self.ping_host(&host.addr).await {
                host_info = host_info.set_up(true);
                
                let mut scan_ports = Vec::new();
                for range in &port_ranges {
                    scan_ports.extend(range.iter());
                }
                
                if self.config.randomize {
                    shuffle(&mut scan_ports);
                }

                let total_ports = scan_ports.len();

                for (i, port) in scan_ports.iter().enumerate() {
                    let port_info = self.udp_scan_port(host.addr, *port).await;
                    host_info = host_info.add_port(port_info);
                    print_scan_progress(i + 1, total_ports, &host.addr.to_string());
                }
            }
            
            results.add_host(host_info);
        }

        println!(); // New line after progress
        results.finalize();
        Ok(results)
    }

    pub async fn comprehensive_scan(&self, targets: Vec<String>, ports: String) -> Result<ScanResults> {
        // First do TCP scan
        let tcp_results = self.connect_scan(targets.clone(), ports.clone()).await?;
        
        // Then do UDP scan on common UDP ports
        let udp_ports = "53,67,68,69,123,161,162,500,514,520,1900,4500";
        let udp_results = self.udp_scan(targets, udp_ports.to_string()).await?;
        
        // Merge results
        let mut combined_results = tcp_results;
        
        for udp_host in udp_results.hosts {
            if let Some(tcp_host) = combined_results.hosts.iter_mut()
                .find(|h| h.addr == udp_host.addr) {
                tcp_host.ports.extend(udp_host.ports);
            }
        }
        
        combined_results.finalize();
        Ok(combined_results)
    }

    async fn parse_all_targets(&self, targets: Vec<String>) -> Result<Vec<hakinet_common::network::HostTarget>> {
        let mut all_targets = Vec::new();
        
        for target in targets {
            let parsed = parse_targets(&target).await?;
            all_targets.extend(parsed);
        }
        
        Ok(all_targets)
    }

    fn parse_ports(&self, ports_str: &str) -> Result<Vec<PortRange>> {
        let mut ranges = Vec::new();
        
        for part in ports_str.split(',') {
            let range = part.trim().parse::<PortRange>()?;
            ranges.push(range);
        }
        
        Ok(ranges)
    }

    async fn ping_host(&self, addr: &IpAddr) -> bool {
        // Simple TCP connect to common ports to check if host is up
        let common_ports = [80, 443, 22, 21, 25, 53];
        
        for port in common_ports {
            let socket_addr = SocketAddr::new(*addr, port);
            if timeout(Duration::from_millis(500), tokio::net::TcpStream::connect(socket_addr)).await.is_ok() {
                return true;
            }
        }
        
        false
    }

    async fn udp_scan_port(&self, addr: IpAddr, port: u16) -> PortInfo {
        let socket_addr = SocketAddr::new(addr, port);
        let start_time = current_timestamp_micros();
        
        match UdpSocket::bind("0.0.0.0:0").await {
            Ok(socket) => {
                match timeout(self.config.timeout, socket.connect(socket_addr)).await {
                    Ok(Ok(_)) => {
                        // Send empty UDP packet
                        let _ = socket.send(&[]).await;
                        
                        // Try to receive response
                        let mut buf = [0u8; 1024];
                        match timeout(Duration::from_millis(100), socket.recv(&mut buf)).await {
                            Ok(Ok(_)) => {
                                let response_time = current_timestamp_micros() - start_time;
                                PortInfo::new(port, "udp".to_string(), PortState::Open)
                                    .with_response_time(response_time)
                            },
                            _ => PortInfo::new(port, "udp".to_string(), PortState::OpenFiltered),
                        }
                    },
                    _ => PortInfo::new(port, "udp".to_string(), PortState::Filtered),
                }
            },
            Err(_) => PortInfo::new(port, "udp".to_string(), PortState::Unknown),
        }
    }
}

impl Default for PortScanner {
    fn default() -> Self {
        Self::new()
    }
}

async fn syn_scan_port(addr: IpAddr, port: u16, timeout_duration: Duration) -> Result<PortState> {
    let socket = TcpSocket::new_v4()?;
    let socket_addr = SocketAddr::new(addr, port);
    
    match timeout(timeout_duration, socket.connect(socket_addr)).await {
        Ok(Ok(_)) => Ok(PortState::Open),
        Ok(Err(_)) => Ok(PortState::Closed),
        Err(_) => Ok(PortState::Filtered),
    }
}

async fn detect_service(addr: IpAddr, port: u16) -> Option<String> {
    // Basic service detection based on port number
    let service = hakinet_common::network::get_service_name(port, Protocol::Tcp);
    
    if let Some(service_name) = service {
        return Some(service_name.to_string());
    }
    
    // Try to grab banner for unknown services
    match grab_banner(addr, port).await {
        Ok(banner) => {
            if banner.to_lowercase().contains("http") {
                Some("http".to_string())
            } else if banner.to_lowercase().contains("ssh") {
                Some("ssh".to_string())
            } else if banner.to_lowercase().contains("ftp") {
                Some("ftp".to_string())
            } else {
                Some("unknown".to_string())
            }
        },
        Err(_) => None,
    }
}

async fn grab_banner(addr: IpAddr, port: u16) -> Result<String> {
    use tokio::io::AsyncReadExt;
    
    let socket_addr = SocketAddr::new(addr, port);
    let mut stream = timeout(Duration::from_secs(2), tokio::net::TcpStream::connect(socket_addr)).await??;
    
    let mut buffer = [0u8; 1024];
    
    match timeout(Duration::from_secs(1), stream.read(&mut buffer)).await {
        Ok(Ok(bytes_read)) => {
            let banner = String::from_utf8_lossy(&buffer[..bytes_read]);
            Ok(banner.trim().to_string())
        },
        _ => Err(anyhow!("Failed to read banner")),
    }
}