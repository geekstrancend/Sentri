import clsx from 'clsx'

/**
 * The signature surface: a dense field of monospace glyphs rendered as real
 * text (not a background image), so it always paints and stays razor-crisp.
 * Echoes the terminal output the product actually emits.
 *
 * Generated deterministically at module load with a seeded PRNG, so the
 * server and client render byte-identical output — no hydration mismatch,
 * and no per-request work.
 */

// mulberry32 — tiny deterministic PRNG.
function makeRng(seed: number) {
  let a = seed >>> 0
  return () => {
    a |= 0
    a = (a + 0x6d2b79f5) | 0
    let t = Math.imul(a ^ (a >>> 15), 1 | a)
    t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t
    return ((t ^ (t >>> 14)) >>> 0) / 4294967296
  }
}

// Weighted charset: mostly sparse marks so it reads as texture, with the
// occasional hex/opcode fragment to feel like real disassembly.
const GLYPHS = '..::  ++  --  ==  01  0x  ff  1a  de  ad  //  \\\\  ||  <>  {}  [] '
const TOKENS = GLYPHS.trim().split(/\s+/)

function buildField(rows: number, cols: number, seed: number): string {
  const rng = makeRng(seed)
  const lines: string[] = []
  for (let r = 0; r < rows; r++) {
    let line = ''
    while (line.length < cols) {
      // Some whitespace so the field breathes, but enough ink that the
      // texture actually reads against near-black.
      line += rng() < 0.4 ? '  ' : TOKENS[(rng() * TOKENS.length) | 0] + ' '
    }
    lines.push(line.slice(0, cols))
  }
  return lines.join('\n')
}

const FIELD = buildField(90, 260, 0x5e17a1)

interface AsciiFieldProps {
  className?: string
  /** 0–1. Kept low: texture, not content. */
  opacity?: number
}

export function AsciiField({ className, opacity = 0.2 }: AsciiFieldProps) {
  return (
    <pre
      aria-hidden
      style={{ opacity }}
      className={clsx(
        'pointer-events-none absolute inset-0 m-0 select-none overflow-hidden',
        'whitespace-pre font-mono text-[12px] leading-[16px] text-indigo-bright',
        className,
      )}
    >
      {FIELD}
    </pre>
  )
}
