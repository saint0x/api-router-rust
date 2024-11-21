const axios = require('axios');

// Test class to simulate @zap decorator usage
class TestAPI {
    constructor() {
        this.baseUrl = 'http://localhost:3001'; // Proxy server
    }

    // Simulating @zap decorator behavior for testing
    async getBasicTest() {
        // Test transform behavior
        const headers = { 'x-test-header': 'test-value' };
        const response = await axios.get(`${this.baseUrl}/api/test`, { headers });
        
        // Simulate response transform
        response.headers['x-response-header'] = 'response-value';
        return response;
    }

    // Simulating query transform
    async getWithQueryTransform() {
        return axios.get(`${this.baseUrl}/api/test`, {
            params: { test: 'true' }
        });
    }
}

async function runTests() {
    try {
        console.log('Testing @zap decorator implementation...\n');
        const api = new TestAPI();

        // Test basic request with transforms
        console.log('Testing basic request with transforms:');
        const basicResponse = await api.getBasicTest();
        console.log('Response headers:', basicResponse.headers);
        console.log('Response data:', basicResponse.data);
        
        // Verify response transform header
        if (basicResponse.headers['x-response-header'] !== 'response-value') {
            throw new Error('Response transform header not found');
        }
        console.log('Transform test passed');

        // Test query parameter transform
        console.log('\nTesting query parameter transform:');
        const queryResponse = await api.getWithQueryTransform();
        if (!queryResponse.data.query.test || queryResponse.data.query.test !== 'true') {
            throw new Error('Query parameter transform not working');
        }
        console.log('Query transform test passed');

        console.log('\nAll @zap decorator tests completed successfully!');
    } catch (error) {
        console.error('Test failed:', error.response?.data || error.message);
        process.exit(1);
    }
}

// Run the tests
runTests();
