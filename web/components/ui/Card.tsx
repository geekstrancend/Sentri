import { ReactNode } from 'react'
import clsx from 'clsx'

interface CardProps {
  children: ReactNode
  className?: string
  /** Add hover lift + indigo edge highlight (for interactive/linked cards). */
  interactive?: boolean
  /** Thin indigo gradient hairline along the top edge. */
  gradientTop?: boolean
  as?: 'div' | 'article' | 'li'
}

/**
 * The standard surface primitive. One elevation tier, one radius, consistent
 * border — so cards read as a single system across every page (DESIGN.md §3).
 */
export function Card({
  children,
  className,
  interactive = false,
  gradientTop = false,
  as: Tag = 'div',
}: CardProps) {
  return (
    <Tag
      className={clsx(
        'relative rounded-xl border border-outline-variant bg-surface-container-low/70 backdrop-blur-[2px]',
        gradientTop && 'card-gradient-border',
        interactive && 'lift-on-hover',
        className,
      )}
    >
      {children}
    </Tag>
  )
}
