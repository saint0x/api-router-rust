import asyncio
import aiohttp
import time
import yaml
import statistics
import argparse
import os
from datetime import datetime
from tabulate import tabulate

# Configuration
CURRENT_URL = "http://127.0.0.1:3001"  # Current implementation
NEW_URL = "http://127.0.0.1:3002"      # New implementation being tested
CONCURRENT_REQUESTS = 100
TOTAL_REQUESTS = 10000
COOLDOWN_SECONDS = 5
ENDPOINTS = [
    {"name": "Simple Ping", "method": "GET", "url": "/ping", "payload": None},
    {"name": "Medium API", "method": "GET", "url": "/api/v1/data", "payload": None},
    {"name": "Complex Process", "method": "POST", "url": "/api/v1/process", "payload": {
        "data": "test",
        "nested": {"field": "value"},
        "array": [1, 2, 3, 4, 5]
    }}
]

class BenchmarkResult:
    def __init__(self, name, rps, avg_latency, p95_latency, p99_latency):
        self.name = name
        self.rps = rps
        self.avg_latency = avg_latency
        self.p95_latency = p95_latency
        self.p99_latency = p99_latency

def calculate_performance_summary(new_results, current_results):
    total_new_rps = sum(r.rps for r in new_results if r)
    total_current_rps = sum(r.rps for r in current_results if r)
    avg_new_latency = statistics.mean(r.avg_latency for r in new_results if r)
    avg_current_latency = statistics.mean(r.avg_latency for r in current_results if r)
    
    rps_change = ((total_new_rps - total_current_rps) / total_current_rps) * 100
    latency_change = ((avg_current_latency - avg_new_latency) / avg_current_latency) * 100

    summary = "\nPerformance Summary:\n"
    summary += f"Total RPS Change: {rps_change:.1f}%\n"
    summary += f"Average Latency Change: {latency_change:.1f}%\n"
    
    # Detailed comparison table
    headers = ["Metric", "Current", "New", "Change"]
    table = [
        ["Total RPS", f"{total_current_rps:.1f}", f"{total_new_rps:.1f}", f"{rps_change:+.1f}%"],
        ["Avg Latency (ms)", f"{avg_current_latency*1000:.1f}", f"{avg_new_latency*1000:.1f}", f"{latency_change:+.1f}%"]
    ]
    summary += "\n" + tabulate(table, headers=headers, tablefmt="grid")
    
    return summary, rps_change, latency_change

def update_feature_history(feature_name, implemented, impact_summary=None):
    history_file = "results/feature_history.yaml"
    
    new_entry = {
        'date': datetime.now().strftime('%Y-%m-%d'),
        'feature': feature_name,
        'test_approach': f"Compared current vs new implementation across {len(ENDPOINTS)} endpoints",
        'technical_outcome': impact_summary if implemented else "Performance testing showed no improvement",
        'impact': "Implemented based on performance results" if implemented else "Not implemented - no performance benefit",
        'implemented': implemented
    }
    
    if os.path.exists(history_file):
        with open(history_file, 'r') as f:
            history = yaml.safe_load(f) or {}
    else:
        history = {'feature_history': {}}
    
    history['feature_history'][feature_name] = new_entry
    
    with open(history_file, 'w') as f:
        yaml.dump(history, f, sort_keys=False, indent=2, default_flow_style=False)

async def make_request(session, base_url, endpoint, request_num):
    method = endpoint["method"]
    url = f"{base_url}{endpoint['url']}"
    start_time = time.time()
    
    try:
        if method == "GET":
            async with session.get(url) as response:
                await response.read()
        else:
            async with session.post(url, json=endpoint["payload"]) as response:
                await response.read()
        
        end_time = time.time()
        return end_time - start_time, response.status
    except Exception as e:
        print(f"Error on request {request_num}: {str(e)}")
        return None, None

