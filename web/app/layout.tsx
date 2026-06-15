import type { Metadata } from 'next'
import '../styles/globals.css'

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
      className="dark"
      suppressHydrationWarning
    >
      <body>
        {children}
      </body>
    </html>
  )
}
