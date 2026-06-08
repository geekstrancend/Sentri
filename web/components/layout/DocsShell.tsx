'use client'

import Link from 'next/link'
import { Search, Moon, HelpCircle, Github, X, Menu } from 'lucide-react'
import { useState } from 'react'
import clsx from 'clsx'

interface DocsSidebarItem {
  label: string
  href: string
  icon: React.ReactNode
  isActive?: boolean
}

interface DocsShellProps {
  children: React.ReactNode
  sidebarItems?: DocsSidebarItem[]
  pageTitle?: string
}

export function DocsShell({ children, sidebarItems = [], pageTitle = 'Protocol Docs' }: DocsShellProps) {
  const [sidebarOpen, setSidebarOpen] = useState(false)

  const defaultItems: DocsSidebarItem[] = [
    { label: 'Introduction', href: '/docs', icon: '◉' },
    { label: 'Getting Started', href: '/docs/getting-started', icon: '🚀' },
    { label: 'CLI Reference', href: '/docs/cli', icon: '⌨️' },
    { label: 'Invariant Library', href: '/library', icon: '🛡️' },
    { label: 'Audit Report Guide', href: '/docs/reports', icon: '📄', isActive: true },
    { label: 'CI/CD Integration', href: '/docs/ci-cd', icon: '🔀' },
  ]

  const items = sidebarItems.length > 0 ? sidebarItems : defaultItems

  return (
    <div className="flex h-screen bg-surface">
      {/* Left Sidebar */}
      <div
        className={clsx(
          'fixed inset-y-0 left-0 z-30 w-60 bg-surface-container-lowest border-r border-outline-variant overflow-y-auto',
          'transform transition-transform duration-300 md:relative md:translate-x-0',
          sidebarOpen ? 'translate-x-0' : '-translate-x-full',
        )}
      >
        <nav className="p-4 space-y-1">
          {items.map((item, idx) => (
            <Link
              key={idx}
              href={item.href}
              className={clsx(
                'block px-4 py-2.5 rounded text-body-md transition-colors',
                item.isActive
                  ? 'bg-indigo/10 border-l-2 border-indigo text-on-surface ml-0.5'
                  : 'text-outline hover:bg-surface-container/50 hover:text-on-surface',
              )}
            >
              <span className="mr-2">{item.icon}</span>
              {item.label}
            </Link>
          ))}
        </nav>

        <div className="border-t border-outline-variant p-4 mt-8 space-y-3">
          <div className="bg-surface-container-low border border-outline-variant rounded px-3 py-2 flex items-center gap-2 text-outline text-xs">
            <span>⌘K</span>
            <span className="ml-auto">Command Palette</span>
          </div>
          <Link href="#" className="flex items-center gap-2 px-3 py-2 text-outline hover:text-on-surface text-body-md transition-colors">
            <span>🔑</span>
            API Keys
          </Link>
          <Link href="#" className="flex items-center gap-2 px-3 py-2 text-outline hover:text-on-surface text-body-md transition-colors">
            <span>💻</span>
            GitHub Repo
          </Link>
        </div>
      </div>

      {/* Main Content */}
      <div className="flex-1 flex flex-col">
        {/* Top Bar */}
        <div className="border-b border-outline-variant bg-surface-container-lowest">
          <div className="max-w-7xl mx-auto px-6 py-3 flex items-center gap-4 justify-between">
            {/* Left */}
            <div className="flex items-center gap-4 flex-1">
              <button
                onClick={() => setSidebarOpen(!sidebarOpen)}
                className="md:hidden p-2 hover:bg-surface-container rounded"
              >
                {sidebarOpen ? <X size={20} /> : <Menu size={20} />}
              </button>
              <div className="hidden md:block">
                <h2 className="font-fraunces text-sm font-[600] text-on-surface">
                  ProtocolDocs
                </h2>
                <p className="text-outline text-xs">v2.4.0-alpha</p>
              </div>
              <div className="hidden md:block w-px h-6 bg-outline-variant" />
              <h1 className="font-fraunces text-lg font-[500] text-on-surface">
                {pageTitle}
              </h1>
            </div>

            {/* Center - Search */}
            <div className="hidden md:block flex-1 max-w-xs">
              <div className="relative">
                <Search size={16} className="absolute left-3 top-1/2 -translate-y-1/2 text-outline" />
                <input
                  type="text"
                  placeholder="Search docs..."
                  className="w-full bg-surface-container-low border border-outline-variant rounded px-3 pl-9 py-2 text-body-md text-on-surface placeholder-outline-variant focus:outline-none focus:border-indigo"
                />
              </div>
            </div>

            {/* Right - Nav Links */}
            <div className="hidden md:flex items-center gap-6">
              <Link href="#" className="text-outline hover:text-on-surface text-body-md transition-colors">
                Guides
              </Link>
              <Link href="#" className="text-indigo text-body-md border-b-2 border-indigo pb-1">
                Reference
              </Link>
              <Link href="#" className="text-outline hover:text-on-surface text-body-md transition-colors">
                Community
              </Link>
            </div>

            {/* Icons */}
            <div className="flex items-center gap-2">
              <button className="p-2 hover:bg-surface-container rounded text-outline hover:text-on-surface">
                <Moon size={18} />
              </button>
              <button className="p-2 hover:bg-surface-container rounded text-outline hover:text-on-surface">
                <HelpCircle size={18} />
              </button>
              <button className="hidden md:flex items-center gap-2 px-3 py-1.5 bg-surface-container-low border border-outline-variant rounded text-body-md text-outline hover:text-on-surface transition-colors text-xs">
                <Github size={14} />
                <span>Edit</span>
              </button>
            </div>
          </div>
        </div>

        {/* Content */}
        <div className="flex-1 overflow-y-auto">
          <div className="max-w-4xl mx-auto px-6 py-12">
            {children}
          </div>
        </div>
      </div>

      {/* Sidebar Overlay */}
      {sidebarOpen && (
        <div
          className="fixed inset-0 bg-black/50 z-20 md:hidden"
          onClick={() => setSidebarOpen(false)}
        />
      )}
    </div>
  )
}
