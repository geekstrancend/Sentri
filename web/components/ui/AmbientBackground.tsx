import clsx from 'clsx'

interface AmbientBackgroundProps {
  /** Tiled ASCII glyph field — the signature texture (DESIGN.md §3). */
  ascii?: boolean
  /** Slow vertical drift on the ASCII field, so it reads as a live scan. */
  drift?: boolean
  /** Faint engineering grid. Off by default; ASCII carries the texture. */
  grid?: boolean
  /** Drifting indigo spotlight blob for depth/focus. */
  spotlight?: boolean
  className?: string
}

/**
 * Decorative-but-purposeful backdrop. Purely presentational — always
 * aria-hidden, sits behind content, and (via globals.css) freezes under
 * prefers-reduced-motion.
 */
export function AmbientBackground({
  ascii = true,
  drift = true,
  grid = false,
  spotlight = true,
  className,
}: AmbientBackgroundProps) {
  return (
    <div
      aria-hidden
      className={clsx('pointer-events-none absolute inset-0 -z-10 overflow-hidden', className)}
    >
      {grid && <div className="absolute inset-0 bg-grid-pattern" />}
      {ascii && (
        <div className={clsx('absolute inset-0 bg-ascii', drift && 'bg-ascii-drift')} />
      )}
      {spotlight && (
        <div className="animate-spotlight absolute -top-32 left-1/2 h-[620px] w-[620px] -translate-x-1/2 rounded-full bg-indigo/10 blur-3xl" />
      )}
    </div>
  )
}
