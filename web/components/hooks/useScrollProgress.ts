'use client'

import { useEffect, useState } from 'react'

/**
 * Document scroll progress, 0 → 1.
 *
 * Reads on rAF rather than on every scroll event, so we never do layout
 * work more than once a frame (DESIGN.md §8 / main-thread budget).
 */
export function useScrollProgress() {
  const [progress, setProgress] = useState(0)

  useEffect(() => {
    let frame = 0

    const read = () => {
      frame = 0
      const scrollable = document.documentElement.scrollHeight - window.innerHeight
      setProgress(scrollable > 0 ? Math.min(1, window.scrollY / scrollable) : 0)
    }

    const onScroll = () => {
      if (!frame) frame = requestAnimationFrame(read)
    }

    read()
    window.addEventListener('scroll', onScroll, { passive: true })
    window.addEventListener('resize', onScroll, { passive: true })
    return () => {
      if (frame) cancelAnimationFrame(frame)
      window.removeEventListener('scroll', onScroll)
      window.removeEventListener('resize', onScroll)
    }
  }, [])

  return progress
}

/**
 * Vertical scroll offset, throttled to one read per frame. Used for
 * parallax on decorative layers.
 *
 * Returns 0 when the user prefers reduced motion, so callers get a static
 * layer for free without branching.
 */
export function useParallax(strength = 0.15) {
  const [offset, setOffset] = useState(0)

  useEffect(() => {
    if (window.matchMedia('(prefers-reduced-motion: reduce)').matches) return

    let frame = 0
    const read = () => {
      frame = 0
      setOffset(window.scrollY * strength)
    }
    const onScroll = () => {
      if (!frame) frame = requestAnimationFrame(read)
    }

    read()
    window.addEventListener('scroll', onScroll, { passive: true })
    return () => {
      if (frame) cancelAnimationFrame(frame)
      window.removeEventListener('scroll', onScroll)
    }
  }, [strength])

  return offset
}
