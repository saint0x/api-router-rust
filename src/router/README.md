# Core Router Implementation

This directory contains the core routing logic, now enhanced with proxy capabilities while maintaining benchmark support.

## Structure

```
router/
├── mod.rs           # Core router implementation
├── proxy.rs         # Proxy routing logic
└── types.rs         # Shared types and constants
```

## Components

### Core Router (mod.rs)
- Route registration and management
- Request handling
- Response processing
- Error handling

### Proxy Handler (proxy.rs)
- URL parsing and validation
- Destination extraction
- Request forwarding
- Response handling
- Connection pooling
- Error recovery

### Types (types.rs)
- Shared type definitions
- Constants
- Error types
- Configuration structures

## Proxy Implementation

1. Request Flow:
```
Incoming Request
    │
    ├── Parse destination from URL
    │   └── Extract from zap.rs/destination/path
    │
    ├── Forward request
    │   ├── Preserve headers
    │   ├── Maintain method
    │   └── Forward body
    │
    └── Return response
        ├── Preserve status
        ├── Forward headers
        └── Stream body
```

2. Performance Features:
- Connection pooling
- Async processing
- Zero-copy forwarding
- Smart retries
- Error handling

## Integration with Benchmarks

- Benchmark routes registered normally
- Proxy handling for external URLs
- Clear separation of concerns
- Independent operation

## Key Considerations

1. Performance:
- Minimal overhead
- Efficient processing
- Resource management
- Connection reuse

2. Reliability:
- Error handling
- Retry logic
- Timeout management
- Circuit breaking

3. Monitoring:
- Request timing
- Error tracking
- Resource usage
- Performance metrics

This implementation maintains the router's high performance while adding proxy capabilities.
