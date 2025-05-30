use serde::{Deserialize, Serialize};

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
