#!/bin/bash

# Kill any existing node processes
pkill node

# Build Rust proxy
echo "Building Rust proxy..."
cargo build --release

# Start Express backend
echo "Starting Express backend on port 3000..."
BACKEND_PORT=3000 node backend.js &
BACKEND_PID=$!

# Wait for backend to start
sleep 2

# Start Rust proxy
echo "Starting Rust proxy on port 3001..."
PORT=3001 ../target/release/rust-router &
PROXY_PID=$!

# Start Express direct implementation
echo "Starting Express direct implementation on port 3002..."
EXPRESS_PORT=3002 node server.js &
EXPRESS_PID=$!

# Wait for all servers to be ready
sleep 2

# Run benchmark
echo "Running benchmark..."
node benchmark.js

# Cleanup
echo "Cleaning up..."
kill $BACKEND_PID
kill $PROXY_PID
kill $EXPRESS_PID

echo "Done!"
