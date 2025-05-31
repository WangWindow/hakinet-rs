#!/bin/bash

# Advanced Hakinet-Scan Usage Examples
# Advanced network scanning techniques and comprehensive security testing

echo "🐱 Advanced Hakinet-Scan Examples"
echo "=================================="

# Color definitions for better output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color


echo ""
echo -e "${RED}⚠️  WARNING: These are advanced scanning techniques!${NC}"
echo -e "${RED}   Only use on networks you own or have explicit permission to test.${NC}"
echo -e "${RED}   Unauthorized network scanning may be illegal in your jurisdiction.${NC}"
echo ""

# Function to run command with description
run_example() {
    local description="$1"
    local command="$2"
    local execute="$3"

    echo -e "${YELLOW}[EXAMPLE]${NC} $description"
    echo -e "${BLUE}Command:${NC} $command"

    if [ "$execute" = "true" ]; then
        echo -e "${GREEN}Executing...${NC}"
        eval $command
        echo ""
    else
        echo -e "${PURPLE}(Example command - not executed)${NC}"
        echo ""
    fi
}

echo -e "${CYAN}=== RECONNAISSANCE PHASE ===${NC}"

run_example "Host Discovery - Ping Sweep" \
"./target/release/hakinet-scan discovery 192.168.1.0/24 --method ping --max-parallel 50" \
false

run_example "Host Discovery - TCP SYN Ping" \
"./target/release/hakinet-scan discovery 192.168.1.0/24 --method tcp-syn --timeout 2" \
false

run_example "Host Discovery - ARP Scan (Local Network)" \
"./target/release/hakinet-scan discovery 192.168.1.0/24 --method arp" \
false

echo -e "${CYAN}=== PORT SCANNING TECHNIQUES ===${NC}"

run_example "Stealth SYN Scan - Top 1000 Ports" \
"./target/release/hakinet-scan scan 127.0.0.1 --scan-type syn --ports 1-1000 --randomize" \
true

run_example "Connect Scan with Service Detection" \
"./target/release/hakinet-scan scan 127.0.0.1 --scan-type connect --service-detection --ports 1-100" \
true

run_example "UDP Scan - Common Services" \
"./target/release/hakinet-scan scan 127.0.0.1 --scan-type udp --ports 53,67,68,69,123,161,162,500,514,520,1900,4500" \
true

run_example "Comprehensive Scan with All Features" \
"./target/release/hakinet-scan scan 127.0.0.1 --scan-type comprehensive --service-detection --os-detection --randomize --verbose" \
true

echo -e "${CYAN}=== SPECIFIC SERVICE SCANNING ===${NC}"

run_example "Web Services Scan" \
"./target/release/hakinet-scan scan example.com --ports 80,443,8080,8443,8000,8888,9000,9090 --service-detection" \
false

run_example "Database Services Scan" \
"./target/release/hakinet-scan scan 192.168.1.100 --ports 1433,1521,3306,5432,6379,27017,5984,9200 --service-detection" \
false

run_example "Mail Services Scan" \
"./target/release/hakinet-scan scan mail.example.com --ports 25,110,143,993,995,587,465 --service-detection" \
false

run_example "Remote Access Services Scan" \
"./target/release/hakinet-scan scan 192.168.1.50 --ports 22,23,3389,5900,5901,5902,5903,5904,5905 --service-detection" \
false

run_example "File Sharing Services Scan" \
"./target/release/hakinet-scan scan 192.168.1.200 --ports 21,22,135,139,445,2049,111,2000 --service-detection" \
false

echo -e "${CYAN}=== PERFORMANCE OPTIMIZATION ===${NC}"

run_example "High-Speed Scan - Maximum Parallelism" \
"./target/release/hakinet-scan scan 127.0.0.1 --max-parallel 500 --timeout 1 --ports 1-1000" \
false

run_example "Slow and Stealthy Scan" \
"./target/release/hakinet-scan scan 192.168.1.1 --max-parallel 10 --timeout 10 --randomize --ports 1-100" \
false

run_example "Balanced Scan with Retry" \
"./target/release/hakinet-scan scan 192.168.1.1 --max-parallel 100 --timeout 3 --ports 1-1000" \
false

echo -e "${CYAN}=== ADVANCED TARGET SPECIFICATION ===${NC}"

run_example "Multiple Target Types" \
"./target/release/hakinet-scan scan 127.0.0.1 192.168.1.1 google.com github.com --ports 80,443" \
false

run_example "CIDR Range Scan" \
"./target/release/hakinet-scan scan 192.168.1.0/24 --ports 22,80,443 --max-parallel 200" \
false

run_example "IP Range Scan" \
"./target/release/hakinet-scan scan 192.168.1.1-192.168.1.50 --ports 1-100" \
false

