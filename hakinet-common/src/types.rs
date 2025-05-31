use serde::{Deserialize, Serialize};
use std::net::IpAddr;

/// Packet information structure for capture and analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PacketInfo {
    pub timestamp: u64,
    pub length: usize,
    pub protocol: String,
    pub src_addr: Option<String>,
    pub dst_addr: Option<String>,
    pub src_port: Option<u16>,
    pub dst_port: Option<u16>,
    pub info: Option<String>,
}

impl PacketInfo {
    pub fn new() -> Self {
        PacketInfo {
            timestamp: 0,
            length: 0,
            protocol: "Unknown".to_string(),
            src_addr: None,
            dst_addr: None,
            src_port: None,
            dst_port: None,
            info: None,
        }
    }
}

impl Default for PacketInfo {
    fn default() -> Self {
        Self::new()
    }
}

/// Port state from scanning
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PortState {
    Open,
    Closed,
    Filtered,
    OpenFiltered,
    ClosedFiltered,
    Unknown,
}

impl PortState {
    pub fn as_str(&self) -> &'static str {
        match self {
            PortState::Open => "open",
            PortState::Closed => "closed",
            PortState::Filtered => "filtered",
            PortState::OpenFiltered => "open|filtered",
            PortState::ClosedFiltered => "closed|filtered",
            PortState::Unknown => "unknown",
        }
    }

    pub fn is_open(&self) -> bool {
        matches!(self, PortState::Open | PortState::OpenFiltered)
    }
}

/// Information about a scanned port
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortInfo {
    pub port: u16,
    pub protocol: String,
    pub state: PortState,
    pub service: Option<String>,
    pub version: Option<String>,
    pub response_time: Option<u64>, // in microseconds
}

impl PortInfo {
    pub fn new(port: u16, protocol: String, state: PortState) -> Self {
        PortInfo {
            port,
            protocol,
            state,
            service: None,
            version: None,
            response_time: None,
        }
    }

    pub fn with_service(mut self, service: String) -> Self {
        self.service = Some(service);
        self
    }

    pub fn with_version(mut self, version: String) -> Self {
        self.version = Some(version);
        self
    }

    pub fn with_response_time(mut self, response_time: u64) -> Self {
        self.response_time = Some(response_time);
        self
    }
}

/// Host information from scanning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HostInfo {
    pub addr: IpAddr,
    pub hostname: Option<String>,
    pub is_up: bool,
    pub ports: Vec<PortInfo>,
    pub os_info: Option<String>,
    pub scan_time: u64, // timestamp
    pub response_time: Option<u64>, // ping response time in microseconds
}

impl HostInfo {
    pub fn new(addr: IpAddr) -> Self {
        HostInfo {
            addr,
            hostname: None,
            is_up: false,
            ports: Vec::new(),
            os_info: None,
            scan_time: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            response_time: None,
        }
    }

    pub fn with_hostname(mut self, hostname: String) -> Self {
        self.hostname = Some(hostname);
        self
    }

    pub fn set_up(mut self, is_up: bool) -> Self {
        self.is_up = is_up;
        self
    }

    pub fn add_port(mut self, port_info: PortInfo) -> Self {
        self.ports.push(port_info);
        self
    }

    pub fn open_ports(&self) -> Vec<&PortInfo> {
        self.ports.iter().filter(|p| p.state.is_open()).collect()
    }

    pub fn display_name(&self) -> String {
        if let Some(ref hostname) = self.hostname {
            format!("{} ({})", hostname, self.addr)
        } else {
            self.addr.to_string()
        }
    }
}

/// Scan configuration
#[derive(Debug, Clone)]
pub struct ScanConfig {
    pub timeout: std::time::Duration,
    pub max_parallel: usize,
    pub delay: std::time::Duration,
    pub retries: usize,
    pub randomize: bool,
}

impl Default for ScanConfig {
    fn default() -> Self {
        ScanConfig {
            timeout: std::time::Duration::from_secs(3),
            max_parallel: 100,
            delay: std::time::Duration::from_millis(0),
            retries: 1,
            randomize: false,
        }
    }
}

/// Scan results summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanResults {
    pub hosts: Vec<HostInfo>,
    pub start_time: u64,
    pub end_time: u64,
    pub total_hosts: usize,
    pub hosts_up: usize,
    pub total_ports_scanned: usize,
    pub open_ports_found: usize,
}

impl ScanResults {
    pub fn new() -> Self {
        ScanResults {
            hosts: Vec::new(),
            start_time: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            end_time: 0,
            total_hosts: 0,
            hosts_up: 0,
            total_ports_scanned: 0,
            open_ports_found: 0,
        }
    }

    pub fn add_host(&mut self, host: HostInfo) {
        self.total_hosts += 1;
        if host.is_up {
            self.hosts_up += 1;
        }
        self.total_ports_scanned += host.ports.len();
        self.open_ports_found += host.open_ports().len();
        self.hosts.push(host);
    }

    pub fn finalize(&mut self) {
        self.end_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }

    pub fn duration(&self) -> u64 {
        self.end_time - self.start_time
    }
}

impl Default for ScanResults {
    fn default() -> Self {
        Self::new()
    }
}