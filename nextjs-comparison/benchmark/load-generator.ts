import { performance } from 'perf_hooks';
import {
    RequestMetrics,
    RouteConfig,
    BenchmarkConfig,
    BenchmarkResults,
    RouteStats,
    TimeSeriesDataPoint,
    BenchmarkError
} from './types';

export class LoadGenerator {
    private metrics: RequestMetrics[] = [];
    private startTime: number = 0;
    private config: BenchmarkConfig;
    private running: boolean = false;

    constructor(config: BenchmarkConfig) {
        this.config = config;
    }

    private async makeRequest(route: RouteConfig): Promise<RequestMetrics> {
        const start = performance.now();
        const startMemory = process.memoryUsage().heapUsed;
        const startCPU = process.cpuUsage();

        try {
            // Build URL with query parameters
            let url = route.path;
            if (route.queryParams) {
                const params = route.queryParams();
                const searchParams = new URLSearchParams();
                Object.entries(params).forEach(([key, value]) => {
                    searchParams.append(key, value.toString());
                });
                url += `?${searchParams.toString()}`;
            }

            // Make the request
            const response = await fetch(url, {
                method: route.method,
                headers: {
                    'Content-Type': 'application/json',
                    ...this.config.options.headers
                },
                body: route.payload ? JSON.stringify(route.payload()) : undefined,
            });

            const endMemory = process.memoryUsage().heapUsed;
            const endCPU = process.cpuUsage(startCPU);
            const end = performance.now();

            const success = this.config.options.validateResponse(response);

            return {
                latency: end - start,
                success,
                memoryUsage: endMemory - startMemory,
                cpuUsage: (endCPU.user + endCPU.system) / 1000000,
                timestamp: Date.now(),
                route: route.path,
                method: route.method,
                statusCode: response.status
            };
        } catch (error) {
            const end = performance.now();
            return {
                latency: end - start,
                success: false,
                memoryUsage: 0,
                cpuUsage: 0,
                timestamp: Date.now(),
                route: route.path,
                method: route.method,
                statusCode: 0,
                errorMessage: error instanceof Error ? error.message : 'Unknown error'
            };
        }
    }

    private selectRoute(): RouteConfig {
        const random = Math.random();
        let sum = 0;
        for (const route of this.config.routes) {
            sum += route.weight;
            if (random <= sum) {
                return route;
            }
        }
        return this.config.routes[this.config.routes.length - 1];
    }

    private async worker(): Promise<void> {
        while (this.running) {
            const route = this.selectRoute();
            const metric = await this.makeRequest(route);
            this.metrics.push(metric);

            // Respect target RPS
            const sleepTime = 1000 / this.config.targetRPS;
            await new Promise(resolve => setTimeout(resolve, sleepTime));
        }
    }

    public async start(): Promise<BenchmarkResults> {
        this.running = true;
        this.startTime = Date.now();
        this.metrics = [];

        // Start workers
        const workers = Array.from(
            { length: this.config.concurrency },
            () => this.worker()
        );

        // Run for specified duration
        await new Promise(resolve => setTimeout(resolve, this.config.duration * 1000));
        
        this.running = false;
        await Promise.all(workers);

        return this.calculateResults();
    }

    private calculateResults(): BenchmarkResults {
        const latencies = this.metrics.map(m => m.latency).sort((a, b) => a - b);
        const memoryUsages = this.metrics.map(m => m.memoryUsage);
        const cpuUsages = this.metrics.map(m => m.cpuUsage);
        const successfulRequests = this.metrics.filter(m => m.success);
        const failedRequests = this.metrics.filter(m => !m.success);

        const routeStats = new Map<string, RouteStats>();
        for (const route of this.config.routes) {
            const routeMetrics = this.metrics.filter(m => m.route === route.path);
            if (routeMetrics.length > 0) {
                routeStats.set(route.path, {
                    requestCount: routeMetrics.length,
                    averageLatency: routeMetrics.reduce((sum, m) => sum + m.latency, 0) / routeMetrics.length,
                    errorRate: routeMetrics.filter(m => !m.success).length / routeMetrics.length,
                    throughput: routeMetrics.length / (this.config.duration)
                });
            }
        }

        const timeSeriesData: TimeSeriesDataPoint[] = [];
        const interval = 1000; // 1 second intervals
        for (let t = this.startTime; t < this.startTime + this.config.duration * 1000; t += interval) {
            const periodMetrics = this.metrics.filter(m => m.timestamp >= t && m.timestamp < t + interval);
            if (periodMetrics.length > 0) {
                timeSeriesData.push({
                    timestamp: t,
                    rps: periodMetrics.length,
                    latency: periodMetrics.reduce((sum, m) => sum + m.latency, 0) / periodMetrics.length,
                    memoryUsage: periodMetrics.reduce((sum, m) => sum + m.memoryUsage, 0) / periodMetrics.length,
                    cpuUsage: periodMetrics.reduce((sum, m) => sum + m.cpuUsage, 0) / periodMetrics.length,
                    errorRate: periodMetrics.filter(m => !m.success).length / periodMetrics.length
                });
            }
        }

        const errors: BenchmarkError[] = Array.from(
            failedRequests.reduce((map, error) => {
                const key = error.route;
                const entry = map.get(key) || { route: key, count: 0, examples: [] };
                entry.count++;
                if (entry.examples.length < 5 && error.errorMessage) {
                    entry.examples.push(error.errorMessage);
                }
                return map.set(key, entry);
            }, new Map<string, BenchmarkError>()).values()
        );

        return {
            summary: {
                totalRequests: this.metrics.length,
                successfulRequests: successfulRequests.length,
                failedRequests: failedRequests.length,
                totalDuration: this.config.duration,
                actualRPS: this.metrics.length / this.config.duration,
                averageLatency: latencies.reduce((a, b) => a + b, 0) / latencies.length,
                p50Latency: latencies[Math.floor(latencies.length * 0.5)],
                p95Latency: latencies[Math.floor(latencies.length * 0.95)],
                p99Latency: latencies[Math.floor(latencies.length * 0.99)],
                maxLatency: Math.max(...latencies),
                minLatency: Math.min(...latencies),
                averageMemoryUsage: memoryUsages.reduce((a, b) => a + b, 0) / memoryUsages.length,
                peakMemoryUsage: Math.max(...memoryUsages),
                averageCPUUsage: cpuUsages.reduce((a, b) => a + b, 0) / cpuUsages.length,
                peakCPUUsage: Math.max(...cpuUsages),
                errorRate: failedRequests.length / this.metrics.length
            },
            routeStats,
            timeSeriesData,
            errors
        };
    }
}
