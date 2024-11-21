const axios = require('axios');

// Test route with @zap decorators
const testRoute = `
/**
 * @zap({
 *   transform: {
 *     request: {
 *       headers: {
 *         "X-Test-Header": "test-value"
 *       },
 *       query: {
 *         "test": "true"
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
 *   rateLimit: {
 *     requests_per_second: 10,
 *     burst: 20
 *   },
 *   middleware: ["logger", "metrics"]
 * })
 */
`;

async function runTests() {
    try {
        console.log('Testing proxy server...');

        // Test basic request
        console.log('\nTesting basic request:');
        const basicResponse = await axios.get('http://localhost:3001/api/test');
        console.log('Basic response:', basicResponse.data);
        console.log('Response headers:', basicResponse.headers);

        // Test with query parameters
        console.log('\nTesting with query parameters:');
        const queryResponse = await axios.get('http://localhost:3001/api/test?param=value');
        console.log('Query response:', queryResponse.data);

        // Test with custom headers
        console.log('\nTesting with custom headers:');
        const headerResponse = await axios.get('http://localhost:3001/api/test', {
            headers: {
                'X-Custom-Header': 'custom-value'
            }
        });
        console.log('Header response:', headerResponse.data);

        // Test rate limiting
        console.log('\nTesting rate limiting:');
        const requests = Array(15).fill().map(() => 
            axios.get('http://localhost:3001/api/test')
        );
        const rateResults = await Promise.allSettled(requests);
        console.log('Rate limit results:', {
            successful: rateResults.filter(r => r.status === 'fulfilled').length,
            failed: rateResults.filter(r => r.status === 'rejected').length
        });

        console.log('\nAll tests completed successfully!');
    } catch (error) {
        console.error('Test failed:', error.response?.data || error.message);
    }
}

// Run the tests
runTests();
