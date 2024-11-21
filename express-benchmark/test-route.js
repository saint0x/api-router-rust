const express = require('express');
const app = express();

// Test endpoint with simulated latency
app.get('/api/test', async (req, res) => {
    // Simulate some processing
    await new Promise(resolve => setTimeout(resolve, 100));
    
    res.json({
        message: 'Test endpoint',
        timestamp: new Date().toISOString(),
        headers: req.headers,
        query: req.query
    });
});

const port = process.env.PORT || 3000;
app.listen(port, () => {
    console.log(`Test server running on port ${port}`);
});
