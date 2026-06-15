'use client'

import { useState } from 'react'
import Link from 'next/link'
import { ShieldCheck } from 'lucide-react'
import { Button } from '../ui/Button'
import { AuthModal } from '../ui/AuthModal'
import clsx from 'clsx'

interface MarketingNavProps {
  className?: string
}

export function MarketingNav({ className }: MarketingNavProps) {
  const [authOpen, setAuthOpen] = useState(false)
  const [authTab, setAuthTab] = useState<'signin' | 'signup'>('signin')

  const handleLogIn = () => {
    setAuthTab('signin')
    setAuthOpen(true)
  }

  const handleStartTrial = () => {
    setAuthTab('signup')
    setAuthOpen(true)
  }

  return (
    <>
      <nav
        className={clsx(
          'sticky top-0 z-40 bg-surface-container-lowest/95 backdrop-blur-sm border-b border-outline-variant',
          className,
        )}
      >
        <div className="max-w-7xl mx-auto px-6 py-4 flex items-center justify-between">
          {/* Logo */}
          <Link href="/" className="flex items-center gap-2 hover:opacity-80 transition-opacity">
            <ShieldCheck size={20} className="text-secondary" />
            <span className="font-mono font-[600] text-on-surface text-base">Sentri</span>
          </Link>

          {/* Center Links */}
          <div className="hidden md:flex items-center gap-8">
            <Link href="#product" className="text-outline hover:text-on-surface text-body-md transition-colors">
              Product
            </Link>
            <Link href="#features" className="text-outline hover:text-on-surface text-body-md transition-colors">
              Features
            </Link>
            <Link href="/library" className="text-outline hover:text-on-surface text-body-md transition-colors">
              Library
            </Link>
            <Link href="/pricing" className="text-outline hover:text-on-surface text-body-md transition-colors">
              Pricing
            </Link>
          </div>

          {/* Right Buttons */}
          <div className="flex items-center gap-3">
            <Button variant="ghost" size="sm" onClick={handleLogIn}>
              Log In
            </Button>
            <Button variant="primary" size="sm" onClick={handleStartTrial}>
              Start Free Trial
            </Button>
          </div>
        </div>
      </nav>

      <AuthModal isOpen={authOpen} onClose={() => setAuthOpen(false)} defaultTab={authTab} />
    </>
  )
}
