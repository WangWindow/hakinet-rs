use anyhow::Result;
use colored::*;
use log::info;
use serde_json;
use std::io::{self, Write};
use tokio::fs::OpenOptions;
use tokio::io::{AsyncWriteExt, BufWriter};

use crate::types::{PacketInfo, PortInfo, ScanResults};

pub struct OutputWriter {
    writer: Option<BufWriter<tokio::fs::File>>,
    output_file: Option<String>,
    packet_count: usize,
}

impl OutputWriter {
    pub fn new(output_file: Option<String>) -> Result<Self> {
        Ok(OutputWriter {
            writer: None,
            output_file,
            packet_count: 0,
        })
    }

    pub async fn write_packet(&mut self, packet: &PacketInfo) -> Result<()> {
        if let Some(output_file) = &self.output_file {
            if self.writer.is_none() {
                let file = OpenOptions::new()
                    .create(true)
                    .write(true)
                    .truncate(true)
                    .open(output_file)
                    .await?;

                let mut buf_writer = BufWriter::new(file);
                buf_writer.write_all(b"[\n").await?;
                self.writer = Some(buf_writer);
                info!("Created output file: {}", output_file);
            }

            if let Some(ref mut writer) = self.writer {
                if self.packet_count > 0 {
                    writer.write_all(b",\n").await?;
                }

                let json = serde_json::to_string_pretty(packet)?;
                writer.write_all(json.as_bytes()).await?;
                self.packet_count += 1;

                if self.packet_count % 10 == 0 {
                    writer.flush().await?;
                }
            }
        }
        Ok(())
    }

    pub async fn close(&mut self) -> Result<()> {
        if let Some(ref mut writer) = self.writer {
            writer.write_all(b"\n]\n").await?;
            writer.flush().await?;

            if let Some(output_file) = &self.output_file {
                info!("Saved {} packets to: {}", self.packet_count, output_file);
                println!("📁 Output saved to: {}", output_file);
            }
        }
        Ok(())
    }
}

pub struct ScanOutputWriter {
    format: OutputFormat,
    file: Option<String>,
}

#[derive(Debug, Clone)]
pub enum OutputFormat {
    Human,
    Json,
    Xml,
    Csv,
}

impl ScanOutputWriter {
    pub fn new(format: OutputFormat, file: Option<String>) -> Self {
        ScanOutputWriter { format, file }
    }

    pub async fn write_results(&self, results: &ScanResults) -> Result<()> {
        let output = match self.format {
            OutputFormat::Human => self.format_human(results),
            OutputFormat::Json => self.format_json(results)?,
            OutputFormat::Xml => self.format_xml(results),
            OutputFormat::Csv => self.format_csv(results),
        };

        if let Some(ref file_path) = self.file {
            tokio::fs::write(file_path, output).await?;
            println!("📁 Results saved to: {}", file_path);
        } else {
            print!("{}", output);
        }

        Ok(())
    }

    fn format_human(&self, results: &ScanResults) -> String {
        let mut output = String::new();
        
        output.push_str(&format!("\n{}\n", "🎯 Scan Results Summary".bright_green().bold()));
        output.push_str(&format!("Duration: {} seconds\n", results.duration()));
        output.push_str(&format!("Total hosts: {}\n", results.total_hosts));
        output.push_str(&format!("Hosts up: {}\n", results.hosts_up));
        output.push_str(&format!("Total ports scanned: {}\n", results.total_ports_scanned));
        output.push_str(&format!("Open ports found: {}\n\n", results.open_ports_found));

        for host in &results.hosts {
            if !host.is_up {
                continue;
            }

            output.push_str(&format!("{}\n", format!("📡 Host: {}", host.display_name()).bright_cyan().bold()));
            
            if let Some(response_time) = host.response_time {
                output.push_str(&format!("Response time: {}μs\n", response_time));
            }

            let open_ports = host.open_ports();
            if open_ports.is_empty() {
                output.push_str("No open ports found\n\n");
                continue;
            }

            output.push_str(&format!("Open ports ({}):\n", open_ports.len()));
            
            for port in open_ports {
                let service_info = if let Some(ref service) = port.service {
                    if let Some(ref version) = port.version {
                        format!(" ({})", format!("{} {}", service, version).bright_black())
                    } else {
                        format!(" ({})", service.bright_black())
                    }
                } else {
                    String::new()
                };

                let response_time = if let Some(time) = port.response_time {
                    format!(" [{}μs]", time.to_string().bright_black())
                } else {
                    String::new()
                };

                output.push_str(&format!(
                    "  {} {}/{} {}{}{}\n",
                    "•".bright_green(),
                    port.port.to_string().bright_white().bold(),
                    port.protocol.bright_blue(),
                    port.state.as_str().bright_green(),
                    service_info,
                    response_time
                ));
            }
            output.push('\n');
        }

        output
    }

