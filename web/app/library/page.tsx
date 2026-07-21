'use client'

import { useState, useMemo } from 'react'
import { Search, Plus, BookOpen, Filter, X, ExternalLink } from 'lucide-react'
import { MarketingNav } from '@/components/layout/MarketingNav'
import { MarketingFooter } from '@/components/layout/MarketingFooter'
import { Button } from '@/components/ui/Button'
import { SeverityBadge } from '@/components/ui/SeverityBadge'
import { AuthModal } from '@/components/ui/AuthModal'

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
  { id: 'EVM-C04', severity: 'critical', title: 'evm_merkle_root_zero_default', description: 'Ensures no Merkle root is initialised as bytes32(0), which makes all proofs trivially valid—the exact vector used in the $190M Nomad exploit.', tags: ['BRIDGES', 'MERKLE'], cvss: 9.9, version: 'v1.0.1', audits: 5, chain: 'EVM', category: 'Bridges' },
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
const SEVERITIES = ['All', 'critical', 'high', 'medium', 'low']

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
  const [severity, setSeverity] = useState('All')
  const [authOpen, setAuthOpen] = useState(false)

  const filtered = useMemo(() => {
    return INVARIANTS.filter((inv) => {
      const q = search.toLowerCase()
      const matchSearch = !q || inv.title.toLowerCase().includes(q) || inv.description.toLowerCase().includes(q) || inv.tags.some(t => t.toLowerCase().includes(q)) || inv.id.toLowerCase().includes(q)
      const matchChain = chain === 'All' || inv.chain === chain
      const matchCat = category === 'All' || inv.category === category
      const matchSev = severity === 'All' || inv.severity === severity
      return matchSearch && matchChain && matchCat && matchSev
    })
  }, [search, chain, category, severity])

  const hasFilters = search || chain !== 'All' || category !== 'All' || severity !== 'All'

  return (
    <div className="min-h-screen bg-bg flex flex-col">
      <MarketingNav />

      <main className="flex-1">
        {/* Hero */}
        <section className="px-6 py-16 border-b border-hair bg-surface-2">
          <div className="max-w-site mx-auto">
            <div className="flex flex-col lg:flex-row lg:items-end justify-between gap-8">
              <div>
                <div className="inline-flex items-center gap-2 px-4 py-2 rounded-full bg-indigo/8 border border-indigo/20 mb-5">
                  <BookOpen size={14} className="text-acc-text" />
                  <span className="text-label-sm text-acc-text">INVARIANT LIBRARY</span>
                </div>
                <h1 className="font-display text-5xl font-[700] text-text mb-3 leading-[64px]">
                  Security Invariant Library
                </h1>
                <p className="text-body-lg text-sec max-w-2xl">
                  50+ battle-tested security checks for EVM, Solana, and Move. Every invariant is mapped to a real exploit.
                </p>
              </div>
              <div className="flex gap-3 flex-shrink-0">
                <Button variant="secondary" size="sm" icon={<ExternalLink size={14} />} onClick={() => setAuthOpen(true)}>
                  Request Invariant
                </Button>
                <Button variant="primary" size="sm" icon={<Plus size={14} />} onClick={() => setAuthOpen(true)}>
                  Custom Library
                </Button>
              </div>
            </div>
          </div>
        </section>

        <section className="px-6 py-8 max-w-site mx-auto">
          {/* Search + filter bar */}
          <div className="flex flex-col lg:flex-row gap-4 mb-6">
            {/* Search */}
            <div className="relative flex-1 max-w-md">
              <Search size={16} className="absolute left-3 top-1/2 -translate-y-1/2 text-sec" />
              <input
                type="text"
                value={search}
                onChange={(e) => setSearch(e.target.value)}
                placeholder="Search by name, tag, or ID…"
                className="w-full bg-surface-2 border border-hair rounded-lg px-4 pl-9 py-2.5 text-body-md text-text placeholder-outline-variant focus:outline-none focus:border-indigo transition-colors"
              />
              {search && (
                <button
                  onClick={() => setSearch('')}
                  aria-label="Clear search"
                  className="absolute right-1 top-1/2 -translate-y-1/2 p-2 text-sec hover:text-text"
                >
                  <X size={14} />
                </button>
              )}
            </div>

            {/* Filter chips */}
            <div className="flex flex-wrap gap-2 items-center">
              <Filter size={14} className="text-sec flex-shrink-0" />

              {/* Severity */}
              <div className="flex gap-1">
                {SEVERITIES.map((s) => {
                  const severityChipClasses: Record<string, string> = {
                    All: 'bg-indigo/15 border-indigo/30 text-acc-text',
                    critical: 'bg-critical-bg border-critical-border text-critical',
                    high: 'bg-high-bg border-high-border text-high',
                    medium: 'bg-medium-bg border-medium-border text-medium',
                    low: 'bg-low-bg border-low-border text-low',
                  }
                  return (
                    <button
                      key={s}
                      onClick={() => setSeverity(s)}
                      className={`px-3 py-1.5 rounded-full text-xs font-[600] uppercase tracking-wide border transition-colors ${
                        severity === s
                          ? severityChipClasses[s]
                          : 'bg-transparent border-hair text-sec hover:border-hair'
                      }`}
                    >
                      {s === 'All' ? 'All' : s}
                    </button>
                  )
                })}
              </div>

              {/* Chain */}
              <div className="flex gap-1">
                {CHAINS.map((c) => (
                  <button
                    key={c}
                    onClick={() => setChain(c)}
                    className={`px-3 py-1 rounded-full text-xs font-[600] border transition-colors ${
                      chain === c
                        ? 'bg-indigo/15 border-indigo/30 text-acc-text'
                        : 'bg-transparent border-hair text-sec hover:border-hair'
                    }`}
                  >
                    {c}
                  </button>
                ))}
              </div>

              {hasFilters && (
                <button
                  onClick={() => { setSearch(''); setChain('All'); setCategory('All'); setSeverity('All') }}
                  className="text-xs text-sec hover:text-text flex items-center gap-1 transition-colors px-2 py-1.5"
                >
                  <X size={12} /> Clear all
                </button>
              )}
            </div>
          </div>

          {/* Category tabs */}
          <div className="flex gap-1 mb-8 overflow-x-auto pb-1">
            {CATEGORIES.map((cat) => (
              <button
                key={cat}
                onClick={() => setCategory(cat)}
                className={`px-4 py-2 rounded-lg text-body-md font-[500] whitespace-nowrap transition-colors flex-shrink-0 ${
                  category === cat
                    ? 'bg-panel text-text'
                    : 'text-sec hover:text-text hover:bg-panel/50'
                }`}
              >
                {cat}
              </button>
            ))}
          </div>

          {/* Results count */}
          <div className="flex items-center justify-between mb-6">
            <p className="text-body-md text-sec">
              <span className="text-text font-[600]">{filtered.length}</span> invariants
              {hasFilters && ' matching filters'}
            </p>
          </div>

          {/* Cards grid */}
          {filtered.length === 0 ? (
            <div className="text-center py-20 text-sec">
              <BookOpen size={40} className="mx-auto mb-4 opacity-30" />
              <p className="font-display text-xl text-sec mb-2">No invariants found</p>
              <p className="text-body-md">Try adjusting your search or filters</p>
            </div>
          ) : (
            <div className="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-5">
              {filtered.map((inv) => (
                <div key={inv.id} className="bg-panel border border-hair rounded-card p-6 lift-on-hover flex flex-col gap-4 group">
                  {/* Header */}
                  <div className="flex justify-between items-start gap-2">
                    <div className="flex items-center gap-2 flex-wrap">
                      <span className="text-xs bg-panel border border-hair text-sec px-2 py-0.5 rounded font-mono">{inv.id}</span>
                      <span className="text-xs text-sec bg-panel border border-hair px-2 py-0.5 rounded">{inv.chain}</span>
                    </div>
                    <SeverityBadge level={inv.severity} />
                  </div>

                  {/* Title */}
                  <h3 className="font-mono text-sm font-[600] text-text group-hover:text-acc-text transition-colors">{inv.title}</h3>

                  {/* Description */}
                  <p className="text-body-md text-sec leading-5 line-clamp-3 flex-1">{inv.description}</p>

                  {/* Tags */}
                  <div className="flex flex-wrap gap-1.5">
                    {inv.tags.map((tag) => (
                      <button
                        key={tag}
                        onClick={() => setSearch(tag.toLowerCase())}
                        className="text-xs bg-panel border border-hair text-sec px-2 py-0.5 rounded hover:border-indigo/50 hover:text-text transition-colors"
                      >
                        {tag}
                      </button>
                    ))}
                  </div>

                  {/* Footer */}
                  <div className="border-t border-hair pt-3 flex justify-between items-center">
                    <div className="flex items-center gap-4 text-xs">
                      <span>
                        <span className="text-sec">CVSS </span>
                        <span className={`font-[700] ${cvssColor(inv.cvss)}`}>{inv.cvss}</span>
                      </span>
                      <span>
                        <span className="text-sec">v </span>
                        <span className="text-sec">{inv.version}</span>
                      </span>
                    </div>
                    <span className="text-xs text-sec">
                      {inv.audits >= 1000 ? `${(inv.audits / 1000).toFixed(1)}k` : inv.audits} audits
                    </span>
                  </div>
                </div>
              ))}
            </div>
          )}
        </section>
      </main>

      <AuthModal isOpen={authOpen} onClose={() => setAuthOpen(false)} defaultTab="signup" />
      <MarketingFooter />
    </div>
  )
}
