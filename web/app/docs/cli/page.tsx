'use client'

import { DocsShell } from '@/components/layout/DocsShell'
import { CodeBlock } from '@/components/ui/CodeBlock'

export default function CLIReferencePage() {
  const toc = [
    { label: 'sentri check', href: '#sentri-check' },
    { label: 'Exit Codes', href: '#exit-codes' },
    { label: 'Examples', href: '#examples' },
  ]

  return (
    <DocsShell pageTitle="CLI Reference" tableOfContents={toc}>
      <article className="space-y-12">
        <div>
          <h1 className="font-fraunces text-5xl font-[600] text-on-surface mb-4">
            CLI Reference
          </h1>
          <p className="text-body-lg text-outline max-w-2xl">
            Complete documentation for the Sentri command-line interface and all available commands.
          </p>
        </div>

        {/* sentri check */}
        <section id="sentri-check">
          <h2 className="font-fraunces text-2xl font-[600] text-on-surface mt-12 mb-4 scroll-mt-24">
            sentri check
          </h2>
          <p className="text-body-md text-on-surface-variant mb-4 leading-relaxed">
            Scan a smart contract directory or file for security vulnerabilities and invariant violations.
          </p>

          <h3 className="font-fraunces text-lg font-[600] text-on-surface mt-6 mb-3">
            Usage
          </h3>
          <CodeBlock
            language="bash"
            code={`sentri check [PATH] [OPTIONS]`}
          />

          <h3 className="font-fraunces text-lg font-[600] text-on-surface mt-6 mb-3">
            Arguments
          </h3>
          <table className="w-full border border-outline-variant rounded text-body-md mt-4">
            <thead>
              <tr className="border-b border-outline-variant">
                <th className="text-left p-3 bg-surface-container-low font-[600] text-on-surface">Argument</th>
                <th className="text-left p-3 bg-surface-container-low font-[600] text-on-surface">Description</th>
                <th className="text-left p-3 bg-surface-container-low font-[600] text-on-surface">Default</th>
              </tr>
            </thead>
            <tbody>
              <tr className="border-b border-outline-variant">
                <td className="p-3"><code className="font-mono text-xs">PATH</code></td>
                <td className="p-3 text-on-surface-variant">Directory or file to scan</td>
                <td className="p-3 text-on-surface-variant">.</td>
              </tr>
            </tbody>
          </table>

          <h3 className="font-fraunces text-lg font-[600] text-on-surface mt-6 mb-3">
            Options
          </h3>
          <table className="w-full border border-outline-variant rounded text-body-md mt-4">
            <thead>
              <tr className="border-b border-outline-variant">
                <th className="text-left p-3 bg-surface-container-low font-[600] text-on-surface">Flag</th>
                <th className="text-left p-3 bg-surface-container-low font-[600] text-on-surface">Description</th>
                <th className="text-left p-3 bg-surface-container-low font-[600] text-on-surface">Default</th>
              </tr>
            </thead>
            <tbody>
              <tr className="border-b border-outline-variant">
                <td className="p-3"><code className="font-mono text-xs">--chain</code></td>
                <td className="p-3 text-on-surface-variant">Blockchain target: evm, solana, move</td>
                <td className="p-3 text-on-surface-variant">evm</td>
              </tr>
              <tr className="border-b border-outline-variant">
                <td className="p-3"><code className="font-mono text-xs">--format</code></td>
                <td className="p-3 text-on-surface-variant">Output format: json, html, pdf</td>
                <td className="p-3 text-on-surface-variant">pdf</td>
              </tr>
              <tr className="border-b border-outline-variant">
                <td className="p-3"><code className="font-mono text-xs">--output</code></td>
                <td className="p-3 text-on-surface-variant">Output file path</td>
                <td className="p-3 text-on-surface-variant">report.pdf</td>
              </tr>
              <tr className="border-b border-outline-variant">
                <td className="p-3"><code className="font-mono text-xs">--seed</code></td>
                <td className="p-3 text-on-surface-variant">Reproducible randomness seed</td>
                <td className="p-3 text-on-surface-variant">auto</td>
              </tr>
              <tr>
                <td className="p-3"><code className="font-mono text-xs">--verbose</code></td>
                <td className="p-3 text-on-surface-variant">Enable verbose logging</td>
                <td className="p-3 text-on-surface-variant">false</td>
              </tr>
            </tbody>
          </table>
        </section>

        {/* Exit Codes */}
        <section id="exit-codes">
          <h2 className="font-fraunces text-2xl font-[600] text-on-surface mt-12 mb-4 scroll-mt-24">
            Exit Codes
          </h2>
          <p className="text-body-md text-on-surface-variant mb-4 leading-relaxed">
            Sentri uses exit codes to indicate scan results. Use these in CI/CD pipelines to gate deployments.
          </p>

          <table className="w-full border border-outline-variant rounded text-body-md mt-4">
            <thead>
              <tr className="border-b border-outline-variant">
                <th className="text-left p-3 bg-surface-container-low font-[600] text-on-surface">Code</th>
                <th className="text-left p-3 bg-surface-container-low font-[600] text-on-surface">Meaning</th>
              </tr>
            </thead>
            <tbody>
              <tr className="border-b border-outline-variant">
                <td className="p-3"><span className="text-low font-[600]">0</span></td>
                <td className="p-3 text-on-surface-variant">Scan completed successfully, no critical/high findings</td>
              </tr>
              <tr className="border-b border-outline-variant">
                <td className="p-3"><span className="text-medium font-[600]">1</span></td>
                <td className="p-3 text-on-surface-variant">Scan found medium or low severity findings</td>
              </tr>
              <tr className="border-b border-outline-variant">
                <td className="p-3"><span className="text-high font-[600]">2</span></td>
                <td className="p-3 text-on-surface-variant">Scan found high severity findings</td>
              </tr>
              <tr>
                <td className="p-3"><span className="text-critical font-[600]">3</span></td>
                <td className="p-3 text-on-surface-variant">Scan found critical findings or scan failed</td>
              </tr>
            </tbody>
          </table>
        </section>

        {/* Examples */}
        <section id="examples">
          <h2 className="font-fraunces text-2xl font-[600] text-on-surface mt-12 mb-4 scroll-mt-24">
            Examples
          </h2>

          <h3 className="font-fraunces text-lg font-[600] text-on-surface mt-6 mb-3">
            Scan current directory for EVM contracts
          </h3>
          <CodeBlock
            language="bash"
            code={`sentri check . --chain evm`}
          />

          <h3 className="font-fraunces text-lg font-[600] text-on-surface mt-6 mb-3">
            Generate JSON report to specific file
          </h3>
          <CodeBlock
            language="bash"
            code={`sentri check ./contracts --chain evm --format json --output security-report.json`}
          />

          <h3 className="font-fraunces text-lg font-[600] text-on-surface mt-6 mb-3">
            Scan Solana project with reproducible seed
          </h3>
          <CodeBlock
            language="bash"
            code={`sentri check . --chain solana --seed 42`}
          />

          <h3 className="font-fraunces text-lg font-[600] text-on-surface mt-6 mb-3">
            CI/CD gating example
          </h3>
          <CodeBlock
            language="bash"
            code={`sentri check . --chain evm --format json --output report.json
if [ $? -ge 2 ]; then
  echo "Critical or high findings detected"
  exit 1
fi`}
          />
        </section>
      </article>
    </DocsShell>
  )
}
