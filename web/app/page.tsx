'use client'

import { useState } from 'react'
import Link from 'next/link'
import {
  ShieldCheck,
  BookOpen,
  Brain,
  RefreshCw,
  ArrowRight,
  Check,
  Zap,
  GitBranch,
  Eye,
  FileText,
  Terminal as TerminalIcon,
} from 'lucide-react'
import { useReveal } from '@/components/hooks/useReveal'
import { AnimatedCounter } from '@/components/ui/AnimatedCounter'
import { AsciiLogo } from '@/components/ui/AsciiLogo'
import { MarketingNav } from '@/components/layout/MarketingNav'
import { MarketingFooter } from '@/components/layout/MarketingFooter'
import { Button } from '@/components/ui/Button'
import { Terminal } from '@/components/ui/Terminal'
import { SeverityBadge } from '@/components/ui/SeverityBadge'
import { AuthModal } from '@/components/ui/AuthModal'
import { SampleReportModal } from '@/components/ui/SampleReportModal'
import { Container, SectionHeading } from '@/components/ui/Section'
import { Card } from '@/components/ui/Card'
import { Badge } from '@/components/ui/Badge'
import { AmbientBackground } from '@/components/ui/AmbientBackground'

const CHAINS = [
  'Ethereum', 'Solana', 'Arbitrum', 'Base', 'Polygon',
  'Optimism', 'Aptos', 'Sui', 'Avalanche', 'BNB Chain', 'Stellar',
]

const STATS = [
  { value: 72, suffix: '+', label: 'Vulnerability detectors' },
  { value: 4, suffix: '', label: 'Chain families' },
  { value: 190, prefix: '$', suffix: 'M+', label: 'Losses studied' },
  { value: 100, suffix: '%', label: 'Reproducible runs' },
]

const EXPLOITS = [
  { protocol: 'Euler Finance', amount: '$197M', year: '2023', type: 'Flash loan + missing health check', invariant: 'evm_missing_post_state_health_check' },
  { protocol: 'Nomad Bridge', amount: '$190M', year: '2022', type: 'Merkle root zero-initialization', invariant: 'evm_merkle_root_zero_default' },
  { protocol: 'KelpDAO', amount: '$292M', year: '2024', type: 'DVN single point of failure', invariant: 'evm_dvn_single_point_failure' },
]

