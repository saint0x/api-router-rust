/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: true,
  output: 'standalone',
  experimental: {
    // Optimize server components
    serverComponentsExternalPackages: [],
    // Optimize middleware
    optimizePackageImports: ['@/lib', '@/components'],
  },
  // Configure headers for API routes
  async headers() {
    return [
      {
        source: '/api/:path*',
        headers: [
          { key: 'Access-Control-Allow-Credentials', value: 'true' },
          { key: 'Access-Control-Allow-Origin', value: '*' },
          { key: 'Access-Control-Allow-Methods', value: 'GET,POST,PUT,DELETE,OPTIONS' },
          { key: 'Access-Control-Allow-Headers', value: 'X-CSRF-Token, X-Requested-With, Accept, Accept-Version, Content-Length, Content-MD5, Content-Type, Date, X-Api-Version' },
          // Add cache control for better performance
          { key: 'Cache-Control', value: 'no-store' },
          // Add timing header
          { key: 'Server-Timing', value: 'miss, dc;desc="origin"' }
        ],
      },
    ]
  },
  // Optimize images (not needed for API benchmarks)
  images: {
    unoptimized: true
  },
  // Minimize during production builds
  swcMinify: true,
  // Configure compiler options
  compiler: {
    // Remove console.log in production
    removeConsole: process.env.NODE_ENV === 'production',
  },
  // Optimize production builds
  productionBrowserSourceMaps: false,
  // Disable telemetry for benchmarking
  telemetry: false
}
