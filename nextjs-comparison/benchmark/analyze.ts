import { writeFileSync } from 'fs';
import { join } from 'path';
import {
    BenchmarkResults,
    TimeSeriesDataPoint,
    RouteStats,
    BenchmarkError
} from './types';

interface ComparisonResults {
    nextjs: BenchmarkResults;
    rust: BenchmarkResults;
}

interface RouteAnalysis {
    latencyImprovement: number;
    throughputImprovement: number;
    errorRateImprovement: number;
}

interface PerformanceComparison {
    latency: {
        improvement: number;  // Percentage improvement in average latency
        p95Improvement: number;
        p99Improvement: number;
    };
    throughput: {
        improvement: number;  // Percentage improvement in RPS
        stabilityScore: number;  // How stable the throughput was (lower variance is better)
    };
    resources: {
        memoryImprovement: number;
        cpuImprovement: number;
    };
    reliability: {
        errorRateImprovement: number;
        stabilityScore: number;
    };
    routeAnalysis: Map<string, RouteAnalysis>;
}

function analyzeResults(results: ComparisonResults): PerformanceComparison {
    const { nextjs, rust } = results;

    // Calculate improvements (positive means Rust is better)
    const latencyImprovement = ((nextjs.summary.averageLatency - rust.summary.averageLatency) / nextjs.summary.averageLatency) * 100;
    const p95Improvement = ((nextjs.summary.p95Latency - rust.summary.p95Latency) / nextjs.summary.p95Latency) * 100;
    const p99Improvement = ((nextjs.summary.p99Latency - rust.summary.p99Latency) / nextjs.summary.p99Latency) * 100;

    const throughputImprovement = ((rust.summary.actualRPS - nextjs.summary.actualRPS) / nextjs.summary.actualRPS) * 100;

    // Calculate throughput stability
    const rustThroughputVariance = calculateVariance(rust.timeSeriesData.map((d: TimeSeriesDataPoint) => d.rps));
    const nextjsThroughputVariance = calculateVariance(nextjs.timeSeriesData.map((d: TimeSeriesDataPoint) => d.rps));
    const throughputStabilityScore = (nextjsThroughputVariance - rustThroughputVariance) / nextjsThroughputVariance;

    // Resource usage improvements
    const memoryImprovement = ((nextjs.summary.averageMemoryUsage - rust.summary.averageMemoryUsage) / nextjs.summary.averageMemoryUsage) * 100;
    const cpuImprovement = ((nextjs.summary.averageCPUUsage - rust.summary.averageCPUUsage) / nextjs.summary.averageCPUUsage) * 100;

    // Error rate analysis
    const errorRateImprovement = ((nextjs.summary.errorRate - rust.summary.errorRate) / (nextjs.summary.errorRate || 1)) * 100;
    const rustErrorVariance = calculateVariance(rust.timeSeriesData.map((d: TimeSeriesDataPoint) => d.errorRate));
    const nextjsErrorVariance = calculateVariance(nextjs.timeSeriesData.map((d: TimeSeriesDataPoint) => d.errorRate));
    const errorStabilityScore = (nextjsErrorVariance - rustErrorVariance) / (nextjsErrorVariance || 1);

    // Per-route analysis
    const routeAnalysis = new Map<string, RouteAnalysis>();
    rust.routeStats.forEach((rustStats: RouteStats, route: string) => {
        const nextjsStats = nextjs.routeStats.get(route);
        if (nextjsStats) {
            routeAnalysis.set(route, {
                latencyImprovement: ((nextjsStats.averageLatency - rustStats.averageLatency) / nextjsStats.averageLatency) * 100,
                throughputImprovement: ((rustStats.throughput - nextjsStats.throughput) / nextjsStats.throughput) * 100,
                errorRateImprovement: ((nextjsStats.errorRate - rustStats.errorRate) / (nextjsStats.errorRate || 1)) * 100,
            });
        }
    });

    return {
        latency: {
            improvement: latencyImprovement,
            p95Improvement,
            p99Improvement,
        },
        throughput: {
            improvement: throughputImprovement,
            stabilityScore: throughputStabilityScore,
        },
        resources: {
            memoryImprovement,
            cpuImprovement,
        },
        reliability: {
            errorRateImprovement,
            stabilityScore: errorStabilityScore,
        },
        routeAnalysis,
    };
}

