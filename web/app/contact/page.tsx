'use client'

import { useState } from 'react'
import { MarketingNav } from '@/components/layout/MarketingNav'
import { MarketingFooter } from '@/components/layout/MarketingFooter'
import { Button } from '@/components/ui/Button'
import { Mail, Phone, MapPin } from 'lucide-react'

export default function ContactPage() {
  const [formData, setFormData] = useState({
    name: '',
    email: '',
    company: '',
    message: '',
  })
  const [submitted, setSubmitted] = useState(false)

  const handleChange = (e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>) => {
    const { name, value } = e.target
    setFormData((prev) => ({ ...prev, [name]: value }))
  }

  const handleSubmit = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault()
    // TODO: Implement form submission
    console.log('Form submitted:', formData)
    setSubmitted(true)
    setTimeout(() => {
      setFormData({ name: '', email: '', company: '', message: '' })
      setSubmitted(false)
    }, 3000)
  }

  return (
    <div className="min-h-screen bg-surface flex flex-col">
      <MarketingNav />

      <main className="flex-1 max-w-4xl mx-auto px-6 py-12 w-full">
        <h1 className="font-fraunces text-5xl font-[600] text-on-surface mb-2">Contact Sales</h1>
        <p className="text-body-lg text-outline mb-12">
          Have questions about enterprise plans? We'd love to hear from you. Get in touch with our team.
        </p>

        <div className="grid grid-cols-1 lg:grid-cols-3 gap-8 mb-12">
          {/* Contact Info */}
          <div className="lg:col-span-1">
            <div className="space-y-6">
              {/* Email */}
              <div className="flex gap-4">
                <div className="flex-shrink-0">
                  <Mail size={24} className="text-secondary" />
                </div>
                <div>
                  <h3 className="text-body-lg font-[600] text-on-surface mb-1">Email</h3>
                  <a href="mailto:sales@sentri.dev" className="text-outline hover:text-on-surface transition-colors">
                    sales@sentri.dev
                  </a>
                </div>
              </div>

              {/* Phone */}
              <div className="flex gap-4">
                <div className="flex-shrink-0">
                  <Phone size={24} className="text-secondary" />
                </div>
                <div>
                  <h3 className="text-body-lg font-[600] text-on-surface mb-1">Phone</h3>
                  <a href="tel:+1234567890" className="text-outline hover:text-on-surface transition-colors">
                    +1 (234) 567-890
                  </a>
                </div>
              </div>

              {/* Address */}
              <div className="flex gap-4">
                <div className="flex-shrink-0">
                  <MapPin size={24} className="text-secondary" />
                </div>
                <div>
                  <h3 className="text-body-lg font-[600] text-on-surface mb-1">Office</h3>
                  <p className="text-outline">
                    123 Security Lane
                    <br />
                    Austin, TX 78701
                    <br />
                    United States
                  </p>
                </div>
              </div>
            </div>
          </div>

          {/* Contact Form */}
          <div className="lg:col-span-2">
            <form onSubmit={handleSubmit} className="space-y-4">
              <div>
                <label htmlFor="name" className="block text-body-md text-on-surface mb-2">
                  Full Name
                </label>
                <input
                  type="text"
                  id="name"
                  name="name"
                  value={formData.name}
                  onChange={handleChange}
                  required
                  className="w-full px-4 py-2 bg-surface-container border border-outline-variant rounded-lg text-on-surface placeholder-outline-variant focus:outline-none focus:ring-2 focus:ring-secondary transition-all"
                  placeholder="Your name"
                />
              </div>

              <div>
                <label htmlFor="email" className="block text-body-md text-on-surface mb-2">
                  Email Address
                </label>
                <input
                  type="email"
                  id="email"
                  name="email"
                  value={formData.email}
                  onChange={handleChange}
                  required
                  className="w-full px-4 py-2 bg-surface-container border border-outline-variant rounded-lg text-on-surface placeholder-outline-variant focus:outline-none focus:ring-2 focus:ring-secondary transition-all"
                  placeholder="your@email.com"
                />
              </div>

              <div>
                <label htmlFor="company" className="block text-body-md text-on-surface mb-2">
                  Company
                </label>
                <input
                  type="text"
                  id="company"
                  name="company"
                  value={formData.company}
                  onChange={handleChange}
                  className="w-full px-4 py-2 bg-surface-container border border-outline-variant rounded-lg text-on-surface placeholder-outline-variant focus:outline-none focus:ring-2 focus:ring-secondary transition-all"
                  placeholder="Your company"
                />
              </div>

              <div>
                <label htmlFor="message" className="block text-body-md text-on-surface mb-2">
                  Message
                </label>
                <textarea
                  id="message"
                  name="message"
                  value={formData.message}
                  onChange={handleChange}
                  required
                  rows={5}
                  className="w-full px-4 py-2 bg-surface-container border border-outline-variant rounded-lg text-on-surface placeholder-outline-variant focus:outline-none focus:ring-2 focus:ring-secondary transition-all"
                  placeholder="Tell us about your needs..."
                />
              </div>

              <Button type="submit" variant="primary" fullWidth>
                {submitted ? 'Message Sent! ✓' : 'Send Message'}
              </Button>
            </form>
          </div>
        </div>
      </main>

      <MarketingFooter />
    </div>
  )
}
