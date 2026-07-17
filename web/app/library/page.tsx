'use client'

import { useState, useMemo } from 'react'
import { Search, Plus, BookOpen, X, ExternalLink } from 'lucide-react'
import { MarketingNav } from '@/components/layout/MarketingNav'
import { MarketingFooter } from '@/components/layout/MarketingFooter'
import { Button } from '@/components/ui/Button'
import { SeverityBadge } from '@/components/ui/SeverityBadge'
import { AuthModal } from '@/components/ui/AuthModal'
import { Container } from '@/components/ui/Section'
import { Badge } from '@/components/ui/Badge'
import { AmbientBackground } from '@/components/ui/AmbientBackground'
import clsx from 'clsx'

type Severity = 'critical' | 'high' | 'medium' | 'low'

interface Invariant {
  id: string
  severity: Severity
  title: string
  description: string
  tags: string[]
  cvss: number
  version: string
  audits: number
  chain: 'EVM' | 'Solana' | 'Move' | 'Generic'
  category: string
}

const INVARIANTS: Invariant[] = [
  { id: 'EVM-C01', severity: 'critical', title: 'evm_reentrancy_protection', description: 'Verifies all external calls following state changes are protected by a nonReentrant modifier or the CEI pattern is strictly followed throughout the function.', tags: ['CORE', 'CEI-PATTERN'], cvss: 9.1, version: 'v4.0.2', audits: 1200, chain: 'EVM', category: 'Core Safety' },
  { id: 'EVM-C02', severity: 'critical', title: 'evm_self_destruct_removal', description: 'Detects legacy SELFDESTRUCT opcodes which are deprecated post-Cancun and can lead to unexpected state destruction in proxy contracts.', tags: ['POST-CANCUN', 'GOVERNANCE'], cvss: 9.8, version: 'v1.1.0', audits: 12, chain: 'EVM', category: 'Core Safety' },
  { id: 'EVM-C03', severity: 'critical', title: 'evm_missing_post_state_health_check', description: 'Checks that after any flash-loan or large-value swap, a protocol health assertion (e.g. total assets >= total liabilities) is evaluated before the transaction finalises.', tags: ['FLASH-LOAN', 'DEFI'], cvss: 9.6, version: 'v1.0.0', audits: 8, chain: 'EVM', category: 'DeFi' },
  { id: 'EVM-C04', severity: 'critical', title: 'evm_merkle_root_zero_default', description: 'Ensures no Merkle root is initialised as bytes32(0), which makes all proofs trivially valid — the exact vector used in the $190M Nomad exploit.', tags: ['BRIDGES', 'MERKLE'], cvss: 9.9, version: 'v1.0.1', audits: 5, chain: 'EVM', category: 'Bridges' },
  { id: 'EVM-H01', severity: 'high', title: 'evm_oracle_heartbeat_freshness', description: 'Validates Oracle responses (Chainlink/Pyth) include an updatedAt timestamp within an acceptable heartbeat window before using the price.', tags: ['DEFI', 'ORACLES'], cvss: 7.5, version: 'v3.2.0', audits: 89, chain: 'EVM', category: 'Oracles' },
  { id: 'EVM-H02', severity: 'high', title: 'evm_cross_chain_arbitrary_message_validation', description: 'Ensures all incoming messages from LayerZero or Axelar endpoints are validated against a known source chain and sender address whitelist.', tags: ['BRIDGES', 'INTEROPERABILITY'], cvss: 8.8, version: 'v2.1.0', audits: 42, chain: 'EVM', category: 'Bridges' },
  { id: 'EVM-H03', severity: 'high', title: 'evm_unbacked_synthetic_mint', description: 'Verifies that every minting of a synthetic or wrapped token is backed 1:1 by a corresponding deposit or collateral lock before the mint executes.', tags: ['DEFI', 'TOKENS'], cvss: 8.2, version: 'v1.0.0', audits: 17, chain: 'EVM', category: 'DeFi' },
  { id: 'EVM-H04', severity: 'high', title: 'evm_dvn_single_point_failure', description: 'Ensures LayerZero DVN configurations require signatures from ≥2 distinct verification networks, preventing a single compromised DVN from authorising large transfers.', tags: ['BRIDGES', 'DVN'], cvss: 8.5, version: 'v1.0.0', audits: 3, chain: 'EVM', category: 'Bridges' },
  { id: 'EVM-M01', severity: 'medium', title: 'evm_integer_precision_loss', description: 'Detects division before multiplication patterns that can silently truncate values to zero in fixed-point arithmetic, particularly in share price calculations.', tags: ['MATH', 'PRECISION'], cvss: 5.9, version: 'v2.0.0', audits: 340, chain: 'EVM', category: 'Core Safety' },
  { id: 'EVM-M02', severity: 'medium', title: 'evm_governance_timelock_minimum', description: 'Asserts that all governance function calls are guarded by a timelock of at least 48 hours, giving stakeholders time to react to malicious proposals.', tags: ['GOVERNANCE', 'TIMELOCK'], cvss: 6.1, version: 'v1.3.0', audits: 78, chain: 'EVM', category: 'Governance' },
  { id: 'SOL-H01', severity: 'high', title: 'solana_account_signer_verification', description: 'Mandates that all accounts modifying program-owned state are verified as transaction signers before any state mutation occurs.', tags: ['SOLANA', 'SIGNER'], cvss: 8.0, version: 'v1.2.0', audits: 65, chain: 'Solana', category: 'Core Safety' },
  { id: 'SOL-M01', severity: 'medium', title: 'solana_account_owner_validation', description: 'Mandates that all AccountInfo data is checked for ownership by the calling program before state transitions are applied.', tags: ['SOLANA', 'OWNERSHIP'], cvss: 5.4, version: 'v1.0.4', audits: 186, chain: 'Solana', category: 'Core Safety' },
  { id: 'GEN-L01', severity: 'low', title: 'generic_event_emission_check', description: 'Standardises that every state-changing function emits a corresponding event for off-chain indexing and auditability.', tags: ['LOGGING', 'STANDARDS'], cvss: 2.1, version: 'v0.9.1', audits: 3450, chain: 'Generic', category: 'Standards' },
  { id: 'GEN-L02', severity: 'low', title: 'generic_access_control_two_step', description: 'Verifies that ownership transfers use a two-step accept pattern (propose + accept) rather than a single direct transfer.', tags: ['ACCESS-CONTROL', 'STANDARDS'], cvss: 3.2, version: 'v1.0.0', audits: 890, chain: 'Generic', category: 'Governance' },
]

