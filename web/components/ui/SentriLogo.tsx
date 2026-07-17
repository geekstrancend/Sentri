import clsx from 'clsx'

interface SentriLogoProps {
  /** Pixel size (width = height). */
  size?: number
  /** Render the blue glow behind the mark. */
  glow?: boolean
  className?: string
  title?: string
}

/**
 * The Sentri mark: an angular, forward-leaning "S" built from three
 * chevron bars, in a blue vertical gradient with an optional glow.
 *
 * Vectorized (not a bitmap) so it stays crisp at every size, carries a
 * transparent background for the dark UI, and can glow via SVG filter.
 */
export function SentriLogo({ size = 28, glow = true, className, title = 'Sentri' }: SentriLogoProps) {
  return (
    <svg
      width={size}
      height={size}
      viewBox="0 0 64 64"
      fill="none"
      role="img"
      aria-label={title}
      className={clsx('shrink-0', className)}
    >
      <defs>
        <linearGradient id="sentri-s" x1="12" y1="58" x2="54" y2="8" gradientUnits="userSpaceOnUse">
          <stop offset="0" stopColor="#173dff" />
          <stop offset="0.5" stopColor="#2f6bff" />
          <stop offset="1" stopColor="#5fa6ff" />
        </linearGradient>
        <filter id="sentri-glow" x="-70%" y="-70%" width="240%" height="240%">
          <feGaussianBlur in="SourceAlpha" stdDeviation="2.6" result="b" />
          <feFlood floodColor="#2f7bff" floodOpacity="0.95" />
          <feComposite in2="b" operator="in" result="g" />
          <feMerge>
            <feMergeNode in="g" />
            <feMergeNode in="SourceGraphic" />
          </feMerge>
        </filter>
      </defs>

      {/* Angular S: top bar → left drop → middle bar → right drop → bottom bar.
          Forward lean + mitred, sheared (parallelogram) ends give the
          chevron character of the mark. */}
      <polyline
        points="54,10 27,17 22,31 47,31 42,47 14,54"
        stroke="url(#sentri-s)"
        strokeWidth="12.5"
        strokeLinejoin="miter"
        strokeMiterlimit="10"
        strokeLinecap="butt"
        filter={glow ? 'url(#sentri-glow)' : undefined}
      />
    </svg>
  )
}
