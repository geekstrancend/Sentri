'use client'

import { ShieldCheck, BookOpen, Brain, RefreshCw, ArrowRight, Check, HelpCircle } from 'lucide-react'
import { useReveal } from '@/components/hooks/useReveal'
import { AnimatedCounter } from '@/components/ui/AnimatedCounter'
import { AsciiLogo } from '@/components/ui/AsciiLogo'
import { MarketingNav } from '@/components/layout/MarketingNav'
import { MarketingFooter } from '@/components/layout/MarketingFooter'
import { Button } from '@/components/ui/Button'
import { Terminal } from '@/components/ui/Terminal'
import { SeverityBadge } from '@/components/ui/SeverityBadge'

export default function HomePage() {
  const leftRef = useReveal<HTMLDivElement>()
  const rightRef = useReveal<HTMLDivElement>()
  const featuresRef = useReveal<HTMLDivElement>()
  const starterRef = useReveal<HTMLDivElement>()
  const proRef = useReveal<HTMLDivElement>()
  const enterpriseRef = useReveal<HTMLDivElement>()

  return (
    <div className="min-h-screen bg-surface flex flex-col">
      <div className="fixed top-0 left-1/2 -translate-x-1/2 w-[800px] h-[800px] bg-indigo/5 rounded-full blur-3xl pointer-events-none -z-10" />
      <MarketingNav />

      {/* Hero Section - Two Column Layout */}
      <section id="product" className="relative px-6 py-24 lg:py-32 max-w-7xl mx-auto overflow-hidden">
        {/* Background layers */}
        <div className="absolute inset-0 bg-grid-pattern pointer-events-none" />
        <div className="absolute -top-20 -right-20 w-[600px] h-[600px] bg-indigo/8 rounded-full blur-3xl animate-spotlight pointer-events-none" />

        {/* Faint ASCII watermark */}
        <div className="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 text-[10px] sm:text-base lg:text-2xl opacity-[0.04] pointer-events-none scale-150 lg:scale-[2]">
          <AsciiLogo />
        </div>

        {/* Two-column grid */}
        <div className="relative grid grid-cols-1 lg:grid-cols-2 gap-16 items-center">
          {/* Left: Copy */}
          <div className="text-left">
            <div className="inline-flex items-center gap-2 px-4 py-2 rounded-full bg-indigo/8 border border-indigo/20 mb-6 animate-fade-in-up">
              <ShieldCheck size={14} className="text-on-secondary-container" />
              <span className="text-label-sm text-on-secondary-container">
                SMART CONTRACT SECURITY INTELLIGENCE
              </span>
            </div>

            <h1 className="text-display-lg text-on-surface mb-1 animate-fade-in-up" style={{ animationDelay: '0.05s' }}>
              Don't get Hacked!
            </h1>
            <h2 className="text-5xl font-[600] leading-[64px] tracking-[-0.02em] text-secondary mb-6 animate-fade-in-up" style={{ animationDelay: '0.1s' }}>
              Audit faster. Find more. Miss nothing.
            </h2>
            <p className="text-body-lg text-outline mb-8 animate-fade-in-up" style={{ animationDelay: '0.2s' }}>
              Sentri combines a growing invariant library with advanced symbolic execution to secure the next generation of DeFi protocols before the first block is ever mined.
            </p>

            <div className="flex flex-col sm:flex-row gap-3 animate-fade-in-up" style={{ animationDelay: '0.3s' }}>
              <Button variant="primary" size="lg" icon={<ArrowRight size={18} />} iconPosition="right">
                Start free trial
              </Button>
              <Button variant="secondary" size="lg">
                View a sample report
              </Button>
            </div>
          </div>

          {/* Right: Terminal */}
          <div className="glass-panel rounded-lg p-2 animate-fade-in-up" style={{ animationDelay: '0.4s' }}>
            <Terminal
              showBanner={true}
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
        </div>
      </section>

      {/* Stats Section */}
      <div className="px-6 py-24 max-w-7xl mx-auto w-full">
        <div className="grid grid-cols-2 md:grid-cols-4 gap-8">
          <div className="text-center">
            <div className="text-display-md text-secondary font-[600] mb-2 count-glow">
              <AnimatedCounter value={1500} />+
            </div>
            <p className="text-body-sm text-outline">Security Invariants</p>
          </div>
          <div className="text-center">
            <div className="text-display-md text-secondary font-[600] mb-2 count-glow">
              <AnimatedCounter value={42} />
            </div>
            <p className="text-body-sm text-outline">Protocols Analyzed</p>
          </div>
          <div className="text-center">
            <div className="text-display-md text-secondary font-[600] mb-2 count-glow">
              $<AnimatedCounter value={190} />M
            </div>
            <p className="text-body-sm text-outline">Losses Prevented</p>
          </div>
          <div className="text-center">
            <div className="text-display-md text-secondary font-[600] mb-2 count-glow">
              <AnimatedCounter value={24} />h
            </div>
            <p className="text-body-sm text-outline">Avg Audit Time</p>
          </div>
        </div>
      </div>

      {/* Features Section - Bento Layout */}
      <div ref={featuresRef} className="px-6 py-24 max-w-7xl mx-auto w-full">
        <div className="grid grid-cols-1 lg:grid-cols-3 gap-4 reveal">
          {/* Large card (left, spans 2 columns) */}
          <div className="lg:col-span-2 bg-surface-container-low p-8 rounded-lg border border-outline-variant lift-on-hover relative overflow-hidden">
            <div className="absolute -right-8 -bottom-8 opacity-[0.04]">
              <AsciiLogo />
            </div>
            <div className="w-12 h-12 rounded bg-surface-container-lowest border border-outline-variant flex items-center justify-center mb-4 relative z-10">
              <BookOpen size={24} className="text-secondary" />
            </div>
            <h3 className="font-fraunces text-xl font-[600] text-on-surface mb-3 relative z-10">
              Invariant Library
            </h3>
            <p className="text-body-md text-outline relative z-10">
              Access over 1,500+ pre-written security invariants specifically designed for ERC-4626, AMMs, and Lending Protocols.
            </p>
          </div>

          {/* Right column: 2 stacked cards */}
          <div className="flex flex-col gap-4">
            {/* Top card */}
            <div className="bg-surface-container-low p-8 rounded-lg border border-outline-variant lift-on-hover">
              <div className="w-12 h-12 rounded bg-surface-container-lowest border border-outline-variant flex items-center justify-center mb-4">
                <Brain size={24} className="text-secondary" />
              </div>
              <h3 className="font-fraunces text-xl font-[600] text-on-surface mb-3">
                AI Co-Auditor
              </h3>
              <p className="text-body-md text-outline">
                Context-aware AI that understands your protocol logic and suggests complex test vectors.
              </p>
            </div>

            {/* Bottom card */}
            <div className="bg-surface-container-low p-8 rounded-lg border border-outline-variant lift-on-hover">
              <div className="w-12 h-12 rounded bg-surface-container-lowest border border-outline-variant flex items-center justify-center mb-4">
                <RefreshCw size={24} className="text-secondary" />
              </div>
              <h3 className="font-fraunces text-xl font-[600] text-on-surface mb-3">
                Self-Improving Engine
              </h3>
              <p className="text-body-md text-outline">
                Our engine learns from every new exploit in the wild, automatically generating new detection modules.
              </p>
            </div>
          </div>
        </div>
      </div>

      {/* Section Divider */}
      <div className="px-6 max-w-7xl mx-auto w-full mb-24">
        <div className="flex items-center gap-4 opacity-20">
          <div className="flex-1 h-px bg-outline-variant" />
          <AsciiLogo className="text-[8px] leading-none flex-shrink-0" />
          <div className="flex-1 h-px bg-outline-variant" />
        </div>
      </div>

      {/* Reports Section */}
      <div className="px-6 py-24 max-w-7xl mx-auto w-full">
        <div className="grid grid-cols-1 lg:grid-cols-2 gap-16 mb-24 items-start">
          {/* Left Column */}
          <div ref={leftRef} className="text-left reveal">
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
          <div ref={rightRef} className="bg-surface-container-low border border-outline-variant rounded-lg p-6 reveal">
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
      </div>

      {/* Pricing Section */}
      <div className="px-6 py-24 max-w-7xl mx-auto w-full text-center">
        <div className="inline-flex items-center gap-2 px-4 py-2 rounded-full bg-indigo/8 border border-indigo/20 mb-6">
          <span className="text-label-sm text-outline">SECURE YOUR FUTURE</span>
        </div>

        <h2 className="font-fraunces text-4xl font-[600] text-on-surface mb-3 leading-[48px]">
          Plans for Every Stage
        </h2>

        <p className="text-body-lg text-outline max-w-2xl mx-auto mb-16">
          From indie developers to enterprise teams, find the right plan to secure your contracts.
        </p>

        {/* Pricing Cards */}
        <div className="grid grid-cols-1 md:grid-cols-3 gap-6 max-w-5xl mx-auto">
          {/* Starter */}
          <div ref={starterRef} className="bg-surface-container-low border border-outline-variant rounded-lg p-8 reveal">
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
          <div ref={proRef} className="relative bg-indigo-container border-2 border-indigo rounded-lg p-8 reveal">
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
          <div ref={enterpriseRef} className="bg-surface-container-low border border-outline-variant rounded-lg p-8 reveal">
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

      <MarketingFooter />
    </div>
  )
}
