const axios = require('axios');
const { performance } = require('perf_hooks');

// Configuration
const EXPRESS_SERVER = 'http://localhost:3002';
const ITERATIONS = 50;  // Total number of requests per endpoint
const DELAY_MS = 500;   // Delay between requests to respect rate limits
const TEST_LOCATIONS = [
    { lat: 52.52, lon: 13.41 },  // Berlin
    { lat: 40.71, lon: -74.01 }, // New York
    { lat: 35.68, lon: 139.69 }, // Tokyo
    { lat: -33.87, lon: 151.21 }, // Sydney
    { lat: 51.51, lon: -0.13 }   // London
];

// Helper to make a request and measure time
async function makeRequest(url, params) {
    const start = performance.now();
    try {
        await axios.get(url, { params });
        const duration = performance.now() - start;
        return { success: true, duration };
    } catch (error) {
        console.error(`Error making request to ${url}:`, error.message);
        return { success: false, duration: 0 };
    }
}

// Sleep helper
const sleep = ms => new Promise(resolve => setTimeout(resolve, ms));

// Run benchmark
async function runBenchmark() {
    const results = {
        zap: [],
        direct: []
    };

    console.log(`Starting benchmark with ${ITERATIONS} iterations per endpoint...`);

    for (let i = 0; i < ITERATIONS; i++) {
        // Pick a random location
        const location = TEST_LOCATIONS[i % TEST_LOCATIONS.length];
        const params = {
            latitude: location.lat,
            longitude: location.lon,
            current: 'temperature_2m,wind_speed_10m',
            hourly: 'temperature_2m,relative_humidity_2m,wind_speed_10m'
        };

        // Test direct API call
        const directResult = await makeRequest(`${EXPRESS_SERVER}/api/direct/forecast`, params);
        if (directResult.success) {
            results.direct.push(directResult.duration);
        }

        // Small delay to respect rate limits
        await sleep(DELAY_MS / 2);

        // Test API call with @zap decorator
        const zapResult = await makeRequest(`${EXPRESS_SERVER}/api/zap/forecast`, params);
        if (zapResult.success) {
            results.zap.push(zapResult.duration);
        }

        // Delay before next iteration
        await sleep(DELAY_MS / 2);

        // Log progress
        if ((i + 1) % 10 === 0) {
            console.log(`Completed ${i + 1}/${ITERATIONS} iterations`);
        }
    }

    // Calculate statistics
    const calculateStats = (durations) => {
        const avg = durations.reduce((a, b) => a + b, 0) / durations.length;
        const sorted = [...durations].sort((a, b) => a - b);
        const median = sorted[Math.floor(sorted.length / 2)];
        const min = Math.min(...durations);
        const max = Math.max(...durations);
        const p95 = sorted[Math.floor(sorted.length * 0.95)];
        return { avg, median, min, max, p95 };
    };

    const zapStats = calculateStats(results.zap);
    const directStats = calculateStats(results.direct);

    console.log('\nResults (in milliseconds):');
    console.log('\nWith @zap Decorator:');
    console.log(`  Average: ${zapStats.avg.toFixed(2)}`);
    console.log(`  Median: ${zapStats.median.toFixed(2)}`);
    console.log(`  Min: ${zapStats.min.toFixed(2)}`);
    console.log(`  Max: ${zapStats.max.toFixed(2)}`);
    console.log(`  P95: ${zapStats.p95.toFixed(2)}`);

    console.log('\nDirect API Calls:');
    console.log(`  Average: ${directStats.avg.toFixed(2)}`);
    console.log(`  Median: ${directStats.median.toFixed(2)}`);
    console.log(`  Min: ${directStats.min.toFixed(2)}`);
    console.log(`  Max: ${directStats.max.toFixed(2)}`);
    console.log(`  P95: ${directStats.p95.toFixed(2)}`);

    // Calculate percentage difference
    const avgDiff = ((zapStats.avg - directStats.avg) / directStats.avg) * 100;
    console.log(`\n@zap decorator routing is ${avgDiff.toFixed(2)}% ${avgDiff > 0 ? 'slower' : 'faster'} than direct API calls`);
}

// Run the benchmark
runBenchmark().catch(console.error);
