'use client'

import Link from 'next/link'
import { ArrowRight } from 'lucide-react'
import { AsciiLogo } from '@/components/ui/AsciiLogo'
import { Button } from '@/components/ui/Button'
import { MarketingNav } from '@/components/layout/MarketingNav'
import { MarketingFooter } from '@/components/layout/MarketingFooter'

export default function NotFound() {
  return (
    <div className="min-h-screen bg-surface flex flex-col">
      <MarketingNav />

      <main className="flex-1 flex items-center justify-center px-6 py-24">
        <div className="text-center max-w-2xl">
          {/* ASCII Logo */}
          <div className="mb-8 flex justify-center opacity-[0.1] scale-75">
            <AsciiLogo />
          </div>

          {/* 404 Content */}
          <h1 className="font-fraunces text-7xl font-[700] text-on-surface mb-4">
            404
          </h1>

          <h2 className="font-fraunces text-3xl font-[600] text-on-surface mb-4">
            Page Not Found
          </h2>

          <p className="text-body-lg text-on-surface-variant mb-8 leading-7">
            The page you're looking for doesn't exist. It might have been moved, deleted, or you might have mistyped the URL. Let's get you back on track.
          </p>

          {/* Suggestions */}
          <div className="grid grid-cols-1 md:grid-cols-2 gap-4 mb-8">
            <div className="p-4 bg-surface-container rounded-lg border border-outline-variant text-left">
              <h3 className="font-[600] text-on-surface mb-2">Helpful Links</h3>
              <ul className="space-y-1 text-sm text-on-surface-variant">
                <li>
                  <Link href="/" className="text-indigo hover:text-primary transition">
                    Home
                  </Link>
                </li>
                <li>
                  <Link href="/docs" className="text-indigo hover:text-primary transition">
                    Documentation
                  </Link>
                </li>
                <li>
                  <Link href="/library" className="text-indigo hover:text-primary transition">
                    Invariant Library
                  </Link>
                </li>
              </ul>
            </div>

            <div className="p-4 bg-surface-container rounded-lg border border-outline-variant text-left">
              <h3 className="font-[600] text-on-surface mb-2">Common Issues</h3>
              <ul className="space-y-1 text-sm text-on-surface-variant">
                <li>Check if the URL is correct</li>
                <li>Try searching our documentation</li>
                <li>Contact support if the issue persists</li>
              </ul>
            </div>
          </div>

          {/* CTA Buttons */}
          <div className="flex flex-col sm:flex-row gap-3 justify-center">
            <Link href="/">
              <Button variant="primary" icon={<ArrowRight size={18} />} iconPosition="right">
                Go Home
              </Button>
            </Link>
            <a href="mailto:support@sentri.dev">
              <Button variant="secondary">
                Contact Support
              </Button>
            </a>
          </div>
        </div>
      </main>

      <MarketingFooter />
    </div>
  )
}
