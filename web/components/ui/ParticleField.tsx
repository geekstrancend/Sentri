'use client'

import { useEffect, useRef } from 'react'

/**
 * Particle wordmark that assembles out of scattered dust and dissolves back
 * into it on scroll.
 *
 * How it works: the word is rasterised once to an offscreen canvas, then every
 * 4th pixel with meaningful alpha becomes a particle target. Each particle also
 * gets a random origin somewhere in the frame plus its own drift and twinkle
 * phase, so the cloud never looks like a grid animating in lockstep.
 *
 * The single `progress` value (0 = dispersed, 1 = formed) drives everything:
 * position lerps origin→target, drift amplitude scales by (1 - progress), and
 * the twinkle fades toward opaque. So the word settles as it forms rather than
 * arriving and then still jittering.
 */

type Particle = {
  /** scattered origin */
  hx: number
  hy: number
  /** glyph pixel target */
  tx: number
  ty: number
  amp: number
  driftPhase: number
  driftSpeed: number
  twPhase: number
  twSpeed: number
}

interface ParticleFieldProps {
  /** Word to render as particles. */
  text?: string
  /** Accent colour for the highlighted 1-in-7 particles. */
  accent?: string
  /** How long the assemble tween runs, ms. */
  introMs?: number
  /** Base particle colour. */
  base?: string
  className?: string
}

const rand = (min: number, max: number) => min + Math.random() * (max - min)
const clamp = (v: number, min: number, max: number) => Math.max(min, Math.min(max, v))
/** power3.out — fast start, long settle. */
const easeOutCubic = (x: number) => 1 - Math.pow(1 - x, 3)

