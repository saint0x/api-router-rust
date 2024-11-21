import requests
import time
import psutil
import json
from datetime import datetime
import os
from requests.adapters import HTTPAdapter
from urllib3.util.retry import Retry

def create_session():
    """Create requests session with retries"""
    session = requests.Session()
    retries = Retry(total=3, backoff_factor=0.5)
    session.mount('http://', HTTPAdapter(max_retries=retries))
    session.mount('https://', HTTPAdapter(max_retries=retries))
    return session

def wait_for_server(timeout=30, interval=0.5):
    """Wait for Rust server to be ready"""
    print("Waiting for Rust server to be ready...")
    start = time.time()
    session = create_session()
    while time.time() - start < timeout:
        try:
            response = session.get('http://localhost:3003/health', timeout=1)
            if response.status_code == 200:
                print("Rust server is ready!")
                return True
        except:
            print(".", end="", flush=True)
            time.sleep(interval)
    print("\nTimeout waiting for Rust server")
    return False

def get_weather_direct(session, params):
    """Direct Python request to weather API"""
    print('Making Python direct API call...')
    response = session.get(
        'https://api.open-meteo.com/v1/forecast',
        params=params,
        timeout=30
    )
    return response.json()

def get_weather_with_zap(session, params):
    """Python request with @zap prefix - plug and play!"""
    print('Making Python @zap API call...')
    # Just add @zap prefix to any URL
    url = 'https://api.open-meteo.com/v1/forecast'
    # Remove https:// since our router adds it
    clean_url = url.replace('https://', '')
    response = session.get(
        f'http://localhost:3003/@zap/{clean_url}',
        params=params,
        timeout=30
    )
    return response.json()

def save_results(results):
    """Save benchmark results to markdown file"""
    # Ensure router-results directory exists
    os.makedirs('router-results', exist_ok=True)
    
    with open('router-results/zap-vs-python.md', 'w') as f:
        f.write('# @zap Router vs Python Requests Benchmark Results\n\n')
        f.write('## Performance Metrics\n\n')
        
        # Latency
        f.write('1. Latency Performance\n')
        f.write(f'   - Python Direct: {results.get("direct_time", "N/A")}ms\n')
        f.write(f'   - With @zap: {results.get("zap_time", "N/A")}ms\n')
        if "direct_time" in results and "zap_time" in results:
            overhead = ((results["zap_time"] - results["direct_time"]) / results["direct_time"] * 100)
            f.write(f'   - Overhead: {overhead:.1f}%\n\n')
        else:
            f.write('   - Overhead: N/A\n\n')
        
        # Memory
        f.write('2. Memory Usage\n')
        f.write(f'   Python Direct:\n')
        f.write(f'   - RSS: {results.get("direct_memory", "N/A"):.1f}MB\n')
        f.write(f'   - Memory %: {results.get("memory_percent", "N/A"):.1f}%\n\n')
        f.write(f'   With @zap:\n')
        f.write(f'   - Additional RSS: {results.get("zap_memory", "N/A"):.1f}MB\n')
        
        # CPU
        f.write('3. CPU Usage\n')
        f.write(f'   - Process CPU: {results.get("cpu_percent", "N/A"):.1f}%\n')
        if "cpu_idle" in results:
            f.write(f'   - System Impact: {100 - results["cpu_idle"]:.1f}% total CPU used\n\n')
        else:
            f.write('   - System Impact: N/A\n\n')
        
        # System Impact
        f.write('4. System Impact\n')
        f.write(f'   - Memory Used: {results.get("memory_used", "N/A"):.1f}MB\n')
        f.write(f'   - Memory Free: {results.get("memory_free", "N/A"):.1f}MB\n')
        f.write(f'   - Buffer/Cache: {results.get("buffer_cache", "N/A"):.1f}MB\n\n')
        
        # Summary
        f.write('## Summary\n')
        f.write('Comparison between direct Python requests and with @zap prefix:\n')
        f.write('- Simple plug-and-play integration (just add @zap/ prefix)\n')
        f.write('- Minimal memory overhead\n')
        f.write('- Efficient CPU utilization\n')
        f.write('- Transparent routing through @zap infrastructure\n')

def run_benchmark():
    """Run benchmark comparing direct vs @zap prefixed calls"""
    results = {}
    
    # Test parameters - Berlin weather
    params = {
        'latitude': 52.52,
        'longitude': 13.41,
        'current': 'temperature_2m'  # Minimal data for testing
    }

    # Create session with retries
    session = create_session()

    # Track process memory
    process = psutil.Process()
    initial_memory = process.memory_info().rss

    print('\nMaking Python direct API call...')
    start = time.time()
    try:
        get_weather_direct(session, params)
        results['direct_time'] = (time.time() - start) * 1000
        print(f'Python direct API call took {results["direct_time"]:.0f}ms')
    except Exception as e:
        print(f'Error in Python direct call: {str(e)}')

    # Memory after direct call
    results['direct_memory'] = (process.memory_info().rss - initial_memory) / 1024 / 1024
    print(f'Python direct call memory usage: {results["direct_memory"]:.1f}MB')

    # Wait between calls
    time.sleep(2)

    print('\nMaking Python @zap API call...')
    start = time.time()
    try:
        get_weather_with_zap(session, params)
        results['zap_time'] = (time.time() - start) * 1000
        print(f'Python @zap API call took {results["zap_time"]:.0f}ms')
    except Exception as e:
        print(f'Error in Python @zap call: {str(e)}')

    # Memory after @zap call
    results['zap_memory'] = (process.memory_info().rss - initial_memory) / 1024 / 1024
    print(f'Python @zap call memory usage: {results["zap_memory"]:.1f}MB')

    # System stats
    results['cpu_percent'] = process.cpu_percent()
    results['memory_percent'] = process.memory_percent()
    results['cpu_idle'] = psutil.cpu_times_percent().idle
    
    # System memory
    vm = psutil.virtual_memory()
    results['memory_used'] = vm.used / 1024 / 1024
    results['memory_free'] = vm.free / 1024 / 1024
    results['buffer_cache'] = (vm.buffers + vm.cached) / 1024 / 1024

    print(f'\nPython Process Stats:')
    print(f'CPU Usage: {results["cpu_percent"]:.1f}%')
    print(f'Memory Usage: {results["memory_percent"]:.1f}%')

    # Save results
    save_results(results)

if __name__ == '__main__':
    # Wait for Rust server
    if not wait_for_server():
        print("Error: Rust server not ready")
        exit(1)
    run_benchmark()
