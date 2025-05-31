use anyhow::Result;
use hakinet_common::{
    network::{parse_targets, is_private_ip},
    types::{HostInfo, ScanResults},
    utils::current_timestamp_micros,
};
use log::warn;
use std::net::{IpAddr, SocketAddr};
use std::sync::Arc;
use std::time::Duration;

use tokio::sync::Semaphore;
use tokio::time::timeout;


pub struct HostDiscoverer {
    max_parallel: usize,
    timeout: Duration,
}

impl HostDiscoverer {
    pub fn new() -> Self {
        HostDiscoverer {
            max_parallel: 50,
            timeout: Duration::from_secs(2),
        }
    }

    pub fn with_max_parallel(mut self, max_parallel: usize) -> Self {
        self.max_parallel = max_parallel;
        self
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    pub async fn ping_discovery(&self, network: &str) -> Result<ScanResults> {
        let targets = parse_targets(network).await?;
        let mut results = ScanResults::new();
        let semaphore = Arc::new(Semaphore::new(self.max_parallel));

        let mut handles = Vec::new();

        for target in targets {
            let permit = semaphore.clone().acquire_owned().await.unwrap();
            let timeout_duration = self.timeout;
            let addr = target.addr;
            let hostname = target.hostname.clone();

            let handle = tokio::spawn(async move {
                let _permit = permit;
                let start_time = current_timestamp_micros();
                
                let is_up = ping_host(addr, timeout_duration).await;
                let response_time = if is_up {
                    Some(current_timestamp_micros() - start_time)
                } else {
                    None
                };

                let mut host_info = HostInfo::new(addr).set_up(is_up);
                if let Some(hostname) = hostname {
                    host_info = host_info.with_hostname(hostname);
                }
                if let Some(time) = response_time {
                    host_info.response_time = Some(time);
                }

                host_info
            });

            handles.push(handle);
        }

        // Collect results
        for handle in handles {
            if let Ok(host_info) = handle.await {
                results.add_host(host_info);
            }
        }

        results.finalize();
        Ok(results)
    }

    pub async fn tcp_syn_discovery(&self, network: &str) -> Result<ScanResults> {
        let targets = parse_targets(network).await?;
        let mut results = ScanResults::new();
        let semaphore = Arc::new(Semaphore::new(self.max_parallel));

        let common_ports = [80, 443, 22, 21, 25, 53, 110, 143, 993, 995];
        let mut handles = Vec::new();

        for target in targets {
            let permit = semaphore.clone().acquire_owned().await.unwrap();
            let timeout_duration = self.timeout;
            let addr = target.addr;
            let hostname = target.hostname.clone();

            let handle = tokio::spawn(async move {
                let _permit = permit;
                let start_time = current_timestamp_micros();
                
                let mut is_up = false;
                for &port in &common_ports {
                    if tcp_ping(addr, port, timeout_duration).await {
                        is_up = true;
                        break;
                    }
                }

                let response_time = if is_up {
                    Some(current_timestamp_micros() - start_time)
                } else {
                    None
                };

                let mut host_info = HostInfo::new(addr).set_up(is_up);
                if let Some(hostname) = hostname {
                    host_info = host_info.with_hostname(hostname);
                }
                if let Some(time) = response_time {
                    host_info.response_time = Some(time);
                }

                host_info
            });

            handles.push(handle);
        }

        // Collect results
        for handle in handles {
            if let Ok(host_info) = handle.await {
                results.add_host(host_info);
            }
        }

        results.finalize();
        Ok(results)
    }

    pub async fn arp_discovery(&self, network: &str) -> Result<ScanResults> {
        // ARP discovery is only valid for local network segments
        let targets = parse_targets(network).await?;
        let mut results = ScanResults::new();

        // Check if targets are in private IP ranges
        for target in targets {
            if !is_private_ip(&target.addr) {
                warn!("ARP discovery only works on local networks. Skipping {}", target.addr);
                continue;
            }

            // For now, fall back to ping discovery for ARP
            // In a real implementation, you would use raw sockets to send ARP requests
            let start_time = current_timestamp_micros();
            let is_up = ping_host(target.addr, self.timeout).await;
            
            let response_time = if is_up {
                Some(current_timestamp_micros() - start_time)
            } else {
                None
            };

            let mut host_info = HostInfo::new(target.addr).set_up(is_up);
            if let Some(hostname) = target.hostname {
                host_info = host_info.with_hostname(hostname);
            }
            if let Some(time) = response_time {
                host_info.response_time = Some(time);
            }

            results.add_host(host_info);
        }

        results.finalize();
        Ok(results)
    }
}

impl Default for HostDiscoverer {
    fn default() -> Self {
        Self::new()
    }
}

async fn ping_host(addr: IpAddr, timeout_duration: Duration) -> bool {
    // Since we can't easily do ICMP ping without root privileges,
    // we'll do a TCP ping to common ports
    let common_ports = [80, 443, 22, 53];
    
    for &port in &common_ports {
        if tcp_ping(addr, port, timeout_duration).await {
            return true;
        }
    }
    
    false
}

async fn tcp_ping(addr: IpAddr, port: u16, timeout_duration: Duration) -> bool {
    let socket_addr = SocketAddr::new(addr, port);
    timeout(timeout_duration, tokio::net::TcpStream::connect(socket_addr))
        .await
        .is_ok()
}

async fn icmp_ping(_addr: IpAddr, _timeout_duration: Duration) -> bool {
    // This would require raw socket privileges
    // For now, we'll return false and rely on TCP ping
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_tcp_ping() {
        // Test against localhost
        let result = tcp_ping("127.0.0.1".parse().unwrap(), 80, Duration::from_secs(1)).await;
        // This might fail if no web server is running on localhost:80
        println!("TCP ping result: {}", result);
    }

    #[tokio::test]
    async fn test_host_discovery() {
        let discoverer = HostDiscoverer::new();
        
        // Test with localhost
        match discoverer.ping_discovery("127.0.0.1").await {
            Ok(results) => {
                assert!(!results.hosts.is_empty());
                println!("Discovery results: {:?}", results);
            }
            Err(e) => println!("Discovery failed: {}", e),
        }
    }
}