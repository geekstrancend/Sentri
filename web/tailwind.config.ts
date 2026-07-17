import type { Config } from 'tailwindcss'

const config: Config = {
  content: [
    './app/**/*.{js,ts,jsx,tsx,mdx}',
    './components/**/*.{js,ts,jsx,tsx,mdx}',
  ],
  darkMode: 'class',
  theme: {
    extend: {
      fontFamily: {
        sans: 'var(--font-sans)',
        mono: 'var(--font-mono)',
        display: 'var(--font-display)',
        // `fraunces` is aliased to the sans (Geist) stack for backward
        // compatibility: existing `font-fraunces` headings now render Geist,
        // matching the current design language (DESIGN.md §3) with no churn.
        fraunces: 'var(--font-sans)',
      },
      colors: {
        surface: 'var(--surface)',
        'surface-dim': 'var(--surface-dim)',
        'surface-bright': 'var(--surface-bright)',
        'surface-container-lowest': 'var(--surface-container-lowest)',
        'surface-container-low': 'var(--surface-container-low)',
        'surface-container': 'var(--surface-container)',
        'surface-container-high': 'var(--surface-container-high)',
        'surface-container-highest': 'var(--surface-container-highest)',
        'on-surface': 'var(--on-surface)',
        'on-surface-variant': 'var(--on-surface-variant)',
        outline: 'var(--outline)',
        'outline-variant': 'var(--outline-variant)',
        primary: 'var(--primary)',
        'primary-container': 'var(--primary-container)',
        secondary: 'var(--secondary)',
        'secondary-container': 'var(--secondary-container)',
        'on-secondary': 'var(--on-secondary)',
        'on-secondary-container': 'var(--on-secondary-container)',
        error: 'var(--error)',
        'error-container': 'var(--error-container)',
        background: 'var(--background)',
        'on-background': 'var(--on-background)',
        'surface-variant': 'var(--surface-variant)',
        critical: 'var(--critical)',
        'critical-bg': 'var(--critical-bg)',
        'critical-border': 'var(--critical-border)',
        high: 'var(--high)',
        'high-bg': 'var(--high-bg)',
        'high-border': 'var(--high-border)',
        medium: 'var(--medium)',
        'medium-bg': 'var(--medium-bg)',
        'medium-border': 'var(--medium-border)',
        low: 'var(--low)',
        'low-bg': 'var(--low-bg)',
        'low-border': 'var(--low-border)',
        indigo: 'var(--indigo)',
        'indigo-bright': 'var(--indigo-bright)',
        'indigo-container': 'var(--indigo-container)',
        signal: 'var(--signal)',
        'signal-bright': 'var(--signal-bright)',
        'signal-bg': 'var(--signal-bg)',
        'signal-border': 'var(--signal-border)',
      },
      boxShadow: {
        glow: '0 0 40px -8px rgba(129, 140, 248, 0.35)',
        'glow-signal': '0 0 40px -8px rgba(74, 222, 128, 0.3)',
        card: '0 1px 2px rgba(0,0,0,0.4), 0 8px 24px -12px rgba(0,0,0,0.5)',
        'card-lg': '0 14px 40px -12px rgba(0,0,0,0.55)',
      },
      spacing: {
        xs: '4px',
        sm: '8px',
        md: '16px',
        lg: '24px',
        xl: '40px',
        xxl: '64px',
        gutter: '16px',
        margin: '24px',
      },
      borderRadius: {
        xs: '2px',
        sm: '2px',
        DEFAULT: '4px',
        md: '6px',
        lg: '8px',
        xl: '12px',
        full: '9999px',
      },
      typography: {
        DEFAULT: {
          css: {
            color: 'var(--on-surface)',
            a: {
              color: 'var(--indigo)',
            },
            code: {
              color: 'var(--secondary)',
              backgroundColor: 'var(--surface-container)',
              padding: '0.2em 0.4em',
              borderRadius: '0.25em',
            },
            pre: {
              backgroundColor: 'var(--surface-container-lowest)',
              borderColor: 'var(--outline-variant)',
            },
          },
        },
      },
    },
  },
  plugins: [require('@tailwindcss/typography')],
}

export default config
