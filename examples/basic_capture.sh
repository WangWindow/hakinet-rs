#!/bin/bash

# Basic Hakinet Packet Capture Usage Examples
# Make sure to run with appropriate permissions (sudo)

echo "🐱 Hakinet Packet Capture Examples"
echo "=================================="

echo ""
echo "Note: Most packet capture operations require sudo privileges"
echo ""

echo "1. List available network interfaces"
./target/release/hakinet interfaces

echo ""
echo "2. Basic packet capture (10 packets)"
echo "sudo ./target/release/hakinet capture --count 10"

echo ""
echo "3. Capture HTTP traffic only"
echo "sudo ./target/release/hakinet capture --filter \"tcp port 80\" --count 5"

echo ""
echo "4. Capture DNS queries"
echo "sudo ./target/release/hakinet capture --filter \"udp port 53\" --count 5"

echo ""
echo "5. Capture traffic from specific host"
echo "sudo ./target/release/hakinet capture --filter \"host 8.8.8.8\" --count 5"

echo ""
echo "6. Capture and save to JSON file"
echo "sudo ./target/release/hakinet capture --count 20 --output packets.json"

echo ""
echo "7. Capture on specific interface with verbose output"
echo "sudo ./target/release/hakinet capture --interface eth0 --count 10 --verbose"

echo ""
echo "8. Capture HTTPS traffic"
echo "sudo ./target/release/hakinet capture --filter \"tcp port 443\" --count 5"

echo ""
echo "9. Capture all TCP traffic"
echo "sudo ./target/release/hakinet capture --filter \"tcp\" --count 10"

echo ""
echo "10. Continuous capture (Ctrl+C to stop)"
echo "sudo ./target/release/hakinet capture --filter \"not host 127.0.0.1\""

echo ""
echo "Examples ready! Run the commands above with sudo to capture packets."
echo "Check packets.json for saved packet data when using --output option."
