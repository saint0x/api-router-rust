import { spawn, ChildProcess, exec } from 'child_process';
import { runBenchmark } from './comprehensive';
import { analyzeAndReport } from './analyze';
import { join } from 'path';
import { promisify } from 'util';

const execAsync = promisify(exec);
const NEXTJS_PORT = process.env.NEXTJS_PORT || 3002;
const RUST_PORT = process.env.RUST_PORT || 3003;
const RUST_PROJECT_DIR = join(__dirname, '../../');

async function main() {
    try {
        console.log('Starting benchmark...');

        // Run benchmarks against already running servers
        const results = await runBenchmark(
            `http://localhost:${NEXTJS_PORT}`,
            `http://localhost:${RUST_PORT}`
        );

        // Analyze results and generate report
        const { analysis, report } = await analyzeAndReport(results);

        console.log('\nBenchmark completed successfully!');
        console.log('Results have been saved to the results directory');
        console.log('\nKey findings:');
        console.log(`- Latency Improvement: ${analysis.latency.improvement.toFixed(2)}%`);
        console.log(`- Throughput Improvement: ${analysis.throughput.improvement.toFixed(2)}%`);
        console.log(`- Memory Usage Improvement: ${analysis.resources.memoryImprovement.toFixed(2)}%`);
        console.log(`- CPU Usage Improvement: ${analysis.resources.cpuImprovement.toFixed(2)}%`);

    } catch (error) {
        console.error('Benchmark failed:', error);
        process.exit(1);
    }
}

// Run the benchmark
main().catch(error => {
    console.error('Fatal error:', error);
    process.exit(1);
});
