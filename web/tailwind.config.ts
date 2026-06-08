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
        fraunces: 'var(--font-fraunces), serif',
        mono: 'var(--font-jetbrains), monospace',
      },
      colors: {
        surface: 'var(--color-surface)',
        'surface-dim': 'var(--color-surface-dim)',
        'surface-bright': 'var(--color-surface-bright)',
        'surface-container-lowest': 'var(--color-surface-container-lowest)',
        'surface-container-low': 'var(--color-surface-container-low)',
        'surface-container': 'var(--color-surface-container)',
        'surface-container-high': 'var(--color-surface-container-high)',
        'surface-container-highest': 'var(--color-surface-container-highest)',
        'on-surface': 'var(--color-on-surface)',
        'on-surface-variant': 'var(--color-on-surface-variant)',
        outline: 'var(--color-outline)',
        'outline-variant': 'var(--color-outline-variant)',
        primary: 'var(--color-primary)',
        'primary-container': 'var(--color-primary-container)',
        secondary: 'var(--color-secondary)',
        'secondary-container': 'var(--color-secondary-container)',
        'on-secondary': 'var(--color-on-secondary)',
        'on-secondary-container': 'var(--color-on-secondary-container)',
        error: 'var(--color-error)',
        'error-container': 'var(--color-error-container)',
        background: 'var(--color-background)',
        'on-background': 'var(--color-on-background)',
        'surface-variant': 'var(--color-surface-variant)',
        critical: 'var(--color-critical)',
        'critical-bg': 'var(--color-critical-bg)',
        'critical-border': 'var(--color-critical-border)',
        high: 'var(--color-high)',
        'high-bg': 'var(--color-high-bg)',
        'high-border': 'var(--color-high-border)',
        medium: 'var(--color-medium)',
        'medium-bg': 'var(--color-medium-bg)',
        'medium-border': 'var(--color-medium-border)',
        low: 'var(--color-low)',
        'low-bg': 'var(--color-low-bg)',
        'low-border': 'var(--color-low-border)',
        indigo: 'var(--color-indigo)',
        'indigo-container': 'var(--color-indigo-container)',
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
            color: 'var(--color-on-surface)',
            a: {
              color: 'var(--color-indigo)',
            },
            code: {
              color: 'var(--color-secondary)',
              backgroundColor: 'var(--color-surface-container)',
              padding: '0.2em 0.4em',
              borderRadius: '0.25em',
            },
            pre: {
              backgroundColor: 'var(--color-surface-container-lowest)',
              borderColor: 'var(--color-outline-variant)',
            },
          },
        },
      },
    },
  },
  plugins: [require('@tailwindcss/typography')],
}

export default config
