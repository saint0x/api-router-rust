# Next.js vs Custom Rust Router Benchmark Suite

This project provides a comprehensive benchmark comparison between Next.js App Router and a custom Rust router implementation. It tests various aspects of routing performance and capabilities to provide meaningful insights into the performance characteristics of both approaches.

## Features

### Complex Routing Scenarios
- Basic routes
- Dynamic routes with parameters
- Nested dynamic routes
- Catch-all routes
- Complex parameter extraction
- Query parameter handling
- Batch request processing

### Comprehensive Metrics
- Latency (min, max, avg, p95, p99)
- Throughput (requests/second)
- Memory usage
- CPU utilization
- Response time distribution
- Error rates
- Concurrent request handling
- Route matching performance
- Parameter extraction speed
- Batch request efficiency

## Prerequisites

- Node.js 18+ and npm
- Rust and Cargo (latest stable)
- Unix-like environment (Linux/macOS)

## Setup

1. Initialize the project:
```bash
chmod +x scripts/init.sh
./scripts/init.sh
```

This will:
- Create necessary directories
- Install dependencies
- Build both Next.js and Rust applications
- Verify the environment is properly set up

## Running Benchmarks

Run the complete benchmark suite:
```bash
npm run benchmark
```

This will:
1. Start both the Next.js and Rust servers
2. Run comprehensive performance tests
3. Generate detailed analysis and reports
4. Clean up the servers

The benchmark results will be saved in the `results` directory:
- `benchmark_raw.json`: Raw benchmark data
- `benchmark_analysis.json`: Processed analysis
- `benchmark_report.md`: Human-readable report

## Available Scripts

- `npm run dev`: Start Next.js development server
- `npm run build`: Build Next.js application
- `npm run start`: Start Next.js production server
- `npm run benchmark`: Run complete benchmark suite
- `npm run benchmark:analyze`: Analyze existing benchmark data
- `npm run clean`: Clean up build and results directories

## Project Structure

```
nextjs-comparison/
├── app/                    # Next.js application
│   ├── api/               # API routes
│   │   ├── complex/       # Complex routing scenarios
│   │   └── users/         # User-related routes
│   └── page.tsx           # Main page
├── benchmark/             # Benchmark infrastructure
│   ├── analyze.ts         # Results analysis
│   ├── comprehensive.ts   # Benchmark implementation
│   ├── run.ts            # Benchmark runner
│   └── types.ts          # TypeScript types
├── results/              # Benchmark results
├── scripts/             # Utility scripts
└── package.json         # Project configuration
```

## Benchmark Configuration

The benchmark suite is highly configurable. Key settings in `benchmark/comprehensive.ts`:

- Test duration
- Concurrency levels
- Request rates
- Route weights
- Payload sizes
- Error rates
- Cache settings

## Understanding Results

The benchmark generates three types of results:

### 1. Raw Data (`benchmark_raw.json`)
- Complete metrics for every request
- Timing data
- Resource usage
- Error information

### 2. Analysis (`benchmark_analysis.json`)
- Statistical analysis
- Performance comparisons
- Resource usage patterns
- Error rate analysis

### 3. Report (`benchmark_report.md`)
- Human-readable summary
- Key findings
- Performance improvements
- Recommendations

## Contributing

Contributions are welcome! Areas for improvement:

- Additional test scenarios
- Enhanced metrics collection
- Improved analysis
- Better visualization
- Documentation improvements

## License

MIT

## Acknowledgments

This benchmark suite is part of a larger project comparing different routing implementations. Special thanks to the Next.js and Rust communities for their excellent documentation and tools.
