'use client'

import { ReactNode } from 'react'
import clsx from 'clsx'
import { Badge } from './Badge'
import { useReveal } from '../hooks/useReveal'

interface ContainerProps {
  children: ReactNode
  className?: string
  /** max-w-7xl (default) vs a tighter reading width for prose sections. */
  size?: 'default' | 'prose'
}

/** Consistent page gutter + max width across every marketing surface. */
export function Container({ children, className, size = 'default' }: ContainerProps) {
  return (
    <div
      className={clsx(
        'mx-auto w-full px-5 sm:px-6 lg:px-8',
        size === 'default' ? 'max-w-7xl' : 'max-w-3xl',
        className,
      )}
    >
      {children}
    </div>
  )
}

interface SectionHeadingProps {
  eyebrow?: string
  title: ReactNode
  description?: ReactNode
  align?: 'left' | 'center'
  className?: string
}

/**
 * The standard section header: mono eyebrow → display title → muted lede.
 * Keeps hierarchy and rhythm identical across sections.
 */
export function SectionHeading({
  eyebrow,
  title,
  description,
  align = 'left',
  className,
}: SectionHeadingProps) {
  const ref = useReveal()
  return (
    <div
      ref={ref}
      className={clsx(
        'reveal flex flex-col gap-4',
        align === 'center' && 'items-center text-center',
        className,
      )}
    >
      {eyebrow && (
        <Badge tone="indigo">{eyebrow}</Badge>
      )}
      <h2 className="text-display-sm text-on-surface max-w-2xl text-balance">{title}</h2>
      {description && (
        <p
          className={clsx(
            'text-body-lg text-on-surface-variant',
            align === 'center' ? 'max-w-2xl' : 'max-w-xl',
          )}
        >
          {description}
        </p>
      )}
    </div>
  )
}
