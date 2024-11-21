import { NextResponse } from 'next/server';

interface UserData {
    name: string;
    email: string;
}

export async function GET() {
    return NextResponse.json([
        { id: 1, name: 'John Doe', email: 'john@example.com' },
        { id: 2, name: 'Jane Smith', email: 'jane@example.com' }
    ]);
}

export async function POST(request: Request) {
    try {
        const body = await request.json() as UserData;
        
        return NextResponse.json({
            id: Math.floor(Math.random() * 1000),
            ...body,
            created: new Date().toISOString()
        }, { status: 201 });
    } catch {
        return NextResponse.json(
            { error: "Invalid request body" },
            { status: 400 }
        );
    }
}

export async function PUT(request: Request) {
    try {
        const body = await request.json() as UserData;
        
        return NextResponse.json({
            ...body,
            updated: new Date().toISOString()
        });
    } catch {
        return NextResponse.json(
            { error: "Invalid request body" },
            { status: 400 }
        );
    }
}

export async function DELETE() {
    return NextResponse.json({
        message: "User deleted",
        timestamp: new Date().toISOString()
    });
}
