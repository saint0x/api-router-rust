#!/usr/bin/env python3
import asyncio
import aiohttp
import time
import statistics
import json
import argparse
from datetime import datetime
from tabulate import tabulate
import numpy as np
from typing import List, Dict, Any

# Test Configuration
CONCURRENT_USERS = 1000
TEST_DURATION = 60  # seconds
RAMP_UP_TIME = 10  # seconds

# Next.js App Router typical endpoints
ENDPOINTS = [
    {
        "name": "Complex Page Load",
        "method": "GET",
        "path": "/dashboard/[user]/analytics",
        "params": {"user": "test-user"},
        "query": "?period=30d&view=detailed",
        "headers": {
            "Accept": "application/json",
            "Cache-Control": "no-cache"
        }
    },
    {
        "name": "Data Mutation",
        "method": "POST",
        "path": "/api/data/batch-update",
        "payload": {
            "updates": [
                {"id": i, "status": "processed", "data": {"field1": "value1", "field2": "value2"}}
                for i in range(50)  # 50 items batch update
            ],
            "options": {
                "validate": True,
                "notify": True,
                "audit": True
            }
        }
    },
    {
        "name": "SSR with Data Fetch",
        "method": "GET",
        "path": "/products/category/[id]/page/[page]",
        "params": {"id": "electronics", "page": "1"},
        "query": "?sort=price&order=desc&filters=inStock,freeShipping"
    }
]

class BenchmarkResult:
    def __init__(self, name: str, latencies: List[float], errors: int):
        self.name = name
        self.latencies = latencies
        self.errors = errors
        self.rps = len(latencies) / TEST_DURATION if latencies else 0
        self.avg_latency = statistics.mean(latencies) if latencies else 0
        self.p95_latency = np.percentile(latencies, 95) if latencies else 0
        self.p99_latency = np.percentile(latencies, 99) if latencies else 0
        self.error_rate = (errors / len(latencies) * 100) if latencies else 0

async def make_request(session: aiohttp.ClientSession, base_url: str, endpoint: Dict[str, Any]) -> float:
    start_time = time.time()
    
    # Build URL with params and query
    path = endpoint["path"]
    if "params" in endpoint:
        for key, value in endpoint["params"].items():
            path = path.replace(f"[{key}]", value)
    
    url = f"{base_url}{path}"
    if "query" in endpoint:
        url += endpoint["query"]

    try:
        if endpoint["method"] == "GET":
            async with session.get(url, headers=endpoint.get("headers", {})) as response:
                await response.read()
                return time.time() - start_time if response.status == 200 else -1
        else:
            async with session.post(url, json=endpoint.get("payload"), headers=endpoint.get("headers", {})) as response:
                await response.read()
                return time.time() - start_time if response.status == 200 else -1
    except Exception as e:
        print(f"Error on {url}: {str(e)}")
        return -1

async def run_benchmark(base_url: str, endpoint: Dict[str, Any]) -> BenchmarkResult:
    print(f"\nTesting {endpoint['name']}...")
    
    async with aiohttp.ClientSession() as session:
        tasks = []
        latencies = []
        errors = 0
        
        # Calculate requests per second for ramp-up
        total_requests = CONCURRENT_USERS * (TEST_DURATION // 2)  # Assuming each user makes a request every 2 seconds
        requests_per_second = total_requests / TEST_DURATION
        
        # Ramp up period
        for _ in range(int(requests_per_second * RAMP_UP_TIME / 10)):
            tasks.append(make_request(session, base_url, endpoint))
            await asyncio.sleep(10 / requests_per_second)  # Gradually increase load
        
        # Full load period
        start_time = time.time()
        while time.time() - start_time < TEST_DURATION:
            tasks.append(make_request(session, base_url, endpoint))
            await asyncio.sleep(1 / requests_per_second)
        
        # Gather results
        results = await asyncio.gather(*tasks)
        latencies = [r for r in results if r > 0]
        errors = len([r for r in results if r < 0])
        
        return BenchmarkResult(endpoint["name"], latencies, errors)

def print_results(direct_results: List[BenchmarkResult], proxy_results: List[BenchmarkResult]):
    print("\nBenchmark Results:\n")
    
    headers = ["Metric", "Direct Next.js", "Through Proxy", "Difference"]
    table = []
    
    for direct, proxy in zip(direct_results, proxy_results):
        table.extend([
            [f"{direct.name} RPS", f"{direct.rps:.1f}", f"{proxy.rps:.1f}", 
             f"{((proxy.rps - direct.rps) / direct.rps * 100):+.1f}%"],
            [f"{direct.name} Avg Latency (ms)", f"{direct.avg_latency*1000:.1f}", 
             f"{proxy.avg_latency*1000:.1f}", 
             f"{((proxy.avg_latency - direct.avg_latency) / direct.avg_latency * 100):+.1f}%"],
            [f"{direct.name} P95 Latency (ms)", f"{direct.p95_latency*1000:.1f}", 
             f"{proxy.p95_latency*1000:.1f}", 
             f"{((proxy.p95_latency - direct.p95_latency) / direct.p95_latency * 100):+.1f}%"],
            [f"{direct.name} Error Rate", f"{direct.error_rate:.2f}%", 
             f"{proxy.error_rate:.2f}%", 
             f"{(proxy.error_rate - direct.error_rate):+.2f}%"],
            ["---", "---", "---", "---"]
        ])
    
    print(tabulate(table, headers=headers, tablefmt="grid"))

async def main():
    parser = argparse.ArgumentParser(description='Run proxy vs direct benchmarks')
    parser.add_argument('--next-url', default='http://localhost:3000', 
                       help='Next.js app URL')
    parser.add_argument('--proxy-url', default='http://localhost:3001/proxy', 
                       help='Proxy URL')
    args = parser.parse_args()

    print(f"Starting benchmark...")
    print(f"Next.js URL: {args.next_url}")
    print(f"Proxy URL: {args.proxy_url}")
    print(f"Concurrent Users: {CONCURRENT_USERS}")
    print(f"Test Duration: {TEST_DURATION}s")
    print(f"Ramp Up Time: {RAMP_UP_TIME}s")

    direct_results = []
    proxy_results = []

    for endpoint in ENDPOINTS:
        # Test direct Next.js
        result = await run_benchmark(args.next_url, endpoint)
        direct_results.append(result)
        
        # Test through proxy
        proxy_endpoint = endpoint.copy()
        proxy_endpoint["path"] = f"{args.next_url}{endpoint['path']}"
        result = await run_benchmark(args.proxy_url, proxy_endpoint)
        proxy_results.append(result)

    print_results(direct_results, proxy_results)

if __name__ == "__main__":
    asyncio.run(main())
