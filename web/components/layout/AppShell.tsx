'use client'

import { useState } from 'react'
import Link from 'next/link'
import {
  LayoutDashboard,
  Shield,
  BookOpen,
  Settings,
  HelpCircle,
  BookMarked,
  LogOut,
  Plus,
  ShieldCheck,
  Menu,
  X,
} from 'lucide-react'
import clsx from 'clsx'

interface AppShellProps {
  children: React.ReactNode
  rightPanel?: React.ReactNode
  currentPage?: 'dashboard' | 'audits' | 'library' | 'settings' | 'support'
  onNewScan?: () => void
}

export function AppShell({
  children,
  rightPanel,
  currentPage = 'dashboard',
  onNewScan,
}: AppShellProps) {
  const [sidebarOpen, setSidebarOpen] = useState(false)

  const navItems = [
    { id: 'dashboard', label: 'Dashboard', icon: LayoutDashboard, href: '/dashboard' },
    { id: 'audits', label: 'Audits', icon: Shield, href: '/dashboard' },
    { id: 'library', label: 'Library', icon: BookOpen, href: '/library' },
    { id: 'settings', label: 'Settings', icon: Settings, href: '/dashboard/settings' },
    { id: 'support', label: 'Support', icon: HelpCircle, href: '/dashboard/support' },
  ]

  return (
    <div className="flex h-screen bg-surface">
      {/* Sidebar */}
      <div
        className={clsx(
          'fixed inset-y-0 left-0 z-40 w-60 bg-surface-container-lowest border-r border-outline-variant flex flex-col',
          'transform transition-transform duration-300',
          sidebarOpen ? 'translate-x-0' : '-translate-x-full md:translate-x-0',
        )}
      >
        {/* Logo Section */}
        <div className="p-5 border-b border-outline-variant">
          <Link href="/dashboard" className="flex items-center gap-2 mb-1">
            <ShieldCheck size={20} className="text-secondary" />
            <h1 className="font-fraunces text-base font-[600] text-on-surface">
              Sentri Security
            </h1>
          </Link>
          <p className="text-outline text-xs ml-7">Protocol Auditor</p>
          <div className="mt-2 ml-7 inline-block">
            <span className="text-label-sm bg-surface-container-low border border-outline-variant px-2 py-0.5 rounded text-low">
              AUTHENTICATED
            </span>
          </div>
        </div>

        {/* Nav Items */}
        <nav className="flex-1 px-2 py-2 space-y-1 overflow-y-auto">
          {navItems.map((item) => {
            const Icon = item.icon
            const isActive = currentPage === item.id
            return (
              <Link
                key={item.id}
                href={item.href}
                className={clsx(
                  'flex items-center gap-3 px-3 py-2.5 rounded text-body-md transition-colors',
                  isActive
                    ? 'bg-indigo/10 border-l-2 border-indigo text-on-surface ml-0.5'
                    : 'text-outline hover:text-on-surface',
                )}
              >
                <Icon size={18} />
                <span>{item.label}</span>
              </Link>
            )
          })}
        </nav>

        {/* Bottom Section */}
        <div className="p-4 border-t border-outline-variant space-y-3">
          <button
            onClick={onNewScan}
            className="w-full flex items-center justify-center gap-2 px-3 py-2.5 bg-surface-container-low border border-outline-variant rounded text-body-md text-on-surface-variant hover:border-indigo transition-colors"
          >
            <Plus size={16} />
            New Scan
          </button>
          <div className="border-t border-outline-variant pt-3 space-y-2">
            <Link href="/docs" className="flex items-center gap-3 px-3 py-2 text-outline hover:text-on-surface transition-colors">
              <BookMarked size={16} />
              <span className="text-body-md">Docs</span>
            </Link>
            <button className="flex items-center gap-3 px-3 py-2 text-outline hover:text-on-surface transition-colors w-full">
              <LogOut size={16} />
              <span className="text-body-md">Logout</span>
            </button>
          </div>
        </div>
      </div>

      {/* Main Content */}
      <div className="flex-1 flex flex-col md:ml-60">
        {/* Mobile Header */}
        <div className="md:hidden flex items-center justify-between px-4 py-4 border-b border-outline-variant bg-surface-container-low">
          <ShieldCheck size={24} className="text-secondary" />
          <button
            onClick={() => setSidebarOpen(!sidebarOpen)}
            className="p-2 hover:bg-surface-container rounded"
          >
            {sidebarOpen ? <X size={24} /> : <Menu size={24} />}
          </button>
        </div>

        {/* Content Area */}
        <div className="flex-1 flex overflow-hidden">
          <main className="flex-1 overflow-y-auto">{children}</main>

          {/* Right Panel */}
          {rightPanel && (
            <div className="hidden lg:block w-72 border-l border-outline-variant bg-surface-container-lowest overflow-y-auto">
              {rightPanel}
            </div>
          )}
        </div>
      </div>

      {/* Mobile Sidebar Overlay */}
      {sidebarOpen && (
        <div
          className="fixed inset-0 bg-black/50 z-30 md:hidden"
          onClick={() => setSidebarOpen(false)}
        />
      )}
    </div>
  )
}
