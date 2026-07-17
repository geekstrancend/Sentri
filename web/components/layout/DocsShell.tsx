'use client'

import Link from 'next/link'
import { Search, X, Menu, ArrowLeft, ArrowRight, Github } from 'lucide-react'
import { useEffect, useMemo, useRef, useState } from 'react'
import { usePathname } from 'next/navigation'
import clsx from 'clsx'
import { useEscapeKey } from '../hooks/useEscapeKey'
import { Badge } from '../ui/Badge'
import { SentriLogo } from '../ui/SentriLogo'

interface DocsSidebarItem {
  label: string
  href: string
  badge?: string
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

const SECTIONS: DocsSidebarSection[] = [
  {
    title: 'Getting started',
    items: [
      { label: 'Introduction', href: '/docs' },
      { label: 'Quick start', href: '/docs/getting-started' },
      { label: 'CLI reference', href: '/docs/cli' },
    ],
  },
  {
    title: 'Security',
    items: [
      { label: 'Invariant library', href: '/library' },
      { label: 'AI co-auditor', href: '/docs/ai', badge: 'Pro' },
      { label: 'Audit report guide', href: '/docs/reports' },
    ],
  },
  {
    title: 'Integrations',
    items: [
      { label: 'CI/CD integration', href: '/docs/ci-cd' },
      { label: 'REST API', href: '/docs/api' },
    ],
  },
]

/** Flattened reading order, used for prev/next pagination. */
const FLAT_PAGES = SECTIONS.flatMap((s) => s.items)

export function DocsShell({
  children,
  pageTitle = 'Documentation',
  tableOfContents,
  editPath,
}: DocsShellProps) {
  const [sidebarOpen, setSidebarOpen] = useState(false)
  const [searchQuery, setSearchQuery] = useState('')
  const [activeId, setActiveId] = useState<string | null>(null)
  const searchRef = useRef<HTMLInputElement>(null)
  const pathname = usePathname()

  useEscapeKey(sidebarOpen, () => setSidebarOpen(false))

  // Lock body scroll while the mobile drawer is open.
  useEffect(() => {
    document.body.style.overflow = sidebarOpen ? 'hidden' : ''
    return () => {
      document.body.style.overflow = ''
    }
  }, [sidebarOpen])

  // Close the drawer on navigation.
  useEffect(() => {
    setSidebarOpen(false)
  }, [pathname])

  // "/" focuses docs search — a convention developers expect.
  useEffect(() => {
    const onKey = (e: KeyboardEvent) => {
      const el = document.activeElement
      const typing =
        el instanceof HTMLInputElement ||
        el instanceof HTMLTextAreaElement ||
        (el as HTMLElement | null)?.isContentEditable
      if (e.key === '/' && !typing) {
        e.preventDefault()
        searchRef.current?.focus()
      }
    }
    document.addEventListener('keydown', onKey)
    return () => document.removeEventListener('keydown', onKey)
  }, [])

  // Scroll-spy: highlight the "on this page" entry for the section in view.
  useEffect(() => {
    if (!tableOfContents?.length) return
    const ids = tableOfContents
      .map((i) => i.href.replace('#', ''))
      .filter(Boolean)
    const headings = ids
      .map((id) => document.getElementById(id))
      .filter((el): el is HTMLElement => Boolean(el))
    if (!headings.length) return

    const observer = new IntersectionObserver(
      (entries) => {
        const visible = entries
          .filter((e) => e.isIntersecting)
          .sort((a, b) => a.boundingClientRect.top - b.boundingClientRect.top)
        if (visible[0]) setActiveId(visible[0].target.id)
      },
      // Bias toward the upper portion of the viewport so the active item
      // tracks what the reader is actually looking at.
      { rootMargin: '-80px 0px -70% 0px', threshold: 0 },
    )
    headings.forEach((h) => observer.observe(h))
    return () => observer.disconnect()
  }, [tableOfContents, pathname])

  const isActive = (href: string) => pathname === href

  const filteredSections = useMemo(() => {
    if (!searchQuery) return SECTIONS
    const q = searchQuery.toLowerCase()
    return SECTIONS.map((s) => ({
      ...s,
      items: s.items.filter((i) => i.label.toLowerCase().includes(q)),
    })).filter((s) => s.items.length > 0)
  }, [searchQuery])

  const currentIndex = FLAT_PAGES.findIndex((p) => p.href === pathname)
  const prevPage = currentIndex > 0 ? FLAT_PAGES[currentIndex - 1] : null
  const nextPage =
    currentIndex >= 0 && currentIndex < FLAT_PAGES.length - 1
      ? FLAT_PAGES[currentIndex + 1]
      : null

  return (
    <div className="min-h-dvh bg-surface">
      {/* ── Sidebar ── */}
      <aside
        className={clsx(
          'fixed inset-y-0 left-0 z-40 flex w-72 flex-col border-r border-outline-variant bg-surface-container-lowest',
          'transition-transform duration-300 ease-out md:translate-x-0',
          sidebarOpen ? 'translate-x-0' : '-translate-x-full',
        )}
      >
        <div className="flex-shrink-0 border-b border-outline-variant p-4">
          <Link href="/" className="mb-4 flex items-center gap-2">
            <SentriLogo size={24} />
            <span className="text-sm font-[600] text-on-surface">Sentri</span>
            <span className="text-label-sm text-outline">Docs</span>
          </Link>

          <div className="relative">
            <Search
              size={13}
              className="pointer-events-none absolute left-2.5 top-1/2 -translate-y-1/2 text-outline"
            />
            <input
              ref={searchRef}
              type="search"
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              placeholder="Search docs…"
              aria-label="Search documentation"
              className="w-full rounded-lg border border-outline-variant bg-surface-container py-1.5 pl-8 pr-8 text-xs text-on-surface placeholder-outline transition-colors focus:border-indigo focus:outline-none"
            />
            {searchQuery ? (
              <button
                onClick={() => setSearchQuery('')}
                aria-label="Clear search"
                className="absolute right-1 top-1/2 -translate-y-1/2 p-1.5 text-outline hover:text-on-surface"
              >
                <X size={12} />
              </button>
            ) : (
              <kbd className="pointer-events-none absolute right-2 top-1/2 hidden -translate-y-1/2 rounded border border-outline-variant bg-surface-container-high px-1.5 py-0.5 font-mono text-[0.6rem] text-outline md:block">
                /
              </kbd>
            )}
          </div>
        </div>

        <nav className="flex-1 space-y-6 overflow-y-auto p-4" aria-label="Documentation">
          {filteredSections.map((section) => (
            <div key={section.title}>
              <p className="mb-2 px-2 text-label-sm text-outline">{section.title}</p>
              <ul className="space-y-0.5">
                {section.items.map((item) => (
                  <li key={item.href}>
                    <Link
                      href={item.href}
                      aria-current={isActive(item.href) ? 'page' : undefined}
                      className={clsx(
                        'flex items-center justify-between rounded-lg px-3 py-2 text-body-md transition-colors',
                        isActive(item.href)
                          ? 'border-l-2 border-indigo bg-indigo/10 pl-[10px] font-[500] text-on-surface'
                          : 'text-on-surface-variant hover:bg-surface-container hover:text-on-surface',
                      )}
                    >
                      <span>{item.label}</span>
                      {item.badge && <Badge tone="indigo">{item.badge}</Badge>}
                    </Link>
                  </li>
                ))}
              </ul>
            </div>
          ))}
          {filteredSections.length === 0 && (
            <p className="px-2 text-body-sm text-outline">
              No pages match “{searchQuery}”.
            </p>
          )}
        </nav>

        <div className="flex-shrink-0 space-y-1 border-t border-outline-variant p-4">
          <a
            href="https://github.com/geekstrancend/Sentri"
            target="_blank"
            rel="noopener noreferrer"
            className="flex items-center gap-2 rounded-lg px-3 py-2 text-body-md text-outline transition-colors hover:bg-surface-container hover:text-on-surface"
          >
            <Github size={14} /> GitHub
          </a>
          <Link
            href="/"
            className="flex items-center gap-2 rounded-lg px-3 py-2 text-body-md text-outline transition-colors hover:bg-surface-container hover:text-on-surface"
          >
            <ArrowLeft size={14} /> Back to home
          </Link>
        </div>
      </aside>

      {/* ── Main column ── */}
      <div className="md:pl-72">
        <header className="glass-nav sticky top-0 z-20 border-b border-outline-variant">
          <div className="flex items-center justify-between gap-4 px-5 py-3 sm:px-6">
            <div className="flex items-center gap-3">
              <button
                onClick={() => setSidebarOpen((v) => !v)}
                aria-label={sidebarOpen ? 'Close menu' : 'Open menu'}
                aria-expanded={sidebarOpen}
                className="rounded-lg p-1.5 text-on-surface-variant transition-colors hover:bg-surface-container md:hidden"
              >
                {sidebarOpen ? <X size={18} /> : <Menu size={18} />}
              </button>
              {/* Breadcrumb — orientation for 2+ level hierarchy */}
              <nav aria-label="Breadcrumb" className="hidden items-center gap-2 text-body-md md:flex">
                <Link href="/docs" className="text-outline transition-colors hover:text-on-surface">
                  Docs
                </Link>
                {pathname !== '/docs' && (
                  <>
                    <span className="text-outline-variant">/</span>
                    <span className="text-on-surface">{pageTitle}</span>
                  </>
                )}
              </nav>
            </div>

            <div className="flex items-center gap-3">
              {editPath && (
                <a
                  href={editPath}
                  target="_blank"
                  rel="noopener noreferrer"
                  className="hidden rounded-lg border border-outline-variant px-2.5 py-1 text-xs text-outline transition-colors hover:border-indigo/50 hover:text-on-surface md:inline-flex"
                >
                  Edit on GitHub
                </a>
              )}
              <Link
                href="/dashboard"
                className="inline-flex items-center gap-1.5 rounded-lg bg-indigo px-3 py-1.5 text-xs font-[600] text-surface-container-lowest transition-colors hover:bg-indigo-bright"
              >
                Dashboard <ArrowRight size={12} />
              </Link>
            </div>
          </div>
        </header>

        <div className="mx-auto flex max-w-6xl gap-8 px-5 sm:px-6 lg:px-8">
          <main id="main" className="min-w-0 flex-1 py-12">
            {/* Long-form docs prose gets a comfortable measure */}
            <div className="max-w-3xl">{children}</div>

            {/* Prev / next pagination */}
            {(prevPage || nextPage) && (
              <nav
                aria-label="Pagination"
                className="mt-16 grid max-w-3xl grid-cols-1 gap-3 border-t border-outline-variant pt-8 sm:grid-cols-2"
              >
                {prevPage ? (
                  <Link
                    href={prevPage.href}
                    className="group rounded-xl border border-outline-variant p-4 transition-colors hover:border-indigo/50"
                  >
                    <span className="flex items-center gap-1.5 text-label-sm text-outline">
                      <ArrowLeft size={12} /> Previous
                    </span>
                    <span className="mt-1 block text-body-md font-[500] text-on-surface">
                      {prevPage.label}
                    </span>
                  </Link>
                ) : (
                  <span />
                )}
                {nextPage && (
                  <Link
                    href={nextPage.href}
                    className="group rounded-xl border border-outline-variant p-4 text-right transition-colors hover:border-indigo/50 sm:col-start-2"
                  >
                    <span className="flex items-center justify-end gap-1.5 text-label-sm text-outline">
                      Next <ArrowRight size={12} />
                    </span>
                    <span className="mt-1 block text-body-md font-[500] text-on-surface">
                      {nextPage.label}
                    </span>
                  </Link>
                )}
              </nav>
            )}
          </main>

          {/* On this page */}
          {tableOfContents && tableOfContents.length > 0 && (
            <aside className="hidden w-56 flex-shrink-0 py-12 xl:block">
              <nav className="sticky top-24" aria-label="On this page">
                <p className="mb-4 text-label-sm text-outline">On this page</p>
                <ul className="space-y-2.5 border-l border-outline-variant">
                  {tableOfContents.map((item) => {
                    const id = item.href.replace('#', '')
                    const active = activeId === id
                    return (
                      <li key={item.href}>
                        <Link
                          href={item.href}
                          aria-current={active ? 'location' : undefined}
                          className={clsx(
                            '-ml-px block border-l pl-4 text-body-sm transition-colors',
                            active
                              ? 'border-indigo font-[500] text-on-surface'
                              : 'border-transparent text-outline hover:border-outline hover:text-on-surface',
                          )}
                        >
                          {item.label}
                        </Link>
                      </li>
                    )
                  })}
                </ul>
              </nav>
            </aside>
          )}
        </div>
      </div>

      {/* Mobile scrim */}
      {sidebarOpen && (
        <div
          className="animate-fade-in fixed inset-0 z-30 bg-black/60 md:hidden"
          onClick={() => setSidebarOpen(false)}
          aria-hidden
        />
      )}
    </div>
  )
}
