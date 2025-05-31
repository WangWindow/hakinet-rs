#!/bin/bash

# Basic Hakinet-Scan Usage Examples
# Network scanning tool with nmap-like functionality

echo "🐱 Hakinet-Scan Examples"
echo "========================"

echo ""
echo "Note: Network scanning should only be performed on networks you own or have permission to test"
echo ""

echo "1. Basic port scan on localhost"
./target/release/hakinet-scan scan 127.0.0.1 --ports 22,80,443

echo ""
echo "2. Quick scan of common ports"
./target/release/hakinet-scan scan 127.0.0.1 --ports 1-1024

echo ""
echo "3. Scan specific port ranges"
echo "./target/release/hakinet-scan scan 192.168.1.1 --ports 80-443"

echo ""
echo "4. Service detection scan"
./target/release/hakinet-scan scan 127.0.0.1 --service-detection --ports 22,80,443

echo ""
echo "5. UDP port scan"
./target/release/hakinet-scan scan 127.0.0.1 --scan-type udp --ports 53,67,123

echo ""
echo "6. TCP connect scan"
./target/release/hakinet-scan scan 127.0.0.1 --scan-type connect --ports 1-100

echo ""
echo "7. TCP SYN scan (stealth)"
./target/release/hakinet-scan scan 127.0.0.1 --scan-type syn --ports 22,80,443

echo ""
echo "8. Comprehensive scan (TCP + UDP)"
./target/release/hakinet-scan scan 127.0.0.1 --scan-type comprehensive --service-detection

echo ""
echo "9. Host discovery on local network"
echo "./target/release/hakinet-scan discovery 192.168.1.0/24"

echo ""
echo "10. Host discovery with TCP SYN method"
echo "./target/release/hakinet-scan discovery 192.168.1.0/24 --method tcp-syn"

echo ""
echo "11. Host discovery with ARP method (local only)"
echo "./target/release/hakinet-scan discovery 192.168.1.0/24 --method arp"

echo ""
echo "12. Scan multiple targets"
echo "./target/release/hakinet-scan scan 127.0.0.1 192.168.1.1 google.com --ports 80,443"

echo ""
echo "13. Scan IP range"
echo "./target/release/hakinet-scan scan 192.168.1.1-192.168.1.10 --ports 22,80"

echo ""
echo "14. Randomized scan order"
./target/release/hakinet-scan scan 127.0.0.1 --randomize --ports 1-100

echo ""
echo "15. High-speed scan with increased parallelism"
./target/release/hakinet-scan scan 127.0.0.1 --max-parallel 200 --ports 1-1000

echo ""
echo "16. Scan with custom timeout"
./target/release/hakinet-scan scan 127.0.0.1 --timeout 5 --ports 22,80,443

echo ""
echo "17. Save results to JSON"
./target/release/hakinet-scan scan 127.0.0.1 --output json --file scan_results.json --ports 1-100

echo ""
echo "18. Save results to XML"
echo "./target/release/hakinet-scan scan 192.168.1.1 --output xml --file scan_results.xml"

echo ""
echo "19. Save results to CSV"
echo "./target/release/hakinet-scan scan 192.168.1.1 --output csv --file scan_results.csv"

echo ""
echo "20. Verbose output with OS detection"
./target/release/hakinet-scan scan 127.0.0.1 --os-detection --verbose --ports 22,80,443

echo ""
echo "21. Scan common web ports"
echo "./target/release/hakinet-scan scan example.com --ports 80,443,8080,8443"

echo ""
echo "22. Scan database ports"
echo "./target/release/hakinet-scan scan 192.168.1.100 --ports 3306,5432,1433,1521"

echo ""
echo "23. Scan mail server ports"
echo "./target/release/hakinet-scan scan mail.example.com --ports 25,110,143,993,995"

echo ""
echo "24. Quick top 100 ports scan"
echo "./target/release/hakinet-scan scan 192.168.1.1 --ports 1-100 --max-parallel 50"

echo ""
echo "25. Full port range scan (be careful!)"
echo "./target/release/hakinet-scan scan 127.0.0.1 --ports 1-65535 --timeout 1"

echo ""
echo "Examples complete! Check the generated output files:"
echo "  - scan_results.json (if JSON output was used)"
echo "  - scan_results.xml (if XML output was used)"
echo "  - scan_results.csv (if CSV output was used)"
echo ""
echo "🐱 Remember to use these tools responsibly and only on networks you own!"