async def run_benchmark(base_url, endpoint):
    print(f"Testing {endpoint['name']}...", end='', flush=True)
    
    async with aiohttp.ClientSession() as session:
        tasks = []
        start_time = time.time()
        
        for i in range(TOTAL_REQUESTS):
            tasks.append(make_request(session, base_url, endpoint, i))
        
        results = await asyncio.gather(*tasks)
        end_time = time.time()
        
        latencies = [r[0] for r in results if r[0] is not None]
        
        if not latencies:
            print(" Failed!")
            return None
        
        total_time = end_time - start_time
        successful_requests = len(latencies)
        rps = TOTAL_REQUESTS / total_time
        avg_latency = statistics.mean(latencies)
        p95_latency = statistics.quantiles(latencies, n=20)[18]
        p99_latency = statistics.quantiles(latencies, n=100)[98]
        
        print(f" {rps:.0f} RPS, {avg_latency*1000:.0f}ms avg")
        
        return BenchmarkResult(
            endpoint["name"],
            rps,
            avg_latency,
            p95_latency,
            p99_latency
        )

def cleanup_old_results():
    results_dir = "results"
    for file in os.listdir(results_dir):
        if file != "feature_history.yaml":
            file_path = os.path.join(results_dir, file)
            try:
                os.remove(file_path)
            except Exception:
                pass

async def run_test_suite(implementation, base_url):
    print(f"\nRunning {implementation} implementation tests:")
    results = []
    
    for endpoint in ENDPOINTS:
        result = await run_benchmark(base_url, endpoint)
        results.append(result)
        
        if endpoint != ENDPOINTS[-1]:
            time.sleep(COOLDOWN_SECONDS)
    
    return results

async def main():
    parser = argparse.ArgumentParser(description='Run API performance benchmarks')
    parser.add_argument('feature', help='Name of the feature being tested (e.g., request-batching)')
    args = parser.parse_args()

    print(f"Starting benchmark for feature: {args.feature}")
    cleanup_old_results()
    
    new_results = await run_test_suite("new", NEW_URL)
    time.sleep(COOLDOWN_SECONDS)
    current_results = await run_test_suite("current", CURRENT_URL)
    
    # Generate and display performance summary
    summary, rps_change, latency_change = calculate_performance_summary(new_results, current_results)
    print(summary)
    
    # Ask for implementation decision
    decision = input("\nBased on these results, should we implement this feature? (yes/no): ").lower()
    implemented = decision == 'yes'
    
    # Generate impact summary if implemented
    impact_summary = None
    if implemented:
        if abs(rps_change) < 1 and abs(latency_change) < 1:
            impact_summary = "No significant impact"
        else:
            impact_parts = []
            if abs(rps_change) >= 1:
                impact_parts.append(f"{'improved' if rps_change > 0 else 'reduced'} throughput by {abs(rps_change):.1f}%")
            if abs(latency_change) >= 1:
                impact_parts.append(f"{'improved' if latency_change > 0 else 'increased'} latency by {abs(latency_change):.1f}%")
            impact_summary = " and ".join(impact_parts)
    
    # Update feature history with implementation decision
    update_feature_history(args.feature, implemented, impact_summary)
    
    # Save detailed results
    timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
    results = {
        'feature': args.feature,
        'timestamp': timestamp,
        'implemented': implemented,
        'config': {
            'total_requests': TOTAL_REQUESTS,
            'concurrent_requests': CONCURRENT_REQUESTS
        },
        'new_results': [
            {
                'name': r.name,
                'rps': float(f"{r.rps:.2f}"),
                'avg_latency': float(f"{r.avg_latency*1000:.2f}"),
                'p95_latency': float(f"{r.p95_latency*1000:.2f}"),
                'p99_latency': float(f"{r.p99_latency*1000:.2f}")
            } for r in new_results if r
        ],
        'current_results': [
            {
                'name': r.name,
                'rps': float(f"{r.rps:.2f}"),
                'avg_latency': float(f"{r.avg_latency*1000:.2f}"),
                'p95_latency': float(f"{r.p95_latency*1000:.2f}"),
                'p99_latency': float(f"{r.p99_latency*1000:.2f}")
            } for r in current_results if r
        ]
    }
    
    filename = f"results/benchmark_comparison_{args.feature}_{timestamp}.yaml"
    with open(filename, 'w') as f:
        yaml.dump(results, f, sort_keys=False, indent=2, default_flow_style=False)
    
    print(f"\nResults saved to: {filename}")

if __name__ == "__main__":
    asyncio.run(main())
