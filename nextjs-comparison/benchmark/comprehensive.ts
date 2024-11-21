import { BenchmarkResults, RouteConfig } from './types';
import { defaultConfig } from './config';
import { LoadGenerator } from './load-generator';

export async function runBenchmark(nextjsUrl: string, rustUrl: string): Promise<{
    nextjs: BenchmarkResults;
    rust: BenchmarkResults;
}> {
    console.log('Starting benchmark...');

    const nextjsConfig = {
        ...defaultConfig,
        routes: defaultConfig.routes.map((r: RouteConfig) => ({ ...r, path: nextjsUrl + r.path }))
    };
    const rustConfig = {
        ...defaultConfig,
        routes: defaultConfig.routes.map((r: RouteConfig) => ({ ...r, path: rustUrl + r.path }))
    };

    const [nextjsResults, rustResults] = await Promise.all([
        new LoadGenerator(nextjsConfig).start(),
        new LoadGenerator(rustConfig).start(),
    ]);

    return {
        nextjs: nextjsResults,
        rust: rustResults,
    };
}

export { LoadGenerator } from './load-generator';
export { defaultConfig } from './config';