export default function HomePage() {
  const [authOpen, setAuthOpen] = useState(false)
  const [authTab, setAuthTab] = useState<'signin' | 'signup'>('signin')
  const [sampleReportOpen, setSampleReportOpen] = useState(false)

  const featuresRef = useReveal()
  const stepsRef = useReveal()
  const exploitsRef = useReveal()
  const reportsLeftRef = useReveal()
  const reportsRightRef = useReveal()
  const pricingRef = useReveal()
  const ctaRef = useReveal()

  const openSignup = () => {
    setAuthTab('signup')
    setAuthOpen(true)
  }

  return (
    <div className="flex min-h-dvh flex-col bg-surface">
      <MarketingNav />

      <main id="main" className="flex-1">
        {/* ───────────────────────── Hero ───────────────────────── */}
        <section id="product" className="relative isolate overflow-hidden">
          <AmbientBackground />
          <Container className="relative z-10 grid grid-cols-1 items-center gap-12 py-16 lg:grid-cols-[1.05fr_1fr] lg:py-24">
            {/* Copy */}
            <div className="relative max-w-xl">
              {/* Soft scrim so body copy stays crisp over the ASCII field. */}
              <div
                aria-hidden
                className="pointer-events-none absolute -inset-x-8 -inset-y-6 -z-[1] bg-[radial-gradient(ellipse_at_center,theme(colors.surface)_45%,transparent_100%)] opacity-80"
              />
              <div className="animate-fade-in-up">
                <Badge tone="indigo" icon={<ShieldCheck size={13} />}>
                  Smart contract security engine
                </Badge>
              </div>

              <h1 className="animate-fade-in-up stagger-1 mt-6 text-display-lg text-on-surface text-balance">
                Don&apos;t get hacked.{' '}
                <span className="gradient-text">Audit faster, miss nothing.</span>
              </h1>

              <p className="animate-fade-in-up stagger-2 mt-6 text-body-lg text-on-surface-variant measure">
                Sentri pairs a multi-chain detector library with dynamic invariant fuzzing to catch
                the exploit before the first block is ever mined — reproducibly, from your terminal
                or your CI.
              </p>

              <div className="animate-fade-in-up stagger-3 mt-8 flex flex-col gap-3 sm:flex-row">
                <Button
                  variant="primary"
                  size="lg"
                  icon={<ArrowRight size={18} />}
                  iconPosition="right"
                  onClick={openSignup}
                >
                  Start free trial
                </Button>
                <Button variant="secondary" size="lg" onClick={() => setSampleReportOpen(true)}>
                  View a sample report
                </Button>
              </div>

              <div className="animate-fade-in-up stagger-4 mt-8 flex flex-wrap items-center gap-x-6 gap-y-2 text-body-sm text-outline">
                <span className="inline-flex items-center gap-2">
                  <Check size={14} className="text-signal" /> No credit card
                </span>
                <span className="inline-flex items-center gap-2">
                  <Check size={14} className="text-signal" /> EVM · Solana · Move · Soroban
                </span>
                <span className="inline-flex items-center gap-2">
                  <Check size={14} className="text-signal" /> CLI + GitHub Actions
                </span>
              </div>
            </div>

            {/* Terminal */}
            <div className="animate-fade-in-up stagger-3 lg:justify-self-end">
              <div className="glass-panel rounded-2xl p-2 shadow-card-lg">
                <div className="flex items-center gap-2 px-3 py-2">
                  <span className="h-3 w-3 rounded-full bg-critical/70" />
                  <span className="h-3 w-3 rounded-full bg-high/70" />
                  <span className="h-3 w-3 rounded-full bg-signal/70" />
                  <span className="ml-2 inline-flex items-center gap-1.5 font-mono text-[0.7rem] text-outline">
                    <TerminalIcon size={11} /> sentri scan Vault.sol
                  </span>
                </div>
                <div className="rounded-xl border border-outline-variant bg-surface-container-lowest/80 p-1">
                  <Terminal
                    showBanner={true}
                    output={[
                      { prefix: 'INFO', text: 'Loading detector library: 72 checks across 4 chains…', type: 'info' },
                      { prefix: 'SCAN', text: 'Dynamic invariant fuzzing started on Vault.sol', type: 'scan' },
                      { text: '' },
                      { prefix: 'CRITICAL', text: 'Reentrancy in `withdrawAll()` — CEI violation', type: 'critical' },
                      { prefix: 'HIGH', text: 'Unchecked return value in `transferFunds()`', type: 'high' },
                      { text: '' },
                      { prefix: 'DONE', text: 'Scan complete. 12 findings, minimal repro emitted.', type: 'done' },
                    ]}
                  />
                </div>
              </div>
            </div>
          </Container>

          {/* Chain ticker */}
          <div className="relative mt-6 border-y border-outline-variant bg-surface-container-lowest/60 py-3.5">
            <div
              aria-hidden
              className="pointer-events-none absolute inset-y-0 left-0 z-10 w-24 bg-gradient-to-r from-surface to-transparent"
            />
            <div
              aria-hidden
              className="pointer-events-none absolute inset-y-0 right-0 z-10 w-24 bg-gradient-to-l from-surface to-transparent"
            />
            <div className="flex animate-marquee whitespace-nowrap">
              {[...CHAINS, ...CHAINS].map((chain, i) => (
                <span
                  key={i}
                  className="flex-shrink-0 px-8 font-mono text-[0.7rem] uppercase tracking-[0.08em] text-outline"
                >
                  <span className="text-indigo/60">◆</span> {chain}
                </span>
              ))}
            </div>
          </div>
        </section>

        {/* ───────────────────────── Stats ───────────────────────── */}
        <section className="py-16 sm:py-20">
          <Container>
            <div className="grid grid-cols-2 gap-6 md:grid-cols-4">
              {STATS.map((s) => (
                <div key={s.label} className="text-center">
                  <div className="font-mono text-4xl font-[600] tracking-tight text-on-surface sm:text-5xl">
                    {s.prefix}
                    <AnimatedCounter value={s.value} />
                    {s.suffix}
                  </div>
                  <p className="mt-2 text-body-sm text-outline">{s.label}</p>
                </div>
              ))}
            </div>
          </Container>
        </section>

        {/* ───────────────────────── Features ───────────────────────── */}
        <section id="features" className="py-16 sm:py-24">
          <Container>
            <SectionHeading
              align="center"
              className="mx-auto mb-14"
              eyebrow="Capabilities"
              title="Every layer of security. One engine."
              description="Static analysis, dynamic fuzzing, and CI enforcement — working from the same detector library."
            />

            <div ref={featuresRef} className="reveal grid grid-cols-1 gap-5 md:grid-cols-2 lg:grid-cols-3">
              <Card
                interactive
                className="group relative overflow-hidden p-8 lg:col-span-2"
              >
                <div className="absolute -bottom-8 -right-8 opacity-[0.05] transition-opacity duration-500 group-hover:opacity-[0.1]">
                  <AsciiLogo />
                </div>
                <FeatureIcon icon={<BookOpen size={22} />} />
                <h3 className="relative z-10 mt-5 text-xl font-[600] text-on-surface">Detector Library</h3>
                <p className="relative z-10 mt-3 max-w-xl text-body-md leading-6 text-on-surface-variant">
                  72 vulnerability detectors for ERC-4626, AMMs, lending, and cross-chain bridges —
                  each mapped to a real-world exploit pattern across EVM, Solana, Move, and Soroban.
                </p>
                <Link
                  href="/library"
                  className="group/link relative z-10 mt-6 inline-flex items-center gap-1.5 text-sm font-[600] text-indigo-bright transition-all hover:gap-2.5"
                >
                  Browse the library <ArrowRight size={14} />
                </Link>
              </Card>

              <FeatureCard
                icon={<Brain size={22} />}
                title="AI co-auditor"
                body="Context-aware analysis that understands your protocol logic and drafts complex test vectors automatically."
              />
              <FeatureCard
                icon={<RefreshCw size={22} />}
                title="Self-improving engine"
                body="Learns from every new exploit in the wild, shipping new detection modules within a day."
              />
              <FeatureCard
                icon={<GitBranch size={22} />}
                title="CI/CD integration"
                body="Native GitHub Actions and GitLab support. Block deploys on critical findings automatically."
              />
              <FeatureCard
                highlighted
                icon={<Zap size={22} />}
                title="Dynamic invariant fuzzing"
                body="A real revm/execution engine explores adversarial call sequences and shrinks failures to a minimal PoC."
              />
            </div>
          </Container>
        </section>

        {/* ───────────────────────── How it works ───────────────────────── */}
        <section className="relative isolate overflow-hidden border-y border-outline-variant bg-surface-container-lowest py-16 sm:py-24">
          <AmbientBackground spotlight={false} />
          <Container className="relative z-10">
            <SectionHeading
              align="center"
              className="mx-auto mb-14"
              eyebrow="How it works"
              title="From code to coverage in minutes"
            />
            <div ref={stepsRef} className="reveal grid grid-cols-1 gap-5 md:grid-cols-3">
              {[
                { step: '01', icon: <GitBranch size={20} />, title: 'Connect your repo', description: 'Link GitHub, GitLab, or point the CLI at a directory. Solidity, Rust (Anchor), and Move supported.' },
                { step: '02', icon: <Eye size={20} />, title: 'Deep scan & fuzz', description: '72 detectors run alongside dynamic invariant fuzzing and full data-flow analysis on every function.' },
                { step: '03', icon: <FileText size={20} />, title: 'Actionable reports', description: 'Prioritized findings with code-level fixes, minimal reproductions, and one-click remediation paths.' },
              ].map((item) => (
                <Card key={item.step} className="p-8">
                  <div className="mb-5 flex items-start gap-4">
                    <span className="select-none font-mono text-5xl font-[700] leading-none text-outline-variant">
                      {item.step}
                    </span>
                    <FeatureIcon icon={item.icon} small />
                  </div>
                  <h3 className="mb-2.5 text-lg font-[600] text-on-surface">{item.title}</h3>
                  <p className="text-body-md leading-6 text-on-surface-variant">{item.description}</p>
                </Card>
              ))}
            </div>
          </Container>
        </section>

        {/* ───────────────────────── Real exploits ───────────────────────── */}
        <section className="py-16 sm:py-24">
          <Container>
            <SectionHeading
              align="center"
              className="mx-auto mb-14"
              eyebrow="Battle-tested against real exploits"
              title="We study every major hack so you don't have to"
              description="Every invariant maps to a real-world exploit. Sentri would have flagged these before deployment."
            />
            <div ref={exploitsRef} className="reveal grid grid-cols-1 gap-5 md:grid-cols-3">
              {EXPLOITS.map((exploit) => (
                <Card key={exploit.protocol} interactive className="relative overflow-hidden p-7">
                  <div className="absolute inset-x-0 top-0 h-0.5 bg-gradient-to-r from-critical via-critical/40 to-transparent" />
                  <div className="mb-4 flex items-start justify-between">
                    <div>
                      <p className="text-label-sm text-outline">{exploit.year} exploit</p>
                      <h3 className="mt-1 text-xl font-[600] text-on-surface">{exploit.protocol}</h3>
                    </div>
                    <span className="font-mono text-xl font-[700] text-critical">{exploit.amount}</span>
                  </div>
                  <p className="mb-5 text-body-md text-on-surface-variant">{exploit.type}</p>
                  <div className="mb-4 flex flex-wrap items-center gap-2">
                    <SeverityBadge level="critical" />
                    <code className="rounded border border-outline-variant bg-surface-container px-2 py-0.5 font-mono text-[0.7rem] text-on-surface-variant">
                      {exploit.invariant}
                    </code>
                  </div>
                  <div className="flex items-center gap-1.5 text-xs font-[600] text-signal">
                    <Check size={13} /> Sentri detects this pattern
                  </div>
                </Card>
              ))}
            </div>
          </Container>
        </section>

        {/* ───────────────────────── Reports ───────────────────────── */}
        <section className="relative isolate overflow-hidden border-y border-outline-variant bg-surface-container-lowest py-16 sm:py-24">
          <AmbientBackground spotlight={false} />
          <Container className="relative z-10">
            <div className="grid grid-cols-1 items-center gap-14 lg:grid-cols-2">
              <div ref={reportsLeftRef} className="reveal">
                <SectionHeading
                  eyebrow="Audit reports"
                  title="Professional-grade reports"
                  description="Executive summaries with granular technical deep-dives, wired into your GitHub and GitLab pipelines out of the box."
                />
                <div className="my-8 space-y-3.5">
                  {['Minimal reproductions for every finding', 'Formal & dynamic verification evidence', 'Automated remediation advice', 'One-click PDF export'].map((item) => (
                    <div key={item} className="flex items-center gap-3">
                      <span className="flex h-5 w-5 items-center justify-center rounded-full bg-signal-bg">
                        <Check size={12} className="text-signal" />
                      </span>
                      <span className="text-body-md text-on-surface-variant">{item}</span>
                    </div>
                  ))}
                </div>
                <Button variant="secondary" onClick={() => setSampleReportOpen(true)}>
                  View sample report
                </Button>
              </div>

              <div ref={reportsRightRef} className="reveal">
                <Card gradientTop className="p-6 shadow-card-lg">
                  <div className="mb-6 flex items-start justify-between">
                    <div>
                      <span className="mb-1 block text-label-sm text-outline">Audit report</span>
                      <h3 className="text-xl font-[600] text-on-surface">Circle-Pay Vault</h3>
                      <p className="mt-1 font-mono text-body-sm text-outline">EVM · v2.1.0</p>
                    </div>
                    <Badge tone="signal">Complete</Badge>
                  </div>
                  <div className="mb-6 grid grid-cols-4 gap-px overflow-hidden rounded-lg bg-outline-variant">
                    {[
                      { label: 'CRIT', count: 5, color: 'text-critical' },
                      { label: 'HIGH', count: 7, color: 'text-high' },
                      { label: 'MED', count: 6, color: 'text-medium' },
                      { label: 'LOW', count: 4, color: 'text-low' },
                    ].map((item) => (
                      <div key={item.label} className="bg-surface-container p-3 text-center">
                        <div className={`font-mono text-3xl font-[600] ${item.color}`}>{item.count}</div>
                        <div className="mt-1 text-label-sm text-outline">{item.label}</div>
                      </div>
                    ))}
                  </div>
                  <div className="mb-6 space-y-2">
                    {[
                      { title: 'Reentrancy in withdrawAll()', sev: 'critical' as const },
                      { title: 'Unchecked external call return', sev: 'high' as const },
                      { title: 'Missing oracle staleness check', sev: 'high' as const },
                    ].map((f) => (
                      <div
                        key={f.title}
                        className="flex items-center gap-3 rounded-lg bg-surface-container px-3 py-2.5"
                      >
                        <SeverityBadge level={f.sev} />
                        <span className="text-body-md text-on-surface-variant">{f.title}</span>
                      </div>
                    ))}
                  </div>
                  <Button variant="secondary" fullWidth size="sm" onClick={() => setSampleReportOpen(true)}>
                    View full report
                  </Button>
                </Card>
              </div>
            </div>
          </Container>
        </section>

        {/* ───────────────────────── Pricing preview ───────────────────────── */}
        <section className="py-16 sm:py-24">
          <Container>
            <SectionHeading
              align="center"
              className="mx-auto mb-14"
              eyebrow="Pricing"
              title="Plans for every stage"
              description="From indie developers to enterprise security teams."
            />
            <div ref={pricingRef} className="reveal mx-auto grid max-w-5xl grid-cols-1 gap-5 md:grid-cols-3">
              <PricingCard
                name="Starter"
                price="$0"
                period="/mo"
                blurb="For early-stage projects"
                features={[
                  { text: '5 scans / month', ok: true },
                  { text: 'Public library access', ok: true },
                  { text: 'AI co-auditor', ok: false },
                  { text: 'Priority support', ok: false },
                ]}
                cta={
                  <Button variant="secondary" fullWidth onClick={openSignup}>
                    Get started free
                  </Button>
                }
              />
              <PricingCard
                featured
                name="Professional"
                price="$499"
                period="/mo"
                blurb="For production protocols"
                features={[
                  { text: 'Unlimited scans', ok: true },
                  { text: 'Priority CI/CD queues', ok: true },
                  { text: 'Full AI co-auditor', ok: true },
                  { text: 'Priority support', ok: true },
                ]}
                cta={
                  <Button variant="primary" fullWidth onClick={openSignup}>
                    Get started
                  </Button>
                }
              />
              <PricingCard
                name="Enterprise"
                price="Custom"
                blurb="For large-scale deployments"
                features={[
                  { text: 'Private detector repo', ok: true },
                  { text: '24/7 security advisor', ok: true },
                  { text: 'On-prem deployment', ok: true },
                  { text: 'SLA guarantee', ok: true },
                ]}
                cta={
                  <Link href="/contact" className="block">
                    <Button variant="secondary" fullWidth>
                      Contact sales
                    </Button>
                  </Link>
                }
              />
            </div>
            <div className="mt-10 text-center">
              <Link
                href="/pricing"
                className="inline-flex items-center gap-1.5 text-sm font-[600] text-indigo-bright transition-all hover:gap-2.5"
              >
                See full pricing & comparison <ArrowRight size={14} />
              </Link>
            </div>
          </Container>
        </section>

        {/* ───────────────────────── Final CTA ───────────────────────── */}
        <section ref={ctaRef} className="reveal pb-24">
          <Container>
            <div className="relative isolate overflow-hidden rounded-3xl border border-indigo/30 bg-gradient-to-br from-indigo/10 via-surface-container-low to-surface-container-lowest p-10 text-center sm:p-16">
              <AmbientBackground spotlight={false} />
              <div className="pointer-events-none absolute -top-32 left-1/2 z-0 h-[380px] w-[600px] -translate-x-1/2 rounded-full bg-indigo/15 blur-3xl" />
              <div className="relative z-10">
                <Badge tone="indigo" icon={<ShieldCheck size={13} />} className="mx-auto">
                  Start securing today
                </Badge>
                <h2 className="mx-auto mt-6 max-w-2xl text-display-sm text-on-surface text-balance">
                  Ready to audit smarter?
                </h2>
                <p className="mx-auto mt-4 max-w-xl text-body-lg text-on-surface-variant">
                  Join teams securing on-chain value with Sentri&apos;s invariant-driven security engine.
                </p>
                <div className="mt-9 flex flex-col justify-center gap-3 sm:flex-row">
                  <Button
                    variant="primary"
                    size="lg"
                    icon={<ArrowRight size={18} />}
                    iconPosition="right"
                    onClick={openSignup}
                  >
                    Start free trial
                  </Button>
                  <Link href="/docs">
                    <Button variant="secondary" size="lg">
                      Read documentation
                    </Button>
                  </Link>
                </div>
              </div>
            </div>
          </Container>
        </section>
      </main>

      <AuthModal isOpen={authOpen} onClose={() => setAuthOpen(false)} defaultTab={authTab} />
      <SampleReportModal isOpen={sampleReportOpen} onClose={() => setSampleReportOpen(false)} />
      <MarketingFooter />
    </div>
  )
}

