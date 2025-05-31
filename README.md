# 🐱 Hakinet Network Tools Suite

A comprehensive network toolkit with cute cat mascots! This workspace contains multiple network tools for packet capture, port scanning, and host discovery.

```
    ╭─────────────────────────────────────────╮
    │                                         │
    │        🐱 Welcome to Hakinet! 🐱        │
    │     Your cute network toolkit cats      │
    │                                         │
    │      /\_/\    Meow! Let's hunt some     │
    │     ( o.o )   network packets! 📦       │
    │      > ^ <                              │
    │                                         │
    ╰─────────────────────────────────────────╯
```

## 🚀 Tools Overview

This workspace contains three main tools:

- **🔍 hakinet** - Network packet capture tool (like tcpdump/Wireshark)
- **🎯 hakinet-scan** - Network scanning tool (like nmap)
- **📚 hakinet-common** - Shared library for common functionality

## 📦 Installation

### Prerequisites

On Linux systems, you need to install libpcap development libraries:

```bash
# Ubuntu/Debian
sudo apt-get install libpcap-dev

# CentOS/RHEL/Fedora
sudo yum install libpcap-devel
# Or for newer systems
sudo dnf install libpcap-devel

# Arch Linux
sudo pacman -S libpcap
```

### Build and Install

```bash
# Clone the project
git clone <repository-url>
cd hakinet-rs

# Build all tools
cargo build --release

# Install all tools to system path (optional)
cargo install --path hakinet
cargo install --path hakinet-scan
```

## 🔍 Hakinet - Packet Capture Tool

### Features

- 🔍 Real-time network packet capture
- 🌈 Colorful terminal output
- 📁 JSON format output files
- 🔧 BPF filter support
- 🖥️ Multiple network interface support
- 🐱 Cute cat interface

### Usage

```bash
# Show help
hakinet --help

# List available network interfaces
hakinet interfaces

# Start capturing packets (default interface, unlimited)
sudo hakinet capture

# Capture specific number of packets
sudo hakinet capture --count 100

# Capture on specific interface
sudo hakinet capture --interface eth0

# Use filter (HTTP traffic only)
sudo hakinet capture --filter "tcp port 80"

# Save to file
sudo hakinet capture --output packets.json

# Verbose output
sudo hakinet capture --verbose
```

### Filter Examples

```bash
# Capture traffic from specific host
sudo hakinet capture --filter "host 192.168.1.1"

# Capture TCP traffic
sudo hakinet capture --filter "tcp"

# Capture specific port
sudo hakinet capture --filter "port 443"

# Capture HTTP and HTTPS traffic
sudo hakinet capture --filter "tcp port 80 or tcp port 443"

# Capture DNS queries
sudo hakinet capture --filter "udp port 53"

# Exclude specific traffic
sudo hakinet capture --filter "not host 192.168.1.1"
```

## 🎯 Hakinet-Scan - Network Scanner

### Features

- 🎯 Port scanning (TCP SYN, Connect, UDP)
- 🌐 Host discovery (Ping, TCP SYN, ARP)
- 🔍 Service detection and version identification
- 📊 Multiple output formats (Human, JSON, XML, CSV)
- ⚡ Parallel scanning for speed
- 🎲 Randomized scan order option
- 🐱 Cute cat progress indicators

### Port Scanning

```bash
# Basic port scan
hakinet-scan scan 192.168.1.1

# Scan multiple hosts
hakinet-scan scan 192.168.1.1 192.168.1.100 example.com

# Scan CIDR range
hakinet-scan scan 192.168.1.0/24

# Scan IP range
hakinet-scan scan 192.168.1.1-192.168.1.50

# Specify ports
hakinet-scan scan 192.168.1.1 --ports 80,443,8080
hakinet-scan scan 192.168.1.1 --ports 1-1000
hakinet-scan scan 192.168.1.1 --ports 80-443

# Different scan types
hakinet-scan scan 192.168.1.1 --scan-type syn       # TCP SYN scan (default)
hakinet-scan scan 192.168.1.1 --scan-type connect   # TCP connect scan
hakinet-scan scan 192.168.1.1 --scan-type udp       # UDP scan
hakinet-scan scan 192.168.1.1 --scan-type comprehensive # TCP + UDP

# Advanced options
hakinet-scan scan 192.168.1.1 --service-detection   # Detect services
hakinet-scan scan 192.168.1.1 --os-detection        # OS fingerprinting
hakinet-scan scan 192.168.1.1 --randomize           # Randomize scan order
hakinet-scan scan 192.168.1.1 --max-parallel 200    # Increase parallelism
hakinet-scan scan 192.168.1.1 --timeout 5           # Set timeout

# Output formats
hakinet-scan scan 192.168.1.1 --output json --file results.json
hakinet-scan scan 192.168.1.1 --output xml --file results.xml
hakinet-scan scan 192.168.1.1 --output csv --file results.csv
```

