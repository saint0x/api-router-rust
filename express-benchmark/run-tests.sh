#!/bin/bash

set -e  # Exit on error

# Function to cleanup processes
cleanup() {
    echo "Cleaning up processes..."
    pkill -f 'node server.js' || true
    pkill -f 'api-router-rust' || true
    lsof -ti:3001 | xargs kill -9 2>/dev/null || true
    lsof -ti:3002 | xargs kill -9 2>/dev/null || true
}

# Set up signal handling
trap cleanup EXIT

# Initial cleanup
cleanup

echo "Starting Express server..."
node server.js &
SERVER_PID=$!

echo "Building and starting Rust proxy..."
cd ..
cargo build
RUST_BACKTRACE=1 cargo run &
PROXY_PID=$!

echo "Waiting for servers to start..."
sleep 5

echo "Running tests..."
cd express-benchmark
node test-proxy.js
TEST_EXIT=$?

echo "Test exit code: $TEST_EXIT"
exit $TEST_EXIT
