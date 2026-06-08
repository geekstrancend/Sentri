import type { Metadata } from 'next'
import { Fraunces, JetBrains_Mono } from 'next/font/google'
import '../styles/globals.css'

const fraunces = Fraunces({
  subsets: ['latin'],
  variable: '--font-fraunces',
  weight: ['300', '500', '600', '700', '900'],
  style: ['normal', 'italic'],
})

const jetbrainsMono = JetBrains_Mono({
  subsets: ['latin'],
  variable: '--font-jetbrains',
  weight: ['400', '500', '600', '700'],
})

export const metadata: Metadata = {
  title: 'Sentri | Smart Contract Security Intelligence',
  description: 'Audit faster. Find more. Miss nothing. Advanced symbolic execution and invariant-based security for DeFi protocols.',
  keywords: ['smart contracts', 'security', 'audit', 'DeFi', 'blockchain', 'invariants'],
  authors: [{ name: 'Sentri Security' }],
  openGraph: {
    title: 'Sentri | Smart Contract Security Intelligence',
    description: 'Don\'t get Hacked!',
    type: 'website',
    url: 'https://sentri.dev',
  },
}

export default function RootLayout({
  children,
}: {
  children: React.ReactNode
}) {
  return (
    <html
      lang="en"
      className={`${fraunces.variable} ${jetbrainsMono.variable}`}
      suppressHydrationWarning
    >
      <body className="dark">
        {children}
      </body>
    </html>
  )
}
