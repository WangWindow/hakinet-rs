use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::*;
use log::info;

mod capture;
mod filter;
mod output;
mod packet;

use capture::PacketCapture;

#[derive(Parser)]
#[command(name = "hakinet")]
#[command(about = "A network packet capture tool with a cute cat mascot 🐱")]
#[command(version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start packet capture
    Capture {
        /// Network interface to capture from
        #[arg(short, long, default_value = "any")]
        interface: String,

        /// Number of packets to capture (0 = unlimited)
        #[arg(short, long, default_value = "0")]
        count: usize,

        /// Packet filter expression (BPF syntax)
        #[arg(short, long)]
        filter: Option<String>,

        /// Output file (JSON format)
        #[arg(short, long)]
        output: Option<String>,

        /// Enable verbose output
        #[arg(short, long)]
        verbose: bool,
    },
    /// List available network interfaces
    Interfaces,
}

fn print_cat_banner() {
    println!(
        "{}",
        "
    ╭─────────────────────────────────────────╮
    │                                         │
    │        🐱 Welcome to Hakinet! 🐱        │
    │     Your cute network sniffer cat       │
    │                                         │
    │      /\\_/\\    Meow! Let's catch some    │
    │     ( o.o )   packets together! 📦      │
    │      > ^ <                              │
    │                                         │
    ╰─────────────────────────────────────────╯
    "
        .bright_cyan()
    );
}

fn print_cat_working() {
    println!(
        "{}",
        "
    🐱 Hakinet is hunting for packets...
       /\\_/\\
      ( ^.^ ) *sniff sniff*
       > ^ <
    "
        .bright_green()
    );
}

fn print_cat_done() {
    println!(
        "{}",
        "
    🐱 Packet hunting complete!
       /\\_/\\
      ( -.- ) *yawn*
       > ^ <
    "
        .bright_yellow()
    );
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize logger
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();

    print_cat_banner();

    match cli.command {
        Commands::Capture {
            interface,
            count,
            filter,
            output,
            verbose,
        } => {
            if verbose {
                env_logger::Builder::from_default_env()
                    .filter_level(log::LevelFilter::Debug)
                    .init();
            }

            info!("Starting packet capture on interface: {}", interface);
            print_cat_working();

            let mut capture = PacketCapture::new(&interface)?;

            if let Some(filter_expr) = filter {
                capture.set_filter(&filter_expr)?;
                info!("Applied filter: {}", filter_expr);
            }

            capture.start_capture(count, output).await?;

            print_cat_done();
            println!("{}", "Thanks for using Hakinet! 🐾".bright_magenta());
        }
        Commands::Interfaces => {
            println!("{}", "Available network interfaces:".bright_blue().bold());
            capture::list_interfaces()?;
        }
    }

    Ok(())
}
