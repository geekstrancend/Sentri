'use client'

import { useState } from 'react'
import { Plus, MoreVertical, CheckCircle, AlertCircle, Clock, TrendingUp, BarChart3, Zap } from 'lucide-react'
import { AppShell } from '@/components/layout/AppShell'
import { Modal } from '@/components/ui/Modal'
import { Button } from '@/components/ui/Button'
import { SeverityBadge } from '@/components/ui/SeverityBadge'

interface Scan {
  id: string
  project: string
  chain: string
  date: string
  findings: { critical: number; high: number; medium: number; low: number }
  status: 'complete' | 'scanning'
}

interface ActivityItem {
  type: 'finding' | 'shared' | 'complete' | 'updated'
  title: string
  description: string
  time: string
  icon: React.ReactNode
  color: string
}

export default function DashboardPage() {
  const [showNewScanModal, setShowNewScanModal] = useState(false)
  const [selectedChain, setSelectedChain] = useState('EVM')
  const [selectedDepth, setSelectedDepth] = useState('Standard')

  const scans: Scan[] = [
    {
      id: '1',
      project: 'Dexalot Contracts',
      chain: 'EVM',
      date: 'Jun 7 2026',
      findings: { critical: 3, high: 6, medium: 7, low: 5 },
      status: 'complete',
    },
    {
      id: '2',
      project: 'Circle-Pay BCH',
      chain: 'Solana',
      date: 'Jun 6 2026',
      findings: { critical: 5, high: 7, medium: 6, low: 4 },
      status: 'complete',
    },
    {
      id: '3',
      project: 'Vault Core V2',
      chain: 'Arbitrum',
      date: 'Jun 5 2026',
      findings: { critical: 0, high: 0, medium: 2, low: 12 },
      status: 'complete',
    },
    {
      id: '4',
      project: 'Protocol X',
      chain: 'Solana',
      date: 'Jun 8 2026',
      findings: { critical: 0, high: 0, medium: 0, low: 0 },
      status: 'scanning',
    },
  ]

  const activityItems: ActivityItem[] = [
    {
      type: 'finding',
      title: 'Confirmed finding',
      description: 'Critical vulnerability in Dexalot Contracts signature logic.',
      time: '2m ago',
      icon: '!',
      color: 'bg-critical',
    },
    {
      type: 'shared',
      title: 'Report shared',
      description: 'Security disclosure Sentri-2026-04 forwarded to project leads.',
      time: '45m ago',
      icon: '↗',
      color: 'bg-medium',
    },
    {
      type: 'complete',
      title: 'Scan complete',
      description: 'Circle-Pay BCH automated analysis finished with 22 findings.',
      time: '3h ago',
      icon: '✓',
      color: 'bg-low',
    },
    {
      type: 'updated',
      title: 'Library updated',
      description: 'New vulnerability patterns synced from global threat database.',
      time: '5h ago',
      icon: '📚',
      color: 'bg-high',
    },
  ]

  const ActivityFeed = () => (
    <div className="space-y-0">
      <h3 className="text-label-sm text-outline px-6 py-4 flex items-center gap-2">
        <Clock size={14} />
        ACTIVITY FEED
      </h3>
      <div className="divide-y divide-outline-variant">
        {activityItems.map((item, idx) => (
          <div key={idx} className="px-6 py-3 hover:bg-surface-container/50 transition-colors">
            <div className="flex gap-3">
              <div className={`w-5 h-5 rounded-full flex-shrink-0 flex items-center justify-center text-white text-xs ${item.color}`}>
                {item.icon}
              </div>
              <div className="flex-1 min-w-0">
                <p className="text-body-md font-[600] text-on-surface mb-1">
                  {item.title}
                </p>
                <p className="text-body-md text-outline truncate">
                  {item.description}
                </p>
                <p className="text-xs text-outline-variant mt-1">{item.time}</p>
              </div>
            </div>
          </div>
        ))}
      </div>

      {/* Global Node Status */}
      <div className="border-t border-outline-variant p-6 mt-6">
        <h4 className="text-label-sm text-outline mb-4">GLOBAL NODE STATUS</h4>
        <div className="bg-surface-container-lowest border border-outline-variant rounded-lg p-4 h-40 flex items-center justify-center mb-3">
          <div className="w-full h-full rounded flex items-center justify-center text-outline">
            [Global Network Map]
          </div>
        </div>
        <div className="flex items-center gap-2">
          <div className="w-2 h-2 rounded-full bg-low" />
          <span className="text-body-md text-low">All Nodes Operational</span>
        </div>
      </div>
    </div>
  )

  return (
    <AppShell currentPage="dashboard" rightPanel={<ActivityFeed />} onNewScan={() => setShowNewScanModal(true)}>
      <div className="p-8">
        {/* Header */}
        <div className="flex justify-between items-center mb-8">
          <h1 className="font-fraunces text-3xl font-[600] text-on-surface">
            Recent Scans
          </h1>
          <Button
            variant="primary"
            size="sm"
            icon={<Plus size={16} />}
            onClick={() => setShowNewScanModal(true)}
          >
            New scan
          </Button>
        </div>

        {/* Stats Grid */}
        <div className="grid grid-cols-4 gap-px bg-outline-variant rounded-lg overflow-hidden mb-8">
          {[
            { label: 'TOTAL SCANS', value: '47', sub: '+12%', icon: BarChart3, color: 'text-on-surface' },
            { label: 'CRITICAL FINDINGS', value: '23', sub: 'High Risk', icon: AlertCircle, color: 'text-critical' },
            { label: 'REPORTS GENERATED', value: '12', sub: '↓', icon: Zap, color: 'text-low' },
            { label: 'INVARIANTS RUN', value: '100', sub: 'Passed', icon: CheckCircle, color: 'text-medium' },
          ].map((stat, idx) => {
            const Icon = stat.icon
            return (
              <div key={idx} className="bg-surface-container-low p-6">
                <div className="flex justify-between items-start mb-4">
                  <span className="text-label-sm text-outline">{stat.label}</span>
                  <Icon size={20} className={stat.color} />
                </div>
                <div className={`font-fraunces text-4xl font-[600] ${stat.color} mb-1`}>
                  {stat.value}
                </div>
                <div className="text-body-md text-outline-variant text-xs">{stat.sub}</div>
              </div>
            )
          })}
        </div>

        {/* Scans Table */}
        <div className="border border-outline-variant rounded-lg overflow-hidden">
          {/* Header */}
          <div className="bg-surface-container-low border-b border-outline-variant px-4 py-3 flex gap-4">
            {['Project', 'Chain', 'Date', 'Findings', 'Status', 'Actions'].map((col) => (
              <div key={col} className="text-label-sm text-outline flex-1">
                {col}
              </div>
            ))}
          </div>

          {/* Rows */}
          <div className="bg-surface divide-y divide-outline-variant">
            {scans.map((scan) => (
              <div key={scan.id} className="px-4 py-4 hover:bg-surface-container-low transition-colors flex gap-4 items-center lift-on-hover">
                {/* Project */}
                <div className="flex-1">
                  <p className="text-body-md font-[600] text-on-surface">{scan.project}</p>
                </div>

                {/* Chain */}
                <div className="flex-1">
                  <p className="text-body-md text-outline">{scan.chain}</p>
                </div>

                {/* Date */}
                <div className="flex-1">
                  <p className="text-body-md text-outline">{scan.date}</p>
                </div>

                {/* Findings */}
                <div className="flex-1 flex gap-1">
                  {scan.findings.critical > 0 && <SeverityBadge level="critical" count={scan.findings.critical} />}
                  {scan.findings.high > 0 && <SeverityBadge level="high" count={scan.findings.high} />}
                  {scan.findings.medium > 0 && <SeverityBadge level="medium" count={scan.findings.medium} />}
                  {scan.findings.low > 0 && <SeverityBadge level="low" count={scan.findings.low} />}
                </div>

                {/* Status */}
                <div className="flex-1">
                  {scan.status === 'complete' ? (
                    <div className="flex items-center gap-2 text-low text-body-md">
                      <CheckCircle size={16} />
                      Complete
                    </div>
                  ) : (
                    <div className="flex items-center gap-2 text-medium text-body-md animate-pulse-dot">
                      <div className="w-4 h-4 border-2 border-medium border-t-transparent rounded-full" />
                      Scanning
                    </div>
                  )}
                </div>

                {/* Actions */}
                <button className="flex-1 text-outline hover:text-on-surface p-1">
                  <MoreVertical size={18} />
                </button>
              </div>
            ))}
          </div>
        </div>
      </div>

      {/* New Scan Modal */}
      <Modal
        isOpen={showNewScanModal}
        onClose={() => setShowNewScanModal(false)}
        title="Start New Scan"
        size="lg"
      >
        <div className="p-6 space-y-6">
          <div>
            <label className="text-label-sm text-outline block mb-3">STEP 1: GITHUB REPOSITORY SELECTION</label>
            <div className="relative">
              <input
                type="text"
                placeholder="Search repositories..."
                className="w-full bg-surface-container-lowest border border-outline-variant rounded px-4 py-2 text-body-md text-on-surface placeholder-outline-variant focus:outline-none focus:border-indigo"
              />
            </div>
          </div>

          <div className="bg-surface-container-lowest border border-outline-variant rounded p-8 text-center">
            <div className="text-4xl mb-3">📁</div>
            <p className="text-body-md font-[600] text-on-surface-variant mb-2">No repositories found</p>
            <p className="text-body-md text-outline max-w-sm mx-auto">
              Connect your GitHub account to import your smart contract repositories.
            </p>
          </div>

          <div className="border-t border-outline-variant pt-6 flex gap-4">
            <div className="flex-1">
              <label className="text-label-sm text-outline block mb-2">CHAIN</label>
              <select className="w-full bg-surface-container-lowest border border-outline-variant rounded px-3 py-2 text-body-md text-on-surface focus:outline-none focus:border-indigo">
                <option>EVM</option>
                <option>Solana</option>
                <option>Cosmos</option>
              </select>
            </div>
            <div className="flex-1">
              <label className="text-label-sm text-outline block mb-2">ANALYSIS DEPTH</label>
              <select className="w-full bg-surface-container-lowest border border-outline-variant rounded px-3 py-2 text-body-md text-on-surface focus:outline-none focus:border-indigo">
                <option>Standard</option>
                <option>Deep</option>
                <option>Comprehensive</option>
              </select>
            </div>
            <Button variant="primary" size="sm" disabled className="mt-8 opacity-50 cursor-not-allowed">
              Launch Scan
            </Button>
          </div>
        </div>
      </Modal>
    </AppShell>
  )
}
