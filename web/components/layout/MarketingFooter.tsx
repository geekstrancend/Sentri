import Link from 'next/link'
import { ShieldCheck, Github, Twitter, ExternalLink } from 'lucide-react'
import { Container } from '../ui/Section'

const columns: {
  title: string
  links: { label: string; href: string; external?: boolean }[]
}[] = [
  {
    title: 'Product',
    links: [
      { label: 'Features', href: '/#features' },
      { label: 'Pricing', href: '/pricing' },
      { label: 'Security Library', href: '/library' },
      { label: 'Changelog', href: '/docs' },
    ],
  },
  {
    title: 'Resources',
    links: [
      { label: 'Documentation', href: '/docs' },
      { label: 'Getting Started', href: '/docs/getting-started' },
      { label: 'CLI Reference', href: '/docs/cli' },
      { label: 'CI/CD Guide', href: '/docs/ci-cd' },
      { label: 'GitHub', href: 'https://github.com/geekstrancend/Sentri', external: true },
    ],
  },
  {
    title: 'Company',
    links: [
      { label: 'Contact Sales', href: '/contact' },
      { label: 'Privacy Policy', href: '/privacy' },
      { label: 'Terms of Service', href: '/terms' },
      {
        label: 'Security Disclosure',
        href: 'https://github.com/geekstrancend/Sentri/security/policy',
        external: true,
      },
    ],
  },
]

export function MarketingFooter() {
  const currentYear = new Date().getFullYear()

  return (
    <footer className="relative mt-24 border-t border-outline-variant bg-surface-container-lowest">
      {/* faint top edge glow */}
      <div
        aria-hidden
        className="pointer-events-none absolute inset-x-0 top-0 h-px bg-gradient-to-r from-transparent via-indigo/40 to-transparent"
      />
      <Container className="pb-8 pt-16">
        <div className="mb-16 grid grid-cols-2 gap-10 md:grid-cols-5">
          {/* Brand */}
          <div className="col-span-2 pr-8">
            <Link href="/" className="mb-4 flex items-center gap-2">
              <span className="flex h-8 w-8 items-center justify-center rounded-lg border border-indigo/30 bg-indigo/10">
                <ShieldCheck size={17} className="text-indigo-bright" />
              </span>
              <span className="text-[0.95rem] font-[600] text-on-surface">Sentri</span>
            </Link>
            <p className="mb-6 max-w-xs text-body-md leading-6 text-on-surface-variant">
              The invariant-driven smart contract security engine. Audit faster. Find more. Miss
              nothing.
            </p>
            <div className="flex items-center gap-2">
              <a
                href="https://github.com/geekstrancend/Sentri"
                target="_blank"
                rel="noopener noreferrer"
                className="flex h-9 w-9 items-center justify-center rounded-lg border border-outline-variant bg-surface-container text-outline transition-colors hover:border-indigo/50 hover:text-on-surface"
                aria-label="Sentri on GitHub"
              >
                <Github size={16} />
              </a>
              <a
                href="https://twitter.com/sentrisec"
                target="_blank"
                rel="noopener noreferrer"
                className="flex h-9 w-9 items-center justify-center rounded-lg border border-outline-variant bg-surface-container text-outline transition-colors hover:border-indigo/50 hover:text-on-surface"
                aria-label="Sentri on X / Twitter"
              >
                <Twitter size={16} />
              </a>
            </div>
          </div>

          {columns.map((col) => (
            <nav key={col.title} aria-label={col.title}>
              <h3 className="mb-5 text-label-sm text-on-surface-variant">{col.title}</h3>
              <ul className="space-y-3">
                {col.links.map((link) => (
                  <li key={link.href}>
                    <Link
                      href={link.href}
                      target={link.external ? '_blank' : undefined}
                      rel={link.external ? 'noopener noreferrer' : undefined}
                      className="inline-flex items-center gap-1 text-body-md text-outline transition-colors hover:text-on-surface"
                    >
                      {link.label}
                      {link.external && <ExternalLink size={11} className="opacity-60" />}
                    </Link>
                  </li>
                ))}
              </ul>
            </nav>
          ))}
        </div>

        <div className="section-divider mb-8" />

        <div className="flex flex-col items-center justify-between gap-4 md:flex-row">
          <div className="flex items-center gap-3">
            <span className="inline-flex items-center gap-2 font-mono text-xs text-outline">
              <span className="relative flex h-2 w-2">
                <span className="absolute inline-flex h-full w-full animate-ping rounded-full bg-signal/60" />
                <span className="relative inline-flex h-2 w-2 rounded-full bg-signal" />
              </span>
              All systems operational
            </span>
          </div>
          <div className="flex items-center gap-6 text-xs text-outline">
            <span className="font-mono">© {currentYear} Sentri Security, Inc.</span>
            <Link href="/privacy" className="transition-colors hover:text-on-surface">
              Privacy
            </Link>
            <Link href="/terms" className="transition-colors hover:text-on-surface">
              Terms
            </Link>
          </div>
        </div>
      </Container>
    </footer>
  )
}
