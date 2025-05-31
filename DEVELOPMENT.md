# Hakinet Development Guide

## Project Overview

Hakinet is a comprehensive network toolkit written in Rust, featuring a modular workspace architecture with three main components:

- **hakinet** - Network packet capture tool (tcpdump/Wireshark alternative)
- **hakinet-scan** - Network scanning tool (nmap alternative) 
- **hakinet-common** - Shared library for common functionality

All tools feature cute cat mascots and colorful terminal output for an enjoyable user experience.

## Architecture

### Workspace Structure

```
hakinet-rs/
├── Cargo.toml                 # Workspace root with shared dependencies
├── hakinet/                   # Packet capture binary
├── hakinet-scan/              # Network scanner binary  
└── hakinet-common/            # Shared library
```

### Key Design Decisions

1. **Workspace Architecture**: Allows code reuse while maintaining separate binaries
2. **Async/Await**: Tokio for high-performance concurrent operations
3. **Structured Logging**: Using `log` crate with configurable levels
4. **Comprehensive CLI**: Using `clap` with derive macros for type-safe argument parsing
5. **Multiple Output Formats**: JSON, XML, CSV, and human-readable formats

## Current Implementation Status

### ✅ Completed Features

#### Hakinet (Packet Capture)
- Raw packet capture using pcap library
- BPF filter support
- Multiple output formats (console, JSON)
- Interface listing and selection
- Protocol parsing (Ethernet, IPv4/IPv6, TCP/UDP, ICMP, ARP)
- Colorful terminal output with progress indicators

#### Hakinet-Scan (Network Scanner)
- TCP SYN scanning
- TCP connect scanning  
- UDP scanning
- Host discovery (ping, TCP SYN, ARP methods)
- Service detection and banner grabbing
- Parallel scanning with configurable concurrency
- Multiple output formats (human, JSON, XML, CSV)
- Port range parsing and CIDR support

#### Hakinet-Common (Shared Library)
- Network utility functions
- Common data types and structures
- Output formatting utilities
- Rate limiting and timing utilities
- Cute cat ASCII art functions

### 🔧 Implemented But Needs Enhancement

1. **Service Detection**: Basic banner grabbing implemented, needs more signatures
2. **OS Fingerprinting**: Placeholder implementation, needs TCP/IP stack analysis
3. **ICMP Support**: Limited due to raw socket requirements
4. **Error Handling**: Basic error handling, could be more granular

### ❌ Not Yet Implemented

1. **Advanced Scan Techniques**: 
   - FIN, NULL, Xmas scans
   - Idle scan
   - Fragment scanning

2. **Advanced Service Detection**:
   - SSL/TLS certificate analysis
   - Version detection improvements
   - Custom probe sequences

3. **Advanced Host Discovery**:
   - IPv6 neighbor discovery
   - Raw ICMP implementation

4. **Output Enhancements**:
   - Grepable output format
   - Real-time streaming output
   - Progress bars for long scans

## Development Workflow

### Building and Testing

```bash
# Build entire workspace
cargo build

# Build release versions
cargo build --release

# Run tests
cargo test

# Build specific component
cargo build -p hakinet-common
cargo build -p hakinet
cargo build -p hakinet-scan

# Check code without building
cargo check

# Format code
cargo fmt

# Lint code
cargo clippy
```

### Running Tools

```bash
# Packet capture (requires sudo)
sudo ./target/release/hakinet capture --count 10

# Port scanning
./target/release/hakinet-scan scan 127.0.0.1

# Interface listing
./target/release/hakinet interfaces
```

## Code Organization

### Hakinet-Common Library

- `lib.rs` - Main library interface and cat animations
- `network.rs` - Network utilities (IP parsing, port ranges, etc.)
- `types.rs` - Shared data structures
- `output.rs` - Output formatting functions
- `utils.rs` - General utility functions

### Hakinet Binary

- `main.rs` - CLI interface and command routing
- `capture.rs` - Packet capture implementation
- `filter.rs` - BPF filter handling
- `output.rs` - Packet output formatting

### Hakinet-Scan Binary

- `main.rs` - CLI interface and scan orchestration
- `scanner.rs` - Port scanning implementations
- `discovery.rs` - Host discovery methods
- `service.rs` - Service detection logic

## Next Development Steps

### High Priority

1. **Improve Service Detection**
   - Add more service signatures
   - Implement proper probe sequences
   - Add SSL/TLS banner grabbing

2. **Enhanced Error Handling**
   - Custom error types
   - Better error messages
   - Graceful degradation

3. **Performance Optimization**
   - Optimize scanning algorithms
   - Better memory management
   - Reduce allocations in hot paths

### Medium Priority

1. **Advanced Scanning Techniques**
   - Implement FIN, NULL, Xmas scans
   - Add timing templates
   - Implement scan randomization

2. **IPv6 Support Enhancement**
   - IPv6 neighbor discovery
   - IPv6-specific scanning optimizations

3. **Configuration System**
   - Configuration file support
   - Scan profiles
   - Custom service definitions

### Low Priority

1. **GUI Interface**
   - Web-based interface
   - Real-time visualization

2. **Plugin System**
   - Custom scan modules
   - External service detectors

3. **Database Integration**
   - Scan result storage
   - Historical tracking

## Testing Strategy

### Unit Tests
- Network utility functions
- Data structure serialization
- Port range parsing
- IP address validation

### Integration Tests
- End-to-end scanning workflows
- Output format validation
- CLI argument parsing

### Security Testing
- Input validation
- Buffer overflow protection
- Privilege escalation prevention

## Contribution Guidelines

### Code Style
- Follow Rust standard conventions
- Use `rustfmt` for formatting
- Address all `clippy` warnings
- Write comprehensive documentation

### Git Workflow
- Feature branches for new functionality
- Descriptive commit messages
- Pull requests for all changes
- Maintain changelog

### Documentation
- Update README for new features
- Add inline documentation
- Include usage examples
- Document breaking changes

## Dependencies Management

### Key Dependencies
- `tokio` - Async runtime
- `clap` - CLI parsing
- `pcap` - Packet capture
- `pnet` - Network packet manipulation
- `serde` - Serialization
- `anyhow` - Error handling
- `colored` - Terminal colors

### Version Policy
- Use workspace dependencies for consistency
- Pin major versions for stability
- Regular dependency updates
- Security vulnerability monitoring

## Performance Considerations

### Scanning Performance
- Default parallelism: 100 concurrent operations
- Configurable timeouts and retries
- Memory-efficient data structures
- Minimal allocations in scan loops

### Packet Capture Performance
- Efficient packet parsing
- Configurable buffer sizes
- Minimal copying operations
- Streaming output for large captures

## Security Considerations

### Privilege Requirements
- Raw socket access for advanced features
- Capability-based permissions on Linux
- User warnings for privilege escalation

### Input Validation
- IP address and hostname validation
- Port range bounds checking
- Filter expression validation
- File path sanitization

## Deployment

### Distribution
- Single binary per tool
- Static linking where possible
- Cross-platform compilation
- Package manager integration

### Installation
- Cargo install from crates.io
- Distribution packages (deb, rpm)
- Docker containers
- Homebrew formula

## Monitoring and Observability

### Logging
- Structured logging with levels
- Performance metrics
- Error tracking
- Debug output for development

### Metrics
- Scan duration tracking
- Packet capture rates
- Error rate monitoring
- Resource usage statistics

This development guide should be updated as the project evolves and new features are implemented.