    fn format_json(&self, results: &ScanResults) -> Result<String> {
        Ok(serde_json::to_string_pretty(results)?)
    }

    fn format_xml(&self, results: &ScanResults) -> String {
        let mut xml = String::new();
        xml.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
        xml.push_str("<scan_results>\n");
        xml.push_str(&format!("  <summary>\n"));
        xml.push_str(&format!("    <duration>{}</duration>\n", results.duration()));
        xml.push_str(&format!("    <total_hosts>{}</total_hosts>\n", results.total_hosts));
        xml.push_str(&format!("    <hosts_up>{}</hosts_up>\n", results.hosts_up));
        xml.push_str(&format!("    <total_ports_scanned>{}</total_ports_scanned>\n", results.total_ports_scanned));
        xml.push_str(&format!("    <open_ports_found>{}</open_ports_found>\n", results.open_ports_found));
        xml.push_str("  </summary>\n");
        
        xml.push_str("  <hosts>\n");
        for host in &results.hosts {
            xml.push_str(&format!("    <host ip=\"{}\" up=\"{}\">\n", host.addr, host.is_up));
            if let Some(ref hostname) = host.hostname {
                xml.push_str(&format!("      <hostname>{}</hostname>\n", hostname));
            }
            
            xml.push_str("      <ports>\n");
            for port in &host.ports {
                xml.push_str(&format!(
                    "        <port number=\"{}\" protocol=\"{}\" state=\"{}\"\n",
                    port.port, port.protocol, port.state.as_str()
                ));
                if let Some(ref service) = port.service {
                    xml.push_str(&format!(" service=\"{}\"", service));
                }
                xml.push_str("/>\n");
            }
            xml.push_str("      </ports>\n");
            xml.push_str("    </host>\n");
        }
        xml.push_str("  </hosts>\n");
        xml.push_str("</scan_results>\n");
        
        xml
    }

    fn format_csv(&self, results: &ScanResults) -> String {
        let mut csv = String::new();
        csv.push_str("host,hostname,port,protocol,state,service,response_time\n");
        
        for host in &results.hosts {
            for port in &host.ports {
                csv.push_str(&format!(
                    "{},{},{},{},{},{},{}\n",
                    host.addr,
                    host.hostname.as_deref().unwrap_or(""),
                    port.port,
                    port.protocol,
                    port.state.as_str(),
                    port.service.as_deref().unwrap_or(""),
                    port.response_time.map(|t| t.to_string()).unwrap_or_default()
                ));
            }
        }
        
        csv
    }
}

pub fn print_scan_progress(current: usize, total: usize, target: &str) {
    let percentage = (current as f32 / total as f32 * 100.0) as u32;
    let bar_width = 30;
    let filled = (current * bar_width / total).min(bar_width);
    
    let bar = format!(
        "[{}{}]",
        "█".repeat(filled).bright_green(),
        "░".repeat(bar_width - filled).bright_black()
    );
    
    print!(
        "\r🔍 Scanning {}: {} {}/{}% ({}/{})",
        target.bright_cyan(),
        bar,
        percentage,
        "%".bright_white(),
        current.to_string().bright_white(),
        total.to_string().bright_white()
    );
    io::stdout().flush().unwrap();
}

pub fn print_port_result(host: &str, port: &PortInfo) {
    let state_color = match port.state {
        crate::types::PortState::Open => port.state.as_str().bright_green(),
        crate::types::PortState::Closed => port.state.as_str().bright_red(),
        crate::types::PortState::Filtered => port.state.as_str().bright_yellow(),
        _ => port.state.as_str().normal(),
    };

    let service_info = if let Some(ref service) = port.service {
        format!(" ({})", service.bright_blue())
    } else {
        String::new()
    };

    println!(
        "{} {}:{}/{} {}{}",
        "🎯".to_string(),
        host.bright_cyan(),
        port.port.to_string().bright_white(),
        port.protocol.bright_magenta(),
        state_color,
        service_info
    );
}