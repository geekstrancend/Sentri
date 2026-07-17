'use client'

import { Fragment, useState } from 'react'
import Link from 'next/link'
import { Check, Minus, ChevronDown } from 'lucide-react'
import { useReveal } from '@/components/hooks/useReveal'
import { MarketingNav } from '@/components/layout/MarketingNav'
import { MarketingFooter } from '@/components/layout/MarketingFooter'
import { Button } from '@/components/ui/Button'
import { AuthModal } from '@/components/ui/AuthModal'
import { Container, SectionHeading } from '@/components/ui/Section'
import { Badge } from '@/components/ui/Badge'
import { AmbientBackground } from '@/components/ui/AmbientBackground'
import clsx from 'clsx'

type BillingCycle = 'monthly' | 'annual'
type PlanId = 'starter' | 'pro' | 'enterprise'

const PLANS: {
  id: PlanId
  name: string
  description: string
  monthlyPrice: number | null
  cta: string
  ctaVariant: 'primary' | 'secondary'
  featured: boolean
  highlights: string[]
}[] = [
  {
    id: 'starter',
    name: 'Starter',
    description: 'For indie auditors and early-stage projects',
    monthlyPrice: 0,
    cta: 'Get started free',
    ctaVariant: 'secondary',
    featured: false,
    highlights: ['5 scans per month', 'Public detector library', 'PDF report export'],
  },
  {
    id: 'pro',
    name: 'Professional',
    description: 'For teams shipping to production',
    monthlyPrice: 499,
    cta: 'Start free trial',
    ctaVariant: 'primary',
    featured: true,
    highlights: ['Unlimited scans', 'Full detector library + AI co-auditor', 'CI/CD and REST API'],
  },
  {
    id: 'enterprise',
    name: 'Enterprise',
    description: 'For organisations securing billions',
    monthlyPrice: null,
    cta: 'Contact sales',
    ctaVariant: 'secondary',
    featured: false,
    highlights: ['On-prem deployment', 'Private detector repo', '24/7 advisor + SLA'],
  },
]

const COMPARISON_ROWS: {
  category: string
  rows: { feature: string; starter: string | boolean; pro: string | boolean; enterprise: string | boolean }[]
}[] = [
  {
    category: 'Scans',
    rows: [
      { feature: 'Scans per month', starter: '5', pro: 'Unlimited', enterprise: 'Unlimited' },
      { feature: 'Scan depth', starter: 'Standard', pro: 'Deep', enterprise: 'Deep + custom' },
      { feature: 'Parallel scan jobs', starter: '1', pro: '10', enterprise: 'Unlimited' },
    ],
  },
  {
    category: 'Security engine',
    rows: [
      { feature: 'Detector library access', starter: 'Public only', pro: 'Full', enterprise: 'Full + custom' },
      { feature: 'Dynamic invariant fuzzing', starter: false, pro: true, enterprise: true },
      { feature: 'AI co-auditor', starter: false, pro: true, enterprise: true },
      { feature: 'Self-improving engine', starter: false, pro: true, enterprise: true },
    ],
  },
  {
    category: 'Integrations',
    rows: [
      { feature: 'GitHub / GitLab CI/CD', starter: false, pro: true, enterprise: true },
      { feature: 'Slack / Discord alerts', starter: false, pro: true, enterprise: true },
      { feature: 'REST API access', starter: false, pro: true, enterprise: true },
      { feature: 'SSO / SAML', starter: false, pro: false, enterprise: true },
    ],
  },
  {
    category: 'Reports',
    rows: [
      { feature: 'PDF report export', starter: true, pro: true, enterprise: true },
      { feature: 'Shareable report links', starter: true, pro: true, enterprise: true },
      { feature: 'Minimal reproductions', starter: false, pro: true, enterprise: true },
      { feature: 'White-label reports', starter: false, pro: false, enterprise: true },
    ],
  },
  {
    category: 'Support',
    rows: [
      { feature: 'Community support', starter: true, pro: true, enterprise: true },
      { feature: 'Priority email support', starter: false, pro: true, enterprise: true },
      { feature: '24/7 security advisor', starter: false, pro: false, enterprise: true },
      { feature: 'Dedicated onboarding', starter: false, pro: false, enterprise: true },
    ],
  },
  {
    category: 'Deployment',
    rows: [
      { feature: 'Cloud hosted', starter: true, pro: true, enterprise: true },
      { feature: 'On-premises deployment', starter: false, pro: false, enterprise: true },
      { feature: 'Private detector repository', starter: false, pro: false, enterprise: true },
      { feature: 'SLA guarantee', starter: false, pro: false, enterprise: true },
    ],
  },
]

