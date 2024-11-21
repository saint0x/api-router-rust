const express = require('express');
const app = express();

// Parse JSON bodies
app.use(express.json());

/**
 * @zap({
 *   transform: {
 *     request: {
 *       headers: {
 *         "X-Test-Header": "test-value"
 *       }
 *     },
 *     response: {
 *       headers: {
 *         "X-Response-Header": "response-value"
 *       }
 *     }
 *   },
 *   cache: {
 *     ttl_seconds: 300,
 *     vary_by: ["test"]
 *   },
 *   middleware: ["logger"]
 * })
 */
app.get('/api/test', (req, res) => {
    res.json({
        message: 'Test endpoint',
        timestamp: new Date().toISOString(),
        headers: req.headers,
        query: req.query
    });
});

/**
 * @zap({
 *   transform: {
 *     request: {
 *       query: {
 *         "test": "true"
 *       }
 *     }
 *   },
 *   rateLimit: {
 *     requests_per_second: 10,
 *     burst: 20
 *   }
 * })
 */
app.get('/api/test/rate-limited', (req, res) => {
    res.json({
        message: 'Rate limited endpoint',
        timestamp: new Date().toISOString(),
        query: req.query
    });
});

/**
 * @zap({
 *   cache: {
 *     ttl_seconds: 60,
 *     vary_by: ["userId"]
 *   }
 * })
 */
app.get('/api/test/cached/:userId', (req, res) => {
    res.json({
        message: 'Cached endpoint',
        userId: req.params.userId,
        timestamp: new Date().toISOString()
    });
});

const port = process.env.PORT || 3000;
app.listen(port, () => {
    console.log(`Test server running on port ${port}`);
});