function calculateVariance(numbers: number[]): number {
    const mean = numbers.reduce((a, b) => a + b, 0) / numbers.length;
    const squareDiffs = numbers.map(n => Math.pow(n - mean, 2));
    return squareDiffs.reduce((a, b) => a + b, 0) / numbers.length;
}

function generateReport(comparison: PerformanceComparison, results: ComparisonResults): string {
    let report = `# Performance Comparison Report\n\n`;

    // Overall Performance Summary
    report += `## Overall Performance Summary\n\n`;
    report += `### Latency\n`;
    report += `- Average Latency Improvement: ${comparison.latency.improvement.toFixed(2)}%\n`;
    report += `- 95th Percentile Improvement: ${comparison.latency.p95Improvement.toFixed(2)}%\n`;
    report += `- 99th Percentile Improvement: ${comparison.latency.p99Improvement.toFixed(2)}%\n\n`;

    report += `### Throughput\n`;
    report += `- Request/Second Improvement: ${comparison.throughput.improvement.toFixed(2)}%\n`;
    report += `- Throughput Stability Score: ${comparison.throughput.stabilityScore.toFixed(2)}\n\n`;

    report += `### Resource Usage\n`;
    report += `- Memory Usage Improvement: ${comparison.resources.memoryImprovement.toFixed(2)}%\n`;
    report += `- CPU Usage Improvement: ${comparison.resources.cpuImprovement.toFixed(2)}%\n\n`;

    report += `### Reliability\n`;
    report += `- Error Rate Improvement: ${comparison.reliability.errorRateImprovement.toFixed(2)}%\n`;
    report += `- Error Stability Score: ${comparison.reliability.stabilityScore.toFixed(2)}\n\n`;

    // Detailed Route Analysis
    report += `## Route-by-Route Analysis\n\n`;
    comparison.routeAnalysis.forEach((analysis: RouteAnalysis, route: string) => {
        report += `### ${route}\n`;
        report += `- Latency Improvement: ${analysis.latencyImprovement.toFixed(2)}%\n`;
        report += `- Throughput Improvement: ${analysis.throughputImprovement.toFixed(2)}%\n`;
        report += `- Error Rate Improvement: ${analysis.errorRateImprovement.toFixed(2)}%\n\n`;
    });

    // Error Analysis
    report += `## Error Analysis\n\n`;
    report += `### Next.js Errors\n`;
    results.nextjs.errors.forEach((error: BenchmarkError) => {
        report += `- ${error.route}: ${error.count} errors\n`;
        report += `  Examples:\n`;
        error.examples.forEach((example: string) => {
            report += `  - ${example}\n`;
        });
    });

    report += `\n### Rust Errors\n`;
    results.rust.errors.forEach((error: BenchmarkError) => {
        report += `- ${error.route}: ${error.count} errors\n`;
        report += `  Examples:\n`;
        error.examples.forEach((example: string) => {
            report += `  - ${example}\n`;
        });
    });

    return report;
}

export async function analyzeAndReport(results: ComparisonResults) {
    const analysis = analyzeResults(results);
    const report = generateReport(analysis, results);
    
    // Save raw results
    writeFileSync(
        join(__dirname, '../results/benchmark_raw.json'),
        JSON.stringify(results, null, 2)
    );

    // Save analysis
    writeFileSync(
        join(__dirname, '../results/benchmark_analysis.json'),
        JSON.stringify(analysis, null, 2)
    );

    // Save report
    writeFileSync(
        join(__dirname, '../results/benchmark_report.md'),
        report
    );

    return {
        analysis,
        report,
    };
}
