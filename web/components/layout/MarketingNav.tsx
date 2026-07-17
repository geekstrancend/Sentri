'use client'

import { useEffect, useState } from 'react'
import Link from 'next/link'
import { usePathname } from 'next/navigation'
import { Menu, X } from 'lucide-react'
import { Button } from '../ui/Button'
import { AuthModal } from '../ui/AuthModal'
import { SentriLogo } from '../ui/SentriLogo'
import { useScrollProgress } from '../hooks/useScrollProgress'
import clsx from 'clsx'

interface MarketingNavProps {
  className?: string
}

const navLinks = [
  { label: 'Product', href: '/#product' },
  { label: 'Features', href: '/#features' },
  { label: 'Library', href: '/library' },
  { label: 'Pricing', href: '/pricing' },
  { label: 'Docs', href: '/docs' },
]

export function MarketingNav({ className }: MarketingNavProps) {
  const [authOpen, setAuthOpen] = useState(false)
  const [authTab, setAuthTab] = useState<'signin' | 'signup'>('signin')
  const [mobileOpen, setMobileOpen] = useState(false)
  const [scrolled, setScrolled] = useState(false)
  const pathname = usePathname()
  const progress = useScrollProgress()

  // Scroll-aware chrome: the nav gains a stronger glass + hairline once the
  // hero scrolls under it, giving spatial feedback (DESIGN.md §5).
  useEffect(() => {
    const onScroll = () => setScrolled(window.scrollY > 8)
    onScroll()
    window.addEventListener('scroll', onScroll, { passive: true })
    return () => window.removeEventListener('scroll', onScroll)
  }, [])

  // Lock body scroll while the mobile sheet is open.
  useEffect(() => {
    document.body.style.overflow = mobileOpen ? 'hidden' : ''
    return () => {
      document.body.style.overflow = ''
    }
  }, [mobileOpen])

  const handleLogIn = () => {
    setAuthTab('signin')
    setAuthOpen(true)
  }
  const handleStartTrial = () => {
    setAuthTab('signup')
    setAuthOpen(true)
  }

  const isActive = (href: string) => {
    const base = href.split('#')[0]
    if (base === '/') return pathname === '/'
    return pathname === base || pathname?.startsWith(base)
  }

  return (
    <>
      <header
        className={clsx(
          'glass-nav sticky top-0 z-40 transition-[border-color,background-color] duration-300',
          scrolled ? 'border-b border-outline-variant' : 'border-b border-transparent',
          className,
        )}
      >
        <div className="mx-auto flex max-w-7xl items-center justify-between px-5 py-3.5 sm:px-6 lg:px-8">
          {/* Logo */}
          <Link
            href="/"
            className="group flex items-center gap-2.5 rounded-lg outline-none focus-visible:outline-2 focus-visible:outline-indigo"
          >
            <SentriLogo size={26} className="transition-transform duration-200 group-hover:scale-105" />
            <span className="text-[0.95rem] font-[600] tracking-tight text-on-surface">Sentri</span>
          </Link>

          {/* Desktop nav */}
          <nav className="hidden items-center gap-1 md:flex" aria-label="Primary">
            {navLinks.map((link) => (
              <Link
                key={link.href}
                href={link.href}
                aria-current={isActive(link.href) ? 'page' : undefined}
                className={clsx(
                  'rounded-lg px-3 py-2 text-[0.9rem] transition-colors',
                  isActive(link.href)
                    ? 'text-on-surface'
                    : 'text-on-surface-variant hover:text-on-surface hover:bg-surface-container-low',
                )}
              >
                {link.label}
              </Link>
            ))}
          </nav>

          {/* Desktop actions */}
          <div className="hidden items-center gap-2 md:flex">
            <Button variant="ghost" size="sm" onClick={handleLogIn}>
              Log in
            </Button>
            <Button variant="primary" size="sm" onClick={handleStartTrial}>
              Start free trial
            </Button>
          </div>

          {/* Mobile trigger */}
          <button
            className="-mr-2 rounded-lg p-2 text-on-surface transition-colors hover:bg-surface-container md:hidden"
            onClick={() => setMobileOpen((v) => !v)}
            aria-label={mobileOpen ? 'Close navigation' : 'Open navigation'}
            aria-expanded={mobileOpen}
          >
            {mobileOpen ? <X size={20} /> : <Menu size={20} />}
          </button>
        </div>

        {/* Mobile sheet */}
        {mobileOpen && (
          <div className="animate-fade-in border-t border-outline-variant bg-surface-container-lowest md:hidden">
            <nav className="space-y-1 px-5 py-4" aria-label="Primary mobile">
              {navLinks.map((link) => (
                <Link
                  key={link.href}
                  href={link.href}
                  className="flex items-center rounded-lg px-3 py-2.5 text-[0.95rem] text-on-surface-variant transition-colors hover:bg-surface-container hover:text-on-surface"
                  onClick={() => setMobileOpen(false)}
                >
                  {link.label}
                </Link>
              ))}
            </nav>
            <div className="flex flex-col gap-2 border-t border-outline-variant px-5 py-4">
              <Button
                variant="secondary"
                fullWidth
                onClick={() => {
                  handleLogIn()
                  setMobileOpen(false)
                }}
              >
                Log in
              </Button>
              <Button
                variant="primary"
                fullWidth
                onClick={() => {
                  handleStartTrial()
                  setMobileOpen(false)
                }}
              >
                Start free trial
              </Button>
            </div>
          </div>
        )}

        {/* Read-progress hairline: scale-only, so it costs no layout work. */}
        <div
          aria-hidden
          className="absolute inset-x-0 bottom-0 h-px origin-left bg-gradient-to-r from-indigo via-indigo-bright to-signal"
          style={{ transform: `scaleX(${progress})` }}
        />
      </header>

      <AuthModal isOpen={authOpen} onClose={() => setAuthOpen(false)} defaultTab={authTab} />
    </>
  )
}
