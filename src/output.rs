use anyhow::Result;
use log::info;
use serde_json;
use tokio::fs::OpenOptions;
use tokio::io::{AsyncWriteExt, BufWriter};

use crate::packet::PacketInfo;

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

                // Write JSON array opening bracket
                buf_writer.write_all(b"[\n").await?;

                self.writer = Some(buf_writer);
                info!("Created output file: {}", output_file);
            }

            if let Some(ref mut writer) = self.writer {
                // Add comma separator for subsequent packets
                if self.packet_count > 0 {
                    writer.write_all(b",\n").await?;
                }

                // Write packet as JSON
                let json = serde_json::to_string_pretty(packet)?;
                writer.write_all(json.as_bytes()).await?;

                self.packet_count += 1;

                // Flush periodically to ensure data is written
                if self.packet_count % 10 == 0 {
                    writer.flush().await?;
                }
            }
        }

        Ok(())
    }

    pub async fn close(&mut self) -> Result<()> {
        if let Some(ref mut writer) = self.writer {
            // Close JSON array
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
