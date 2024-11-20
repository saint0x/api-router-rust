# Benchmark Scenarios

This directory contains the benchmark endpoints and test scenarios separated from the core routing logic.

## Structure

```
benchmarks/
├── mod.rs           # Exports and route registration
├── scenarios.rs     # Test endpoint implementations
└── types.rs         # Shared types and utilities
```

## Endpoint Categories

1. Simple (Baseline)
- Endpoint: GET /ping
- Purpose: Basic performance baseline
- Response: Simple JSON with timestamp
- Use: Measuring minimal overhead

2. Medium (API Simulation)
- Endpoint: GET /api/v1/data
- Purpose: Typical API workload
- Response: Structured JSON with nested data
- Use: Real-world performance testing

3. Complex (Processing)
- Endpoint: POST /api/v1/process
- Purpose: Heavy workload simulation
- Response: Processed request data
- Use: Stress testing and optimization

## Usage

These endpoints are used for:
1. Feature testing
2. Performance benchmarking
3. Optimization validation
4. Regression testing

## Integration

The benchmarks can be:
1. Enabled/disabled via configuration
2. Run alongside proxy functionality
3. Used for continuous performance testing
4. Deployed in test environments

## Note

Keep these separate from core routing to:
- Maintain clean separation of concerns
- Allow independent testing
- Prevent benchmark code from affecting production
- Enable easy feature comparison