export function ParticleField({
  text = 'SENTRI',
  accent = '#818CF8',
  base = '#8a8a99',
  introMs = 2200,
  className = '',
}: ParticleFieldProps) {
  const hostRef = useRef<HTMLDivElement | null>(null)
  const canvasRef = useRef<HTMLCanvasElement | null>(null)

  useEffect(() => {
    const host = hostRef.current
    const canvas = canvasRef.current
    const ctx = canvas?.getContext('2d')
    if (!host || !canvas || !ctx) return

    const reduced = window.matchMedia('(prefers-reduced-motion: reduce)').matches
    const dpr = Math.min(window.devicePixelRatio || 1, 2)

    let width = 0
    let height = 0
    let baseParticles: Particle[] = []
    let accentParticles: Particle[] = []
    let raf = 0
    let visible = true
    let disposed = false
    let introStart = 0

    /** Rasterise the word and sample it into particles. */
    const build = () => {
      const rect = host.getBoundingClientRect()
      width = rect.width
      height = rect.height
      if (width <= 0 || height <= 0) return

      canvas.width = Math.floor(width * dpr)
      canvas.height = Math.floor(height * dpr)
      canvas.style.width = `${width}px`
      canvas.style.height = `${height}px`
      ctx.setTransform(dpr, 0, 0, dpr, 0, 0)

      // Offscreen raster at CSS resolution — we only need alpha coverage.
      const off = document.createElement('canvas')
      off.width = Math.floor(width)
      off.height = Math.floor(height)
      const octx = off.getContext('2d', { willReadFrequently: true })
      if (!octx) return

      const display =
        getComputedStyle(document.documentElement)
          .getPropertyValue('--font-fraunces')
          .trim() || 'serif'
      // Fit the word to the frame. Font size alone doesn't determine rendered
      // width, so measure at a trial size and scale down to fit — otherwise a
      // long word overflows on narrow viewports.
      let size = Math.min(height * 0.5, 300)
      octx.font = `700 ${size}px ${display}`
      const maxWidth = width * 0.86
      const measured = octx.measureText(text).width
      if (measured > maxWidth && measured > 0) {
        size = Math.max(size * (maxWidth / measured), 12)
      }
      octx.fillStyle = '#fff'
      octx.font = `700 ${size}px ${display}`
      octx.textAlign = 'center'
      octx.textBaseline = 'middle'
      octx.fillText(text, width / 2, height * 0.5)

      const data = octx.getImageData(0, 0, off.width, off.height).data
      const hits: Array<[number, number]> = []
      for (let y = 0; y < off.height; y += 4) {
        for (let x = 0; x < off.width; x += 4) {
          if (data[(y * off.width + x) * 4 + 3] > 128) hits.push([x, y])
        }
      }

      // Cap the count so the cost is bounded regardless of viewport size.
      const cap = window.innerWidth < 768 ? 1300 : 3600
      const stride = hits.length > cap ? hits.length / cap : 1
      const count = Math.min(hits.length, cap)

      baseParticles = []
      accentParticles = []
      for (let i = 0; i < count; i++) {
        const [tx, ty] = hits[Math.floor(i * stride)]
        const p: Particle = {
          hx: Math.random() * width,
          hy: Math.random() * height,
          tx,
          ty,
          amp: rand(5, 16),
          driftPhase: rand(0, Math.PI * 2),
          driftSpeed: rand(0.25, 0.8),
          twPhase: rand(0, Math.PI * 2),
          twSpeed: rand(1, 3),
        }
        // Every 7th particle carries the accent, so the colour reads as
        // scattered sparks rather than a second solid word.
        if (i % 7 === 0) accentParticles.push(p)
        else baseParticles.push(p)
      }
    }

    const paint = (
      list: Particle[],
      colour: string,
      t: number,
      progress: number,
      dotSize: number,
    ) => {
      ctx.fillStyle = colour
      const inv = 1 - progress
      for (let i = 0; i < list.length; i++) {
        const p = list[i]
        // Drift shrinks to nothing as the word forms.
        const dx = p.hx + p.amp * Math.sin(t * p.driftSpeed + p.driftPhase) * inv
        const dy = p.hy + p.amp * Math.cos(t * p.driftSpeed * 0.85 + p.driftPhase) * inv
        const x = dx + (p.tx - dx) * progress
        const y = dy + (p.ty - dy) * progress
        const tw = 0.4 + 0.6 * (0.5 + 0.5 * Math.sin(t * p.twSpeed + p.twPhase))
        ctx.globalAlpha = tw + (1 - tw) * progress
        ctx.fillRect(x, y, dotSize, dotSize)
      }
    }

    const render = (now: number, progress: number) => {
      const dotSize = window.innerWidth < 768 ? 1.7 : 2
      const t = now * 0.001
      ctx.clearRect(0, 0, width, height)
      paint(baseParticles, base, t, progress, dotSize)
      paint(accentParticles, accent, t, progress, dotSize)
      ctx.globalAlpha = 1
    }

    /** Assemble on entry, then dissolve as the hero scrolls away. */
    const progressAt = (now: number) => {
      const intro = easeOutCubic(clamp((now - introStart) / introMs, 0, 1))
      const rect = host.getBoundingClientRect()
      const travel = Math.max(rect.height, 1)
      const scrolled = clamp(-rect.top / travel, 0, 1)
      return intro * (1 - scrolled)
    }

    build()

    if (reduced) {
      // No motion: show the formed wordmark, once.
      render(0, 1)
    } else {
      introStart = performance.now()
      const loop = (now: number) => {
        if (visible) render(now, progressAt(now))
        raf = requestAnimationFrame(loop)
      }
      raf = requestAnimationFrame(loop)
    }

    // Re-fit on resize; fonts loading late would otherwise leave a stale raster.
    let resizeTimer = 0
    const onResize = () => {
      window.clearTimeout(resizeTimer)
      resizeTimer = window.setTimeout(() => {
        if (disposed) return
        build()
        if (reduced) render(0, 1)
      }, 150)
    }
    window.addEventListener('resize', onResize)

    document.fonts?.ready.then(() => {
      if (disposed) return
      build()
      if (reduced) render(0, 1)
    })

    // Don't burn frames when the hero is off screen.
    const io = new IntersectionObserver(([entry]) => {
      visible = entry?.isIntersecting ?? true
    })
    io.observe(host)

    return () => {
      disposed = true
      cancelAnimationFrame(raf)
      window.clearTimeout(resizeTimer)
      window.removeEventListener('resize', onResize)
      io.disconnect()
    }
  }, [text, accent, base, introMs])

  return (
    <div ref={hostRef} className={`pointer-events-none ${className}`} aria-hidden="true">
      <canvas ref={canvasRef} className="absolute inset-0 h-full w-full" />
    </div>
  )
}
