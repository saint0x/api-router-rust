# Language-Specific SDK Adapters

This directory contains the adapter implementations for different programming languages and frameworks.

## Structure

```
adapters/
├── typescript/
│   ├── core.ts            # Core TypeScript implementation
│   ├── react.ts           # React-specific hooks
│   └── vue.ts             # Vue-specific composables
├── python/
│   ├── core.py            # Core Python implementation
│   ├── async.py           # Async support
│   └── django.py          # Django integration
├── java/
│   ├── ZapCore.java       # Core Java implementation
│   └── Spring.java        # Spring framework support
└── go/
    └── zap.go             # Go implementation
```

## Adapter Responsibilities

Each adapter must implement:

1. Core Functionality
   - @zap prefix handling
   - Request interception
   - Error handling
   - Response processing

2. Language-Specific Features
   - Native HTTP client wrapping
   - Idiomatic error handling
   - Framework integrations
   - Type definitions

3. Developer Experience
   - Language-appropriate syntax
   - Framework-specific utilities
   - Type safety where applicable
   - Debugging helpers

## Integration Pattern

Each adapter follows a common pattern:
1. Intercept @zap prefixed calls
2. Transform to core router format
3. Handle response/errors idiomatically
4. Provide language-specific utilities

## Key Considerations

- Minimal dependencies
- Native language patterns
- Framework compatibility
- Type safety where possible
- Consistent behavior across languages
