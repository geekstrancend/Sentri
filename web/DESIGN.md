# Sentri — Web Platform Design System

> The design brief for the Sentri marketing site, docs, and dashboard.
> Read this before building or changing any page. Every surface should be
> traceable back to a principle here.

## 1. Goal

Sentri is a smart-contract security engine (multi-chain static analysis +
dynamic/invariant fuzzing). The web platform has one job: **make a technical,
skeptical audience — protocol engineers and security researchers — trust
Sentri in under 30 seconds, then get out of their way.**

Success looks like:

- A developer lands, immediately understands _what it does_ and _that it is
  real_ (not vaporware), and reaches a terminal command or a sample report
  without friction.
- The docs read like great engineering docs: scannable, precise, copy-paste
  ready, never marketing fluff.
- The whole thing feels **fast, precise, and quietly premium** — the way the
  product itself aims to be.

## 2. Audience & tone

- **Who:** Solidity/Rust engineers, protocol founders, audit firms, bug-bounty
  hunters. They distrust hype and reward substance.
- **Voice:** Direct, technical, confident, zero fluff. Show real output (CLI
  traces, findings, detector counts) instead of adjectives. Numbers over
  claims.
- **Feeling:** A precision instrument. Think a well-built terminal, an oscilloscope,
  a trading terminal — not a generic SaaS gradient soup.

## 3. Design language — "Modern Dark, Technical"

Dark-primary, cinematic, atmospheric — validated against the product type
(developer/security tooling). Deep near-black surfaces (never pure `#000` —
avoids OLED smear and lets elevation read), a disciplined indigo accent for
brand/interaction, and a green "signal/pass" accent reserved for
success/run/verified states. Restrained glass and glow used to convey depth
and focus, never as decoration.

### Core principles

1. **Substance is the aesthetic.** Real terminal output, real finding cards,
   real numbers are the hero visuals. Design frames them; it doesn't replace
   them.
2. **Two voices: sans + mono.** Geist carries UI and prose (calm, legible,
   engineered). Geist Mono is the _technical voice_ — labels, code, data,
   badges, terminal, chain names. The mono/sans duality **is** the brand
   personality; we don't need a decorative serif to be distinctive.
3. **The surface is a terminal.** The signature texture is a tiled field of
   monospace glyphs (`.bg-ascii`) drifting slowly behind the content — the
   product's own output, used as wallpaper. It sits far below the contrast
   floor: texture, never noise.
4. **Motion means something.** Every animation expresses cause→effect or
   spatial continuity (entrance from below = deeper, staggered reveals track
   reading order). Ambient motion stays slow and near-subliminal. No motion
   that's purely ornamental. Always honors `prefers-reduced-motion`.
5. **Elevation is a system, not vibes.** A fixed surface + shadow + glow scale.
   Cards, popovers, and modals each sit at a defined tier; shadows are never
   ad-hoc.
6. **Accessible by construction.** ≥4.5:1 text contrast on every surface,
   visible focus rings, semantic tokens (never raw hex in components),
   keyboard-complete, color never the sole signal (severity always pairs a
   label).

## 4. Tokens (source of truth: `styles/globals.css`)

- **Surfaces:** `--surface` (#131314 base) → `surface-container-{lowest…highest}`
  form the elevation ramp. Background ambient glow is indigo at very low alpha.
- **Text:** `on-surface` (primary, ~AA+), `on-surface-variant` (secondary),
  `outline` (tertiary/muted — still ≥3:1). Never gray-on-gray below spec.
- **Accents:** `indigo` (brand/interaction/links), `signal` green
  (success / pass / "scan complete"). Severity ramp: `critical` `high`
  `medium` `low`, each with `-bg` and `-border` companions.
- **Radius:** 2 / 4 / 6 / 8 / 12 / full. Cards use `lg` (8) or `xl` (12).
- **Spacing:** 4-based scale. Section rhythm tiers: 16 / 24 / 32 / 48 / 80.

## 5. Motion language

- **Easing:** `--ease-out-expo` `cubic-bezier(0.16,1,0.3,1)` for entrances;
  standard ease for hovers. Exits ~65% of entrance duration.
- **Durations:** micro-interactions 150–250ms; entrances 400–600ms; never >600ms.
- **Patterns:** `fade-in-up` for entrances, `stagger` (40ms/item) for lists and
  grids following reading order, scroll-reveal via `useReveal`, subtle
  `hover-lift` (translateY -2px + shadow) on interactive cards, ambient
  `spotlight` drift on hero background. Only transform/opacity animate.
- **Reduced motion:** all of the above collapse to instant/none.

## 6. Information architecture

- **Marketing:** `/` (home) · `/pricing` · `/library` (invariant/detector
  catalog) · `/contact` · legal (`/privacy`, `/terms`).
- **Docs:** `/docs` hub → getting-started · cli · api · ci-cd · ai · reports.
  Persistent left sidebar, right "on this page" TOC, sticky sub-nav.
- **App:** `/dashboard` (+ scan, settings, support), `/reports/[id]`. AppShell
  with left nav; distinct from marketing chrome.

## 7. Per-page intent

| Page | Primary job | Hero visual |
|------|-------------|-------------|
| Home | Trust + "it's real" in 30s → CLI / sample report | Live terminal scan + finding cards |
| Pricing | Remove doubt, compare plans honestly | Plan matrix, FAQ |
| Library | Prove depth (N detectors across chains) | Filterable catalog |
| Docs hub | Route to the right doc fast | Quick-start snippet grid |
| Dashboard | Get to a scan / see results | Scan status + recent runs |

## 8. Quality bar (every page)

- Mobile-first; verified at 375 / 768 / 1024 / 1440.
- No horizontal scroll; wide content (tables/code) scrolls in its own container.
- One primary CTA per view; secondary actions subordinate.
- Loading/empty/error states designed, not afterthoughts.
- `tsc --noEmit` clean; `next build` clean.
