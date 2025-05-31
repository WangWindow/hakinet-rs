use hakinet_common::PacketInfo;
use anyhow::Result;

pub struct PacketFilter {
    filter_expr: Option<String>,
}

impl PacketFilter {
    pub fn new(filter_expr: Option<String>) -> Self {
        PacketFilter { filter_expr }
    }

    pub fn matches(&self, _packet: &PacketInfo) -> bool {
        // For now, we rely on pcap's built-in filtering
        // This could be extended for additional custom filtering
        true
    }

    pub fn is_valid_bpf_filter(filter: &str) -> Result<()> {
        // Basic validation of BPF filter syntax
        // This is a simplified check - pcap will do the real validation
        if filter.trim().is_empty() {
            return Err(anyhow::anyhow!("Filter expression cannot be empty"));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_validation() {
        assert!(PacketFilter::is_valid_bpf_filter("tcp port 80").is_ok());
        assert!(PacketFilter::is_valid_bpf_filter("udp and port 53").is_ok());
        assert!(PacketFilter::is_valid_bpf_filter("host 192.168.1.1").is_ok());
        assert!(PacketFilter::is_valid_bpf_filter("").is_err());
    }
}
