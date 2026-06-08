'use client'

import { Check } from 'lucide-react'
import { MarketingNav } from '@/components/layout/MarketingNav'
import { MarketingFooter } from '@/components/layout/MarketingFooter'
import { Button } from '@/components/ui/Button'

export default function PricingPage() {
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
          {/* Header */}
          <div className="text-center mb-16">
            <div className="inline-flex items-center gap-2 px-4 py-2 rounded-full bg-indigo/8 border border-indigo/20 mb-6">
              <span className="text-label-sm text-outline">SECURE YOUR FUTURE</span>
            </div>
            <h1 className="font-fraunces text-4xl font-[600] text-on-surface mb-4 leading-[48px]">
              Simple, predictable pricing for every stage of your protocol.
            </h1>
            <p className="text-body-lg text-outline max-w-2xl mx-auto">
              Choose the plan that fits your security needs. All plans include access to our growing invariant library and community support.
            </p>
          </div>

          {/* Pricing Cards Grid */}
          <div className="grid grid-cols-1 md:grid-cols-3 gap-8 mb-20">
            {plans.map((plan, idx) => (
              <div
                key={idx}
                className={`relative rounded-lg overflow-hidden ${
                  plan.featured
                    ? 'bg-indigo-container border-2 border-indigo'
                    : 'bg-surface-container-low border border-outline-variant'
                }`}
              >
                {plan.featured && (
                  <div className="absolute -top-3 left-1/2 -translate-x-1/2 bg-secondary-container border border-indigo text-on-secondary-container px-4 py-1.5 rounded-full text-label-sm">
                    MOST POPULAR
                  </div>
                )}

                <div className="p-8">
                  {/* Plan Name */}
                  <span
                    className={`text-label-sm ${
                      plan.featured ? 'text-on-secondary-container' : 'text-outline'
                    }`}
                  >
                    {plan.name}
                  </span>

                  {/* Price */}
                  <div className="mt-4 mb-2">
                    {plan.price !== null ? (
                      <>
                        <div className={`font-fraunces text-5xl font-[700] ${plan.featured ? 'text-on-surface' : 'text-on-surface'}`}>
                          ${plan.price}
                        </div>
                        <div className={`text-body-md ${plan.featured ? 'text-outline' : 'text-outline'}`}>
                          /mo
                        </div>
                      </>
                    ) : (
                      <div className="font-fraunces text-4xl font-[700] text-on-surface">
                        Custom
                      </div>
                    )}
                  </div>

                  {/* Description */}
                  <p className={`text-body-md mb-8 ${plan.featured ? 'text-outline' : 'text-outline'}`}>
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
                      <div
                        key={fidx}
                        className="flex items-start gap-3"
                      >
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
                  <Button
                    variant={plan.featured ? 'primary' : 'secondary'}
                    fullWidth
                  >
                    {plan.cta}
                  </Button>
                </div>
              </div>
            ))}
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
                  <p className="text-body-md text-outline">
                    {item.a}
                  </p>
                </div>
              ))}
            </div>
          </div>
        </div>
      </main>

      <MarketingFooter />
    </div>
  )
}
