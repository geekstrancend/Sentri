'use client'

import { useState, useMemo } from 'react'
import { BookOpen, Clock, Plus } from 'lucide-react'
import { DocsShell } from '@/components/layout/DocsShell'
import { Button } from '@/components/ui/Button'
import { SeverityBadge } from '@/components/ui/SeverityBadge'

interface InvariantCard {
  id: string
  severity: 'critical' | 'high' | 'medium' | 'low'
  title: string
  description: string
  tags: string[]
  cvss: number
  version: string
  audits: number
}

export default function LibraryPage() {
  const invariants: InvariantCard[] = [
    {
      id: 'EVM-H27',
      severity: 'high',
      title: 'evm_cross_chain_arbitrary_message_validation',
      description: 'Ensures all incoming messages from LayerZero or Axelar endpoints are validated against a known source...',
      tags: ['BRIDGES', 'INTEROPERABILITY'],
      cvss: 8.8,
      version: 'v2.1.0',
      audits: 42,
    },
    {
      id: 'EVM-H14',
      severity: 'critical',
      title: 'evm_reentrancy_protection',
      description: 'Verifies that all external calls following state changes are protected by a nonReentrant modifier...',
      tags: ['CORE', 'CEI PATTERN'],
      cvss: 9.1,
      version: 'v4.0.2',
      audits: 1200,
    },
    {
      id: 'SOL-M05',
      severity: 'medium',
      title: 'solana_account_owner_validation',
      description: 'Mandates that all AccountInfo data is checked for ownership by the calling program before state...',
      tags: ['SOLANA', 'OWNERSHIP'],
      cvss: 5.4,
      version: 'v1.0.4',
      audits: 186,
    },
    {
      id: 'GEN-L02',
      severity: 'low',
      title: 'generic_event_emission_check',
      description: 'Standardizes that every state-changing function emits a corresponding event for off-chain indexing...',
      tags: ['LOGGING', 'STANDARDS'],
      cvss: 2.1,
      version: 'v0.9.1',
      audits: 3450,
    },
    {
      id: 'EVM-H31',
      severity: 'high',
      title: 'evm_oracle_heartbeat_freshness',
      description: 'Validates that Oracle responses (Chainlink/Pyth) include a updatedAt timestamp that is within the...',
      tags: ['DEFI', 'ORACLES'],
      cvss: 7.5,
      version: 'v3.2.0',
      audits: 89,
    },
    {
      id: 'EVM-C01',
      severity: 'critical',
      title: 'evm_self_destruct_removal',
      description: 'Detects legacy SELFDESTRUCT opcodes which are deprecated post-Cancun and can lead to...',
      tags: ['POST-CANCUN', 'GOVERNANCE'],
      cvss: 9.8,
      version: 'v1.1.0',
      audits: 12,
    },
  ]

  const InvariantCard = ({ invariant }: { invariant: InvariantCard }) => (
    <div className="bg-surface-container-low border border-outline-variant rounded-lg p-6 lift-on-hover flex flex-col gap-3">
      {/* Header */}
      <div className="flex justify-between items-start">
        <span className="text-label-sm bg-surface-container border border-outline-variant text-outline px-2 py-1 rounded">
          {invariant.id}
        </span>
        <SeverityBadge level={invariant.severity} />
      </div>

      {/* Title */}
      <h3 className="font-mono text-body-md font-[600] text-on-surface">{invariant.title}</h3>

      {/* Description */}
      <p className="text-body-md text-outline leading-5 line-clamp-2">{invariant.description}</p>

      {/* Tags */}
      <div className="flex flex-wrap gap-1.5">
        {invariant.tags.map((tag) => (
          <span key={tag} className="text-xs bg-surface-container border border-outline-variant text-outline px-2 py-1 rounded">
            [{tag}]
          </span>
        ))}
      </div>

      {/* Footer */}
      <div className="border-t border-outline-variant pt-3 mt-auto flex justify-between items-end">
        <div className="space-y-1 text-xs">
          <div>
            <span className="text-outline-variant">CVSS</span>
            <span className="text-on-surface ml-2 font-[600]">{invariant.cvss}</span>
          </div>
          <div>
            <span className="text-outline-variant">VERSION</span>
            <span className="text-on-surface ml-2 font-[600]">{invariant.version}</span>
          </div>
        </div>
        <div className="flex items-center gap-1 text-outline text-xs">
          <Clock size={14} />
          {invariant.audits > 1000 ? `${(invariant.audits / 1000).toFixed(1)}k` : invariant.audits} Audits
        </div>
      </div>
    </div>
  )

  const [searchQuery, setSearchQuery] = useState('')
  const [severityFilter, setSeverityFilter] = useState<string>('All')

  const filteredInvariants = useMemo(() => {
    return invariants.filter((inv) => {
      const matchesSearch =
        inv.title.toLowerCase().includes(searchQuery.toLowerCase()) ||
        inv.description.toLowerCase().includes(searchQuery.toLowerCase()) ||
        inv.tags.some((tag) => tag.toLowerCase().includes(searchQuery.toLowerCase()))
      const matchesSeverity =
        severityFilter === 'All' || inv.severity === severityFilter.toLowerCase()
      return matchesSearch && matchesSeverity
    })
  }, [searchQuery, severityFilter])

  return (
    <DocsShell pageTitle="Security Invariant Library">
      <div className="max-w-6xl mx-auto">
        {/* Header */}
        <div className="mb-8">
          <h1 className="font-fraunces text-4xl font-[600] text-on-surface mb-4">Security Invariant Library</h1>
          <p className="text-body-lg text-outline max-w-2xl mb-6">
            Browse our comprehensive library of 1,400+ automated security checks designed for EVM, Solana, and Cosmos smart contracts. Every invariant is battle-tested through real-world audits.
          </p>

          <div className="flex gap-3 flex-wrap">
            <input
              type="text"
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              placeholder="Search invariants (e.g., reentrancy, oracle)..."
              className="flex-1 min-w-xs max-w-md bg-surface-container-lowest border border-outline-variant rounded px-4 py-2 text-body-md text-on-surface placeholder-outline-variant focus:outline-none focus:border-indigo"
            />
            <Button variant="primary" size="sm" icon={<Plus size={16} />}>
              Suggest invariant
            </Button>
          </div>
        </div>

        {/* Filters */}
        <div className="flex flex-col sm:flex-row items-start sm:items-center gap-4 mb-8 pb-6 border-b border-outline-variant">
          <div className="flex gap-2 flex-wrap">
            {['All', 'EVM', 'Solana', 'Cosmos'].map((chain) => (
              <button
                key={chain}
                className={`px-4 py-1.5 rounded-full text-body-md transition-colors border ${
                  chain === 'All'
                    ? 'bg-secondary-container border-indigo text-on-background'
                    : 'bg-transparent border-outline-variant text-outline hover:border-indigo hover:text-on-surface'
                }`}
              >
                {chain}
              </button>
            ))}
          </div>

          <div className="w-px h-6 bg-outline-variant hidden sm:block" />

          <select
            value={severityFilter}
            onChange={(e) => setSeverityFilter(e.target.value)}
            className="bg-transparent border border-outline-variant rounded px-3 py-1.5 text-outline text-body-md focus:outline-none focus:border-indigo"
          >
            <option>All Severity</option>
            <option value="critical">Critical</option>
            <option value="high">High</option>
            <option value="medium">Medium</option>
            <option value="low">Low</option>
          </select>

          <div className="ml-auto text-label-sm text-outline">
            {filteredInvariants.length} OF {invariants.length}
          </div>
        </div>

        {/* Invariant Cards Grid */}
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4 mb-16">
          {filteredInvariants.map((invariant) => (
            <InvariantCard key={invariant.id} invariant={invariant} />
          ))}
        </div>

        {/* Stats Section */}
        <div className="bg-surface-container rounded-lg p-8 mb-8">
          <div className="grid grid-cols-1 md:grid-cols-3 gap-8 text-center">
            <div>
              <div className="font-fraunces text-4xl font-[700] text-secondary mb-2">1,402</div>
              <p className="text-on-surface-variant">Automated Checks</p>
            </div>
            <div>
              <div className="font-fraunces text-4xl font-[700] text-secondary mb-2">5.3k</div>
              <p className="text-on-surface-variant">Protocols Protected</p>
            </div>
            <div>
              <div className="font-fraunces text-4xl font-[700] text-secondary mb-2">$2B+</div>
              <p className="text-on-surface-variant">Secured Value</p>
            </div>
          </div>
        </div>

        {/* Contribute Section */}
        <div className="bg-indigo-container border border-indigo rounded-lg p-8 text-center">
          <h3 className="font-fraunces text-2xl font-[600] text-on-surface mb-3">
            Found a new vulnerability pattern?
          </h3>
          <p className="text-on-surface-variant mb-4 max-w-2xl mx-auto">
            Help us strengthen the security of the entire Web3 ecosystem by suggesting new invariants based on your research and findings.
          </p>
          <Button variant="primary">
            Submit New Invariant
          </Button>
        </div>
      </div>
    </DocsShell>
  )
}
