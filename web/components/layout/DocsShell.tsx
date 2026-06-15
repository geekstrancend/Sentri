'use client'

import Link from 'next/link'
import { Search, X, Menu } from 'lucide-react'
import { useState } from 'react'
import { usePathname } from 'next/navigation'
import clsx from 'clsx'

interface DocsSidebarItem {
  label: string
  href: string
}

interface DocsSidebarSection {
  title: string
  items: DocsSidebarItem[]
}

interface TableOfContentsItem {
  label: string
  href: string
}

interface DocsShellProps {
  children: React.ReactNode
  pageTitle?: string
  tableOfContents?: TableOfContentsItem[]
  editPath?: string
}

export function DocsShell({
  children,
  pageTitle = 'Documentation',
  tableOfContents,
  editPath,
}: DocsShellProps) {
  const [sidebarOpen, setSidebarOpen] = useState(false)
  const pathname = usePathname()

  const sections: DocsSidebarSection[] = [
    {
      title: 'GETTING STARTED',
      items: [
        { label: 'Introduction', href: '/docs' },
        { label: 'Quick Start', href: '/docs/getting-started' },
        { label: 'CLI Reference', href: '/docs/cli' },
      ],
    },
    {
      title: 'SECURITY',
      items: [
        { label: 'Invariant Library', href: '/library' },
        { label: 'Audit Report Guide', href: '/docs/reports' },
      ],
    },
    {
      title: 'INTEGRATIONS',
      items: [
        { label: 'CI/CD Integration', href: '/docs/ci-cd' },
      ],
    },
  ]

  const isActive = (href: string) => pathname === href

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
        <nav className="p-6 space-y-8">
          {sections.map((section, idx) => (
            <div key={idx}>
              <h3 className="text-xs uppercase tracking-wide font-medium text-outline-variant mb-3">
                {section.title}
              </h3>
              <ul className="space-y-2">
                {section.items.map((item, itemIdx) => (
                  <li key={itemIdx}>
                    <Link
                      href={item.href}
                      className={clsx(
                        'block px-3 py-2 rounded text-body-md transition-colors',
                        isActive(item.href)
                          ? 'bg-indigo/10 text-on-surface font-[500]'
                          : 'text-outline hover:bg-surface-container/50 hover:text-on-surface',
                      )}
                    >
                      {item.label}
                    </Link>
                  </li>
                ))}
              </ul>
            </div>
          ))}
        </nav>

        <div className="border-t border-outline-variant p-6 mt-8">
          <Link
            href="https://github.com/geekstrancend/Sentri"
            target="_blank"
            rel="noopener"
            className="text-outline hover:text-on-surface text-body-md transition-colors"
          >
            GitHub Repository
          </Link>
        </div>
      </div>

      {/* Main Content */}
      <div className="flex-1 flex flex-col">
        {/* Top Bar */}
        <div className="border-b border-outline-variant bg-surface-container-lowest sticky top-0 z-20">
          <div className="max-w-7xl mx-auto px-6 py-3 flex items-center gap-4 justify-between">
            {/* Left */}
            <div className="flex items-center gap-4">
              <button
                onClick={() => setSidebarOpen(!sidebarOpen)}
                className="md:hidden p-2 hover:bg-surface-container rounded text-outline"
              >
                {sidebarOpen ? <X size={20} /> : <Menu size={20} />}
              </button>
              <h1 className="font-fraunces text-lg font-[500] text-on-surface">
                {pageTitle}
              </h1>
            </div>

            {/* Center - Search */}
            <div className="hidden md:flex flex-1 max-w-xs">
              <div className="relative w-full">
                <Search size={16} className="absolute left-3 top-1/2 -translate-y-1/2 text-outline" />
                <input
                  type="text"
                  placeholder="Search..."
                  className="w-full bg-surface-container-low border border-outline-variant rounded px-3 pl-9 py-2 text-body-md text-on-surface placeholder-outline-variant focus:outline-none focus:border-indigo"
                />
              </div>
            </div>

            {/* Right - Edit Link */}
            {editPath && (
              <Link
                href={editPath}
                target="_blank"
                rel="noopener"
                className="hidden md:inline-flex items-center gap-2 px-3 py-1.5 bg-surface-container-low border border-outline-variant rounded text-body-md text-outline hover:text-on-surface transition-colors text-xs"
              >
                Edit this page
              </Link>
            )}
          </div>
        </div>

        {/* Content Wrapper */}
        <div className="flex-1 overflow-y-auto">
          <div className="flex max-w-7xl mx-auto">
            {/* Main Content */}
            <div className="flex-1 px-6 py-12 max-w-4xl">
              {children}
            </div>

            {/* Right Sidebar - Table of Contents */}
            {tableOfContents && tableOfContents.length > 0 && (
              <div className="hidden lg:block w-48 px-6 py-12 border-l border-outline-variant">
                <nav className="sticky top-20">
                  <h3 className="text-xs uppercase tracking-wide font-medium text-outline-variant mb-4">
                    On this page
                  </h3>
                  <ul className="space-y-2">
                    {tableOfContents.map((item, idx) => (
                      <li key={idx}>
                        <Link
                          href={item.href}
                          className="text-body-sm text-outline hover:text-on-surface transition-colors"
                        >
                          {item.label}
                        </Link>
                      </li>
                    ))}
                  </ul>
                </nav>
              </div>
            )}
          </div>
        </div>
      </div>
    </div>
  )
}
