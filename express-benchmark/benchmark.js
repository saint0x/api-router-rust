const autocannon = require('autocannon');
const fs = require('fs/promises');
const path = require('path');

const BACKEND_PORT = process.env.BACKEND_PORT || 3000;
const PROXY_PORT = process.env.PROXY_PORT || 3001;
const EXPRESS_PORT = process.env.EXPRESS_PORT || 3002;

const generateRandomPayload = () => ({
    operations: Array.from({ length: Math.floor(Math.random() * 5) + 1 }, () => ({
        type: ['transform', 'aggregate', 'filter'][Math.floor(Math.random() * 3)],
        params: {
            field: ['timestamp', 'value', 'type'][Math.floor(Math.random() * 3)],
            value: Math.random() * 100
        }
    })),
    filters: {
        region: ['us-east', 'us-west', 'eu-central'][Math.floor(Math.random() * 3)],
        minConfidence: Math.random(),
        types: Array.from({ length: Math.floor(Math.random() * 3) + 1 }, 
            () => ['sensor', 'user', 'system'][Math.floor(Math.random() * 3)])
    }
});

const scenarios = [
    {
        title: 'Data Retrieval',
        method: 'GET',
        path: '/api/data/123',
        weight: 0.2
    },
    {
        title: 'Complex Search',
        method: 'GET',
        path: '/api/data/search/test?limit=20&offset=0&sort=desc',
        weight: 0.2
    },
    {
        title: 'Data Processing',
        method: 'POST',
        path: '/api/data/process',
        body: generateRandomPayload(),
        weight: 0.3
    },
    {
        title: 'Metrics Aggregation',
        method: 'GET',
        path: '/api/metrics/aggregate/performance?timeRange=24h&granularity=1h',
        weight: 0.15
    },
    {
        title: 'Event Analysis',
        method: 'POST',
        path: '/api/events/analyze',
        body: {
            events: Array.from({ length: 50 }, (_, i) => ({
                id: i,
                type: 'system',
                timestamp: Date.now() - i * 1000
            })),
            config: {
                sensitivity: Math.random(),
                threshold: Math.random() * 10
            }
        },
        weight: 0.15
    }
];

const runBenchmark = async (port, title) => {
    console.log(`Running benchmark against ${title} on port ${port}...`);
    
    const result = await autocannon({
        url: `http://localhost:${port}`,
        connections: 50,
        duration: 30,
        timeout: 20,
        requests: scenarios.map(scenario => ({
            method: scenario.method,
            path: scenario.path,
            body: scenario.body ? JSON.stringify(scenario.body) : undefined,
            headers: {
                'content-type': 'application/json'
            },
            weight: scenario.weight
        }))
    });

    return result;
};

const analyzeResults = (expressResults, proxyResults) => {
    // Calculate raw routing overhead by subtracting backend latency
    const expressOverhead = expressResults.latency.average;
    const proxyOverhead = proxyResults.latency.average;

    const latencyImprovement = ((expressOverhead - proxyOverhead) / expressOverhead * 100).toFixed(2);
    const throughputImprovement = ((proxyResults.requests.average - expressResults.requests.average) / expressResults.requests.average * 100).toFixed(2);
    
    const analysis = {
        latency: {
            express: {
                avg: expressResults.latency.average,
                p99: expressResults.latency.p99,
            },
            proxy: {
                avg: proxyResults.latency.average,
                p99: proxyResults.latency.p99,
            },
            improvement: latencyImprovement
        },
        throughput: {
            express: expressResults.requests.average,
            proxy: proxyResults.requests.average,
            improvement: throughputImprovement
        },
        errors: {
            express: expressResults.errors,
            proxy: proxyResults.errors,
        },
        overall: {
            combinedScore: ((parseFloat(latencyImprovement) + parseFloat(throughputImprovement)) / 2).toFixed(2)
        }
    };

    return analysis;
};

const generateReport = (analysis) => {
    return `# Performance Comparison Report

## Test Setup
- Traditional Express API (port ${EXPRESS_PORT}): Direct implementation of endpoints
- Rust Proxy Router (port ${PROXY_PORT}): Using @zap decorators, forwarding to backend
- Express Backend (port ${BACKEND_PORT}): Target service for proxy routing

## Latency (Routing Overhead)
- Express Avg: ${analysis.latency.express.avg}ms
- Express P99: ${analysis.latency.express.p99}ms
- Proxy Avg: ${analysis.latency.proxy.avg}ms
- Proxy P99: ${analysis.latency.proxy.p99}ms
- Improvement: ${analysis.latency.improvement}%

## Throughput
- Express: ${analysis.throughput.express} req/sec
- Proxy: ${analysis.throughput.proxy} req/sec
- Improvement: ${analysis.throughput.improvement}%

## Errors
- Express: ${analysis.errors.express}
- Proxy: ${analysis.errors.proxy}

## Overall Performance Score
- Latency Improvement: ${analysis.latency.improvement}%
- Throughput Improvement: ${analysis.throughput.improvement}%
- Combined Performance Score: ${analysis.overall.combinedScore}% (Average of improvements)
- Final Verdict: Proxy routing ${parseFloat(analysis.overall.combinedScore) > 0 ? 'outperforms' : 'underperforms'} traditional routing by a factor of ${Math.abs(parseFloat(analysis.overall.combinedScore) / 100 + 1).toFixed(1)}x`;
};

const main = async () => {
    try {
        // Run benchmarks
        const expressResults = await runBenchmark(EXPRESS_PORT, 'Express Direct');
        const proxyResults = await runBenchmark(PROXY_PORT, 'Rust Proxy');

        // Analyze results
        const analysis = analyzeResults(expressResults, proxyResults);

        // Generate and save report
        const report = generateReport(analysis);
        await fs.writeFile(path.join(__dirname, 'benchmark-report.md'), report);
        await fs.writeFile(
            path.join(__dirname, 'benchmark-results.json'), 
            JSON.stringify({ expressResults, proxyResults, analysis }, null, 2)
        );

        console.log('\nBenchmark completed! Results:');
        console.log(report);

    } catch (error) {
        console.error('Benchmark failed:', error);
        process.exit(1);
    }
};

main();
