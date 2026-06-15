'use client'

import { useState } from 'react'
import { X, Eye, EyeOff, Mail, Lock, User } from 'lucide-react'
import { Button } from './Button'

interface AuthModalProps {
  isOpen: boolean
  onClose: () => void
  defaultTab?: 'signin' | 'signup'
}

export function AuthModal({ isOpen, onClose, defaultTab = 'signin' }: AuthModalProps) {
  const [tab, setTab] = useState<'signin' | 'signup'>(defaultTab)
  const [email, setEmail] = useState('')
  const [password, setPassword] = useState('')
  const [showPassword, setShowPassword] = useState(false)
  const [fullName, setFullName] = useState('')

  if (!isOpen) return null

  const handleSignIn = () => {
    console.log('Sign in with:', { email, password })
  }

  const handleSignUp = () => {
    console.log('Sign up with:', { fullName, email, password })
  }

  return (
    <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
      <div className="bg-surface rounded-xl shadow-2xl w-full max-w-md relative">
        {/* Close button */}
        <button
          onClick={onClose}
          className="absolute top-4 right-4 p-1 hover:bg-outline-variant rounded-lg transition"
        >
          <X className="w-5 h-5 text-on-surface" />
        </button>

        {/* Logo / Header */}
        <div className="px-6 pt-6 pb-2">
          <h2 className="text-2xl font-bold text-on-surface mb-1">Sentri</h2>
          <p className="text-sm text-on-surface-variant">Smart contract invariant checking</p>
        </div>

        {/* Tabs */}
        <div className="flex border-b border-outline-variant px-6">
          <button
            onClick={() => setTab('signin')}
            className={`flex-1 py-4 text-sm font-medium transition ${
              tab === 'signin'
                ? 'text-primary border-b-2 border-primary'
                : 'text-on-surface-variant hover:text-on-surface'
            }`}
          >
            Sign In
          </button>
          <button
            onClick={() => setTab('signup')}
            className={`flex-1 py-4 text-sm font-medium transition ${
              tab === 'signup'
                ? 'text-primary border-b-2 border-primary'
                : 'text-on-surface-variant hover:text-on-surface'
            }`}
          >
            Sign Up
          </button>
        </div>

        {/* Content */}
        <div className="px-6 py-6 space-y-4">
          {/* OAuth Buttons */}
          <div className="space-y-2">
            <p className="text-xs text-on-surface-variant text-center mb-3">
              Note: OAuth requires backend configuration
            </p>
            <Button
              variant="secondary"
              className="w-full justify-center gap-2"
              onClick={() => console.log('OAuth: Google')}
            >
              <svg className="w-4 h-4" viewBox="0 0 24 24" fill="currentColor">
                <path d="M22.56 12.25c0-.78-.07-1.53-.2-2.25H12v4.26h5.92c-.26 1.37-1.04 2.53-2.21 3.31v2.77h3.57c2.08-1.92 3.28-4.74 3.28-8.09z" />
                <path d="M12 23c2.97 0 5.46-.98 7.28-2.66l-3.57-2.77c-.98.66-2.23 1.06-3.71 1.06-2.86 0-5.29-1.93-6.16-4.53H2.18v2.84C3.99 20.53 7.7 23 12 23z" />
                <path d="M5.84 14.09c-.22-.66-.35-1.36-.35-2.09s.13-1.43.35-2.09V7.07H2.18C1.43 8.55 1 10.22 1 12s.43 3.45 1.18 4.93l2.85-2.22.81-.62z" />
                <path d="M12 5.38c1.62 0 3.06.56 4.21 1.64l3.15-3.15C17.45 2.09 14.97 1 12 1 7.7 1 3.99 3.47 2.18 7.07l3.66 2.84c.87-2.6 3.3-4.53 6.16-4.53z" />
              </svg>
              Google
            </Button>
            <Button
              variant="secondary"
              className="w-full justify-center gap-2"
              onClick={() => console.log('OAuth: Apple')}
            >
              <svg className="w-4 h-4" viewBox="0 0 24 24" fill="currentColor">
                <path d="M17.05 13.5c-.91 2.04-.39 3.81 1.26 5.05.92.74 2.22.9 3.28.5.37-.15.7-.36.99-.63-.41-.62-.96-1.04-1.49-1.42-.3-.23-.59-.45-.78-.75-.52-.87.12-2.28 1.16-2.75-.38-1.63-1.74-2.73-3.42-2.73-1.18 0-2.3.5-3.24 1.4.6 1.08 1.23 2.13 1.91 3.13.24.38.37.8.37 1.27z" />
                <path d="M5.17 7.28c.67-.84 1.62-1.39 2.68-1.39.9 0 1.73.38 2.31 1 .46-.55.73-1.27.73-2.06 0-1.69-1.37-3.06-3.06-3.06-1.37 0-2.56.88-3 2.1.48.35.92.76 1.34 1.21z" />
              </svg>
              Apple
            </Button>
            <Button
              variant="secondary"
              className="w-full justify-center gap-2"
              onClick={() => console.log('OAuth: Microsoft')}
            >
              <svg className="w-4 h-4" viewBox="0 0 24 24" fill="currentColor">
                <rect x="2" y="2" width="9" height="9" />
                <rect x="13" y="2" width="9" height="9" />
                <rect x="2" y="13" width="9" height="9" />
                <rect x="13" y="13" width="9" height="9" />
              </svg>
              Microsoft
            </Button>
            <Button
              variant="secondary"
              className="w-full justify-center gap-2"
              onClick={() => console.log('OAuth: Yahoo')}
            >
              <svg className="w-4 h-4" viewBox="0 0 24 24" fill="currentColor">
                <path d="M9 15l-3-3m0 0l3-3m-3 3h12M4 4h16a2 2 0 012 2v12a2 2 0 01-2 2H4a2 2 0 01-2-2V6a2 2 0 012-2z" stroke="currentColor" strokeWidth="2" fill="none" />
              </svg>
              Yahoo
            </Button>
          </div>

          <div className="relative">
            <div className="absolute inset-0 flex items-center">
              <div className="w-full border-t border-outline-variant"></div>
            </div>
            <div className="relative flex justify-center text-sm">
              <span className="px-2 bg-surface text-on-surface-variant">or</span>
            </div>
          </div>

          {/* Email / Password Form */}
          <div className="space-y-4">
            {tab === 'signup' && (
              <div>
                <label className="block text-sm font-medium text-on-surface mb-2">
                  Full Name
                </label>
                <div className="relative">
                  <User className="absolute left-3 top-1/2 transform -translate-y-1/2 w-4 h-4 text-on-surface-variant" />
                  <input
                    type="text"
                    value={fullName}
                    onChange={(e) => setFullName(e.target.value)}
                    placeholder="John Doe"
                    className="w-full pl-10 pr-4 py-2.5 bg-surface-variant text-on-surface placeholder-on-surface-variant rounded-lg border border-outline-variant focus:outline-none focus:border-primary transition"
                  />
                </div>
              </div>
            )}

            <div>
              <label className="block text-sm font-medium text-on-surface mb-2">
                Email Address
              </label>
              <div className="relative">
                <Mail className="absolute left-3 top-1/2 transform -translate-y-1/2 w-4 h-4 text-on-surface-variant" />
                <input
                  type="email"
                  value={email}
                  onChange={(e) => setEmail(e.target.value)}
                  placeholder="you@example.com"
                  className="w-full pl-10 pr-4 py-2.5 bg-surface-variant text-on-surface placeholder-on-surface-variant rounded-lg border border-outline-variant focus:outline-none focus:border-primary transition"
                />
              </div>
            </div>

            <div>
              <label className="block text-sm font-medium text-on-surface mb-2">
                Password
              </label>
              <div className="relative">
                <Lock className="absolute left-3 top-1/2 transform -translate-y-1/2 w-4 h-4 text-on-surface-variant" />
                <input
                  type={showPassword ? 'text' : 'password'}
                  value={password}
                  onChange={(e) => setPassword(e.target.value)}
                  placeholder="••••••••"
                  className="w-full pl-10 pr-10 py-2.5 bg-surface-variant text-on-surface placeholder-on-surface-variant rounded-lg border border-outline-variant focus:outline-none focus:border-primary transition"
                />
                <button
                  onClick={() => setShowPassword(!showPassword)}
                  className="absolute right-3 top-1/2 transform -translate-y-1/2 text-on-surface-variant hover:text-on-surface transition"
                >
                  {showPassword ? (
                    <EyeOff className="w-4 h-4" />
                  ) : (
                    <Eye className="w-4 h-4" />
                  )}
                </button>
              </div>
            </div>
          </div>

          {/* Forgot Password / Links */}
          {tab === 'signin' && (
            <div className="text-right">
              <button className="text-sm text-primary hover:text-primary-variant transition">
                Forgot password?
              </button>
            </div>
          )}

          {/* CTA Button */}
          <Button
            className="w-full"
            onClick={tab === 'signin' ? handleSignIn : handleSignUp}
          >
            {tab === 'signin' ? 'Sign In' : 'Create Account'}
          </Button>

          {/* Trial Disclaimer */}
          <p className="text-xs text-on-surface-variant text-center">
            {tab === 'signup' ? (
              <>
                Start your free 14-day trial.
                <br />
                No credit card required.
              </>
            ) : (
              <>
                Don't have an account?{' '}
                <button
                  onClick={() => setTab('signup')}
                  className="text-primary hover:text-primary-variant transition"
                >
                  Sign up
                </button>
              </>
            )}
          </p>
        </div>
      </div>
    </div>
  )
}