/* ───────────────────────── local building blocks ───────────────────────── */

function FeatureIcon({ icon, small = false }: { icon: React.ReactNode; small?: boolean }) {
  return (
    <span
      className={`flex items-center justify-center rounded-xl border border-indigo/20 bg-indigo/10 text-indigo-bright ${
        small ? 'h-10 w-10' : 'h-12 w-12'
      }`}
    >
      {icon}
    </span>
  )
}

function FeatureCard({
  icon,
  title,
  body,
  highlighted = false,
}: {
  icon: React.ReactNode
  title: string
  body: string
  highlighted?: boolean
}) {
  return (
    <Card interactive className={`p-8 ${highlighted ? 'border-indigo/25 bg-indigo/[0.05]' : ''}`}>
      <FeatureIcon icon={icon} />
      <h3 className="mt-5 text-xl font-[600] text-on-surface">{title}</h3>
      <p className="mt-3 text-body-md leading-6 text-on-surface-variant">{body}</p>
    </Card>
  )
}

function PricingCard({
  name,
  price,
  period,
  blurb,
  features,
  cta,
  featured = false,
}: {
  name: string
  price: string
  period?: string
  blurb: string
  features: { text: string; ok: boolean }[]
  cta: React.ReactNode
  featured?: boolean
}) {
  return (
    <div
      className={`relative rounded-2xl border p-8 lift-on-hover ${
        featured
          ? 'border-indigo bg-indigo/[0.06] animate-border-glow'
          : 'border-outline-variant bg-surface-container-low/70'
      }`}
    >
      {featured && (
        <div className="absolute -top-3 left-1/2 -translate-x-1/2">
          <Badge tone="indigo">Most popular</Badge>
        </div>
      )}
      <span className="block text-label-sm text-on-surface-variant">{name}</span>
      <div className="mt-3 flex items-baseline gap-1">
        <span className="font-mono text-4xl font-[700] text-on-surface">{price}</span>
        {period && <span className="text-body-md text-outline">{period}</span>}
      </div>
      <p className="mt-2 text-body-md text-outline">{blurb}</p>
      <div className="my-6 h-px bg-outline-variant" />
      <ul className="mb-8 space-y-3">
        {features.map((f) => (
          <li key={f.text} className={`flex items-center gap-2.5 ${!f.ok ? 'opacity-45' : ''}`}>
            {f.ok ? (
              <Check size={15} className="flex-shrink-0 text-signal" />
            ) : (
              <span className="w-[15px] flex-shrink-0 text-center text-sm text-outline">–</span>
            )}
            <span className={`text-body-md ${f.ok ? 'text-on-surface-variant' : 'text-outline'}`}>
              {f.text}
            </span>
          </li>
        ))}
      </ul>
      {cta}
    </div>
  )
}
