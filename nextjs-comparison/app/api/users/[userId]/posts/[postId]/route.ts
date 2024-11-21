import { NextResponse } from 'next/server';

interface PostData {
    title: string;
    content: string;
}

export async function GET(
    request: Request,
    { params }: { params: { userId: string; postId: string } }
) {
    return NextResponse.json({
        id: parseInt(params.postId),
        userId: parseInt(params.userId),
        title: 'Sample Post',
        content: 'This is a sample post content',
        created: new Date().toISOString()
    });
}

export async function PUT(
    request: Request,
    { params }: { params: { userId: string; postId: string } }
) {
    try {
        const body = await request.json() as PostData;
        
        return NextResponse.json({
            id: parseInt(params.postId),
            userId: parseInt(params.userId),
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
    { params }: { params: { userId: string; postId: string } }
) {
    return NextResponse.json({
        message: `Post ${params.postId} for user ${params.userId} deleted successfully`,
        timestamp: new Date().toISOString()
    });
}
