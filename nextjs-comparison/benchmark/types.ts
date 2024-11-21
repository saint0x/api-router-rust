export interface RequestMetrics {
    latency: number;
    success: boolean;
    memoryUsage: number;
    cpuUsage: number;
    timestamp: number;
    route: string;
    method: string;
    statusCode: number;
    errorMessage?: string;
}

export interface RouteConfig {
    path: string;
    method: string;
    weight: number;
    payload?: () => any;
    queryParams?: () => Record<string, string | number>;
}

export interface BenchmarkOptions {
    timeout: number;
    followRedirects: boolean;
    headers: Record<string, string>;
    validateResponse: (response: Response) => boolean;
}

export interface BenchmarkConfig {
    duration: number;
    concurrency: number;
    rampUpTime: number;
    coolDownTime: number;
    targetRPS: number;
    routes: RouteConfig[];
    options: BenchmarkOptions;
}

export interface RouteStats {
    requestCount: number;
    averageLatency: number;
    errorRate: number;
    throughput: number;
}

export interface TimeSeriesDataPoint {
    timestamp: number;
    rps: number;
    latency: number;
    memoryUsage: number;
    cpuUsage: number;
    errorRate: number;
}

export interface BenchmarkError {
    route: string;
    count: number;
    examples: string[];
}

export interface BenchmarkResults {
    summary: {
        totalRequests: number;
        successfulRequests: number;
        failedRequests: number;
        totalDuration: number;
        actualRPS: number;
        averageLatency: number;
        p50Latency: number;
        p95Latency: number;
        p99Latency: number;
        maxLatency: number;
        minLatency: number;
        averageMemoryUsage: number;
        peakMemoryUsage: number;
        averageCPUUsage: number;
        peakCPUUsage: number;
        errorRate: number;
    };
    routeStats: Map<string, RouteStats>;
    timeSeriesData: TimeSeriesDataPoint[];
    errors: BenchmarkError[];
}
