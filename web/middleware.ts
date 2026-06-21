import { withAuth } from 'next-auth/middleware'
import { NextRequest } from 'next/server'

export default withAuth(
  function middleware(request: NextRequest) {
    // This is called after the auth check
    // Add any additional logic here if needed
  },
  {
    callbacks: {
      authorized: ({ token }) => !!token,
    },
  }
)

export const config = {
  matcher: ['/dashboard/:path*', '/reports/:path*', '/api/auth/session'],
}
