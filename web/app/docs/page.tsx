'use client'

import Link from 'next/link'
import { DocsShell } from '@/components/layout/DocsShell'
import { ArrowRight, BookOpen, Zap, Layers, Code2 } from 'lucide-react'

export default function DocsPage() {
  const sections = [
    {
      title: 'Getting Started',
      description: 'Install Sentri CLI and run your first security scan in minutes.',
      icon: <Zap size={24} className="text-secondary" />,
      href: '/docs/getting-started',
    },
    {
      title: 'CLI Reference',
      description: 'Complete command-line interface documentation and options.',
      icon: <Code2 size={24} className="text-secondary" />,
      href: '/docs/cli',
    },
    {
      title: 'Invariant Library',
      description: 'Browse 1,400+ security invariants and understand detection patterns.',
      icon: <Layers size={24} className="text-secondary" />,
      href: '/library',
    },
    {
      title: 'AI Co-Auditor',
      description: 'Learn how our reasoning engine detects complex logical vulnerabilities.',
      icon: <BookOpen size={24} className="text-secondary" />,
      href: '/docs/ai',
    },
  ]

  return (
    <DocsShell pageTitle="Protocol Docs">
      <article className="space-y-12">
        {/* Hero */}
        <div className="text-center">
          <h1 className="font-fraunces text-5xl font-[600] text-on-surface mb-4">
            Security Documentation
          </h1>
          <p className="text-body-lg text-outline max-w-2xl mx-auto">
            Learn how to use Sentri to secure your smart contract protocols. From CLI basics to advanced AI-powered analysis.
          </p>
        </div>

        {/* Quick Links Grid */}
        <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
          {sections.map((section, idx) => (
            <Link
              key={idx}
              href={section.href}
              className="group bg-surface-container-low border border-outline-variant rounded-lg p-6 hover:border-indigo transition-colors"
            >
              <div className="flex items-start gap-4">
                <div className="flex-shrink-0">{section.icon}</div>
                <div className="flex-1">
                  <h3 className="font-fraunces text-lg font-[600] text-on-surface mb-2 group-hover:text-indigo transition-colors">
                    {section.title}
                  </h3>
                  <p className="text-body-md text-outline mb-4">{section.description}</p>
                  <div className="flex items-center gap-1 text-medium group-hover:gap-2 transition-all">
                    <span className="text-body-md font-[600]">Learn more</span>
                    <ArrowRight size={16} />
                  </div>
                </div>
              </div>
            </Link>
          ))}
        </div>

        {/* Featured Section */}
        <div className="bg-indigo/10 border border-indigo/30 rounded-lg p-8">
          <h2 className="font-fraunces text-2xl font-[600] text-on-surface mb-4">
            Pro Tips
          </h2>
          <ul className="space-y-3 text-body-md text-on-surface-variant">
            <li>
              ✓ Create a <code className="bg-surface-container border border-outline-variant px-1.5 py-0.5 rounded font-mono text-xs">SENTRI.md</code> in your repo root to provide context to the AI Co-Auditor
            </li>
            <li>
              ✓ Integrate Sentri into your CI/CD pipeline to scan on every pull request
            </li>
            <li>
              ✓ Use custom invariant libraries to define protocol-specific security rules
            </li>
            <li>
              ✓ Share reports directly with your audit team via GitHub and GitLab integrations
            </li>
          </ul>
        </div>

        {/* API Reference Link */}
        <div className="text-center pt-8 border-t border-outline-variant">
          <p className="text-body-md text-outline mb-4">
            Looking for API documentation?
          </p>
          <Link
            href="#"
            className="inline-flex items-center gap-2 text-indigo font-[600] hover:text-indigo/80 transition-colors"
          >
            View REST API Reference
            <ArrowRight size={16} />
          </Link>
        </div>
      </article>
    </DocsShell>
  )
}
