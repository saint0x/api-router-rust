import { NextResponse } from 'next/server';

interface BatchRequest {
    path: string;
    method?: string;
    body?: Record<string, unknown>;
}

// Root API handler
export async function GET() {
    return NextResponse.json({
        message: "API root endpoint",
        version: "1.0.0",
        timestamp: new Date().toISOString()
    });
}

// Batch request handler
export async function POST(request: Request) {
    try {
        const body = await request.json();
        
        if (!Array.isArray(body)) {
            return NextResponse.json(
                { error: "Request body must be an array" },
                { status: 400 }
            );
        }

        // Process each request in the batch
        const results = await Promise.all(body.map(async (req: BatchRequest) => {
            const { path, method = 'GET', body: reqBody } = req;
            
            try {
                const response = await fetch(`${request.url}/${path}`, {
                    method,
                    headers: {
                        'Content-Type': 'application/json',
                    },
                    body: reqBody ? JSON.stringify(reqBody) : undefined,
                });

                const data = await response.json();
                return {
                    status: response.status,
                    data,
                };
            } catch {
                return {
                    status: 500,
                    error: 'Internal request failed',
                };
            }
        }));

        return NextResponse.json(results);
    } catch {
        return NextResponse.json(
            { error: "Invalid request format" },
            { status: 400 }
        );
    }
}
