import clsx from 'clsx'

interface AmbientBackgroundProps {
  /** Show the masked grid pattern layer. */
  grid?: boolean
  /** Drifting indigo spotlight blob. */
  spotlight?: boolean
  className?: string
}

/**
 * Decorative-but-purposeful backdrop: a faint masked grid to signal
 * "engineering surface" plus a slow indigo spotlight for depth/focus.
 * Purely presentational — always aria-hidden, sits behind content, and
 * (via globals.css) freezes under prefers-reduced-motion.
 */
export function AmbientBackground({
  grid = true,
  spotlight = true,
  className,
}: AmbientBackgroundProps) {
  return (
    <div
      aria-hidden
      className={clsx('pointer-events-none absolute inset-0 -z-10 overflow-hidden', className)}
    >
      {grid && <div className="absolute inset-0 bg-grid-pattern" />}
      {spotlight && (
        <div className="animate-spotlight absolute -top-32 left-1/2 h-[620px] w-[620px] -translate-x-1/2 rounded-full bg-indigo/10 blur-3xl" />
      )}
    </div>
  )
}
