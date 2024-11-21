import os from 'os';
import { BenchmarkConfig } from './types';

export const defaultConfig: BenchmarkConfig = {
    duration: 300, // 5 minutes for thorough testing
    concurrency: os.cpus().length * 2,
    rampUpTime: 30,
    coolDownTime: 30,
    targetRPS: 2000,
    routes: [
        // Basic routes
        {
            path: '/api',
            method: 'GET',
            weight: 0.1,
        },
        {
            path: '/api/users',
            method: 'GET',
            weight: 0.15,
        },
        {
            path: '/api/users/1',
            method: 'GET',
            weight: 0.15,
        },
        // Complex route scenarios
        {
            path: '/api/complex/users/search',
            method: 'GET',
            weight: 0.15,
            queryParams: () => ({
                page: Math.floor(Math.random() * 10) + 1,
                limit: [10, 20, 50, 100][Math.floor(Math.random() * 4)],
                sort: ['asc', 'desc'][Math.floor(Math.random() * 2)],
                fields: ['name', 'email', 'status', 'created_at']
                    .slice(0, Math.floor(Math.random() * 4) + 1)
                    .join(',')
            })
        },
        // User operations
        {
            path: '/api/users',
            method: 'POST',
            weight: 0.15,
            payload: () => ({
                name: `Test User ${Math.random()}`,
                email: `user${Math.random()}@example.com`
            })
        },
        {
            path: '/api/users/1',
            method: 'PATCH',
            weight: 0.15,
            payload: () => ({
                name: `Updated User ${Math.random()}`
            })
        },
        // Nested routes
        {
            path: '/api/users/1/posts/1',
            method: 'GET',
            weight: 0.1,
        },
        {
            path: '/api/users/1/posts/1',
            method: 'PUT',
            weight: 0.05,
            payload: () => ({
                title: `Updated Post ${Math.random()}`,
                content: `Updated content ${Math.random()}`
            })
        }
    ],
    options: {
        timeout: 10000,
        followRedirects: true,
        headers: {
            'Accept': 'application/json',
            'X-Test-Client': 'benchmark-suite'
        },
        validateResponse: (response: Response) => {
            if (!response.ok) return false;
            const contentType = response.headers.get('content-type');
            return contentType?.includes('application/json') ?? false;
        }
    }
};
