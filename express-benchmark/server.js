const express = require('express');
const app = express();

app.use(express.json());

// Data Retrieval
app.get('/api/data/:id', (req, res) => {
    res.json({
        id: req.params.id,
        timestamp: Date.now(),
        data: {
            value: Math.random() * 100,
            type: 'sensor'
        }
    });
});

// Complex Search
app.get('/api/data/search/test', (req, res) => {
    const { limit, offset, sort } = req.query;
    res.json({
        results: Array.from({ length: parseInt(limit) || 10 }, (_, i) => ({
            id: i + (parseInt(offset) || 0),
            value: Math.random() * 100,
            timestamp: Date.now() - i * 1000
        })),
        metadata: { total: 100, limit, offset, sort }
    });
});

// Data Processing
app.post('/api/data/process', (req, res) => {
    const { operations, filters } = req.body;
    res.json({
        processed: true,
        timestamp: Date.now(),
        results: operations.map(op => ({
            type: op.type,
            result: Math.random() * 100
        })),
        filters
    });
});

// Metrics Aggregation
app.get('/api/metrics/aggregate/performance', (req, res) => {
    const { timeRange, granularity } = req.query;
    res.json({
        timeRange,
        granularity,
        metrics: Array.from({ length: 24 }, () => ({
            timestamp: Date.now(),
            value: Math.random() * 100
        }))
    });
});

// Event Analysis
app.post('/api/events/analyze', (req, res) => {
    const { events, config } = req.body;
    res.json({
        analyzed: true,
        timestamp: Date.now(),
        results: events.map(event => ({
            id: event.id,
            score: Math.random() * config.threshold,
            confidence: Math.random()
        }))
    });
});

// Test endpoints
app.get('/api/test', (req, res) => {
    res.json({
        message: 'Test endpoint',
        timestamp: Date.now(),
        headers: req.headers,
        query: req.query
    });
});

app.get('/api/test/rate-limited', (req, res) => {
    res.json({
        message: 'Rate limited endpoint',
        timestamp: Date.now()
    });
});

app.get('/api/test/cached/:userId?', (req, res) => {
    res.json({
        message: 'Cached endpoint',
        userId: req.params.userId,
        timestamp: Date.now()
    });
});

const port = process.env.PORT || 3002;
app.listen(port, () => {
    console.log(`Express server running on port ${port}`);
});
