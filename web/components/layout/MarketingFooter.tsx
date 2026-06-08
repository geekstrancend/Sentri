import Link from 'next/link'
import { ShieldCheck } from 'lucide-react'

export function MarketingFooter() {
  const currentYear = new Date().getFullYear()

  return (
    <footer className="border-t border-outline-variant bg-surface-container-lowest">
      <div className="max-w-7xl mx-auto px-6 py-12">
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
              <Link href="#" className="block text-outline hover:text-on-surface text-body-md transition-colors">
                Features
              </Link>
              <Link href="#" className="block text-outline hover:text-on-surface text-body-md transition-colors">
                Pricing
              </Link>
              <Link href="#" className="block text-outline hover:text-on-surface text-body-md transition-colors">
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
              <Link href="#" className="block text-outline hover:text-on-surface text-body-md transition-colors">
                Blog
              </Link>
              <Link href="#" className="block text-outline hover:text-on-surface text-body-md transition-colors">
                Community
              </Link>
            </div>
          </div>

          <div>
            <h3 className="text-label-sm text-on-surface mb-4">Legal</h3>
            <div className="space-y-3">
              <Link href="#" className="block text-outline hover:text-on-surface text-body-md transition-colors">
                Privacy Policy
              </Link>
              <Link href="#" className="block text-outline hover:text-on-surface text-body-md transition-colors">
                Terms of Service
              </Link>
              <Link href="#" className="block text-outline hover:text-on-surface text-body-md transition-colors">
                Security Disclosure
              </Link>
            </div>
          </div>
        </div>

        {/* Bottom divider */}
        <div className="border-t border-outline-variant pt-8">
          <div className="flex flex-col md:flex-row justify-between items-center gap-4 text-outline-variant text-xs">
            <p>Made with ❤️ by Sentri Security</p>
            <Link href="#" className="hover:text-outline transition-colors">
              Contact
            </Link>
          </div>
        </div>
      </div>
    </footer>
  )
}
