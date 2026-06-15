'use client'

import { DocsShell } from '@/components/layout/DocsShell'

export default function ReportsPage() {
  const toc = [
    { label: 'Report Structure', href: '#report-structure' },
    { label: 'Severity Definitions', href: '#severity-definitions' },
    { label: 'Reading a Finding', href: '#reading-finding' },
    { label: 'Exporting Reports', href: '#exporting-reports' },
  ]

  return (
    <DocsShell pageTitle="Audit Report Guide" tableOfContents={toc}>
      <article className="space-y-12">
        <div>
          <h1 className="font-fraunces text-5xl font-[600] text-on-surface mb-4">
            Audit Report Guide
          </h1>
          <p className="text-body-lg text-outline max-w-2xl">
            Learn how to read, interpret, and share Sentri security audit reports with your team.
          </p>
        </div>

        {/* Report Structure */}
        <section id="report-structure">
          <h2 className="font-fraunces text-2xl font-[600] text-on-surface mt-12 mb-4 scroll-mt-24">
            Report Structure
          </h2>
          <p className="text-body-md text-on-surface-variant mb-4 leading-relaxed">
            Sentri audit reports are organized into three main sections:
          </p>

          <h3 className="font-fraunces text-lg font-[600] text-on-surface mt-6 mb-3">
            1. Executive Summary
          </h3>
          <p className="text-body-md text-on-surface-variant mb-4 leading-relaxed">
            High-level overview suitable for non-technical stakeholders. Includes total finding count by severity, scan date, and key risk assessment.
          </p>

          <h3 className="font-fraunces text-lg font-[600] text-on-surface mt-6 mb-3">
            2. Findings Section
          </h3>
          <p className="text-body-md text-on-surface-variant mb-4 leading-relaxed">
            Detailed technical findings organized by severity level. Each finding includes file location, line numbers, invariant violated, and remediation guidance.
          </p>

          <h3 className="font-fraunces text-lg font-[600] text-on-surface mt-6 mb-3">
            3. Invariant Coverage Appendix
          </h3>
          <p className="text-body-md text-on-surface-variant mb-4 leading-relaxed">
            Complete list of all 1,400+ invariants checked during the scan, with pass/fail status and coverage statistics.
          </p>
        </section>

        {/* Severity Definitions */}
        <section id="severity-definitions">
          <h2 className="font-fraunces text-2xl font-[600] text-on-surface mt-12 mb-4 scroll-mt-24">
            Severity Definitions
          </h2>
          <p className="text-body-md text-on-surface-variant mb-4 leading-relaxed">
            Findings are classified by potential impact to protocol security and user funds:
          </p>

          <table className="w-full border border-outline-variant rounded text-body-md mt-6">
            <thead>
              <tr className="border-b border-outline-variant">
                <th className="text-left p-3 bg-surface-container-low font-[600] text-on-surface">Severity</th>
                <th className="text-left p-3 bg-surface-container-low font-[600] text-on-surface">Definition</th>
              </tr>
            </thead>
            <tbody>
              <tr className="border-b border-outline-variant">
                <td className="p-3"><span className="text-critical font-[600]">Critical</span></td>
                <td className="p-3 text-on-surface-variant">Immediate fund loss or protocol compromise possible. Deploy-blocking.</td>
              </tr>
              <tr className="border-b border-outline-variant">
                <td className="p-3"><span className="text-high font-[600]">High</span></td>
                <td className="p-3 text-on-surface-variant">Significant vulnerability requiring urgent remediation before deployment.</td>
              </tr>
              <tr className="border-b border-outline-variant">
                <td className="p-3"><span className="text-medium font-[600]">Medium</span></td>
                <td className="p-3 text-on-surface-variant">Notable security issue that should be addressed before deployment.</td>
              </tr>
              <tr className="border-b border-outline-variant">
                <td className="p-3"><span className="text-low font-[600]">Low</span></td>
                <td className="p-3 text-on-surface-variant">Minor issue or optimization opportunity with limited impact.</td>
              </tr>
              <tr>
                <td className="p-3"><span className="text-outline font-[600]">Info</span></td>
                <td className="p-3 text-on-surface-variant">Informational finding or best practice recommendation.</td>
              </tr>
            </tbody>
          </table>
        </section>

        {/* Reading a Finding */}
        <section id="reading-finding">
          <h2 className="font-fraunces text-2xl font-[600] text-on-surface mt-12 mb-4 scroll-mt-24">
            Reading a Finding
          </h2>
          <p className="text-body-md text-on-surface-variant mb-4 leading-relaxed">
            Each finding contains standardized fields to help your team understand the vulnerability:
          </p>

          <h3 className="font-fraunces text-lg font-[600] text-on-surface mt-6 mb-3">
            Finding Fields
          </h3>
          <ul className="space-y-4 text-body-md text-on-surface-variant">
            <li>
              <strong className="text-on-surface">Title</strong> — Brief name of the vulnerability
            </li>
            <li>
              <strong className="text-on-surface">Severity</strong> — Critical, High, Medium, Low, or Info
            </li>
            <li>
              <strong className="text-on-surface">File</strong> — Source file path containing the issue
            </li>
            <li>
              <strong className="text-on-surface">Lines</strong> — Specific line numbers where vulnerability occurs
            </li>
            <li>
              <strong className="text-on-surface">Invariant</strong> — Which security invariant was violated
            </li>
            <li>
              <strong className="text-on-surface">Description</strong> — Detailed explanation of the vulnerability
            </li>
            <li>
              <strong className="text-on-surface">Impact</strong> — Potential consequences if not remediated
            </li>
            <li>
              <strong className="text-on-surface">Remediation</strong> — Specific steps to fix the issue
            </li>
          </ul>
        </section>

        {/* Exporting Reports */}
        <section id="exporting-reports">
          <h2 className="font-fraunces text-2xl font-[600] text-on-surface mt-12 mb-4 scroll-mt-24">
            Exporting Reports
          </h2>
          <p className="text-body-md text-on-surface-variant mb-4 leading-relaxed">
            Sentri generates reports in three formats to suit different workflows:
          </p>

          <h3 className="font-fraunces text-lg font-[600] text-on-surface mt-6 mb-3">
            PDF Format
          </h3>
          <p className="text-body-md text-on-surface-variant mb-4 leading-relaxed">
            Professional, printable report ideal for sharing with auditors and stakeholders. Default output format.
          </p>

          <h3 className="font-fraunces text-lg font-[600] text-on-surface mt-6 mb-3">
            JSON Format
          </h3>
          <p className="text-body-md text-on-surface-variant mb-4 leading-relaxed">
            Machine-readable format for CI/CD integration, report aggregation, and custom tooling. Use in automated pipelines to gate deployments.
          </p>

          <h3 className="font-fraunces text-lg font-[600] text-on-surface mt-6 mb-3">
            HTML Format
          </h3>
          <p className="text-body-md text-on-surface-variant mb-4 leading-relaxed">
            Interactive report viewable in any browser. Includes searchable findings and collapsible sections for easy navigation.
          </p>
        </section>
      </article>
    </DocsShell>
  )
}
