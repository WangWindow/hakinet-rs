use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};
use colored::*;
use log::info;
use hakinet_common::{print_cat_banner, print_cat_working, print_cat_done, print_cat_error};

mod scanner;
mod discovery;
mod service;

use scanner::PortScanner;

#[derive(Parser)]
#[command(name = "hakinet-scan")]
#[command(about = "A network scanning tool with nmap-like functionality and a cute cat mascot 🐱")]
#[command(version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Scan ports on target hosts
    Scan {
        /// Target hosts (IP, hostname, CIDR, or range)
        #[arg(value_name = "TARGETS")]
        targets: Vec<String>,

        /// Port specification (e.g., 80, 1-1000, 80,443,8080)
        #[arg(short, long, default_value = "1-1000")]
        ports: String,

        /// Scan technique
        #[arg(short = 's', long, default_value = "syn")]
        scan_type: ScanType,

        /// Maximum number of parallel scans
        #[arg(long, default_value = "100")]
        max_parallel: usize,

        /// Connection timeout in seconds
        #[arg(short, long, default_value = "3")]
        timeout: u64,

        /// Output format
        #[arg(short, long, default_value = "human")]
        output: OutputFormat,

        /// Output file
        #[arg(short, long)]
        file: Option<String>,

        /// Enable service detection
        #[arg(long)]
        service_detection: bool,

        /// Enable OS detection
        #[arg(long)]
        os_detection: bool,

        /// Randomize scan order
        #[arg(long)]
        randomize: bool,

        /// Enable verbose output
        #[arg(short, long)]
        verbose: bool,
    },
    /// Discover hosts on network
    Discovery {
        /// Target network (CIDR notation)
        #[arg(value_name = "NETWORK")]
        network: String,

        /// Discovery method
        #[arg(short, long, default_value = "ping")]
        method: DiscoveryMethod,

        /// Maximum number of parallel scans
        #[arg(long, default_value = "50")]
        max_parallel: usize,

        /// Timeout in seconds
        #[arg(short, long, default_value = "2")]
        timeout: u64,

        /// Output format
        #[arg(short, long, default_value = "human")]
        output: OutputFormat,

        /// Output file
        #[arg(short, long)]
        file: Option<String>,

        /// Enable verbose output
        #[arg(short, long)]
        verbose: bool,
    },
}

#[derive(ValueEnum, Clone, Debug)]
enum ScanType {
    /// TCP SYN scan (stealth scan)
    Syn,
    /// TCP connect scan
    Connect,
    /// UDP scan
    Udp,
    /// Comprehensive scan (TCP + UDP)
    Comprehensive,
}

#[derive(ValueEnum, Clone, Debug)]
enum DiscoveryMethod {
    /// ICMP ping
    Ping,
    /// TCP SYN ping
    TcpSyn,
    /// ARP ping (local network)
    Arp,
}

#[derive(ValueEnum, Clone, Debug)]
enum OutputFormat {
    /// Human-readable output
    Human,
    /// JSON format
    Json,
    /// XML format
    Xml,
    /// CSV format
    Csv,
}

impl From<OutputFormat> for hakinet_common::output::OutputFormat {
    fn from(format: OutputFormat) -> Self {
        match format {
            OutputFormat::Human => hakinet_common::output::OutputFormat::Human,
            OutputFormat::Json => hakinet_common::output::OutputFormat::Json,
            OutputFormat::Xml => hakinet_common::output::OutputFormat::Xml,
            OutputFormat::Csv => hakinet_common::output::OutputFormat::Csv,
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize logger
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();

    print_cat_banner("Hakinet-Scan", "Your cute network scanning cat");

    match cli.command {
        Commands::Scan {
            targets,
            ports,
            scan_type,
            max_parallel,
            timeout,
            output,
            file,
            service_detection,
            os_detection,
            randomize,
            verbose,
        } => {
            if verbose {
                env_logger::Builder::from_default_env()
                    .filter_level(log::LevelFilter::Debug)
                    .init();
            }

            if targets.is_empty() {
                print_cat_error("No targets specified!");
                std::process::exit(1);
            }

            info!("Starting port scan on targets: {:?}", targets);
            print_cat_working("Scanning ports like a ninja cat...");

            let scanner = PortScanner::new()
                .with_max_parallel(max_parallel)
                .with_timeout(std::time::Duration::from_secs(timeout))
                .with_randomize(randomize)
                .with_service_detection(service_detection)
                .with_os_detection(os_detection);

            let results = match scan_type {
                ScanType::Syn => scanner.syn_scan(targets, ports).await?,
                ScanType::Connect => scanner.connect_scan(targets, ports).await?,
                ScanType::Udp => scanner.udp_scan(targets, ports).await?,
                ScanType::Comprehensive => scanner.comprehensive_scan(targets, ports).await?,
            };

            let output_writer = hakinet_common::output::ScanOutputWriter::new(output.into(), file);
            output_writer.write_results(&results).await?;

            print_cat_done("Port scanning complete!");
            println!(
                "{}",
                format!(
                    "Found {} open ports on {} hosts!",
                    results.open_ports_found,
                    results.hosts_up
                )
                .bright_green()
            );
        }
        Commands::Discovery {
            network,
            method,
            max_parallel,
            timeout,
            output,
            file,
            verbose,
        } => {
            if verbose {
                env_logger::Builder::from_default_env()
                    .filter_level(log::LevelFilter::Debug)
                    .init();
            }

            info!("Starting host discovery on network: {}", network);
            print_cat_working("Discovering hosts like a detective cat...");

            let discoverer = discovery::HostDiscoverer::new()
                .with_max_parallel(max_parallel)
                .with_timeout(std::time::Duration::from_secs(timeout));

            let results = match method {
                DiscoveryMethod::Ping => discoverer.ping_discovery(&network).await?,
                DiscoveryMethod::TcpSyn => discoverer.tcp_syn_discovery(&network).await?,
                DiscoveryMethod::Arp => discoverer.arp_discovery(&network).await?,
            };

            let output_writer = hakinet_common::output::ScanOutputWriter::new(output.into(), file);
            output_writer.write_results(&results).await?;

            print_cat_done("Host discovery complete!");
            println!(
                "{}",
                format!("Discovered {} live hosts!", results.hosts_up).bright_green()
            );
        }
    }

    println!("{}", "Thanks for using Hakinet-Scan! 🐾".bright_magenta());
    Ok(())
}