const CATEGORIES = ['All', 'Core Safety', 'DeFi', 'Bridges', 'Oracles', 'Governance', 'Standards']
const CHAINS = ['All', 'EVM', 'Solana', 'Move', 'Generic']
const SEVERITIES = ['All', 'critical', 'high', 'medium', 'low'] as const

const SEVERITY_CHIP: Record<string, string> = {
  All: 'bg-indigo/15 border-indigo/30 text-indigo-bright',
  critical: 'bg-critical-bg border-critical-border text-critical',
  high: 'bg-high-bg border-high-border text-high',
  medium: 'bg-medium-bg border-medium-border text-medium',
  low: 'bg-low-bg border-low-border text-low',
}

function cvssColor(cvss: number) {
  if (cvss >= 9) return 'text-critical'
  if (cvss >= 7) return 'text-high'
  if (cvss >= 4) return 'text-medium'
  return 'text-low'
}

export default function LibraryPage() {
  const [search, setSearch] = useState('')
  const [chain, setChain] = useState('All')
  const [category, setCategory] = useState('All')
  const [severity, setSeverity] = useState<string>('All')
  const [authOpen, setAuthOpen] = useState(false)

  const filtered = useMemo(() => {
    return INVARIANTS.filter((inv) => {
      const q = search.trim().toLowerCase()
      const matchSearch =
        !q ||
        inv.title.toLowerCase().includes(q) ||
        inv.description.toLowerCase().includes(q) ||
        inv.tags.some((t) => t.toLowerCase().includes(q)) ||
        inv.id.toLowerCase().includes(q)
      const matchChain = chain === 'All' || inv.chain === chain
      const matchCat = category === 'All' || inv.category === category
      const matchSev = severity === 'All' || inv.severity === severity
      return matchSearch && matchChain && matchCat && matchSev
    })
  }, [search, chain, category, severity])

  const hasFilters = Boolean(search) || chain !== 'All' || category !== 'All' || severity !== 'All'

  const clearAll = () => {
    setSearch('')
    setChain('All')
    setCategory('All')
    setSeverity('All')
  }

  return (
    <div className="flex min-h-dvh flex-col bg-surface">
      <MarketingNav />

      <main id="main" className="flex-1">
        {/* ── Hero ── */}
        <section className="relative isolate overflow-hidden border-b border-outline-variant bg-surface-container-lowest py-14">
          <AmbientBackground spotlight={false} />
          <Container className="relative z-10">
            <div className="flex flex-col justify-between gap-8 lg:flex-row lg:items-end">
              <div>
                <Badge tone="indigo" icon={<BookOpen size={13} />}>
                  Detector library
                </Badge>
                <h1 className="mt-5 text-display-md text-on-surface text-balance">
                  Every check, mapped to a real exploit
                </h1>
                <p className="mt-4 max-w-2xl text-body-lg text-on-surface-variant">
                  Battle-tested detectors for EVM, Solana, Move, and Soroban. Each one exists
                  because a protocol lost money without it.
                </p>
              </div>
              <div className="flex flex-shrink-0 gap-3">
                <Button
                  variant="secondary"
                  size="sm"
                  icon={<ExternalLink size={14} />}
                  onClick={() => setAuthOpen(true)}
                >
                  Request a detector
                </Button>
                <Button
                  variant="primary"
                  size="sm"
                  icon={<Plus size={14} />}
                  onClick={() => setAuthOpen(true)}
                >
                  Custom library
                </Button>
              </div>
            </div>
          </Container>
        </section>

        <Container className="py-8">
          {/* ── Filters ── */}
          <div className="mb-6 flex flex-col gap-4 lg:flex-row lg:items-center">
            {/* Search */}
            <div className="relative max-w-md flex-1">
              <label htmlFor="library-search" className="sr-only">
                Search detectors
              </label>
              <Search
                size={15}
                aria-hidden
                className="pointer-events-none absolute left-3 top-1/2 -translate-y-1/2 text-outline"
              />
              <input
                id="library-search"
                type="search"
                value={search}
                onChange={(e) => setSearch(e.target.value)}
                placeholder="Search by name, tag, or ID…"
                className="w-full rounded-lg border border-outline-variant bg-surface-container-lowest py-2.5 pl-9 pr-9 text-body-md text-on-surface placeholder-outline transition-colors focus:border-indigo focus:outline-none"
              />
              {search && (
                <button
                  onClick={() => setSearch('')}
                  aria-label="Clear search"
                  className="absolute right-1 top-1/2 -translate-y-1/2 p-2 text-outline transition-colors hover:text-on-surface"
                >
                  <X size={14} />
                </button>
              )}
            </div>

            <div className="flex flex-wrap items-center gap-3">
              {/* Severity */}
              <div className="flex gap-1" role="group" aria-label="Filter by severity">
                {SEVERITIES.map((s) => (
                  <button
                    key={s}
                    onClick={() => setSeverity(s)}
                    aria-pressed={severity === s}
                    className={clsx(
                      'rounded-full border px-3 py-1.5 font-mono text-[0.65rem] font-[600] uppercase tracking-wide transition-colors',
                      severity === s
                        ? SEVERITY_CHIP[s]
                        : 'border-outline-variant bg-transparent text-outline hover:border-outline hover:text-on-surface',
                    )}
                  >
                    {s}
                  </button>
                ))}
              </div>

              {/* Chain */}
              <div>
                <label htmlFor="chain-filter" className="sr-only">
                  Filter by chain
                </label>
                <select
                  id="chain-filter"
                  value={chain}
                  onChange={(e) => setChain(e.target.value)}
                  className="rounded-lg border border-outline-variant bg-surface-container-lowest px-3 py-2 text-body-md text-on-surface transition-colors focus:border-indigo focus:outline-none"
                >
                  {CHAINS.map((c) => (
                    <option key={c} value={c}>
                      {c === 'All' ? 'All chains' : c}
                    </option>
                  ))}
                </select>
              </div>

              {hasFilters && (
                <button
                  onClick={clearAll}
                  className="flex items-center gap-1 px-2 py-1.5 text-xs text-outline transition-colors hover:text-on-surface"
                >
                  <X size={12} /> Clear all
                </button>
              )}
            </div>
          </div>

          {/* Category tabs */}
          <div
            className="mb-8 flex gap-1 overflow-x-auto pb-1"
            role="group"
            aria-label="Filter by category"
          >
            {CATEGORIES.map((cat) => (
              <button
                key={cat}
                onClick={() => setCategory(cat)}
                aria-pressed={category === cat}
                className={clsx(
                  'flex-shrink-0 whitespace-nowrap rounded-lg px-4 py-2 text-body-md font-[500] transition-colors',
                  category === cat
                    ? 'bg-surface-container-high text-on-surface'
                    : 'text-outline hover:bg-surface-container/50 hover:text-on-surface',
                )}
              >
                {cat}
              </button>
            ))}
          </div>

          {/* Results count — announced to screen readers as filters change */}
          <p className="mb-6 text-body-md text-outline" role="status" aria-live="polite">
            <span className="font-mono font-[600] text-on-surface">{filtered.length}</span>{' '}
            {filtered.length === 1 ? 'detector' : 'detectors'}
            {hasFilters && ' matching your filters'}
          </p>

          {/* ── Results ── */}
          {filtered.length === 0 ? (
            <div className="rounded-2xl border border-dashed border-outline-variant py-20 text-center">
              <BookOpen size={36} className="mx-auto mb-4 text-outline-variant" aria-hidden />
              <p className="mb-1 text-lg font-[600] text-on-surface">No detectors match</p>
              <p className="mb-6 text-body-md text-outline">
                Try a different search term or clear your filters.
              </p>
              <Button variant="secondary" size="sm" onClick={clearAll}>
                Clear all filters
              </Button>
            </div>
          ) : (
            <ul className="grid grid-cols-1 gap-5 md:grid-cols-2 xl:grid-cols-3">
              {filtered.map((inv) => (
                <li
                  key={inv.id}
                  className="group flex flex-col gap-4 rounded-xl border border-outline-variant bg-surface-container-low/70 p-6 lift-on-hover"
                >
                  <div className="flex items-start justify-between gap-2">
                    <div className="flex flex-wrap items-center gap-2">
                      <span className="rounded border border-outline-variant bg-surface-container px-2 py-0.5 font-mono text-[0.7rem] text-outline">
                        {inv.id}
                      </span>
                      <span className="rounded border border-outline-variant bg-surface-container px-2 py-0.5 font-mono text-[0.7rem] text-outline">
                        {inv.chain}
                      </span>
                    </div>
                    <SeverityBadge level={inv.severity} />
                  </div>

                  <h3 className="font-mono text-sm font-[600] text-on-surface transition-colors group-hover:text-indigo-bright">
                    {inv.title}
                  </h3>

                  <p className="line-clamp-3 flex-1 text-body-md leading-5 text-on-surface-variant">
                    {inv.description}
                  </p>

                  <div className="flex flex-wrap gap-1.5">
                    {inv.tags.map((tag) => (
                      <button
                        key={tag}
                        onClick={() => setSearch(tag.toLowerCase())}
                        className="rounded border border-outline-variant bg-surface-container px-2 py-0.5 font-mono text-[0.65rem] text-outline transition-colors hover:border-indigo/50 hover:text-on-surface"
                      >
                        {tag}
                      </button>
                    ))}
                  </div>

                  <div className="flex items-center justify-between border-t border-outline-variant pt-3 font-mono text-xs">
                    <div className="flex items-center gap-4">
                      <span>
                        <span className="text-outline-variant">CVSS </span>
                        <span className={clsx('font-[700] tabular-nums', cvssColor(inv.cvss))}>
                          {inv.cvss.toFixed(1)}
                        </span>
                      </span>
                      <span className="text-outline">{inv.version}</span>
                    </div>
                    <span className="tabular-nums text-outline">
                      {inv.audits >= 1000 ? `${(inv.audits / 1000).toFixed(1)}k` : inv.audits} audits
                    </span>
                  </div>
                </li>
              ))}
            </ul>
          )}
        </Container>
      </main>

      <AuthModal isOpen={authOpen} onClose={() => setAuthOpen(false)} />
      <MarketingFooter />
    </div>
  )
}
