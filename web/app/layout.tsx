import type { Metadata } from 'next'
import '../styles/globals.css'
import { AuthProvider } from './providers'

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
      {/* App Router root layout is the global place for these, so the
          no-page-custom-font rule (which targets pages/_document.js) does not
          apply. next/font is not usable here: it breaks under this project's
          Turbopack dev server. */}
      <head>
        <link rel="preconnect" href="https://fonts.googleapis.com" />
        <link rel="preconnect" href="https://fonts.gstatic.com" crossOrigin="anonymous" />
        {/* eslint-disable-next-line @next/next/no-page-custom-font */}
        <link
          href="https://fonts.googleapis.com/css2?family=Bricolage+Grotesque:opsz,wght@12..96,400;12..96,600;12..96,700;12..96,800&family=Hanken+Grotesk:wght@400;500;600&family=IBM+Plex+Mono:wght@400;500&display=swap"
          rel="stylesheet"
        />
      </head>
      <body>
        <AuthProvider>
          {children}
        </AuthProvider>
      </body>
    </html>
  )
}
