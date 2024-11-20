# Zap.rs - High Performance API Router

A high-performance, concurrent API router implemented in Rust that can be used as a direct proxy for any API endpoint.

## Core Concept

Route any API call through Zap.rs for instant performance benefits:
```
Original: https://api.example.com/data
Through Zap: https://zap.rs/api.example.com/data
```

## How It Works

1. Request Flow:
```
Client Request -> Zap Router -> Destination API
                     │
                     ├── Connection pooling
                     ├── Concurrent processing
                     ├── Automatic retries
                     └── Error handling
```

2. Performance Benefits:
- Async request handling
- Pre-warmed connections
- Smart request coalescing
- Zero-copy operations
- Thread-safe concurrency

3. Added Features:
- Automatic error recovery
- Performance monitoring
- Connection management
- Request optimization
- Reliability improvements

## Project Structure

```
/
├── src/
│   ├── router/           # Core router implementation
│   │   ├── mod.rs       # Main router logic
│   │   └── proxy.rs     # Proxy handling
│   │
│   ├── benchmarks/      # Performance testing endpoints
│   │   ├── mod.rs       # Benchmark route definitions
│   │   └── scenarios.rs # Test scenarios (easy/medium/hard)
│   │
│   └── main.rs          # Server initialization
│
└── tests/               # Integration tests
```

## Implementation Notes

1. Core Router:
- Handles all incoming requests
- Manages connection pools
- Processes responses
- Handles errors

2. Proxy Layer:
- Extracts destination from URL
- Forwards requests
- Manages connections
- Preserves headers

3. Performance:
- Minimal overhead
- Efficient processing
- Smart caching
- Connection reuse

4. Monitoring:
- Request timing
- Error rates
- Latency tracking
- Resource usage

## Development

Benchmark endpoints are separated from core routing to:
1. Keep core router focused
2. Allow independent testing
3. Maintain clean separation
4. Enable easy feature testing

## Future Enhancements

1. Immediate:
- URL parsing
- Header preservation
- Error handling
- Performance monitoring

2. Planned:
- Custom domains
- Rate limiting
- Request validation
- Advanced monitoring
