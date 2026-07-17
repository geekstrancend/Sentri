import { ReactNode } from 'react'
import clsx from 'clsx'

type BadgeTone = 'neutral' | 'indigo' | 'signal' | 'outline'

interface BadgeProps {
  children: ReactNode
  tone?: BadgeTone
  icon?: ReactNode
  className?: string
}

const toneStyles: Record<BadgeTone, string> = {
  neutral: 'bg-surface-container border-outline-variant text-on-surface-variant',
  indigo: 'bg-indigo/10 border-indigo/25 text-indigo-bright',
  signal: 'bg-signal-bg border-signal-border text-signal-bright',
  outline: 'bg-transparent border-outline-variant text-outline',
}

/**
 * Small pill for labels, statuses, and eyebrow tags. Mono type is the
 * technical "voice" per DESIGN.md §3.
 */
export function Badge({ children, tone = 'neutral', icon, className }: BadgeProps) {
  return (
    <span
      className={clsx(
        'inline-flex items-center gap-1.5 rounded-full border px-2.5 py-1',
        'font-mono text-[0.6875rem] font-[600] uppercase tracking-[0.06em]',
        toneStyles[tone],
        className,
      )}
    >
      {icon && <span className="flex items-center">{icon}</span>}
      {children}
    </span>
  )
}