const FAQS = [
  {
    q: 'What counts as a "scan"?',
    a: 'A scan is one analysis run over a set of contracts. You can include multiple Solidity, Rust, or Move files in a single scan. Sentri runs the full detector library plus dynamic invariant fuzzing in one pass.',
  },
  {
    q: 'Can I try Professional features before paying?',
    a: 'Yes. Professional includes a 14-day free trial with full access to the AI co-auditor, unlimited scans, and CI/CD integrations. No credit card required to start.',
  },
  {
    q: 'Which chains are supported?',
    a: 'EVM-compatible chains (Ethereum, Arbitrum, Base, Polygon, Optimism, Avalanche, BNB Chain), Solana (Anchor programs), Move-based chains (Aptos, Sui), and Soroban (Stellar). More are added regularly.',
  },
  {
    q: 'How does annual billing work?',
    a: 'Annual billing is charged once per year at 20% off the monthly rate. You get one invoice per year and can cancel before renewal for a prorated refund.',
  },
  {
    q: 'What is the Enterprise SLA?',
    a: 'Enterprise includes a 99.9% uptime SLA for the scanning API and a 4-hour maximum response time for P1 security incidents. Custom SLAs are available on request.',
  },
  {
    q: 'Can I use Sentri for client audit work?',
    a: 'Yes. Professional lets you generate reports for up to 10 separate client protocols per month. Enterprise has unlimited client workspaces and white-label reporting.',
  },
]

function CellValue({ value }: { value: string | boolean }) {
  // Never rely on color alone: booleans use a distinct icon + screen-reader text.
  if (value === true) {
    return (
      <>
        <Check size={16} className="mx-auto text-signal" aria-hidden />
        <span className="sr-only">Included</span>
      </>
    )
  }
  if (value === false) {
    return (
      <>
        <Minus size={14} className="mx-auto text-outline" aria-hidden />
        <span className="sr-only">Not included</span>
      </>
    )
  }
  return <span className="text-body-md text-on-surface-variant">{value}</span>
}

