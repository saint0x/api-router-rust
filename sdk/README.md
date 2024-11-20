# Zap.rs SDK Core

This directory contains the core SDK implementation for Zap.rs, our high-performance API router.

## Directory Structure

```
sdk/
├── core/           # Core router implementation
├── adapters/       # Language-specific SDK adapters
├── parser/         # Request/prefix parsing logic
├── transport/      # Network transport layer
└── utils/          # Shared utilities
```

## Overview

The SDK structure is designed to:
1. Maintain the high performance of our Rust router
2. Provide easy integration points for different languages
3. Keep the codebase modular and maintainable
4. Enable future extensions without core changes

## Key Components

- Core Router: The foundational Rust implementation
- Adapters: Language-specific SDK implementations
- Parser: Handles @zap prefix and routing logic
- Transport: Manages network communication
- Utils: Shared functionality across components
