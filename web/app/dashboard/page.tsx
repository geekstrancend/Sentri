'use client'

import { useState } from 'react'
import Link from 'next/link'
import {
  Plus, CheckCircle2, AlertTriangle, BarChart3,
  Shield, Zap, ArrowUpRight, RefreshCw,
} from 'lucide-react'
import { AppShell } from '@/components/layout/AppShell'
import { Button } from '@/components/ui/Button'
import { ScanModal } from '@/components/ui/ScanModal'
import clsx from 'clsx'

interface Scan {
  id: string
  project: string
  chain: 'EVM' | 'Solana' | 'Arbitrum' | 'Base'
  date: string
  findings: { critical: number; high: number; medium: number; low: number }
  status: 'complete' | 'scanning' | 'failed'
  duration: string
}

interface Activity {
  type: 'finding' | 'shared' | 'complete' | 'updated' | 'failed'
  title: string
  description: string
  time: string
}

const SCANS: Scan[] = [
  { id: 'SENT-2026-042', project: 'Dexalot Contracts', chain: 'EVM', date: 'Jul 7, 2026', findings: { critical: 3, high: 6, medium: 7, low: 5 }, status: 'complete', duration: '4m 12s' },
  { id: 'SENT-2026-041', project: 'Circle-Pay BCH', chain: 'Solana', date: 'Jul 6, 2026', findings: { critical: 5, high: 7, medium: 6, low: 4 }, status: 'complete', duration: '6m 55s' },
  { id: 'SENT-2026-040', project: 'Vault Core V2', chain: 'Arbitrum', date: 'Jul 5, 2026', findings: { critical: 0, high: 0, medium: 2, low: 12 }, status: 'complete', duration: '2m 08s' },
  { id: 'SENT-2026-043', project: 'Protocol X LendingPool', chain: 'Base', date: 'Jul 8, 2026', findings: { critical: 0, high: 0, medium: 0, low: 0 }, status: 'scanning', duration: '–' },
]

const ACTIVITY: Activity[] = [
  { type: 'finding', title: 'Critical confirmed', description: 'Reentrancy vulnerability in Dexalot signature logic', time: '2m ago' },
  { type: 'shared', title: 'Report shared', description: 'Security disclosure SENT-2026-041 sent to Circle-Pay team', time: '45m ago' },
  { type: 'complete', title: 'Scan complete', description: 'Circle-Pay BCH finished — 22 findings across 3 contracts', time: '3h ago' },
  { type: 'updated', title: 'Library updated', description: '47 new patterns synced from global exploit database', time: '5h ago' },
  { type: 'complete', title: 'Scan complete', description: 'Vault Core V2 — Clean result, 14 low-risk observations', time: '1d ago' },
]

const METRICS = [
  { label: 'Total Scans', value: '43', delta: '+8 this month', icon: <BarChart3 size={18} className="text-acc-text" />, positive: true },
  { label: 'Critical Findings', value: '8', delta: '-3 resolved', icon: <AlertTriangle size={18} className="text-critical" />, positive: true },
  { label: 'Protocols Monitored', value: '12', delta: '+2 this month', icon: <Shield size={18} className="text-acc-text" />, positive: true },
  { label: 'Avg Scan Time', value: '4m 20s', delta: '↓ 18% faster', icon: <Zap size={18} className="text-acc-text" />, positive: true },
]

function ActivityIcon({ type }: { type: Activity['type'] }) {
  const styles: Record<Activity['type'], { cls: string; symbol: string }> = {
    finding:  { cls: 'bg-critical/20 text-critical', symbol: '!' },
    shared:   { cls: 'bg-medium/20 text-medium', symbol: '↗' },
    complete: { cls: 'bg-low/20 text-low', symbol: '✓' },
    updated:  { cls: 'bg-high/20 text-high', symbol: '↺' },
    failed:   { cls: 'bg-critical/20 text-critical', symbol: '✗' },
  }
  const { cls, symbol } = styles[type]
  return (
    <div className={clsx('w-7 h-7 rounded-full flex items-center justify-center text-xs font-[700] flex-shrink-0', cls)}>
      {symbol}
    </div>
  )
}