### Host Discovery

```bash
# Ping discovery
hakinet-scan discovery 192.168.1.0/24

# TCP SYN discovery
hakinet-scan discovery 192.168.1.0/24 --method tcp-syn

# ARP discovery (local network only)
hakinet-scan discovery 192.168.1.0/24 --method arp

# Advanced discovery options
hakinet-scan discovery 192.168.1.0/24 --max-parallel 100 --timeout 3
```

### Output Examples

Human-readable output:
```
🎯 Scan Results Summary
Duration: 15 seconds
Total hosts: 254
Hosts up: 12
Total ports scanned: 1000
Open ports found: 45

📡 Host: example.com (93.184.216.34)
Response time: 125ms
Open ports (3):
  • 80/tcp open (http)
  • 443/tcp open (https)
  • 22/tcp open (ssh OpenSSH 8.0)
```

## 📚 Hakinet-Common Library

The shared library provides common functionality:

- 🌐 Network utilities (IP parsing, port ranges, CIDR handling)
- 📊 Data types (packet info, scan results, host info)
- 📄 Output formatting (JSON, XML, CSV, human-readable)
- 🛠️ Utility functions (timing, rate limiting, formatting)
- 🐱 Shared cat animations and UI elements

## 🔧 Workspace Structure

```
hakinet-rs/
├── Cargo.toml                 # Workspace root
├── hakinet/                   # Packet capture tool
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs
│       ├── capture.rs
│       ├── filter.rs
│       └── output.rs
├── hakinet-scan/              # Network scanner tool
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs
│       ├── scanner.rs
│       ├── discovery.rs
│       └── service.rs
└── hakinet-common/            # Shared library
    ├── Cargo.toml
    └── src/
        ├── lib.rs
        ├── network.rs
        ├── types.rs
        ├── output.rs
        └── utils.rs
```

## 🔐 Permissions

Both tools require elevated privileges for raw socket access:

```bash
# Run with sudo
sudo hakinet capture
sudo hakinet-scan scan 192.168.1.1

# Or set capabilities (Linux only)
sudo setcap cap_net_raw,cap_net_admin=eip ./target/release/hakinet
sudo setcap cap_net_raw,cap_net_admin=eip ./target/release/hakinet-scan
```

## 🧪 Development

### Build and Test

```bash
# Build entire workspace
cargo build

# Build specific tool
cargo build -p hakinet
cargo build -p hakinet-scan

# Run tests
cargo test

# Development run
cargo run -p hakinet -- capture --help
cargo run -p hakinet-scan -- scan --help
```

### Adding New Features

The modular workspace structure makes it easy to:
- Add new scanning techniques to `hakinet-scan`
- Extend packet analysis in `hakinet`
- Share common functionality via `hakinet-common`
- Create new tools that leverage existing components

## 📋 Protocol Support

### Packet Capture
- ✅ Ethernet
- ✅ IPv4/IPv6
- ✅ TCP/UDP
- ✅ ICMP
- ✅ ARP

### Port Scanning
- ✅ TCP SYN scan
- ✅ TCP connect scan
- ✅ UDP scan
- ✅ Service detection
- ⚠️ OS fingerprinting (basic)

### Service Detection
- ✅ HTTP/HTTPS
- ✅ SSH
- ✅ FTP
- ✅ SMTP
- ✅ DNS
- ✅ MySQL
- ✅ PostgreSQL
- ✅ And more...

## 🤝 Contributing

We welcome contributions! Please feel free to submit issues and pull requests.

### Guidelines
- Follow Rust best practices
- Add tests for new functionality
- Update documentation
- Maintain the cute cat theme! 🐱

## 📜 License

This project is licensed under the MIT License.

## 🐱 Cat Says

```
   /\_/\
  ( ^.^ ) "Thanks for using Hakinet! Remember to use these tools responsibly, meow!"
   > ^ <
```

---

**⚠️ Important**: Please ensure you have permission to scan target networks and comply with all applicable laws and regulations. These tools should only be used on networks you own or have explicit permission to test.

## 🔗 Comparison with Popular Tools

| Feature | Hakinet | hakinet-scan | tcpdump | nmap | Wireshark |
|---------|---------|--------------|---------|------|-----------|
| Packet Capture | ✅ | ❌ | ✅ | ❌ | ✅ |
| Port Scanning | ❌ | ✅ | ❌ | ✅ | ❌ |
| Service Detection | ❌ | ✅ | ❌ | ✅ | ❌ |
| Host Discovery | ❌ | ✅ | ❌ | ✅ | ❌ |
| Cute Cats | 🐱 | 🐱 | ❌ | ❌ | ❌ |
| JSON Output | ✅ | ✅ | ❌ | ❌ | ❌ |
| Colorful CLI | ✅ | ✅ | ❌ | ❌ | N/A |
| Cross-platform | ✅ | ✅ | ✅ | ✅ | ✅ |