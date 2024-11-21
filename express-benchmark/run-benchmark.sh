#!/bin/bash

# Kill any existing processes
pkill -f 'node server.js' || true
pkill -f 'api-router-rust' || true
lsof -ti:3001 | xargs kill -9 2>/dev/null || true
lsof -ti:3002 | xargs kill -9 2>/dev/null || true

# Wait for ports to be freed
sleep 2

# Start the Express server
node server.js &
SERVER_PID=$!

# Build and start the Rust proxy
cd ..
cargo build && cargo run &
PROXY_PID=$!

# Wait for servers to start
sleep 5

# Run benchmark
cd express-benchmark
node benchmark.js

# Cleanup
kill $SERVER_PID $PROXY_PID 2>/dev/null || true
pkill -f 'node server.js' || true
pkill -f 'api-router-rust' || true
