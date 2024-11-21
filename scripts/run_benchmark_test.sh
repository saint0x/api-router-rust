#!/bin/bash

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
RED='\033[0;31m'
NC='\033[0m'

echo -e "${BLUE}Starting test environment...${NC}"

# Start our proxy server with specific binary
echo -e "${GREEN}Starting Zap.rs proxy server...${NC}"
cargo run --release --bin rust-router &
PROXY_PID=$!

# Wait for server to start
echo "Waiting for server to initialize..."
sleep 5

# Verify server is running
if ! curl -s http://localhost:3001/health > /dev/null; then
    echo -e "${RED}Proxy server failed to start${NC}"
    kill $PROXY_PID
    exit 1
fi

echo -e "${BLUE}Running benchmark tests...${NC}"
echo "This will take about 2 minutes to complete..."
python3 scripts/proxy_benchmark.py --proxy-url http://localhost:3001/proxy

# Save results
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
mkdir -p results
mv benchmark_results_*.json results/ 2>/dev/null || true

# Cleanup
echo -e "${BLUE}Cleaning up...${NC}"
kill $PROXY_PID

echo -e "${GREEN}Benchmark complete! Results saved in results/ directory${NC}"
