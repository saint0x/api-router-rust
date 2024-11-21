async function TestAPI() {
  const baseUrl = process.env.NODE_ENV === 'development' 
    ? 'http://localhost:3000' 
    : 'https://your-production-url.com';
  
  // Test single route
  const userResponse = await fetch(`${baseUrl}/api/users/1`, {
    cache: 'no-store'
  });
  const userData = await userResponse.json();

  // Test batch route
  const batchResponse = await fetch(`${baseUrl}/api`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify([
      { path: 'users/1' },
      { path: 'users/1/posts/1' },
      { path: 'users/2' }
    ]),
    cache: 'no-store'
  });
  const batchData = await batchResponse.json();

  return (
    <div className="space-y-8">
      <div>
        <h2 className="text-xl font-bold mb-2">Single Route Test (/api/users/1)</h2>
        <pre className="bg-gray-100 p-4 rounded-lg overflow-auto">
          {JSON.stringify(userData, null, 2)}
        </pre>
      </div>

      <div>
        <h2 className="text-xl font-bold mb-2">Batch Route Test</h2>
        <pre className="bg-gray-100 p-4 rounded-lg overflow-auto">
          {JSON.stringify(batchData, null, 2)}
        </pre>
      </div>

      <div className="space-y-4">
        <h2 className="text-xl font-bold">Available Routes:</h2>
        <ul className="list-disc list-inside space-y-2">
          <li>GET /api - API info</li>
          <li>POST /api - Batch requests</li>
          <li>GET /api/users - List users</li>
          <li>POST /api/users - Create user</li>
          <li>GET /api/users/[id] - Get user</li>
          <li>PATCH /api/users/[id] - Update user</li>
          <li>DELETE /api/users/[id] - Delete user</li>
          <li>GET /api/users/[userId]/posts/[postId] - Get user post</li>
          <li>PUT /api/users/[userId]/posts/[postId] - Update user post</li>
          <li>DELETE /api/users/[userId]/posts/[postId] - Delete user post</li>
        </ul>
      </div>
    </div>
  );
}

export default function Home() {
  return (
    <main className="min-h-screen p-8">
      <h1 className="text-3xl font-bold mb-8">Next.js Complex Router Demo</h1>
      <TestAPI />
    </main>
  );
}
