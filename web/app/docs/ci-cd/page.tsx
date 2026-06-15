'use client'

import { DocsShell } from '@/components/layout/DocsShell'
import { CodeBlock } from '@/components/ui/CodeBlock'

export default function CICDPage() {
  const toc = [
    { label: 'GitHub Actions', href: '#github-actions' },
    { label: 'GitLab CI', href: '#gitlab-ci' },
    { label: 'Failing on Findings', href: '#failing-on-findings' },
    { label: 'Uploading Artifacts', href: '#uploading-artifacts' },
  ]

  return (
    <DocsShell pageTitle="CI/CD Integration" tableOfContents={toc}>
      <article className="space-y-12">
        <div>
          <h1 className="font-fraunces text-5xl font-[600] text-on-surface mb-4">
            CI/CD Integration
          </h1>
          <p className="text-body-lg text-outline max-w-2xl">
            Integrate Sentri security scanning into your deployment pipeline to catch vulnerabilities early.
          </p>
        </div>

        <section id="github-actions">
          <h2 className="font-fraunces text-2xl font-[600] text-on-surface mt-12 mb-4 scroll-mt-24">
            GitHub Actions
          </h2>
          <p className="text-body-md text-on-surface-variant mb-4 leading-relaxed">
            Add security scanning to your GitHub workflow:
          </p>
          <CodeBlock language="yaml" code={`name: Sentri Security
on: [push, pull_request]
jobs:
  security:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - run: cargo install sentri-cli
      - run: sentri check . --chain evm --format json`} />
        </section>

        <section id="gitlab-ci">
          <h2 className="font-fraunces text-2xl font-[600] text-on-surface mt-12 mb-4 scroll-mt-24">
            GitLab CI
          </h2>
          <p className="text-body-md text-on-surface-variant mb-4 leading-relaxed">
            Configure your .gitlab-ci.yml:
          </p>
          <CodeBlock language="yaml" code={`security-scan:
  image: rust:latest
  script:
    - cargo install sentri-cli
    - sentri check . --chain evm --format json`} />
        </section>

        <section id="failing-on-findings">
          <h2 className="font-fraunces text-2xl font-[600] text-on-surface mt-12 mb-4 scroll-mt-24">
            Failing on Findings
          </h2>
          <p className="text-body-md text-on-surface-variant mb-4 leading-relaxed">
            Use exit codes to gate deployments:
          </p>
          <table className="w-full border border-outline-variant rounded text-body-md mt-4">
            <thead>
              <tr className="border-b border-outline-variant">
                <th className="text-left p-3 bg-surface-container-low font-[600] text-on-surface">Code</th>
                <th className="text-left p-3 bg-surface-container-low font-[600] text-on-surface">Result</th>
              </tr>
            </thead>
            <tbody>
              <tr className="border-b border-outline-variant">
                <td className="p-3">0</td>
                <td className="p-3 text-on-surface-variant">Success</td>
              </tr>
              <tr className="border-b border-outline-variant">
                <td className="p-3">2</td>
                <td className="p-3 text-on-surface-variant">High findings - fail</td>
              </tr>
              <tr>
                <td className="p-3">3</td>
                <td className="p-3 text-on-surface-variant">Critical - fail</td>
              </tr>
            </tbody>
          </table>
        </section>

        <section id="uploading-artifacts">
          <h2 className="font-fraunces text-2xl font-[600] text-on-surface mt-12 mb-4 scroll-mt-24">
            Uploading Artifacts
          </h2>
          <p className="text-body-md text-on-surface-variant mb-4 leading-relaxed">
            Store reports for audit trails and compliance:
          </p>
          <CodeBlock language="yaml" code={`- uses: actions/upload-artifact@v3
  with:
    name: sentri-report
    path: report.json
    retention-days: 90`} />
        </section>
      </article>
    </DocsShell>
  )
}
