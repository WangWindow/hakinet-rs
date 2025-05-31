use anyhow::{anyhow, Result};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::str::FromStr;

/// Port range for scanning
#[derive(Debug, Clone)]
pub struct PortRange {
    pub start: u16,
    pub end: u16,
}

impl PortRange {
    pub fn new(start: u16, end: u16) -> Result<Self> {
        if start > end {
            return Err(anyhow!("Start port cannot be greater than end port"));
        }
        Ok(PortRange { start, end })
    }

    pub fn single(port: u16) -> Self {
        PortRange { start: port, end: port }
    }

    pub fn all() -> Self {
        PortRange { start: 1, end: 65535 }
    }

    pub fn common() -> Self {
        // Common ports for quick scanning
        PortRange { start: 1, end: 1024 }
    }

    pub fn iter(&self) -> impl Iterator<Item = u16> {
        self.start..=self.end
    }

    pub fn contains(&self, port: u16) -> bool {
        port >= self.start && port <= self.end
    }

    pub fn count(&self) -> usize {
        (self.end - self.start + 1) as usize
    }
}

impl FromStr for PortRange {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        if s.contains('-') {
            let parts: Vec<&str> = s.split('-').collect();
            if parts.len() != 2 {
                return Err(anyhow!("Invalid port range format. Use 'start-end'"));
            }
            let start = parts[0].parse::<u16>()?;
            let end = parts[1].parse::<u16>()?;
            PortRange::new(start, end)
        } else {
            let port = s.parse::<u16>()?;
            Ok(PortRange::single(port))
        }
    }
}

/// Protocol types for scanning
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Protocol {
    Tcp,
    Udp,
    Icmp,
}

impl Protocol {
    pub fn as_str(&self) -> &'static str {
        match self {
            Protocol::Tcp => "TCP",
            Protocol::Udp => "UDP",
            Protocol::Icmp => "ICMP",
        }
    }

    pub fn to_protocol_number(&self) -> u8 {
        match self {
            Protocol::Tcp => 6,
            Protocol::Udp => 17,
            Protocol::Icmp => 1,
        }
    }
}

impl FromStr for Protocol {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "tcp" => Ok(Protocol::Tcp),
            "udp" => Ok(Protocol::Udp),
            "icmp" => Ok(Protocol::Icmp),
            _ => Err(anyhow!("Unknown protocol: {}", s)),
        }
    }
}

/// Host target for scanning
#[derive(Debug, Clone)]
pub struct HostTarget {
    pub addr: IpAddr,
    pub hostname: Option<String>,
}

impl HostTarget {
    pub fn new(addr: IpAddr) -> Self {
        HostTarget {
            addr,
            hostname: None,
        }
    }

    pub fn with_hostname(addr: IpAddr, hostname: String) -> Self {
        HostTarget {
            addr,
            hostname: Some(hostname),
        }
    }

    pub fn display_name(&self) -> String {
        if let Some(ref hostname) = self.hostname {
            format!("{} ({})", hostname, self.addr)
        } else {
            self.addr.to_string()
        }
    }
}

/// Parse target hosts from string (supports IP, hostname, CIDR)
pub async fn parse_targets(target: &str) -> Result<Vec<HostTarget>> {
    let mut targets = Vec::new();

    if target.contains('/') {
        // CIDR notation
        targets.extend(parse_cidr_range(target)?);
    } else if target.contains('-') {
        // IP range notation like 192.168.1.1-192.168.1.10
        targets.extend(parse_ip_range(target)?);
    } else {
        // Single host (IP or hostname)
        if let Ok(addr) = target.parse::<IpAddr>() {
            targets.push(HostTarget::new(addr));
        } else {
            // Try to resolve hostname
            match dns_lookup::lookup_host(target) {
                Ok(ips) => {
                    for ip in ips {
                        targets.push(HostTarget::with_hostname(ip, target.to_string()));
                    }
                }
                Err(e) => {
                    return Err(anyhow!("Failed to resolve hostname '{}': {}", target, e));
                }
            }
        }
    }

    Ok(targets)
}

/// Parse CIDR range into individual hosts
fn parse_cidr_range(cidr: &str) -> Result<Vec<HostTarget>> {
    let parts: Vec<&str> = cidr.split('/').collect();
    if parts.len() != 2 {
        return Err(anyhow!("Invalid CIDR format"));
    }

    let base_ip: Ipv4Addr = parts[0].parse()?;
    let prefix_len: u8 = parts[1].parse()?;

    if prefix_len > 32 {
        return Err(anyhow!("Invalid CIDR prefix length"));
    }

    // Calculate network mask
    let mask = !((1u32 << (32 - prefix_len)) - 1);
    let network = u32::from(base_ip) & mask;
    let broadcast = network | !mask;

    let mut targets = Vec::new();
    for ip_u32 in (network + 1)..broadcast {
        let ip = Ipv4Addr::from(ip_u32);
        targets.push(HostTarget::new(IpAddr::V4(ip)));
    }

    Ok(targets)
}

