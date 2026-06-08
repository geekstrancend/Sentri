'use client'

import { ShieldCheck, BookOpen, Brain, RefreshCw, ArrowRight, Check, HelpCircle } from 'lucide-react'
import { MarketingNav } from '@/components/layout/MarketingNav'
import { MarketingFooter } from '@/components/layout/MarketingFooter'
import { Button } from '@/components/ui/Button'
import { Terminal } from '@/components/ui/Terminal'
import { SeverityBadge } from '@/components/ui/SeverityBadge'

export default function HomePage() {
  return (
    <div className="min-h-screen bg-surface flex flex-col">
      <MarketingNav />

      {/* Hero Section */}
      <section className="flex-1 px-6 py-20 text-center max-w-6xl mx-auto">
        {/* Eyebrow Badge */}
        <div className="inline-flex items-center gap-2 px-4 py-2 rounded-full bg-indigo/8 border border-indigo/20 mb-6">
          <ShieldCheck size={14} className="text-on-secondary-container" />
          <span className="text-label-sm text-on-secondary-container">
            SMART CONTRACT SECURITY INTELLIGENCE
          </span>
        </div>

        {/* Main Headline */}
        <h1 className="text-display-lg text-on-surface mb-1">
          Don't get Hacked!
        </h1>
        <h2 className="text-5xl font-[600] leading-[64px] tracking-[-0.02em] text-secondary mb-6">
          Audit faster. Find more. Miss nothing.
        </h2>

        {/* Subheadline */}
        <p className="text-body-lg text-outline max-w-2xl mx-auto mb-8">
          Sentri combines a growing invariant library with advanced symbolic execution to secure the next generation of DeFi protocols before the first block is ever mined.
        </p>

        {/* CTA Buttons */}
        <div className="flex flex-col sm:flex-row items-center justify-center gap-3 mb-12">
          <Button variant="primary" size="lg" icon={<ArrowRight size={18} />} iconPosition="right">
            Start free trial
          </Button>
          <Button variant="secondary" size="lg">
            View a sample report
          </Button>
        </div>

        {/* Terminal Component */}
        <div className="max-w-2xl mx-auto mb-12">
          <Terminal
            output={[
              { prefix: 'INFO', text: 'Initializing Invariant Library: 1,402 checks loaded...', type: 'info' },
              { prefix: 'SCAN', text: 'Symbolic execution engine started on Vault.sol', type: 'scan' },
              { text: '' },
              { prefix: 'CRITICAL', text: 'Reentrancy vulnerability detected in `withdrawAll()`', type: 'critical' },
              { prefix: 'HIGH', text: 'Unchecked return value in `transferFunds()`', type: 'high' },
              { text: '' },
              { prefix: 'DONE', text: 'Scan complete. 12 vulnerabilities found.', type: 'done' },
            ]}
          />
        </div>

        {/* Feature Cards Grid */}
        <div className="grid grid-cols-1 md:grid-cols-3 gap-px bg-outline-variant rounded-lg overflow-hidden mb-24">
          {/* Card 1 */}
          <div className="bg-surface-container-low p-8">
            <div className="w-12 h-12 rounded bg-surface-container-lowest border border-outline-variant flex items-center justify-center mb-4 mx-auto">
              <BookOpen size={24} className="text-secondary" />
            </div>
            <h3 className="font-fraunces text-xl font-[600] text-on-surface mb-3">
              Invariant Library
            </h3>
            <p className="text-body-md text-outline">
              Access over 1,500+ pre-written security invariants specifically designed for ERC-4626, AMMs, and Lending Protocols.
            </p>
          </div>

          {/* Card 2 */}
          <div className="bg-surface-container-low p-8">
            <div className="w-12 h-12 rounded bg-surface-container-lowest border border-outline-variant flex items-center justify-center mb-4 mx-auto">
              <Brain size={24} className="text-secondary" />
            </div>
            <h3 className="font-fraunces text-xl font-[600] text-on-surface mb-3">
              AI Co-Auditor
            </h3>
            <p className="text-body-md text-outline">
              Context-aware AI that understands your protocol logic and suggests complex test vectors that standard fuzzers miss.
            </p>
          </div>

          {/* Card 3 */}
          <div className="bg-surface-container-low p-8">
            <div className="w-12 h-12 rounded bg-surface-container-lowest border border-outline-variant flex items-center justify-center mb-4 mx-auto">
              <RefreshCw size={24} className="text-secondary" />
            </div>
            <h3 className="font-fraunces text-xl font-[600] text-on-surface mb-3">
              Self-Improving Engine
            </h3>
            <p className="text-body-md text-outline">
              Our engine learns from every new exploit in the wild, automatically generating new detection modules for your stack.
            </p>
          </div>
        </div>

        {/* Reports Section */}
        <div className="grid grid-cols-1 lg:grid-cols-2 gap-16 mb-24 items-start">
          {/* Left Column */}
          <div className="text-left">
            <h2 className="font-fraunces text-4xl font-[600] text-on-surface mb-6 leading-[48px]">
              Professional Grade Reports
            </h2>
            <p className="text-body-lg text-outline mb-6">
              Generate executive-ready audit summaries with granular technical deep-dives for developers. Fully integrated with GitHub and GitLab CI/CD pipelines.
            </p>

            {/* Checklist */}
            <div className="space-y-3">
              {['Gas Optimization Insights', 'Formal Verification Proofs', 'Automated Remediation Advice'].map((item) => (
                <div key={item} className="flex items-center gap-3">
                  <Check size={18} className="text-low flex-shrink-0" />
                  <span className="text-body-md text-on-surface-variant">{item}</span>
                </div>
              ))}
            </div>
          </div>

          {/* Right Column - Report Preview */}
          <div className="bg-surface-container-low border border-outline-variant rounded-lg p-6">
            <div className="flex justify-between items-start mb-4">
              <span className="text-label-sm text-outline">AUDIT REPORT</span>
              <ArrowRight size={16} className="text-outline" />
            </div>

            <h3 className="font-fraunces text-xl font-[600] text-on-surface mb-4">
              Circle-Pay BCH
            </h3>

            {/* Severity Grid */}
            <div className="grid grid-cols-4 gap-px bg-outline-variant rounded-lg overflow-hidden mb-4">
              {[
                { label: 'CRITICAL', count: 5, color: 'text-critical' },
                { label: 'HIGH', count: 7, color: 'text-high' },
                { label: 'MED', count: 6, color: 'text-medium' },
                { label: 'LOW', count: 4, color: 'text-low' },
              ].map((item) => (
                <div key={item.label} className="bg-surface-container p-3 text-center">
                  <div className={`font-fraunces text-3xl font-[600] ${item.color} mb-1`}>
                    {item.count}
                  </div>
                  <div className="text-label-sm text-outline">{item.label}</div>
                </div>
              ))}
            </div>

            {/* Progress Bar */}
            <div className="mb-4">
              <div className="flex justify-between mb-2">
                <span className="text-body-md text-outline">Scan Progress</span>
                <span className="text-body-md text-outline">100% Verified</span>
              </div>
              <div className="w-full h-1 bg-surface-container rounded-full overflow-hidden">
                <div className="h-full bg-gradient-to-r from-medium to-low w-full rounded-full" />
              </div>
            </div>

            {/* Download Button */}
            <Button variant="secondary" fullWidth size="sm">
              ↓ Download PDF Report
            </Button>
          </div>
        </div>

        {/* Pricing Section */}
        <div className="text-center mb-24">
          <div className="inline-flex items-center gap-2 px-4 py-2 rounded-full bg-indigo/8 border border-indigo/20 mb-6">
            <span className="text-label-sm text-outline">SECURE YOUR FUTURE</span>
          </div>
          <h2 className="font-fraunces text-4xl font-[600] text-on-surface mb-12 leading-[48px] max-w-2xl mx-auto">
            Simple, predictable pricing for every stage of your protocol.
          </h2>

          {/* Pricing Cards */}
          <div className="grid grid-cols-1 md:grid-cols-3 gap-6 max-w-5xl mx-auto">
            {/* Starter */}
            <div className="bg-surface-container-low border border-outline-variant rounded-lg p-8">
              <span className="text-label-sm text-outline">Starter</span>
              <div className="mt-2 mb-1">
                <div className="font-fraunces text-5xl font-[700] text-on-surface">
                  $0
                </div>
                <div className="text-body-md text-outline">/mo</div>
              </div>
              <div className="border-t border-outline-variant my-6" />
              <div className="space-y-3 mb-6">
                <div className="flex items-center gap-2">
                  <Check size={16} className="text-low" />
                  <span className="text-body-md text-on-surface-variant">5 Scans / month</span>
                </div>
                <div className="flex items-center gap-2">
                  <Check size={16} className="text-low" />
                  <span className="text-body-md text-on-surface-variant">Public Library Access</span>
                </div>
                <div className="flex items-center gap-2 opacity-50">
                  <span className="text-body-md text-critical">✗</span>
                  <span className="text-body-md text-on-surface-variant line-through">AI Co-Auditor</span>
                </div>
              </div>
              <Button variant="secondary" fullWidth>
                Choose Free
              </Button>
            </div>

            {/* Professional - Featured */}
            <div className="relative bg-indigo-container border-2 border-indigo rounded-lg p-8">
              <div className="absolute -top-3 left-1/2 -translate-x-1/2 bg-secondary-container border border-indigo text-on-secondary-container px-3 py-1 rounded-full text-label-sm">
                MOST POPULAR
              </div>
              <span className="text-label-sm text-on-secondary-container">Professional</span>
              <div className="mt-2 mb-1">
                <div className="font-fraunces text-5xl font-[700] text-on-surface">
                  $499
                </div>
                <div className="text-body-md text-outline">/mo</div>
              </div>
              <div className="border-t border-indigo/30 my-6" />
              <div className="space-y-3 mb-6">
                {['Unlimited Scans', 'Priority CI/CD Queues', 'Full AI Co-Auditor'].map((item) => (
                  <div key={item} className="flex items-center gap-2">
                    <Check size={16} className="text-low" />
                    <span className="text-body-md text-on-surface-variant">{item}</span>
                  </div>
                ))}
              </div>
              <Button variant="primary" fullWidth>
                Get Started
              </Button>
            </div>

            {/* Enterprise */}
            <div className="bg-surface-container-low border border-outline-variant rounded-lg p-8">
              <span className="text-label-sm text-outline">Enterprise</span>
              <div className="mt-2 mb-1">
                <div className="font-fraunces text-4xl font-[700] text-on-surface">
                  Custom
                </div>
              </div>
              <div className="border-t border-outline-variant my-6" />
              <div className="space-y-3 mb-6">
                {['Private Invariant Repo', '24/7 Security Advisor', 'On-prem deployment'].map((item) => (
                  <div key={item} className="flex items-center gap-2">
                    <Check size={16} className="text-low" />
                    <span className="text-body-md text-on-surface-variant">{item}</span>
                  </div>
                ))}
              </div>
              <Button variant="secondary" fullWidth>
                Contact Sales
              </Button>
            </div>
          </div>
        </div>
      </section>

      <MarketingFooter />
    </div>
  )
}
