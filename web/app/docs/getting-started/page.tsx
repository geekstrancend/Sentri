'use client'

import { DocsShell } from '@/components/layout/DocsShell'
import { CodeBlock } from '@/components/ui/CodeBlock'

export default function GettingStartedPage() {
  const toc = [
    { label: 'Installation', href: '#installation' },
    { label: 'Running Your First Scan', href: '#first-scan' },
    { label: 'Understanding the Output', href: '#understanding-output' },
    { label: 'Next Steps', href: '#next-steps' },
  ]

  return (
    <DocsShell pageTitle="Getting Started" tableOfContents={toc}>
      <article className="space-y-12">
        <div>
          <h1 className="font-fraunces text-5xl font-[600] text-on-surface mb-4">
            Getting Started
          </h1>
          <p className="text-body-lg text-outline max-w-2xl">
            Install Sentri and run your first security scan in minutes. This guide will walk you through the essential steps.
          </p>
        </div>

        {/* Installation */}
        <section id="installation">
          <h2 className="font-fraunces text-2xl font-[600] text-on-surface mt-12 mb-4 scroll-mt-24">
            Installation
          </h2>
          <p className="text-body-md text-on-surface-variant mb-4 leading-relaxed">
            Sentri provides two installation methods: Rust CLI for local scanning and npm package for JavaScript integration.
          </p>

          <h3 className="font-fraunces text-lg font-[600] text-on-surface mt-6 mb-3">
            Rust CLI (Recommended)
          </h3>
          <p className="text-body-md text-on-surface-variant mb-4 leading-relaxed">
            Install the Sentri CLI via Cargo:
          </p>
          <CodeBlock
            language="bash"
            code={`cargo install sentri-cli`}
          />
          <p className="text-body-sm text-outline-variant mt-2">
            Requires Rust 1.70 or later. Install Rust at <a href="https://rustup.rs" target="_blank" rel="noopener" className="text-indigo hover:text-indigo/80">rustup.rs</a>.
          </p>

          <h3 className="font-fraunces text-lg font-[600] text-on-surface mt-6 mb-3">
            npm Package
          </h3>
          <p className="text-body-md text-on-surface-variant mb-4 leading-relaxed">
            For JavaScript/TypeScript projects:
          </p>
          <CodeBlock
            language="bash"
            code={`npm install -g @dextonicx/cli`}
          />
        </section>

        {/* First Scan */}
        <section id="first-scan">
          <h2 className="font-fraunces text-2xl font-[600] text-on-surface mt-12 mb-4 scroll-mt-24">
            Running Your First Scan
          </h2>
          <p className="text-body-md text-on-surface-variant mb-4 leading-relaxed">
            Once installed, run Sentri on a smart contract directory:
          </p>
          <CodeBlock
            language="bash"
            code={`sentri check . --chain evm`}
          />
          <p className="text-body-md text-on-surface-variant mb-4 leading-relaxed mt-4">
            This command scans all Solidity files in the current directory using the EVM analyzer. Replace <code className="bg-surface-container border border-outline-variant px-1.5 py-0.5 rounded font-mono text-xs">evm</code> with <code className="bg-surface-container border border-outline-variant px-1.5 py-0.5 rounded font-mono text-xs">solana</code> for Rust programs or <code className="bg-surface-container border border-outline-variant px-1.5 py-0.5 rounded font-mono text-xs">move</code> for Move modules.
          </p>

          <h3 className="font-fraunces text-lg font-[600] text-on-surface mt-6 mb-3">
            Common Options
          </h3>
          <table className="w-full border border-outline-variant rounded text-body-md mt-4">
            <thead>
              <tr className="border-b border-outline-variant">
                <th className="text-left p-3 bg-surface-container-low font-[600] text-on-surface">Flag</th>
                <th className="text-left p-3 bg-surface-container-low font-[600] text-on-surface">Description</th>
              </tr>
            </thead>
            <tbody>
              <tr className="border-b border-outline-variant">
                <td className="p-3"><code className="font-mono text-xs">--chain</code></td>
                <td className="p-3 text-on-surface-variant">Blockchain: evm, solana, move</td>
              </tr>
              <tr className="border-b border-outline-variant">
                <td className="p-3"><code className="font-mono text-xs">--format</code></td>
                <td className="p-3 text-on-surface-variant">Output format: json, html, pdf (default: pdf)</td>
              </tr>
              <tr className="border-b border-outline-variant">
                <td className="p-3"><code className="font-mono text-xs">--output</code></td>
                <td className="p-3 text-on-surface-variant">Output file path</td>
              </tr>
              <tr>
                <td className="p-3"><code className="font-mono text-xs">--seed</code></td>
                <td className="p-3 text-on-surface-variant">Random seed for reproducible results</td>
              </tr>
            </tbody>
          </table>
        </section>

        {/* Understanding Output */}
        <section id="understanding-output">
          <h2 className="font-fraunces text-2xl font-[600] text-on-surface mt-12 mb-4 scroll-mt-24">
            Understanding the Output
          </h2>
          <p className="text-body-md text-on-surface-variant mb-4 leading-relaxed">
            Sentri reports findings using a standard severity classification system. Each finding is categorized by impact level:
          </p>

          <h3 className="font-fraunces text-lg font-[600] text-on-surface mt-6 mb-3">
            Severity Levels
          </h3>
          <table className="w-full border border-outline-variant rounded text-body-md mt-4">
            <thead>
              <tr className="border-b border-outline-variant">
                <th className="text-left p-3 bg-surface-container-low font-[600] text-on-surface">Severity</th>
                <th className="text-left p-3 bg-surface-container-low font-[600] text-on-surface">Description</th>
              </tr>
            </thead>
            <tbody>
              <tr className="border-b border-outline-variant">
                <td className="p-3"><span className="text-critical font-[600]">Critical</span></td>
                <td className="p-3 text-on-surface-variant">Immediate fund loss or protocol compromise possible</td>
              </tr>
              <tr className="border-b border-outline-variant">
                <td className="p-3"><span className="text-high font-[600]">High</span></td>
                <td className="p-3 text-on-surface-variant">Significant vulnerability requiring urgent remediation</td>
              </tr>
              <tr className="border-b border-outline-variant">
                <td className="p-3"><span className="text-medium font-[600]">Medium</span></td>
                <td className="p-3 text-on-surface-variant">Notable issue that should be addressed before deployment</td>
              </tr>
              <tr className="border-b border-outline-variant">
                <td className="p-3"><span className="text-low font-[600]">Low</span></td>
                <td className="p-3 text-on-surface-variant">Minor issue or optimization opportunity</td>
              </tr>
              <tr>
                <td className="p-3"><span className="text-outline font-[600]">Info</span></td>
                <td className="p-3 text-on-surface-variant">Informational finding or best practice recommendation</td>
              </tr>
            </tbody>
          </table>
        </section>

        {/* Next Steps */}
        <section id="next-steps">
          <h2 className="font-fraunces text-2xl font-[600] text-on-surface mt-12 mb-4 scroll-mt-24">
            Next Steps
          </h2>
          <p className="text-body-md text-on-surface-variant mb-6 leading-relaxed">
            Now that you've run your first scan, explore the full capabilities of Sentri:
          </p>
          <ul className="list-disc list-inside space-y-3 text-body-md text-on-surface-variant">
            <li>
              <strong className="text-on-surface">CLI Reference</strong> — Deep dive into all available commands and flags
            </li>
            <li>
              <strong className="text-on-surface">Invariant Library</strong> — Browse the 1,400+ built-in security checks
            </li>
            <li>
              <strong className="text-on-surface">Audit Report Guide</strong> — Learn how to interpret and share reports
            </li>
            <li>
              <strong className="text-on-surface">CI/CD Integration</strong> — Automate security scanning in your pipeline
            </li>
          </ul>
        </section>
      </article>
    </DocsShell>
  )
}
