use anyhow::{anyhow, Result};
use colored::*;
use log::{debug, info, warn};
use pcap::{Active, Capture, Device};
use pnet::packet::ethernet::{EtherTypes, EthernetPacket};
use pnet::packet::icmp::IcmpPacket;
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::ipv6::Ipv6Packet;
use pnet::packet::tcp::TcpPacket;
use pnet::packet::udp::UdpPacket;
use pnet::packet::Packet;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::output::OutputWriter;
use hakinet_common::PacketInfo;

pub struct PacketCapture {
    capture: Capture<Active>,
    interface_name: String,
}

impl PacketCapture {
    pub fn new(interface_name: &str) -> Result<Self> {
        let device = if interface_name == "any" {
            Device::lookup()?.ok_or_else(|| anyhow!("No default device found"))?
        } else {
            Device::list()?
                .into_iter()
                .find(|d| d.name == interface_name)
                .ok_or_else(|| anyhow!("Interface '{}' not found", interface_name))?
        };

        info!(
            "Opening device: {} ({})",
            device.name,
            device.desc.as_deref().unwrap_or("No description")
        );

        let capture = Capture::from_device(device)?
            .promisc(true)
            .snaplen(65535)
            .timeout(1000)
            .open()?;

        Ok(PacketCapture {
            capture,
            interface_name: interface_name.to_string(),
        })
    }

    pub fn set_filter(&mut self, filter: &str) -> Result<()> {
        self.capture.filter(filter, true)?;
        Ok(())
    }

    pub async fn start_capture(&mut self, count: usize, output_file: Option<String>) -> Result<()> {
        let mut output_writer = OutputWriter::new(output_file)?;
        let mut packet_count = 0;
        let unlimited = count == 0;

        println!(
            "{}",
            format!("🔍 Capturing packets on interface: {}", self.interface_name).bright_green()
        );
        if !unlimited {
            println!(
                "{}",
                format!("📊 Will capture {} packets", count).bright_blue()
            );
        } else {
            println!(
                "{}",
                "📊 Capturing unlimited packets (Ctrl+C to stop)".bright_blue()
            );
        }
        println!();

        loop {
            if !unlimited && packet_count >= count {
                break;
            }

            match self.capture.next_packet() {
                Ok(packet) => {
                    packet_count += 1;

                    let timestamp = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_secs();

                    let packet_data = packet.data.to_vec();
                    let packet_info = self.parse_packet(&packet_data, timestamp);

                    // Print packet info to console
                    self.print_packet_info(&packet_info, packet_count);

                    // Write to output file if specified
                    output_writer.write_packet(&packet_info).await?;

                    // Small delay to make output readable
                    tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
                }
                Err(pcap::Error::TimeoutExpired) => {
                    // Timeout is normal, continue
                    continue;
                }
                Err(e) => {
                    warn!("Error capturing packet: {}", e);
                    continue;
                }
            }
        }

        output_writer.close().await?;

        println!();
        println!(
            "{}",
            format!("🎉 Captured {} packets successfully!", packet_count)
                .bright_green()
                .bold()
        );

        Ok(())
    }

