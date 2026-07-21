'use client'
import { useEffect, useRef, useState } from 'react'

interface AnimatedCounterProps {
  value: number
  suffix?: string
  duration?: number
  /** Fractional digits to hold, for values like 1.76. */
  decimals?: number
  className?: string
}

export function AnimatedCounter({
  value,
  suffix = '',
  duration = 1200,
  decimals = 0,
  className,
}: AnimatedCounterProps) {
  const [count, setCount] = useState(0)
  const ref = useRef<HTMLSpanElement>(null)
  const started = useRef(false)

  useEffect(() => {
    const el = ref.current
    if (!el) return

    // Counting up is decorative; honour a reduced-motion preference by simply
    // presenting the figure.
    if (window.matchMedia('(prefers-reduced-motion: reduce)').matches) {
      setCount(value)
      return
    }

    const run = () => {
      if (started.current) return
      started.current = true
      const start = performance.now()
      const tick = (now: number) => {
        const progress = Math.min((now - start) / duration, 1)
        const eased = 1 - Math.pow(1 - progress, 3)
        // Rounding to the target precision, not flooring to an integer —
        // flooring would pin a value like 1.76 at "1" for the whole tween.
        const factor = 10 ** decimals
        setCount(Math.round(eased * value * factor) / factor)
        if (progress < 1) requestAnimationFrame(tick)
        else setCount(value)
      }
      requestAnimationFrame(tick)
    }

    const observer = new IntersectionObserver(
      ([entry]) => {
        if (entry.isIntersecting) {
          run()
          observer.unobserve(el)
        }
      },
      { threshold: 0.3 },
    )
    observer.observe(el)

    // Safety net: if this mounts already in view — a restored scroll position,
    // a deep link, hydrating after the browser has scrolled — the observer can
    // miss its first callback, and a stat frozen at 0 is far worse than one
    // that skipped its count-up.
    const settle = window.setTimeout(() => {
      const r = el.getBoundingClientRect()
      if (r.top < window.innerHeight && r.bottom > 0) run()
    }, 600)

    return () => {
      observer.disconnect()
      window.clearTimeout(settle)
    }
  }, [value, duration, decimals])

  return (
    <span ref={ref} className={className}>
      {count.toLocaleString(undefined, {
        minimumFractionDigits: decimals,
        maximumFractionDigits: decimals,
      })}
      {suffix}
    </span>
  )
}