export default function PricingPage() {
  const [billing, setBilling] = useState<BillingCycle>('monthly')
  const [openFaq, setOpenFaq] = useState<number | null>(null)
  const [authOpen, setAuthOpen] = useState(false)
  const [authTab, setAuthTab] = useState<'signin' | 'signup'>('signin')

  const cardsRef = useReveal()
  const tableRef = useReveal()
  const faqRef = useReveal()

  const getPrice = (monthlyPrice: number | null) => {
    if (monthlyPrice === null) return null
    if (monthlyPrice === 0) return 0
    return billing === 'annual' ? Math.round(monthlyPrice * 0.8) : monthlyPrice
  }

  const openSignup = () => {
    setAuthTab('signup')
    setAuthOpen(true)
  }

  return (
    <div className="flex min-h-dvh flex-col bg-surface">
      <MarketingNav />

      <main id="main" className="flex-1">
        {/* ── Hero ── */}
        <section className="relative overflow-hidden">
          <AmbientBackground />
          <Container className="py-16 text-center sm:py-20">
            <div className="animate-fade-in-up flex justify-center">
              <Badge tone="indigo">Simple, transparent pricing</Badge>
            </div>
            <h1 className="animate-fade-in-up stagger-1 mx-auto mt-6 max-w-3xl text-display-md text-on-surface text-balance">
              Plans for every stage
            </h1>
            <p className="animate-fade-in-up stagger-2 mx-auto mt-5 max-w-xl text-body-lg text-on-surface-variant">
              Start free. Scale when you&apos;re ready. No hidden fees.
            </p>

            {/* Billing toggle */}
            <div
              className="animate-fade-in-up stagger-3 mt-10 inline-flex items-center gap-1 rounded-xl border border-outline-variant bg-surface-container-low p-1"
              role="group"
              aria-label="Billing cycle"
            >
              {(['monthly', 'annual'] as const).map((cycle) => (
                <button
                  key={cycle}
                  onClick={() => setBilling(cycle)}
                  aria-pressed={billing === cycle}
                  className={clsx(
                    'flex items-center gap-2 rounded-lg px-4 py-1.5 text-sm font-[600] transition-colors',
                    billing === cycle
                      ? 'bg-surface-container-high text-on-surface'
                      : 'text-outline hover:text-on-surface',
                  )}
                >
                  {cycle === 'monthly' ? 'Monthly' : 'Annual'}
                  {cycle === 'annual' && (
                    <span className="rounded border border-signal-border bg-signal-bg px-1.5 py-0.5 font-mono text-[0.65rem] text-signal">
                      −20%
                    </span>
                  )}
                </button>
              ))}
            </div>
          </Container>
        </section>

        {/* ── Plan cards ── */}
        <section className="pb-20">
          <Container>
            <div ref={cardsRef} className="reveal mx-auto grid max-w-5xl grid-cols-1 gap-5 md:grid-cols-3">
              {PLANS.map((plan) => {
                const price = getPrice(plan.monthlyPrice)
                return (
                  <div
                    key={plan.id}
                    className={clsx(
                      'relative flex flex-col rounded-2xl p-8',
                      plan.featured
                        ? 'border-2 border-indigo bg-indigo/[0.06] animate-border-glow'
                        : 'border border-outline-variant bg-surface-container-low/70 lift-on-hover',
                    )}
                  >
                    {plan.featured && (
                      <div className="absolute -top-3 left-1/2 -translate-x-1/2">
                        <Badge tone="indigo">Most popular</Badge>
                      </div>
                    )}

                    <span className="text-label-sm text-on-surface-variant">{plan.name}</span>

                    <div className="mt-3 flex items-end gap-1">
                      {price === null ? (
                        <span className="font-mono text-4xl font-[700] text-on-surface">Custom</span>
                      ) : (
                        <>
                          <span className="font-mono text-5xl font-[700] tracking-tight text-on-surface">
                            ${price}
                          </span>
                          <span className="mb-1.5 text-body-md text-outline">/mo</span>
                        </>
                      )}
                    </div>

                    {/* Reserve the line so cards don't shift when toggling billing */}
                    <p className="mt-1 min-h-5 text-xs text-signal">
                      {billing === 'annual' && price !== null && price > 0
                        ? `Billed $${price * 12}/year`
                        : ' '}
                    </p>

                    <p className="mt-2 text-body-md text-outline">{plan.description}</p>

                    <div
                      className={clsx(
                        'my-6 h-px',
                        plan.featured ? 'bg-indigo/30' : 'bg-outline-variant',
                      )}
                    />

                    <ul className="mb-8 space-y-3">
                      {plan.highlights.map((h) => (
                        <li key={h} className="flex items-start gap-2.5">
                          <Check size={15} className="mt-0.5 flex-shrink-0 text-signal" />
                          <span className="text-body-md text-on-surface-variant">{h}</span>
                        </li>
                      ))}
                    </ul>

                    <div className="mt-auto">
                      {plan.id === 'enterprise' ? (
                        <Link href="/contact" className="block">
                          <Button variant={plan.ctaVariant} fullWidth>
                            {plan.cta}
                          </Button>
                        </Link>
                      ) : (
                        <Button variant={plan.ctaVariant} fullWidth onClick={openSignup}>
                          {plan.cta}
                        </Button>
                      )}
                      <p className="mt-3 text-center text-xs text-outline">
                        {plan.id === 'starter'
                          ? 'No credit card required'
                          : plan.id === 'pro'
                            ? '14-day free trial included'
                            : 'Custom contract & SLA'}
                      </p>
                    </div>
                  </div>
                )
              })}
            </div>
          </Container>
        </section>

        {/* ── Comparison table ── */}
        <section className="border-y border-outline-variant bg-surface-container-lowest py-20">
          <Container>
            <SectionHeading
              align="center"
              className="mx-auto mb-12"
              eyebrow="Compare"
              title="Every feature, side by side"
            />

            {/* Wide content scrolls inside its own container — the page never
                scrolls horizontally. */}
            <div ref={tableRef} className="reveal -mx-5 overflow-x-auto px-5 sm:mx-0 sm:px-0">
              <table className="w-full min-w-[640px] border-collapse text-left">
                <caption className="sr-only">Feature comparison across Sentri plans</caption>
                <thead>
                  <tr className="border-b border-outline-variant">
                    <th scope="col" className="py-4 pr-4 text-label-sm text-outline">
                      Feature
                    </th>
                    {PLANS.map((p) => (
                      <th
                        key={p.id}
                        scope="col"
                        className={clsx(
                          'w-[140px] px-4 py-4 text-center text-body-md font-[600]',
                          p.featured ? 'text-indigo-bright' : 'text-on-surface',
                        )}
                      >
                        {p.name}
                      </th>
                    ))}
                  </tr>
                </thead>
                <tbody>
                  {COMPARISON_ROWS.map((group) => (
                    <Fragment key={group.category}>
                      <tr>
                        <th
                          scope="colgroup"
                          colSpan={4}
                          className="pb-2 pt-8 text-left text-label-sm text-indigo-bright"
                        >
                          {group.category}
                        </th>
                      </tr>
                      {group.rows.map((row) => (
                        <tr
                          key={row.feature}
                          className="border-b border-outline-variant/60 transition-colors hover:bg-surface-container-low/50"
                        >
                          <th
                            scope="row"
                            className="py-3.5 pr-4 text-body-md font-[400] text-on-surface-variant"
                          >
                            {row.feature}
                          </th>
                          <td className="px-4 py-3.5 text-center">
                            <CellValue value={row.starter} />
                          </td>
                          <td
                            className={clsx(
                              'px-4 py-3.5 text-center',
                              'bg-indigo/[0.03]',
                            )}
                          >
                            <CellValue value={row.pro} />
                          </td>
                          <td className="px-4 py-3.5 text-center">
                            <CellValue value={row.enterprise} />
                          </td>
                        </tr>
                      ))}
                    </Fragment>
                  ))}
                </tbody>
              </table>
            </div>
          </Container>
        </section>

        {/* ── FAQ ── */}
        <section className="py-20">
          <Container size="prose">
            <SectionHeading
              align="center"
              className="mx-auto mb-12"
              eyebrow="FAQ"
              title="Questions, answered"
            />
            <div ref={faqRef} className="reveal space-y-3">
              {FAQS.map((faq, i) => {
                const open = openFaq === i
                return (
                  <div
                    key={faq.q}
                    className="overflow-hidden rounded-xl border border-outline-variant bg-surface-container-low/60"
                  >
                    <h3>
                      <button
                        onClick={() => setOpenFaq(open ? null : i)}
                        aria-expanded={open}
                        aria-controls={`faq-panel-${i}`}
                        id={`faq-trigger-${i}`}
                        className="flex w-full items-center justify-between gap-4 px-5 py-4 text-left transition-colors hover:bg-surface-container/50"
                      >
                        <span className="text-body-md font-[500] text-on-surface">{faq.q}</span>
                        <ChevronDown
                          size={16}
                          aria-hidden
                          className={clsx(
                            'flex-shrink-0 text-outline transition-transform duration-200',
                            open && 'rotate-180',
                          )}
                        />
                      </button>
                    </h3>
                    {open && (
                      <div
                        id={`faq-panel-${i}`}
                        role="region"
                        aria-labelledby={`faq-trigger-${i}`}
                        className="animate-fade-in px-5 pb-4"
                      >
                        <p className="text-body-md leading-6 text-on-surface-variant">{faq.a}</p>
                      </div>
                    )}
                  </div>
                )
              })}
            </div>

            <div className="mt-12 text-center">
              <p className="text-body-md text-outline">
                Still have questions?{' '}
                <Link href="/contact" className="text-indigo-bright hover:underline">
                  Talk to us
                </Link>
                .
              </p>
            </div>
          </Container>
        </section>
      </main>

      <AuthModal isOpen={authOpen} onClose={() => setAuthOpen(false)} defaultTab={authTab} />
      <MarketingFooter />
    </div>
  )
}
