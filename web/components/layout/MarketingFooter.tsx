import Link from 'next/link'
import { ShieldCheck } from 'lucide-react'
import { AsciiLogo } from '@/components/ui/AsciiLogo'

export function MarketingFooter() {
  const currentYear = new Date().getFullYear()

  return (
    <footer className="border-t border-outline-variant bg-surface-container-lowest">
      <div className="max-w-7xl mx-auto px-6 py-12">
        {/* Full-width ASCII logo header */}
        <div className="overflow-hidden mb-8 -mt-4">
          <AsciiLogo className="text-[6px] sm:text-[10px] md:text-sm opacity-[0.06] leading-none whitespace-pre text-center mx-auto w-full" />
        </div>

        <div className="grid grid-cols-2 md:grid-cols-4 gap-8 mb-12">
          {/* Brand */}
          <div>
            <Link href="/" className="flex items-center gap-2 mb-4">
              <ShieldCheck size={20} className="text-secondary" />
              <span className="font-mono font-[600] text-on-surface">Sentri</span>
            </Link>
            <p className="text-outline text-body-md mb-4">
              Don't get Hacked!
            </p>
            <p className="text-outline-variant text-xs">
              © {currentYear} Sentri Security. All rights reserved.
            </p>
          </div>

          {/* Links Columns */}
          <div>
            <h3 className="text-label-sm text-on-surface mb-4">Product</h3>
            <div className="space-y-3">
              <Link href="/#features" className="block text-outline hover:text-on-surface text-body-md transition-colors">
                Features
              </Link>
              <Link href="/pricing" className="block text-outline hover:text-on-surface text-body-md transition-colors">
                Pricing
              </Link>
              <Link href="/library" className="block text-outline hover:text-on-surface text-body-md transition-colors">
                Security
              </Link>
            </div>
          </div>

          <div>
            <h3 className="text-label-sm text-on-surface mb-4">Resources</h3>
            <div className="space-y-3">
              <Link href="/docs" className="block text-outline hover:text-on-surface text-body-md transition-colors">
                Documentation
              </Link>
              <Link href="https://github.com/geekstrancend/Sentri" target="_blank" rel="noopener" className="block text-outline hover:text-on-surface text-body-md transition-colors">
                GitHub
              </Link>
              <Link href="https://github.com/geekstrancend/Sentri/discussions" target="_blank" rel="noopener" className="block text-outline hover:text-on-surface text-body-md transition-colors">
                Community
              </Link>
            </div>
          </div>

          <div>
            <h3 className="text-label-sm text-on-surface mb-4">Legal</h3>
            <div className="space-y-3">
              <Link href="/privacy" className="block text-outline hover:text-on-surface text-body-md transition-colors">
                Privacy Policy
              </Link>
              <Link href="/terms" className="block text-outline hover:text-on-surface text-body-md transition-colors">
                Terms of Service
              </Link>
              <Link href="https://github.com/geekstrancend/Sentri/security/policy" target="_blank" rel="noopener" className="block text-outline hover:text-on-surface text-body-md transition-colors">
                Security Disclosure
              </Link>
            </div>
          </div>
        </div>

        {/* Bottom divider */}
        <div className="border-t border-outline-variant pt-8">
          <div className="flex flex-col md:flex-row justify-between items-center gap-4 text-outline-variant text-xs">
            <p>Made with ❤️ by Sentri Security</p>
            <a href="mailto:contact@sentri.dev" className="hover:text-outline transition-colors">
              Contact
            </a>
          </div>
        </div>
      </div>
    </footer>
  )
}
