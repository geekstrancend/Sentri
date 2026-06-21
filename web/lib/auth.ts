// Authentication utility functions and hooks

import { useSession } from 'next-auth/react'
import { useCallback } from 'react'

/**
 * Hook to get current user session and loading state
 * Use this in client components to check authentication status
 */
export function useAuth() {
  const { data: session, status, update } = useSession()

  const isLoading = status === 'loading'
  const isAuthenticated = status === 'authenticated'
  const user = session?.user

  return {
    user,
    isLoading,
    isAuthenticated,
    session,
    update,
  }
}

/**
 * Hook to check if user is authenticated
 * Redirects to home page if not authenticated
 */
export function useRequireAuth() {
  const { isAuthenticated, isLoading } = useAuth()

  if (!isLoading && !isAuthenticated) {
    // Component will handle redirect logic
    return false
  }

  return true
}

/**
 * Fetch the current user session
 * Use this in server components or API routes
 */
export async function getSession() {
  const response = await fetch(`/api/auth/session`)
  if (!response.ok) {
    return null
  }
  return response.json()
}

/**
 * Sign out the current user
 */
export async function signOutUser() {
  const response = await fetch(`/api/auth/signout`, { method: 'POST' })
  if (response.ok) {
    // Redirect to home page
    window.location.href = '/'
  }
}
