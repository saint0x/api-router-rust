# Benchmark Results Directory

This directory contains the results of benchmark comparisons between Next.js App Router and our custom Rust router implementation.

## File Types

- `benchmark_raw.json`: Raw benchmark data including all metrics collected during the test
- `benchmark_analysis.json`: Processed analysis of the benchmark results
- `benchmark_report.md`: Human-readable report summarizing the benchmark findings

## Metrics Collected

The benchmark results include:

### Performance Metrics
- Latency (min, max, avg, p95, p99)
- Throughput (requests/second)
- Response time distribution

### Resource Usage
- Memory usage
- CPU utilization

### Reliability Metrics
- Error rates
- Success/failure ratios
- Error types and frequencies

### Route-specific Analysis
- Per-route performance metrics
- Route matching performance
- Parameter extraction speed

### Batch Processing Metrics
- Batch request efficiency
- Concurrent request handling
- Request batching overhead

## Running Benchmarks

To run a new benchmark:

```bash
npm run benchmark
```

This will:
1. Start both Next.js and Rust servers
2. Run comprehensive performance tests
3. Generate new result files in this directory

## Analyzing Results

To analyze existing benchmark results:

```bash
npm run benchmark:analyze
```

This will process the raw benchmark data and generate new analysis and report files.

## Historical Results

Previous benchmark results are not stored in version control. Each new benchmark run will overwrite the previous results. If you need to keep historical data, make sure to copy the results files to a different location before running new benchmarks.
