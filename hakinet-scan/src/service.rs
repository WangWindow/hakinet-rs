use anyhow::{anyhow, Result};
use hakinet_common::{
    network::{get_service_name, Protocol},
};
use std::collections::HashMap;
use std::net::{IpAddr, SocketAddr};
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream as AsyncTcpStream;
use tokio::time::timeout;

pub struct ServiceDetector {
    timeout: Duration,
    probe_data: HashMap<u16, Vec<u8>>,
}

impl ServiceDetector {
    pub fn new() -> Self {
        let mut detector = ServiceDetector {
            timeout: Duration::from_secs(3),
            probe_data: HashMap::new(),
        };
        detector.init_probes();
        detector
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    pub async fn detect_service(&self, addr: IpAddr, port: u16) -> Option<ServiceInfo> {
        // First try to identify by well-known port
        if let Some(service_name) = get_service_name(port, Protocol::Tcp) {
            let mut service_info = ServiceInfo {
                name: service_name.to_string(),
                version: None,
                banner: None,
                confidence: 90,
            };

            // Try to get version information
            if let Ok(banner) = self.grab_banner(addr, port).await {
                service_info.banner = Some(banner.clone());
                service_info.version = self.extract_version(&banner, service_name);
                if service_info.version.is_some() {
                    service_info.confidence = 95;
                }
            }

            return Some(service_info);
        }

        // Try banner grabbing for unknown services
        match self.grab_banner(addr, port).await {
            Ok(banner) => self.analyze_banner(&banner, port),
            Err(_) => {
                // Try service-specific probes
                self.probe_service(addr, port).await
            }
        }
    }

    async fn grab_banner(&self, addr: IpAddr, port: u16) -> Result<String> {
        let socket_addr = SocketAddr::new(addr, port);
        
        let mut stream = timeout(
            self.timeout,
            AsyncTcpStream::connect(socket_addr)
        ).await??;

        // Wait for initial banner
        let mut buffer = vec![0u8; 1024];
        match timeout(Duration::from_secs(2), stream.read(&mut buffer)).await {
            Ok(Ok(bytes_read)) if bytes_read > 0 => {
                let banner = String::from_utf8_lossy(&buffer[..bytes_read]);
                Ok(banner.trim().to_string())
            }
            _ => {
                // If no initial banner, try sending HTTP request
                if port == 80 || port == 8080 || port == 443 {
                    let http_request = b"GET / HTTP/1.1\r\nHost: localhost\r\n\r\n";
                    if stream.write_all(http_request).await.is_ok() {
                        match timeout(Duration::from_secs(2), stream.read(&mut buffer)).await {
                            Ok(Ok(bytes_read)) if bytes_read > 0 => {
                                let response = String::from_utf8_lossy(&buffer[..bytes_read]);
                                return Ok(response.trim().to_string());
                            }
                            _ => {}
                        }
                    }
                }
                Err(anyhow!("No banner received"))
            }
        }
    }

    async fn probe_service(&self, addr: IpAddr, port: u16) -> Option<ServiceInfo> {
        if let Some(probe_data) = self.probe_data.get(&port) {
            match self.send_probe(addr, port, probe_data).await {
                Ok(response) => self.analyze_banner(&response, port),
                Err(_) => None,
            }
        } else {
            None
        }
    }

    async fn send_probe(&self, addr: IpAddr, port: u16, probe_data: &[u8]) -> Result<String> {
        let socket_addr = SocketAddr::new(addr, port);
        
        let mut stream = timeout(
            self.timeout,
            AsyncTcpStream::connect(socket_addr)
        ).await??;

        stream.write_all(probe_data).await?;
        
        let mut buffer = vec![0u8; 1024];
        let bytes_read = timeout(Duration::from_secs(2), stream.read(&mut buffer)).await??;
        
        if bytes_read > 0 {
            let response = String::from_utf8_lossy(&buffer[..bytes_read]);
            Ok(response.trim().to_string())
        } else {
            Err(anyhow!("No response to probe"))
        }
    }

    fn analyze_banner(&self, banner: &str, port: u16) -> Option<ServiceInfo> {
        let banner_lower = banner.to_lowercase();
        
        // HTTP services
        if banner_lower.contains("http/") {
            let mut service_info = ServiceInfo {
                name: "http".to_string(),
                version: self.extract_http_version(banner),
                banner: Some(banner.to_string()),
                confidence: 85,
            };
            
            if banner_lower.contains("apache") {
                service_info.name = "apache".to_string();
                service_info.version = self.extract_apache_version(banner);
                service_info.confidence = 90;
            } else if banner_lower.contains("nginx") {
                service_info.name = "nginx".to_string();
                service_info.version = self.extract_nginx_version(banner);
                service_info.confidence = 90;
            } else if banner_lower.contains("iis") {
                service_info.name = "iis".to_string();
                service_info.version = self.extract_iis_version(banner);
                service_info.confidence = 90;
            }
            
            return Some(service_info);
        }
        
        // SSH services
        if banner_lower.starts_with("ssh-") {
            return Some(ServiceInfo {
                name: "ssh".to_string(),
                version: self.extract_ssh_version(banner),
                banner: Some(banner.to_string()),
                confidence: 95,
            });
        }
        
        // FTP services
        if banner_lower.contains("ftp") || banner.starts_with("220") {
            return Some(ServiceInfo {
                name: "ftp".to_string(),
                version: self.extract_ftp_version(banner),
                banner: Some(banner.to_string()),
                confidence: 85,
            });
        }
        
        // SMTP services
        if banner.starts_with("220") && banner_lower.contains("smtp") {
            return Some(ServiceInfo {
                name: "smtp".to_string(),
                version: self.extract_smtp_version(banner),
                banner: Some(banner.to_string()),
                confidence: 85,
            });
        }
        
        // Telnet services
        if banner_lower.contains("telnet") || port == 23 {
            return Some(ServiceInfo {
                name: "telnet".to_string(),
                version: None,
                banner: Some(banner.to_string()),
                confidence: 70,
            });
        }
        
        // MySQL services
        if banner_lower.contains("mysql") || (port == 3306 && banner.len() > 10) {
            return Some(ServiceInfo {
                name: "mysql".to_string(),
                version: self.extract_mysql_version(banner),
                banner: Some(banner.to_string()),
                confidence: 80,
            });
        }
        
        // PostgreSQL services
        if banner_lower.contains("postgresql") || port == 5432 {
            return Some(ServiceInfo {
                name: "postgresql".to_string(),
                version: None,
                banner: Some(banner.to_string()),
                confidence: 75,
            });
        }
        
        // Unknown service with banner
        if !banner.is_empty() {
            Some(ServiceInfo {
                name: "unknown".to_string(),
                version: None,
                banner: Some(banner.to_string()),
                confidence: 30,
            })
        } else {
            None
        }
    }

    fn init_probes(&mut self) {
        // HTTP probe
        self.probe_data.insert(80, b"GET / HTTP/1.1\r\nHost: localhost\r\n\r\n".to_vec());
        self.probe_data.insert(8080, b"GET / HTTP/1.1\r\nHost: localhost\r\n\r\n".to_vec());
        
        // HTTPS probe (will likely fail without TLS, but worth trying)
        self.probe_data.insert(443, b"GET / HTTP/1.1\r\nHost: localhost\r\n\r\n".to_vec());
        
        // SMTP probe
        self.probe_data.insert(25, b"EHLO localhost\r\n".to_vec());
        
        // FTP probe
        self.probe_data.insert(21, b"USER anonymous\r\n".to_vec());
        
        // POP3 probe
        self.probe_data.insert(110, b"USER test\r\n".to_vec());
        
        // IMAP probe
        self.probe_data.insert(143, b"A001 CAPABILITY\r\n".to_vec());
    }

    fn extract_version(&self, banner: &str, service: &str) -> Option<String> {
        match service {
            "http" => self.extract_http_version(banner),
            "ssh" => self.extract_ssh_version(banner),
            "ftp" => self.extract_ftp_version(banner),
            "smtp" => self.extract_smtp_version(banner),
            "mysql" => self.extract_mysql_version(banner),
            _ => None,
        }
    }

    fn extract_http_version(&self, banner: &str) -> Option<String> {
        if let Some(start) = banner.find("HTTP/") {
            let version_part = &banner[start..];
            if let Some(end) = version_part.find(' ') {
                return Some(version_part[..end].to_string());
            }
        }
        None
    }

    fn extract_apache_version(&self, banner: &str) -> Option<String> {
        if let Some(start) = banner.find("Apache/") {
            let version_part = &banner[start + 7..];
            if let Some(end) = version_part.find(' ') {
                return Some(format!("Apache/{}", &version_part[..end]));
            }
        }
        None
    }

    fn extract_nginx_version(&self, banner: &str) -> Option<String> {
        if let Some(start) = banner.find("nginx/") {
            let version_part = &banner[start + 6..];
            if let Some(end) = version_part.find(' ') {
                return Some(format!("nginx/{}", &version_part[..end]));
            }
        }
        None
    }

    fn extract_iis_version(&self, banner: &str) -> Option<String> {
        if let Some(start) = banner.find("IIS/") {
            let version_part = &banner[start + 4..];
            if let Some(end) = version_part.find(' ') {
                return Some(format!("IIS/{}", &version_part[..end]));
            }
        }
        None
    }

    fn extract_ssh_version(&self, banner: &str) -> Option<String> {
        if banner.starts_with("SSH-") {
            if let Some(end) = banner.find('\r') {
                return Some(banner[..end].to_string());
            } else if let Some(end) = banner.find('\n') {
                return Some(banner[..end].to_string());
            } else {
                return Some(banner.to_string());
            }
        }
        None
    }

    fn extract_ftp_version(&self, banner: &str) -> Option<String> {
        // FTP banners are usually in format "220 ServiceName Version ready"
        let parts: Vec<&str> = banner.split_whitespace().collect();
        if parts.len() >= 3 && parts[0] == "220" {
            return Some(parts[1..].join(" "));
        }
        None
    }

    fn extract_smtp_version(&self, banner: &str) -> Option<String> {
        // SMTP banners are similar to FTP
        let parts: Vec<&str> = banner.split_whitespace().collect();
        if parts.len() >= 2 && parts[0] == "220" {
            return Some(parts[1..].join(" "));
        }
        None
    }

    fn extract_mysql_version(&self, banner: &str) -> Option<String> {
        // MySQL version is usually in the first few bytes of the handshake packet
        // This is a simplified extraction
        if banner.contains("mysql") {
            // Try to find version pattern like "5.7.32"
            for part in banner.split_whitespace() {
                if part.contains('.') && part.chars().next().unwrap_or('a').is_ascii_digit() {
                    return Some(part.to_string());
                }
            }
        }
        None
    }
}

impl Default for ServiceDetector {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct ServiceInfo {
    pub name: String,
    pub version: Option<String>,
    pub banner: Option<String>,
    pub confidence: u8, // 0-100
}

impl ServiceInfo {
    pub fn display_string(&self) -> String {
        match &self.version {
            Some(version) => format!("{} {}", self.name, version),
            None => self.name.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_version_extraction() {
        let detector = ServiceDetector::new();
        let banner = "HTTP/1.1 200 OK\r\nServer: Apache/2.4.41";
        assert_eq!(detector.extract_http_version(banner), Some("HTTP/1.1".to_string()));
    }

    #[test]
    fn test_ssh_version_extraction() {
        let detector = ServiceDetector::new();
        let banner = "SSH-2.0-OpenSSH_7.4\r\n";
        assert_eq!(detector.extract_ssh_version(banner), Some("SSH-2.0-OpenSSH_7.4".to_string()));
    }

    #[test]
    fn test_service_analysis() {
        let detector = ServiceDetector::new();
        
        let http_banner = "HTTP/1.1 200 OK\r\nServer: nginx/1.18.0";
        let service = detector.analyze_banner(http_banner, 80).unwrap();
        assert_eq!(service.name, "nginx");
        
        let ssh_banner = "SSH-2.0-OpenSSH_7.4";
        let service = detector.analyze_banner(ssh_banner, 22).unwrap();
        assert_eq!(service.name, "ssh");
    }
}