    fn parse_packet(&self, data: &[u8], timestamp: u64) -> PacketInfo {
        let mut packet_info = PacketInfo {
            timestamp,
            length: data.len(),
            protocol: "Unknown".to_string(),
            src_addr: None,
            dst_addr: None,
            src_port: None,
            dst_port: None,
            info: None,
        };

        if let Some(ethernet) = EthernetPacket::new(data) {
            debug!(
                "Ethernet packet: {} -> {}",
                ethernet.get_source(),
                ethernet.get_destination()
            );

            match ethernet.get_ethertype() {
                EtherTypes::Ipv4 => {
                    if let Some(ipv4) = Ipv4Packet::new(ethernet.payload()) {
                        packet_info.src_addr = Some(ipv4.get_source().to_string());
                        packet_info.dst_addr = Some(ipv4.get_destination().to_string());

                        match ipv4.get_next_level_protocol() {
                            pnet::packet::ip::IpNextHeaderProtocols::Tcp => {
                                packet_info.protocol = "TCP".to_string();
                                if let Some(tcp) = TcpPacket::new(ipv4.payload()) {
                                    packet_info.src_port = Some(tcp.get_source());
                                    packet_info.dst_port = Some(tcp.get_destination());
                                    packet_info.info =
                                        Some(format!("Flags: {:?}", tcp.get_flags()));
                                }
                            }
                            pnet::packet::ip::IpNextHeaderProtocols::Udp => {
                                packet_info.protocol = "UDP".to_string();
                                if let Some(udp) = UdpPacket::new(ipv4.payload()) {
                                    packet_info.src_port = Some(udp.get_source());
                                    packet_info.dst_port = Some(udp.get_destination());
                                }
                            }
                            pnet::packet::ip::IpNextHeaderProtocols::Icmp => {
                                packet_info.protocol = "ICMP".to_string();
                                if let Some(icmp) = IcmpPacket::new(ipv4.payload()) {
                                    packet_info.info = Some(format!(
                                        "Type: {:?}, Code: {:?}",
                                        icmp.get_icmp_type(),
                                        icmp.get_icmp_code()
                                    ));
                                }
                            }
                            _ => {
                                packet_info.protocol =
                                    format!("IPv4 ({})", ipv4.get_next_level_protocol());
                            }
                        }
                    }
                }
                EtherTypes::Ipv6 => {
                    if let Some(ipv6) = Ipv6Packet::new(ethernet.payload()) {
                        packet_info.protocol = "IPv6".to_string();
                        packet_info.src_addr = Some(ipv6.get_source().to_string());
                        packet_info.dst_addr = Some(ipv6.get_destination().to_string());
                    }
                }
                EtherTypes::Arp => {
                    packet_info.protocol = "ARP".to_string();
                }
                _ => {
                    packet_info.protocol = format!("Ethernet ({})", ethernet.get_ethertype());
                }
            }
        }

        packet_info
    }

    fn print_packet_info(&self, packet: &PacketInfo, count: usize) {
        let timestamp_str = chrono::DateTime::from_timestamp(packet.timestamp as i64, 0)
            .map(|dt| dt.format("%H:%M:%S").to_string())
            .unwrap_or_else(|| "Unknown".to_string());

        let protocol_colored = match packet.protocol.as_str() {
            "TCP" => packet.protocol.bright_red(),
            "UDP" => packet.protocol.bright_blue(),
            "ICMP" => packet.protocol.bright_yellow(),
            "ARP" => packet.protocol.bright_green(),
            _ => packet.protocol.normal(),
        };

        let src_dst = if let (Some(src), Some(dst)) = (&packet.src_addr, &packet.dst_addr) {
            if let (Some(src_port), Some(dst_port)) = (packet.src_port, packet.dst_port) {
                format!("{}:{} → {}:{}", src, src_port, dst, dst_port)
            } else {
                format!("{} → {}", src, dst)
            }
        } else {
            "Unknown".to_string()
        };

        print!("{} ", format!("[{}]", count).bright_cyan());
        print!("{} ", timestamp_str.bright_black());
        print!("{:>8} ", protocol_colored);
        print!("{:>6} bytes ", packet.length.to_string().bright_magenta());
        print!("{}", src_dst.bright_white());

        if let Some(info) = &packet.info {
            print!(" {}", info.bright_black());
        }

        println!();
    }
}

pub fn list_interfaces() -> Result<()> {
    let devices = Device::list()?;

    if devices.is_empty() {
        println!("{}", "No network interfaces found!".bright_red());
        return Ok(());
    }

    for (i, device) in devices.iter().enumerate() {
        let status = if device.flags.is_up() {
            "UP".bright_green()
        } else {
            "DOWN".bright_red()
        };

        println!(
            "{} {}: {} [{}]",
            "🔌".to_string(),
            (i + 1).to_string().bright_cyan(),
            device.name.bright_white().bold(),
            status
        );

        if let Some(desc) = &device.desc {
            println!("   📝 {}", desc.bright_black());
        }

        for addr in &device.addresses {
            println!("   🌐 {}", addr.addr.to_string().bright_blue());
        }
        println!();
    }

    Ok(())
}