export default function DashboardPage() {
  const [showScanModal, setShowScanModal] = useState(false)

  return (
    <AppShell currentPage="dashboard" onNewScan={() => setShowScanModal(true)}>
      <div className="p-6 lg:p-8 max-w-6xl">
        {/* Header */}
        <div className="flex items-center justify-between mb-8">
          <div>
            <h1 className="font-display text-3xl font-[600] text-text mb-1">Dashboard</h1>
            <p className="text-body-md text-sec">Welcome back, Alex. Here&apos;s your security overview.</p>
          </div>
          <Button variant="primary" size="sm" icon={<Plus size={16} />} onClick={() => setShowScanModal(true)}>
            New Scan
          </Button>
        </div>

        {/* Metrics row */}
        <div className="grid grid-cols-2 lg:grid-cols-4 gap-4 mb-8">
          {METRICS.map((m, i) => (
            <div key={i} className="bg-panel border border-hair rounded-card p-5">
              <div className="flex items-center justify-between mb-3">
                <span className="text-body-md text-sec">{m.label}</span>
                <div className="w-8 h-8 rounded-lg bg-panel flex items-center justify-center">{m.icon}</div>
              </div>
              <div className="font-display text-3xl font-[600] text-text mb-1">{m.value}</div>
              <div className={clsx('text-xs font-[600]', m.positive ? 'text-low' : 'text-critical')}>{m.delta}</div>
            </div>
          ))}
        </div>

        {/* Main content + Activity side */}
        <div className="grid grid-cols-1 xl:grid-cols-3 gap-6">
          {/* Scans table */}
          <div className="xl:col-span-2 bg-panel border border-hair rounded-card overflow-hidden">
            <div className="flex items-center justify-between px-6 py-4 border-b border-hair">
              <h2 className="font-display text-lg font-[600] text-text">Recent Scans</h2>
              <Link href="/dashboard" className="text-acc-text text-sm font-[600] hover:text-acc-text/80 flex items-center gap-1 transition-colors">
                View all <ArrowUpRight size={14} />
              </Link>
            </div>

            <div className="overflow-x-auto">
              <table className="w-full">
                <thead>
                  <tr className="border-b border-hair">
                    {['Project', 'Chain', 'Findings', 'Date', 'Status', 'Actions'].map((h) => (
                      <th key={h} scope="col" className={clsx('text-left text-label-sm text-sec px-6 py-3 font-[600]', h === 'Actions' && 'sr-only')}>{h}</th>
                    ))}
                  </tr>
                </thead>
                <tbody>
                  {SCANS.map((scan) => (
                    <tr key={scan.id} className="border-b border-hair/50 hover:bg-panel/50 transition-colors group">
                      <td className="px-6 py-4">
                        <div>
                          <p className="text-body-md font-[600] text-text">{scan.project}</p>
                          <p className="text-xs text-sec font-mono">{scan.id}</p>
                        </div>
                      </td>
                      <td className="px-6 py-4">
                        <span className="text-xs text-sec bg-panel border border-hair px-2 py-0.5 rounded font-mono">{scan.chain}</span>
                      </td>
                      <td className="px-6 py-4">
                        {scan.status === 'scanning' ? (
                          <span className="text-sec text-body-md">—</span>
                        ) : (
                          <div className="flex items-center gap-1.5">
                            {scan.findings.critical > 0 && <span className="text-xs font-[700] text-critical">{scan.findings.critical}C</span>}
                            {scan.findings.high > 0 && <span className="text-xs font-[700] text-high">{scan.findings.high}H</span>}
                            {scan.findings.medium > 0 && <span className="text-xs font-[600] text-medium">{scan.findings.medium}M</span>}
                            {scan.findings.low > 0 && <span className="text-xs font-[600] text-low">{scan.findings.low}L</span>}
                            {scan.findings.critical === 0 && scan.findings.high === 0 && <span className="text-xs text-low font-[600]">Clean</span>}
                          </div>
                        )}
                      </td>
                      <td className="px-6 py-4">
                        <span className="text-body-md text-sec whitespace-nowrap">{scan.date}</span>
                      </td>
                      <td className="px-6 py-4">
                        {scan.status === 'scanning' ? (
                          <span className="inline-flex items-center gap-1.5 text-xs text-high font-[600]">
                            <RefreshCw size={12} className="animate-spin" /> Scanning
                          </span>
                        ) : scan.status === 'complete' ? (
                          <span className="inline-flex items-center gap-1.5 text-xs text-low font-[600]">
                            <CheckCircle2 size={12} /> Complete
                          </span>
                        ) : (
                          <span className="inline-flex items-center gap-1.5 text-xs text-critical font-[600]">
                            <AlertTriangle size={12} /> Failed
                          </span>
                        )}
                      </td>
                      <td className="px-6 py-4">
                        {scan.status === 'complete' && (
                          <Link
                            href={`/reports/${scan.id}`}
                            className="text-acc-text text-xs font-[600] hover:text-acc-text/80 flex items-center gap-1 whitespace-nowrap"
                          >
                            View report <ArrowUpRight size={12} />
                          </Link>
                        )}
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          </div>

          {/* Activity feed */}
          <div className="bg-panel border border-hair rounded-card overflow-hidden">
            <div className="px-5 py-4 border-b border-hair">
              <h2 className="font-display text-lg font-[600] text-text">Activity</h2>
            </div>
            <div className="divide-y divide-hair/50">
              {ACTIVITY.map((item, i) => (
                <div key={i} className="flex gap-3 px-5 py-4 hover:bg-panel/40 transition-colors">
                  <ActivityIcon type={item.type} />
                  <div className="flex-1 min-w-0">
                    <p className="text-body-md font-[600] text-text">{item.title}</p>
                    <p className="text-xs text-sec mt-0.5 leading-4">{item.description}</p>
                    <p className="text-xs text-sec mt-1">{item.time}</p>
                  </div>
                </div>
              ))}
            </div>
          </div>
        </div>
      </div>

      <ScanModal isOpen={showScanModal} onClose={() => setShowScanModal(false)} />
    </AppShell>
  )
}
