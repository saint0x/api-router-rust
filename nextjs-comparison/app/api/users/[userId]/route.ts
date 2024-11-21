import { NextResponse } from 'next/server';

interface UserUpdateData {
    name?: string;
    email?: string;
}

export async function GET(
    request: Request,
    { params }: { params: { userId: string } }
) {
    return NextResponse.json({
        id: parseInt(params.userId),
        name: 'John Doe',
        email: 'john@example.com',
        created: new Date().toISOString()
    });
}

export async function PATCH(
    request: Request,
    { params }: { params: { userId: string } }
) {
    try {
        const body = await request.json() as UserUpdateData;
        return NextResponse.json({
            id: parseInt(params.userId),
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

export async function DELETE(
    request: Request,
    { params }: { params: { userId: string } }
) {
    return NextResponse.json({
        message: `User ${params.userId} deleted successfully`,
        timestamp: new Date().toISOString()
    });
}
