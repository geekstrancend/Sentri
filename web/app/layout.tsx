import type { Metadata, Viewport } from 'next'
import { Inter, JetBrains_Mono } from 'next/font/google'
import '../styles/globals.css'
import { AuthProvider } from './providers'

// Inter carries all UI and prose; JetBrains Mono is the "technical voice"
// (code, labels, data, terminal). See DESIGN.md §3.
const inter = Inter({
  subsets: ['latin'],
  display: 'swap',
  variable: '--font-inter',
  weight: ['400', '500', '600', '700'],
})

const jetbrainsMono = JetBrains_Mono({
  subsets: ['latin'],
  display: 'swap',
  variable: '--font-jet',
  weight: ['400', '500', '600'],
})

const SITE_URL = 'https://sentri.dev'

export const metadata: Metadata = {
  metadataBase: new URL(SITE_URL),
  title: {
    default: 'Sentri — Smart Contract Security Intelligence',
    template: '%s · Sentri',
  },
  description:
    'Audit faster. Find more. Miss nothing. Multi-chain static analysis and dynamic invariant fuzzing that secures DeFi protocols before the first block is mined.',
  keywords: [
    'smart contract security',
    'smart contract audit',
    'invariant fuzzing',
    'DeFi security',
    'blockchain',
    'Solidity',
    'static analysis',
  ],
  authors: [{ name: 'Sentri Security' }],
  openGraph: {
    title: 'Sentri — Smart Contract Security Intelligence',
    description: 'Audit faster. Find more. Miss nothing.',
    type: 'website',
    url: SITE_URL,
    siteName: 'Sentri',
  },
  twitter: {
    card: 'summary_large_image',
    title: 'Sentri — Smart Contract Security Intelligence',
    description: 'Audit faster. Find more. Miss nothing.',
  },
}

export const viewport: Viewport = {
  themeColor: '#131314',
  width: 'device-width',
  initialScale: 1,
}

export default function RootLayout({
  children,
}: {
  children: React.ReactNode
}) {
  return (
    <html
      lang="en"
      className={`dark ${inter.variable} ${jetbrainsMono.variable}`}
      suppressHydrationWarning
    >
      <body>
        <a
          href="#main"
          className="sr-only focus:not-sr-only focus:fixed focus:top-4 focus:left-4 focus:z-[100] focus:rounded-lg focus:bg-surface-container-high focus:px-4 focus:py-2 focus:text-sm focus:text-on-surface focus:outline-2 focus:outline-indigo"
        >
          Skip to content
        </a>
        <AuthProvider>{children}</AuthProvider>
      </body>
    </html>
  )
}
