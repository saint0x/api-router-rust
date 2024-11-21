const express = require('express');
const axios = require('axios');
const app = express();

// Direct API call endpoint
app.get('/api/direct/forecast', async (req, res) => {
    try {
        const response = await axios.get('https://api.open-meteo.com/v1/forecast', {
            params: req.query
        });
        res.json(response.data);
    } catch (error) {
        console.error('Error proxying request:', error.message);
        res.status(500).json({ error: 'Failed to proxy request' });
    }
});

// Endpoint that uses our Rust router with @zap decorator
app.get('/api/zap/forecast', async (req, res) => {
    try {
        const response = await axios.get('http://localhost:3003/api/forecast', {
            params: req.query
        });
        res.json(response.data);
    } catch (error) {
        console.error('Error proxying request:', error.message);
        res.status(500).json({ error: 'Failed to proxy request' });
    }
});

const port = process.env.PORT || 3002;
app.listen(port, () => {
    console.log(`Express server running on port ${port}`);
});