/// Parse IP range like 192.168.1.1-192.168.1.10
fn parse_ip_range(range: &str) -> Result<Vec<HostTarget>> {
    let parts: Vec<&str> = range.split('-').collect();
    if parts.len() != 2 {
        return Err(anyhow!("Invalid IP range format"));
    }

    let start_ip: Ipv4Addr = parts[0].parse()?;
    let end_ip: Ipv4Addr = parts[1].parse()?;

    let start_u32 = u32::from(start_ip);
    let end_u32 = u32::from(end_ip);

    if start_u32 > end_u32 {
        return Err(anyhow!("Start IP cannot be greater than end IP"));
    }

    let mut targets = Vec::new();
    for ip_u32 in start_u32..=end_u32 {
        let ip = Ipv4Addr::from(ip_u32);
        targets.push(HostTarget::new(IpAddr::V4(ip)));
    }

    Ok(targets)
}

/// Common well-known ports
pub fn get_common_ports() -> Vec<u16> {
    vec![
        21, 22, 23, 25, 53, 80, 110, 111, 135, 139, 143, 443, 993, 995, 1723, 3306, 3389, 5432,
        5900, 8080,
    ]
}

/// Get service name for a port number
pub fn get_service_name(port: u16, protocol: Protocol) -> Option<&'static str> {
    match (protocol, port) {
        (Protocol::Tcp, 21) => Some("ftp"),
        (Protocol::Tcp, 22) => Some("ssh"),
        (Protocol::Tcp, 23) => Some("telnet"),
        (Protocol::Tcp, 25) => Some("smtp"),
        (Protocol::Tcp | Protocol::Udp, 53) => Some("dns"),
        (Protocol::Tcp, 80) => Some("http"),
        (Protocol::Tcp, 110) => Some("pop3"),
        (Protocol::Tcp | Protocol::Udp, 111) => Some("rpc"),
        (Protocol::Tcp, 135) => Some("msrpc"),
        (Protocol::Tcp | Protocol::Udp, 139) => Some("netbios"),
        (Protocol::Tcp, 143) => Some("imap"),
        (Protocol::Tcp, 443) => Some("https"),
        (Protocol::Tcp, 993) => Some("imaps"),
        (Protocol::Tcp, 995) => Some("pop3s"),
        (Protocol::Tcp, 1723) => Some("pptp"),
        (Protocol::Tcp, 3306) => Some("mysql"),
        (Protocol::Tcp, 3389) => Some("rdp"),
        (Protocol::Tcp, 5432) => Some("postgresql"),
        (Protocol::Tcp, 5900) => Some("vnc"),
        (Protocol::Tcp, 8080) => Some("http-alt"),
        _ => None,
    }
}

/// Validate if an IP address is in a private range
pub fn is_private_ip(ip: &IpAddr) -> bool {
    match ip {
        IpAddr::V4(ipv4) => {
            let octets = ipv4.octets();
            // 10.0.0.0/8
            octets[0] == 10
            // 172.16.0.0/12
            || (octets[0] == 172 && octets[1] >= 16 && octets[1] <= 31)
            // 192.168.0.0/16
            || (octets[0] == 192 && octets[1] == 168)
            // 127.0.0.0/8 (loopback)
            || octets[0] == 127
        }
        IpAddr::V6(ipv6) => {
            // ::1 (loopback)
            *ipv6 == Ipv6Addr::LOCALHOST
            // fc00::/7 (unique local addresses)
            || (ipv6.segments()[0] & 0xfe00) == 0xfc00
            // fe80::/10 (link-local)
            || (ipv6.segments()[0] & 0xffc0) == 0xfe80
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_port_range() {
        let range = PortRange::new(80, 443).unwrap();
        assert_eq!(range.count(), 364);
        assert!(range.contains(80));
        assert!(range.contains(443));
        assert!(!range.contains(79));
        assert!(!range.contains(444));
    }

    #[test]
    fn test_port_range_parsing() {
        assert_eq!(PortRange::from_str("80").unwrap().count(), 1);
        assert_eq!(PortRange::from_str("80-443").unwrap().count(), 364);
        assert!(PortRange::from_str("443-80").is_err());
    }

    #[test]
    fn test_protocol_parsing() {
        assert_eq!(Protocol::from_str("tcp").unwrap(), Protocol::Tcp);
        assert_eq!(Protocol::from_str("TCP").unwrap(), Protocol::Tcp);
        assert!(Protocol::from_str("invalid").is_err());
    }

    #[test]
    fn test_private_ip() {
        assert!(is_private_ip(&"192.168.1.1".parse().unwrap()));
        assert!(is_private_ip(&"10.0.0.1".parse().unwrap()));
        assert!(is_private_ip(&"172.16.0.1".parse().unwrap()));
        assert!(!is_private_ip(&"8.8.8.8".parse().unwrap()));
    }
}