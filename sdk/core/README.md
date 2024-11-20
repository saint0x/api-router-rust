# Core Router Implementation

This directory contains the foundational router implementation that powers Zap.rs.

## Files

```
core/
├── router/
│   ├── mod.rs              # Main router implementation
│   ├── handler.rs          # Request handler logic
│   └── response.rs         # Response formatting
├── prefix/
│   ├── mod.rs              # Prefix detection and parsing
│   └── validator.rs        # @zap prefix validation
└── error/
    ├── mod.rs              # Error types and handling
    └── response.rs         # Error response formatting
```

## Component Responsibilities

### Router
- Route registration and management
- Request handling and routing
- Performance optimization
- Thread safety and concurrency

### Prefix Handler
- @zap prefix detection
- Route extraction
- Validation of SDK usage

### Error Handling
- SDK-specific error types
- Error response formatting
- Debug information management

## Integration Points

The core router provides:
1. High-performance request routing
2. Thread-safe handler management
3. Efficient response generation
4. Error handling primitives

This serves as the foundation for all language-specific SDKs.
