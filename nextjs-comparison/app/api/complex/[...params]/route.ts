import { NextResponse } from 'next/server';

interface RequestData {
    path: string;
    method: string;
    body?: Record<string, unknown>;
}

export async function GET(
    request: Request,
    { params }: { params: { params: string[] } }
) {
    const searchParams = new URL(request.url).searchParams;
    const limit = parseInt(searchParams.get('limit') || '10');
    const page = parseInt(searchParams.get('page') || '1');

    // Simulate processing delay
    await new Promise(resolve => setTimeout(resolve, Math.random() * 100));

    return NextResponse.json({
        params: params.params,
        query: {
            limit,
            page
        },
        timestamp: new Date().toISOString(),
        data: Array.from({ length: limit }, (_, i) => ({
            id: i + (page - 1) * limit,
            name: `Item ${i + (page - 1) * limit}`,
            timestamp: new Date().toISOString()
        }))
    });
}

export async function POST(
    request: Request,
    { params }: { params: { params: string[] } }
) {
    try {
        const body = await request.json() as RequestData;
        
        // Simulate processing delay
        await new Promise(resolve => setTimeout(resolve, Math.random() * 100));

        return NextResponse.json({
            success: true,
            params: params.params,
            received: body,
            timestamp: new Date().toISOString()
        });
    } catch {
        return NextResponse.json(
            { error: 'Invalid request body' },
            { status: 400 }
        );
    }
}
