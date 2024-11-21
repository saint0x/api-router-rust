const express = require('express');
const app = express();
const crypto = require('crypto');

app.use(express.json());

// Simulate database latency
const simulateDbLatency = () => new Promise(resolve => 
    setTimeout(resolve, Math.random() * 50)
);

// Complex data generation
const generateComplexData = (size) => {
    return Array.from({ length: size }, (_, i) => ({
        id: crypto.randomUUID(),
        timestamp: Date.now(),
        metadata: {
            region: ['us-east', 'us-west', 'eu-central'][Math.floor(Math.random() * 3)],
            priority: Math.floor(Math.random() * 10),
            tags: Array.from({ length: Math.floor(Math.random() * 5) + 1 }, 
                () => crypto.randomBytes(8).toString('hex'))
        },
        data: {
            values: Array.from({ length: Math.floor(Math.random() * 20) + 1 }, 
                () => Math.random() * 1000),
            type: ['sensor', 'user', 'system'][Math.floor(Math.random() * 3)],
            status: ['active', 'pending', 'error'][Math.floor(Math.random() * 3)]
        }
    }));
};

// Complex routes
app.get('/api/data/:id', async (req, res) => {
    await simulateDbLatency();
    res.json(generateComplexData(1)[0]);
});

app.get('/api/data/search/:query', async (req, res) => {
    const { query } = req.params;
    const { limit = 10, offset = 0, sort = 'desc' } = req.query;
    await simulateDbLatency();
    res.json({
        query,
        results: generateComplexData(parseInt(limit)),
        metadata: {
            total: 1000,
            offset: parseInt(offset),
            sort,
            processingTime: Math.random() * 100
        }
    });
});

app.post('/api/data/process', async (req, res) => {
    const { operations = [], filters = {} } = req.body;
    await simulateDbLatency();
    const results = generateComplexData(Math.floor(Math.random() * 50) + 10);
    res.json({
        results,
        metadata: {
            operations,
            filters,
            processingTime: Math.random() * 200,
            resultCount: results.length
        }
    });
});

app.get('/api/metrics/aggregate/:type', async (req, res) => {
    const { type } = req.params;
    const { timeRange, granularity } = req.query;
    await simulateDbLatency();
    const dataPoints = Math.floor(Math.random() * 100) + 50;
    res.json({
        type,
        timeRange,
        granularity,
        data: Array.from({ length: dataPoints }, (_, i) => ({
            timestamp: Date.now() - i * 60000,
            value: Math.random() * 1000,
            confidence: Math.random()
        }))
    });
});

app.post('/api/events/analyze', async (req, res) => {
    const { events, config } = req.body;
    await simulateDbLatency();
    res.json({
        analysis: {
            patterns: Array.from({ length: Math.floor(Math.random() * 5) + 1 }, () => ({
                type: ['anomaly', 'trend', 'spike'][Math.floor(Math.random() * 3)],
                confidence: Math.random(),
                impact: Math.random() * 10
            })),
            summary: {
                totalEvents: events?.length || Math.floor(Math.random() * 1000),
                processedAt: new Date().toISOString(),
                config
            }
        }
    });
});

const port = process.env.EXPRESS_PORT || 3002;
app.listen(port, () => {
    console.log(`Express server running on port ${port}`);
});
