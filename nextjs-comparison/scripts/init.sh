#!/bin/bash

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
RED='\033[0;31m'
NC='\033[0m'

echo -e "${BLUE}Initializing Next.js comparison project...${NC}"

# Create necessary directories
echo -e "${GREEN}Creating directories...${NC}"
mkdir -p results

# Install dependencies
echo -e "${GREEN}Installing dependencies...${NC}"
npm install

# Build the project
echo -e "${GREEN}Building Next.js application...${NC}"
npm run build

# Verify Rust project is accessible
echo -e "${GREEN}Checking Rust project...${NC}"
if ! cargo --version > /dev/null 2>&1; then
    echo -e "${RED}Error: Rust is not installed${NC}"
    echo "Please install Rust from https://rustup.rs/"
    exit 1
fi

cd ../
if [ ! -f "Cargo.toml" ]; then
    echo -e "${RED}Error: Rust project not found${NC}"
    echo "Please ensure you're running this from the nextjs-comparison directory inside the Rust project"
    exit 1
fi

echo -e "${GREEN}Building Rust project...${NC}"
cargo build --release

cd nextjs-comparison

echo -e "${BLUE}Initialization complete!${NC}"
echo -e "You can now run benchmarks with: ${GREEN}npm run benchmark${NC}"