run_example "Mixed Targets with Service Detection" \
"./target/release/hakinet-scan scan 127.0.0.1 example.com 192.168.1.0/28 --service-detection --ports 22,80,443" \
false

echo -e "${CYAN}=== OUTPUT AND REPORTING ===${NC}"

run_example "JSON Output for Automation" \
"./target/release/hakinet-scan scan 127.0.0.1 --output json --file advanced_scan.json --ports 1-100" \
true

run_example "XML Output for Compatibility" \
"./target/release/hakinet-scan scan 127.0.0.1 --output xml --file advanced_scan.xml --ports 22,80,443" \
false

run_example "CSV Output for Analysis" \
"./target/release/hakinet-scan scan 127.0.0.1 --output csv --file advanced_scan.csv --ports 1-100" \
false

run_example "Human-Readable Verbose Output" \
"./target/release/hakinet-scan scan 127.0.0.1 --output human --verbose --service-detection --ports 22,80,443" \
false

echo -e "${CYAN}=== SPECIALIZED SCANS ===${NC}"

run_example "Top 100 Most Common Ports" \
"./target/release/hakinet-scan scan 192.168.1.1 --ports 7,9,13,21,22,23,25,26,37,53,79,80,81,88,106,110,111,113,119,135,139,143,144,179,199,389,427,443,444,445,465,513,514,515,543,544,548,554,587,631,646,873,990,993,995,1025,1026,1027,1028,1029,1110,1433,1720,1723,1755,1900,2000,2001,2049,2121,2717,3000,3128,3306,3389,3986,4899,5000,5009,5051,5060,5101,5190,5357,5432,5631,5666,5800,5900,6000,6001,6646,7070,8000,8008,8009,8080,8081,8443,8888,9100,9999,10000,32768,49152,49153,49154,49155,49156,49157" \
false

run_example "Full Port Range Scan (USE WITH CAUTION)" \
"./target/release/hakinet-scan scan 127.0.0.1 --ports 1-65535 --max-parallel 1000 --timeout 1" \
false

run_example "Interesting Ports Only" \
"./target/release/hakinet-scan scan 192.168.1.1 --ports 21,22,23,25,53,80,110,111,135,139,143,443,993,995,1723,3306,3389,5432,5900,8080" \
false

echo -e "${CYAN}=== EVASION TECHNIQUES ===${NC}"

run_example "Randomized Scan Order" \
"./target/release/hakinet-scan scan 192.168.1.1 --randomize --ports 1-1000 --max-parallel 50" \
false

run_example "Slow Scan to Avoid Detection" \
"./target/release/hakinet-scan scan 192.168.1.1 --max-parallel 5 --timeout 10 --ports 1-100" \
false

echo -e "${CYAN}=== NETWORK MAPPING ===${NC}"

run_example "Subnet Discovery and Port Scan" \
"./target/release/hakinet-scan discovery 192.168.1.0/24 --output json --file discovered_hosts.json && ./target/release/hakinet-scan scan 192.168.1.0/24 --ports 22,80,443 --output json --file port_scan_results.json" \
false

run_example "Service Enumeration Scan" \
"./target/release/hakinet-scan scan 192.168.1.0/24 --service-detection --os-detection --ports 21,22,23,25,53,80,110,143,443,993,995 --output json --file service_enum.json" \
false

echo ""
echo -e "${GREEN}=== RESULTS AND FILES ===${NC}"
echo "Generated files:"
if [ -f "advanced_scan.json" ]; then
    echo -e "${GREEN}✓${NC} advanced_scan.json - JSON scan results"
fi
if [ -f "advanced_scan.xml" ]; then
    echo -e "${GREEN}✓${NC} advanced_scan.xml - XML scan results"
fi
if [ -f "advanced_scan.csv" ]; then
    echo -e "${GREEN}✓${NC} advanced_scan.csv - CSV scan results"
fi
if [ -f "discovered_hosts.json" ]; then
    echo -e "${GREEN}✓${NC} discovered_hosts.json - Host discovery results"
fi
if [ -f "port_scan_results.json" ]; then
    echo -e "${GREEN}✓${NC} port_scan_results.json - Port scan results"
fi
if [ -f "service_enum.json" ]; then
    echo -e "${GREEN}✓${NC} service_enum.json - Service enumeration results"
fi

echo ""
echo -e "${CYAN}=== ANALYSIS TIPS ===${NC}"
echo "• Use JSON output for automated processing and integration"
echo "• Combine host discovery with targeted port scanning"
echo "• Service detection helps identify specific applications"
echo "• Randomization helps avoid simple detection mechanisms"
echo "• Adjust parallelism based on network capacity and stealth requirements"
echo "• Always check local laws and obtain proper authorization"

echo ""
echo -e "${PURPLE}🐱 Advanced scanning complete! Remember: With great power comes great responsibility.${NC}"
echo -e "${YELLOW}   Always ensure you have proper authorization before scanning networks.${NC}"
