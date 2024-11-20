# Request Parser

This directory contains the logic for parsing and handling @zap prefixed requests.

## Structure

```
parser/
├── prefix/
│   ├── detector.rs        # @zap prefix detection
│   └── validator.rs       # Prefix validation rules
├── route/
│   ├── extractor.rs       # Route path extraction
│   └── transformer.rs     # Route transformation
└── headers/
    ├── generator.rs       # Zap-specific headers
    └── validator.rs       # Header validation
```

## Component Responsibilities

### Prefix Detection
- Identifies @zap prefixed requests
- Validates prefix format
- Extracts SDK version information
- Handles prefix variations

### Route Processing
- Extracts original route from request
- Transforms routes for internal routing
- Handles path parameters
- Manages query strings

### Header Management
- Generates Zap-specific headers
- Validates required headers
- Manages metadata headers
- Handles custom headers

## Integration Flow

1. Request Received
   - Detect @zap prefix
   - Validate prefix format
   - Extract route information

2. Route Processing
   - Transform route for internal use
   - Handle path parameters
   - Process query strings

3. Header Generation
   - Add Zap-specific headers
   - Include metadata
   - Validate requirements

## Performance Considerations

- Minimal string allocations
- Efficient prefix detection
- Zero-copy where possible
- Cache-friendly operations

This component is critical for maintaining low overhead while providing SDK functionality.
