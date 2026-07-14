'use client'

import { useState } from 'react'
import { MarketingNav } from '@/components/layout/MarketingNav'
import { MarketingFooter } from '@/components/layout/MarketingFooter'
import { Button } from '@/components/ui/Button'
import { Mail, MessageSquare, Building2, CheckCircle2, ArrowRight, ShieldCheck, ChevronDown } from 'lucide-react'

const CONTACT_REASONS = [
  'Enterprise plan inquiry',
  'On-premises deployment',
  'Custom invariant library',
  'Integration partnership',
  'Security research collaboration',
  'Other',
]

export default function ContactPage() {
  const [form, setForm] = useState({ name: '', email: '', company: '', reason: '', message: '' })
  const [submitted, setSubmitted] = useState(false)
  const [loading, setLoading] = useState(false)

  const handleChange = (e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement | HTMLSelectElement>) => {
    setForm((prev) => ({ ...prev, [e.target.name]: e.target.value }))
  }

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    setLoading(true)
    // Simulate API call
    await new Promise((r) => setTimeout(r, 1200))
    setLoading(false)
    setSubmitted(true)
  }

  return (
    <div className="min-h-screen bg-surface flex flex-col">
      <MarketingNav />

      <main className="flex-1">
        {/* Hero */}
        <section className="px-6 py-20 border-b border-outline-variant bg-surface-container-lowest">
          <div className="max-w-5xl mx-auto">
            <div className="inline-flex items-center gap-2 px-4 py-2 rounded-full bg-indigo/8 border border-indigo/20 mb-6">
              <MessageSquare size={14} className="text-on-secondary-container" />
              <span className="text-label-sm text-on-secondary-container">CONTACT SALES</span>
            </div>
            <h1 className="font-fraunces text-5xl font-[700] text-on-surface mb-4 leading-[64px]">
              Let's secure your protocol
            </h1>
            <p className="text-body-lg text-outline max-w-xl">
              Reach out to discuss Enterprise plans, custom deployments, or research partnerships. Our team responds within 24 hours.
            </p>
          </div>
        </section>

        <section className="px-6 py-16 max-w-5xl mx-auto">
          <div className="grid grid-cols-1 lg:grid-cols-3 gap-12">

            {/* Left: info */}
            <div className="space-y-8">
              {[
                {
                  icon: <Mail size={20} className="text-secondary" />,
                  title: 'Email',
                  value: 'sales@sentri.dev',
                  link: 'mailto:sales@sentri.dev',
                },
                {
                  icon: <Building2 size={20} className="text-secondary" />,
                  title: 'Enterprise',
                  value: 'Custom contracts & SLAs available',
                  link: null,
                },
                {
                  icon: <ShieldCheck size={20} className="text-secondary" />,
                  title: 'Security Disclosure',
                  value: 'security@sentri.dev',
                  link: 'mailto:security@sentri.dev',
                },
              ].map((item, i) => (
                <div key={i} className="flex gap-4">
                  <div className="w-10 h-10 rounded-lg bg-indigo/10 border border-indigo/20 flex items-center justify-center flex-shrink-0">
                    {item.icon}
                  </div>
                  <div>
                    <p className="text-label-sm text-outline mb-1">{item.title}</p>
                    {item.link ? (
                      <a href={item.link} className="text-body-md text-on-surface hover:text-secondary transition-colors">
                        {item.value}
                      </a>
                    ) : (
                      <p className="text-body-md text-on-surface-variant">{item.value}</p>
                    )}
                  </div>
                </div>
              ))}

              {/* Expectation list */}
              <div className="bg-surface-container-low border border-outline-variant rounded-xl p-6 space-y-3">
                <p className="text-label-sm text-outline mb-4">WHAT HAPPENS NEXT</p>
                {[
                  'We\'ll reply within 1 business day',
                  'A security engineer will join the call',
                  'We\'ll provide a custom proof-of-concept scan',
                ].map((item, i) => (
                  <div key={i} className="flex items-start gap-2">
                    <span className="text-secondary font-[700] flex-shrink-0 text-sm mt-0.5">{String(i + 1).padStart(2, '0')}</span>
                    <p className="text-body-md text-on-surface-variant">{item}</p>
                  </div>
                ))}
              </div>
            </div>

            {/* Right: form */}
            <div className="lg:col-span-2">
              {submitted ? (
                <div className="flex flex-col items-center justify-center text-center py-16 bg-surface-container-low border border-outline-variant rounded-xl">
                  <div className="w-16 h-16 rounded-full bg-low/10 border border-low/20 flex items-center justify-center mb-5">
                    <CheckCircle2 size={32} className="text-low" />
                  </div>
                  <h2 className="font-fraunces text-2xl font-[600] text-on-surface mb-3">Message sent!</h2>
                  <p className="text-body-lg text-outline max-w-sm">
                    Thanks for reaching out. A member of our team will get back to you within 24 hours.
                  </p>
                  <button
                    onClick={() => { setSubmitted(false); setForm({ name: '', email: '', company: '', reason: '', message: '' }) }}
                    className="mt-6 text-secondary text-sm font-[600] hover:text-secondary/80 transition-colors"
                  >
                    Send another message
                  </button>
                </div>
              ) : (
                <form onSubmit={handleSubmit} className="space-y-5">
                  <div className="grid grid-cols-1 sm:grid-cols-2 gap-5">
                    <div>
                      <label htmlFor="contact-name" className="block text-body-md text-on-surface font-[500] mb-2">Full Name *</label>
                      <input
                        id="contact-name"
                        type="text"
                        name="name"
                        required
                        value={form.name}
                        onChange={handleChange}
                        placeholder="Jane Smith"
                        className="w-full px-4 py-2.5 bg-surface-container-lowest border border-outline-variant rounded-lg text-body-md text-on-surface placeholder-outline-variant focus:outline-none focus:border-indigo transition-colors"
                      />
                    </div>
                    <div>
                      <label htmlFor="contact-email" className="block text-body-md text-on-surface font-[500] mb-2">Work Email *</label>
                      <input
                        id="contact-email"
                        type="email"
                        name="email"
                        required
                        value={form.email}
                        onChange={handleChange}
                        placeholder="jane@protocol.io"
                        className="w-full px-4 py-2.5 bg-surface-container-lowest border border-outline-variant rounded-lg text-body-md text-on-surface placeholder-outline-variant focus:outline-none focus:border-indigo transition-colors"
                      />
                    </div>
                  </div>

                  <div>
                    <label htmlFor="contact-company" className="block text-body-md text-on-surface font-[500] mb-2">Company / Protocol</label>
                    <input
                      id="contact-company"
                      type="text"
                      name="company"
                      value={form.company}
                      onChange={handleChange}
                      placeholder="Acme Protocol"
                      className="w-full px-4 py-2.5 bg-surface-container-lowest border border-outline-variant rounded-lg text-body-md text-on-surface placeholder-outline-variant focus:outline-none focus:border-indigo transition-colors"
                    />
                  </div>

                  <div>
                    <label htmlFor="contact-reason" className="block text-body-md text-on-surface font-[500] mb-2">Reason for Inquiry</label>
                    <div className="relative">
                      <select
                        id="contact-reason"
                        name="reason"
                        value={form.reason}
                        onChange={handleChange}
                        className="w-full px-4 py-2.5 bg-surface-container-lowest border border-outline-variant rounded-lg text-body-md text-on-surface focus:outline-none focus:border-indigo transition-colors appearance-none"
                      >
                        <option value="">Select a reason…</option>
                        {CONTACT_REASONS.map((r) => <option key={r} value={r}>{r}</option>)}
                      </select>
                      <ChevronDown size={16} className="absolute right-4 top-1/2 -translate-y-1/2 text-outline pointer-events-none" />
                    </div>
                  </div>

                  <div>
                    <label htmlFor="contact-message" className="block text-body-md text-on-surface font-[500] mb-2">Message *</label>
                    <textarea
                      id="contact-message"
                      name="message"
                      required
                      rows={5}
                      value={form.message}
                      onChange={handleChange}
                      placeholder="Tell us about your protocol, team size, and what you'd like to achieve with Sentri…"
                      className="w-full px-4 py-2.5 bg-surface-container-lowest border border-outline-variant rounded-lg text-body-md text-on-surface placeholder-outline-variant focus:outline-none focus:border-indigo transition-colors resize-none"
                    />
                  </div>

                  <Button
                    type="submit"
                    variant="primary"
                    size="lg"
                    fullWidth
                    icon={loading ? undefined : <ArrowRight size={16} />}
                    iconPosition="right"
                    disabled={loading}
                  >
                    {loading ? 'Sending…' : 'Send Message'}
                  </Button>

                  <p className="text-xs text-outline text-center">
                    By submitting, you agree to our{' '}
                    <a href="/privacy" className="hover:text-outline transition-colors underline">Privacy Policy</a>.
                  </p>
                </form>
              )}
            </div>
          </div>
        </section>
      </main>

      <MarketingFooter />
    </div>
  )
}
