#!/bin/bash

# Hakinet Network Tools Demo
# Simple demonstration of packet capture and network scanning capabilities

echo "🐱 Hakinet Network Tools Demo"
echo "============================="

# Build tools quietly
echo "Building tools..."
RUSTFLAGS="-A warnings" cargo build --release > /dev/null 2>&1

if [ $? -ne 0 ]; then
    echo "❌ Build failed!"
    exit 1
fi

echo "✅ Build completed successfully!"
echo ""

# Demo 1: List network interfaces
echo "📡 Demo 1: Available Network Interfaces"
echo "----------------------------------------"
./target/release/hakinet interfaces
echo ""

# Demo 2: Quick port scan on localhost
echo "🎯 Demo 2: Quick Port Scan on Localhost"
echo "----------------------------------------"
./target/release/hakinet-scan scan 127.0.0.1 --ports 22,80,443 --timeout 2
echo ""

# Demo 3: Host discovery on localhost
echo "🔍 Demo 3: Host Discovery"
echo "------------------------"
./target/release/hakinet-scan discovery 127.0.0.1 --timeout 1
echo ""

# Demo 4: Save scan results to JSON
echo "📁 Demo 4: Saving Results to JSON"
echo "---------------------------------"
./target/release/hakinet-scan scan 127.0.0.1 --ports 1-100 --timeout 1 --output json --file demo_results.json
if [ -f "demo_results.json" ]; then
    echo "✅ Results saved to demo_results.json"
    echo "📊 File size: $(ls -lh demo_results.json | awk '{print $5}')"
else
    echo "❌ Failed to save results"
fi
echo ""

# Demo 5: Show packet capture help (doesn't require sudo)
echo "📦 Demo 5: Packet Capture Tool Help"
echo "-----------------------------------"
echo "Note: Actual packet capture requires sudo privileges"
./target/release/hakinet capture --help
echo ""

echo "🎉 Demo Complete!"
echo ""
echo "Next steps:"
echo "• Try: sudo ./target/release/hakinet capture --count 10"
echo "• Try: ./target/release/hakinet-scan scan <your-network-ip>"
echo "• Check: cat demo_results.json for scan results"
echo ""
echo "🐱 Thanks for trying Hakinet!"