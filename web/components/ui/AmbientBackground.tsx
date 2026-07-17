'use client'

import clsx from 'clsx'
import { useParallax } from '../hooks/useScrollProgress'
import { AsciiField } from './AsciiField'

interface AmbientBackgroundProps {
  /** Tiled ASCII glyph field — the signature texture (DESIGN.md §3). */
  ascii?: boolean
  /** Move the field against the scroll for depth. */
  parallax?: boolean
  /** Faint engineering grid. Off by default; ASCII carries the texture. */
  grid?: boolean
  /** Drifting indigo spotlight blob for depth/focus. */
  spotlight?: boolean
  /** How strongly the ASCII field reads. */
  asciiOpacity?: number
  className?: string
}

/**
 * Decorative-but-purposeful backdrop. Purely presentational — always
 * aria-hidden and behind content. Parallax returns 0 under
 * prefers-reduced-motion, so the layers simply hold still.
 */
export function AmbientBackground({
  ascii = true,
  parallax = true,
  grid = false,
  spotlight = true,
  asciiOpacity,
  className,
}: AmbientBackgroundProps) {
  const offset = useParallax(0.15)

  return (
    // z-0 (not a negative z-index): a negative-z child paints *behind* an
    // ancestor's background and vanishes. The host section uses `isolate`
    // and puts its content at z-10, so this layer sits cleanly between.
    <div
      aria-hidden
      className={clsx('pointer-events-none absolute inset-0 z-0 overflow-hidden', className)}
    >
      {grid && <div className="absolute inset-0 bg-grid-pattern" />}
      {ascii && (
        <div
          className="ascii-mask absolute -inset-y-40 inset-x-0"
          style={parallax ? { transform: `translate3d(0, ${offset}px, 0)` } : undefined}
        >
          <AsciiField opacity={asciiOpacity} />
        </div>
      )}
      {spotlight && (
        <div className="animate-spotlight absolute -top-32 left-1/2 h-[620px] w-[620px] -translate-x-1/2 rounded-full bg-indigo/10 blur-3xl" />
      )}
    </div>
  )
}
