#!/bin/bash

# Kill any existing processes
echo "Cleaning up existing processes..."
pkill -f "api_router_rust"
sleep 2

# Start Rust server
echo "Starting Rust server..."
cargo run &
RUST_PID=$!

# Wait for server to start
echo "Waiting for Rust server to start..."
sleep 5

# Run Python benchmark
echo "Running Python benchmark..."
python3 express-benchmark/weather-benchmark.py

# Cleanup
echo "Cleaning up..."
kill $RUST_PID
