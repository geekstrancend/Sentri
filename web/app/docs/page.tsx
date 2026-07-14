'use client'

import Link from 'next/link'
import { ArrowRight, Terminal, BookOpen, Zap, GitBranch, Brain, Code2 } from 'lucide-react'
import { DocsShell } from '@/components/layout/DocsShell'

const QUICK_START_CARDS = [
  {
    icon: <Terminal size={20} className="text-secondary" />,
    title: 'Quick Start',
    description: 'Install the CLI and run your first scan in under 5 minutes.',
    href: '/docs/getting-started',
    cta: 'Get started',
  },
  {
    icon: <Code2 size={20} className="text-secondary" />,
    title: 'CLI Reference',
    description: 'Every command, flag, and configuration option documented.',
    href: '/docs/cli',
    cta: 'View commands',
  },
  {
    icon: <BookOpen size={20} className="text-secondary" />,
    title: 'Invariant Library',
    description: '50+ security checks. Browse and filter by chain and severity.',
    href: '/library',
    cta: 'Browse invariants',
  },
  {
    icon: <GitBranch size={20} className="text-secondary" />,
    title: 'CI/CD Integration',
    description: 'Connect Sentri to GitHub Actions or GitLab CI in one step.',
    href: '/docs/ci-cd',
    cta: 'Set up pipeline',
  },
  {
    icon: <Brain size={20} className="text-secondary" />,
    title: 'AI Co-Auditor',
    description: 'How the AI layer detects protocol-level logical vulnerabilities.',
    href: '/docs/ai',
    cta: 'Learn more',
    badge: 'Pro',
  },
  {
    icon: <Zap size={20} className="text-secondary" />,
    title: 'REST API',
    description: 'Integrate Sentri headlessly into your own tooling and workflows.',
    href: '/docs/api',
    cta: 'API reference',
  },
]

const FEATURED_SNIPPETS = [
  {
    label: 'Install',
    code: 'cargo install sentri-cli',
  },
  {
    label: 'Scan',
    code: 'sentri check ./contracts/ --chain evm',
  },
  {
    label: 'CI',
    code: 'uses: sentri-dev/sentri-action@v1',
  },
]

export default function DocsPage() {
  return (
    <DocsShell pageTitle="Documentation">
      <article className="space-y-16">
        {/* Hero */}
        <div>
          <div className="inline-flex items-center gap-2 px-3 py-1 rounded-full bg-indigo/8 border border-indigo/20 mb-5">
            <span className="text-label-sm text-on-secondary-container">DOCUMENTATION</span>
          </div>
          <h1 className="font-fraunces text-5xl font-[700] text-on-surface mb-4 leading-[1.1]">
            Sentri Documentation
          </h1>
          <p className="text-body-lg text-outline max-w-2xl leading-7">
            Everything you need to audit, secure, and ship smart contracts with confidence. From first scan to CI/CD integration.
          </p>
        </div>

        {/* Quick install bar */}
        <div className="bg-surface-container-lowest border border-outline-variant rounded-xl overflow-hidden">
          <div className="flex items-center gap-0 overflow-x-auto">
            {FEATURED_SNIPPETS.map((s, i) => (
              <div key={i} className={`flex items-center gap-4 px-5 py-3 flex-1 min-w-0 ${i < FEATURED_SNIPPETS.length - 1 ? 'border-r border-outline-variant' : ''}`}>
                <span className="text-label-sm text-outline flex-shrink-0">{s.label}</span>
                <code className="font-mono text-sm text-secondary truncate">{s.code}</code>
              </div>
            ))}
          </div>
        </div>

        {/* Cards grid */}
        <div>
          <h2 className="font-fraunces text-2xl font-[600] text-on-surface mb-6">Explore the Docs</h2>
          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            {QUICK_START_CARDS.map((card, idx) => (
              <Link
                key={idx}
                href={card.href}
                className="group bg-surface-container-low border border-outline-variant rounded-xl p-6 hover:border-indigo transition-all duration-200 hover:bg-surface-container flex flex-col gap-3"
              >
                <div className="flex items-center justify-between">
                  <div className="w-9 h-9 rounded-lg bg-indigo/10 border border-indigo/20 flex items-center justify-center">
                    {card.icon}
                  </div>
                  {card.badge && (
                    <span className="text-xs text-on-secondary-container bg-indigo/15 border border-indigo/20 px-1.5 py-0.5 rounded font-[600]">
                      {card.badge}
                    </span>
                  )}
                </div>
                <div>
                  <h3 className="font-fraunces text-base font-[600] text-on-surface mb-1.5 group-hover:text-secondary transition-colors">
                    {card.title}
                  </h3>
                  <p className="text-body-md text-outline leading-5">{card.description}</p>
                </div>
                <div className="flex items-center gap-1 text-secondary text-xs font-[600] mt-auto group-hover:gap-2 transition-all">
                  {card.cta} <ArrowRight size={12} />
                </div>
              </Link>
            ))}
          </div>
        </div>

        {/* Pro tip box */}
        <div className="bg-indigo/8 border border-indigo/20 rounded-xl p-6">
          <h3 className="font-fraunces text-lg font-[600] text-on-surface mb-4">Pro Tips</h3>
          <ul className="space-y-3">
            {[
              <>Add a <code className="bg-surface-container border border-outline-variant px-1.5 py-0.5 rounded font-mono text-xs text-secondary">SENTRI.md</code> in your repo root to give context to the AI Co-Auditor</>,
              'Integrate Sentri into CI/CD to scan on every pull request before merge',
              'Use custom invariant libraries to define protocol-specific security rules',
              'Share reports directly with your audit team via GitHub and GitLab integrations',
            ].map((tip, i) => (
              <li key={i} className="flex items-start gap-3 text-body-md text-on-surface-variant">
                <span className="text-secondary font-[700] text-sm flex-shrink-0 mt-0.5">{String(i + 1).padStart(2, '0')}</span>
                <span className="leading-5">{tip}</span>
              </li>
            ))}
          </ul>
        </div>
      </article>
    </DocsShell>
  )
}
