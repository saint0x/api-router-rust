#!/bin/bash

# Kill any existing processes
pkill node
pkill rust-router

# Wait for ports to be available
while lsof -i:3000 > /dev/null 2>&1; do
    echo "Waiting for port 3000 to be available..."
    sleep 1
done

while lsof -i:3001 > /dev/null 2>&1; do
    echo "Waiting for port 3001 to be available..."
    sleep 1
done

# Start Express backend
echo "Starting Express backend on port 3000..."
PORT=3000 node backend.js &
BACKEND_PID=$!

# Wait for backend to be ready
sleep 2

# Check if backend started successfully
if ! lsof -i:3000 > /dev/null 2>&1; then
    echo "Failed to start backend"
    exit 1
fi

echo "Backend started successfully"

# Start Rust proxy
echo "Starting Rust proxy on port 3001..."
cd ..
PORT=3001 cargo run --bin rust-router &
PROXY_PID=$!

# Wait for proxy to be ready
sleep 2

# Check if proxy started successfully
if ! lsof -i:3001 > /dev/null 2>&1; then
    echo "Failed to start proxy"
    kill $BACKEND_PID
    exit 1
fi

echo "Proxy started successfully"
echo "Both services are running. Press Ctrl+C to stop."

# Wait for Ctrl+C
trap "kill $BACKEND_PID $PROXY_PID; exit" INT
wait
