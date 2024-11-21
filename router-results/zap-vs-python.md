# @zap Router vs Python Requests Benchmark Results

## Performance Metrics

1. Latency Performance
   - Python Direct: 30500.093460083008ms
   - With @zap: 548.5677719116211ms
   - Overhead: -98.2%

2. Memory Usage
   Python Direct:
   - RSS: 0.6MB
   - Memory %: 0.4%

   With @zap:
   - Additional RSS: 0.6MB
3. CPU Usage
   - Process CPU: 0.0%
   - System Impact: 15.2% total CPU used

4. System Impact
   - Memory Used: 3789.9MB
   - Memory Free: 197.6MB
   - Buffer/Cache: 3942.0MB

## Summary
Comparison between direct Python requests and with @zap prefix:
- Simple plug-and-play integration (just add @zap/ prefix)
- Minimal memory overhead
- Efficient CPU utilization
- Transparent routing through @zap infrastructure
