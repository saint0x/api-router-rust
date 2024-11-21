#!/bin/bash

# Kill any existing processes on our port
echo "Cleaning up existing processes..."
lsof -ti:3003 | xargs kill -9 2>/dev/null

# Start Rust server in the background
echo "Starting Rust server..."
cd .. && cargo run &
RUST_PID=$!

# Wait for Rust server to start
echo "Waiting for Rust server to start..."
sleep 5

# Run benchmark
echo "Running benchmark..."
node weather-api.js

# Cleanup
echo "Cleaning up..."
kill $RUST_PID
