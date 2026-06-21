'use client'

import { useState } from 'react'
import Link from 'next/link'
import { Check, Globe, Zap } from 'lucide-react'
import { useReveal } from '@/components/hooks/useReveal'
import { AsciiLogo } from '@/components/ui/AsciiLogo'
import { MarketingNav } from '@/components/layout/MarketingNav'
import { MarketingFooter } from '@/components/layout/MarketingFooter'
import { Button } from '@/components/ui/Button'

export default function PricingPage() {
  const starterRef = useReveal()
  const proRef = useReveal()
  const enterpriseRef = useReveal()
  const [currency, setCurrency] = useState<'USD' | 'EUR' | 'GBP'>('USD')

  const getCryptoPrice = (usdPrice: number) => {
    // Approximate conversion: 1 USD = 0.000025 BTC, 0.012 ETH
    return {
      btc: (usdPrice * 0.000025).toFixed(6),
      eth: (usdPrice * 0.012).toFixed(4),
    }
  }

  const getCurrencySymbol = () => {
    switch (currency) {
      case 'EUR':
        return '€'
      case 'GBP':
        return '£'
      default:
        return '$'
    }
  }

  const convertPrice = (usdPrice: number) => {
    switch (currency) {
      case 'EUR':
        return Math.round(usdPrice * 0.92)
      case 'GBP':
        return Math.round(usdPrice * 0.79)
      default:
        return usdPrice
    }
  }

  const plans = [
    {
      name: 'Starter',
      price: 0,
      description: 'For early-stage projects',
      features: [
        { text: '5 Scans / month', included: true },
        { text: 'Public Library Access', included: true },
        { text: 'AI Co-Auditor', included: false },
        { text: 'Priority Support', included: false },
      ],
      featured: false,
      cta: 'Choose Free',
    },
    {
      name: 'Professional',
      price: 499,
      description: 'For production protocols',
      features: [
        { text: 'Unlimited Scans', included: true },
        { text: 'Priority CI/CD Queues', included: true },
        { text: 'Full AI Co-Auditor', included: true },
        { text: 'Priority Support', included: true },
      ],
      featured: true,
      cta: 'Get Started',
    },
    {
      name: 'Enterprise',
      price: null,
      description: 'For large-scale deployments',
      features: [
        { text: 'Private Invariant Repo', included: true },
        { text: '24/7 Security Advisor', included: true },
        { text: 'On-prem deployment', included: true },
        { text: 'SLA Guarantee', included: true },
      ],
      featured: false,
      cta: 'Contact Sales',
    },
  ]

  return (
    <div className="min-h-screen bg-surface flex flex-col">
      <MarketingNav />

      <main className="flex-1 px-6 py-20">
        <div className="max-w-6xl mx-auto">
          {/* Header with ASCII Logo */}
          <div className="text-center mb-16">
            <div className="flex justify-center mb-8 opacity-[0.08] scale-50">
              <AsciiLogo />
            </div>
            <div className="inline-flex items-center gap-2 px-4 py-2 rounded-full bg-indigo/8 border border-indigo/20 mb-6">
              <span className="text-label-sm text-outline">SECURE YOUR FUTURE</span>
            </div>
            <h1 className="font-fraunces text-5xl font-[700] text-on-surface mb-4 leading-[64px]">
              Simple, predictable pricing for every stage of your protocol.
            </h1>
            <p className="text-body-lg text-outline max-w-2xl mx-auto mb-8">
              Choose the plan that fits your security needs. All plans include access to our growing invariant library and community support.
            </p>

            {/* Currency Selector */}
            <div className="flex items-center justify-center gap-3">
              <Globe size={16} className="text-on-surface-variant" />
              <div className="flex gap-2">
                {['USD', 'EUR', 'GBP'].map((curr) => (
                  <button
                    key={curr}
                    onClick={() => setCurrency(curr as any)}
                    className={`px-3 py-1.5 rounded-lg text-sm font-[600] transition ${
                      currency === curr
                        ? 'bg-indigo text-on-secondary-container'
                        : 'bg-surface-container text-outline hover:text-on-surface'
                    }`}
                  >
                    {curr}
                  </button>
                ))}
              </div>
            </div>
          </div>

          {/* Pricing Cards Grid */}
          <div className="grid grid-cols-1 md:grid-cols-3 gap-8 mb-20">
            {plans.map((plan, idx) => {
              const displayPrice = convertPrice(plan.price || 0)
              const crypto = plan.price ? getCryptoPrice(plan.price) : null

              return (
                <div
                  key={idx}
                  ref={idx === 0 ? starterRef : idx === 1 ? proRef : enterpriseRef}
                  className={`relative rounded-lg overflow-hidden reveal lift-on-hover transition ${
                    plan.featured
                      ? 'bg-gradient-to-br from-indigo-container via-secondary-container to-indigo-container border-2 border-indigo scale-105'
                      : 'bg-surface-container-low border border-outline-variant'
                  }`}
                >
                  {plan.featured && (
                    <div className="absolute -top-3 left-1/2 -translate-x-1/2 bg-secondary-container border border-indigo text-on-secondary-container px-4 py-1.5 rounded-full text-label-sm font-[600]">
                      MOST POPULAR
                    </div>
                  )}

                  <div className="p-8">
                    {/* Plan Name */}
                    <span
                      className={`text-label-sm font-[600] ${
                        plan.featured ? 'text-on-secondary-container' : 'text-outline'
                      }`}
                    >
                      {plan.name}
                    </span>

                    {/* Price */}
                    <div className="mt-4 mb-2">
                      {plan.price !== null ? (
                        <>
                          <div
                            className={`font-fraunces text-5xl font-[700] ${
                              plan.featured ? 'text-on-surface' : 'text-on-surface'
                            }`}
                          >
                            {getCurrencySymbol()}{displayPrice}
                          </div>
                          <div
                            className={`text-body-md ${
                              plan.featured ? 'text-outline' : 'text-outline'
                            }`}
                          >
                            /mo
                          </div>
                          {/* Crypto Option */}
                          {crypto && (
                            <div className="mt-2 p-2 bg-black/20 rounded text-xs text-on-surface-variant">
                              <p>or {crypto.btc} BTC / {crypto.eth} ETH</p>
                            </div>
                          )}
                        </>
                      ) : (
                        <div className="font-fraunces text-4xl font-[700] text-on-surface">
                          Custom
                        </div>
                      )}
                    </div>

                    {/* Description */}
                    <p
                      className={`text-body-md mb-8 ${
                        plan.featured ? 'text-outline' : 'text-outline'
                      }`}
                    >
                      {plan.description}
                    </p>

                    {/* Divider */}
                    <div
                      className={`border-t ${
                        plan.featured ? 'border-indigo/30' : 'border-outline-variant'
                      } my-8`}
                    />

                    {/* Features */}
                    <div className="space-y-4 mb-8">
                      {plan.features.map((feature, fidx) => (
                        <div key={fidx} className="flex items-start gap-3">
                          {feature.included ? (
                            <Check size={18} className="text-low flex-shrink-0 mt-1" />
                          ) : (
                            <span className="text-critical flex-shrink-0 mt-1">✗</span>
                          )}
                          <span
                            className={`text-body-md ${
                              feature.included
                                ? 'text-on-surface-variant'
                                : 'text-on-surface-variant line-through opacity-50'
                            }`}
                          >
                            {feature.text}
                          </span>
                        </div>
                      ))}
                    </div>

                    {/* CTA */}
                    {plan.name === 'Enterprise' ? (
                      <a href="mailto:sales@sentri.dev">
                        <Button
                          variant={plan.featured ? 'primary' : 'secondary'}
                          fullWidth
                        >
                          {plan.cta}
                        </Button>
                      </a>
                    ) : (
                      <Link href="/dashboard">
                        <Button
                          variant={plan.featured ? 'primary' : 'secondary'}
                          fullWidth
                        >
                          {plan.cta}
                        </Button>
                      </Link>
                    )}
                  </div>
                </div>
              )
            })}
          </div>

          {/* FAQ-style section */}
          <div className="max-w-3xl mx-auto">
            <h2 className="font-fraunces text-3xl font-[600] text-on-surface mb-8 text-center">
              Frequently Asked Questions
            </h2>

            <div className="space-y-6">
              {[
                {
                  q: 'Can I switch plans anytime?',
                  a: 'Yes. Upgrade or downgrade your plan at any time. Changes take effect at the start of your next billing cycle.',
                },
                {
                  q: 'What is included in unlimited scans?',
                  a: 'Unlimited scans includes all basic security checks, access to the 1,400+ invariant library, and full reports. Enterprise-grade features like AI Co-Auditor and private invariant repos require specific plans.',
                },
                {
                  q: 'Do you offer volume discounts?',
                  a: 'Yes! Contact our sales team for custom pricing on Enterprise plans, including volume discounts for large portfolios.',
                },
                {
                  q: 'Is there a free trial?',
                  a: 'The Starter plan is free forever. For Professional and Enterprise features, we offer a 14-day free trial with full functionality.',
                },
              ].map((item, idx) => (
                <div
                  key={idx}
                  className="bg-surface-container-low border border-outline-variant rounded-lg p-6"
                >
                  <h3 className="text-body-lg font-[600] text-on-surface mb-3">
                    {item.q}
                  </h3>
                  <p className="text-body-md text-outline">{item.a}</p>
                </div>
              ))}
            </div>
          </div>

          {/* Payment Methods */}
          <div className="mt-16 p-8 bg-surface-container rounded-lg border border-outline-variant text-center">
            <h3 className="font-fraunces text-2xl font-[600] text-on-surface mb-4">
              Flexible Payment Methods
            </h3>
            <p className="text-on-surface-variant mb-6">
              We accept credit cards, cryptocurrency (BTC, ETH), and bank transfers for Enterprise customers.
            </p>
            <div className="flex flex-wrap justify-center gap-4 text-sm text-on-surface-variant">
              <span className="flex items-center gap-2">
                <span className="text-xl">💳</span> Credit Card
              </span>
              <span className="flex items-center gap-2">
                <span className="text-xl">₿</span> Bitcoin
              </span>
              <span className="flex items-center gap-2">
                <span className="text-xl">Ξ</span> Ethereum
              </span>
              <span className="flex items-center gap-2">
                <span className="text-xl">🏦</span> Bank Transfer
              </span>
            </div>
          </div>
        </div>
      </main>

      <MarketingFooter />
    </div>
  )
}
