'use client'

import { AppShell } from '@/components/layout/AppShell'

export default function SettingsPage() {
  return (
    <AppShell currentPage="settings">
      <div className="p-8">
        <h1 className="font-fraunces text-3xl font-[600] text-on-surface mb-4">
          Settings
        </h1>
        <p className="text-body-md text-on-surface-variant">
          Coming soon.
        </p>
      </div>
    </AppShell>
  )
}
