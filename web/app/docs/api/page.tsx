'use client'

import { DocsShell } from '@/components/layout/DocsShell'
import { CodeBlock } from '@/components/ui/CodeBlock'

export default function APIReferencePage() {
  const toc = [
    { label: 'Authentication', href: '#authentication' },
    { label: 'POST /v1/scans', href: '#post-scans' },
    { label: 'GET /v1/scans/:id', href: '#get-scan' },
    { label: 'Rate Limits', href: '#rate-limits' },
  ]

  return (
    <DocsShell pageTitle="API Reference" tableOfContents={toc}>
      <article className="space-y-12">
        <div>
          <h1 className="font-fraunces text-5xl font-[600] text-on-surface mb-4">
            REST API Reference
          </h1>
          <p className="text-body-lg text-outline max-w-2xl">
            Integrate Sentri programmatically with the REST API for headless security scanning and report generation.
          </p>
        </div>

        {/* Authentication */}
        <section id="authentication">
          <h2 className="font-fraunces text-2xl font-[600] text-on-surface mt-12 mb-4 scroll-mt-24">
            Authentication
          </h2>
          <p className="text-body-md text-on-surface-variant mb-4 leading-relaxed">
            All API requests require authentication using an API key passed in the <code className="bg-surface-container border border-outline-variant px-1.5 py-0.5 rounded font-mono text-xs">Authorization</code> header:
          </p>

          <CodeBlock
            language="bash"
            code={`Authorization: Bearer YOUR_API_KEY`}
          />

          <h3 className="font-fraunces text-lg font-[600] text-on-surface mt-6 mb-3">
            Obtaining an API Key
          </h3>
          <p className="text-body-md text-on-surface-variant mb-4 leading-relaxed">
            Generate API keys in your Sentri dashboard under Settings → API Keys. Keep them secure and rotate regularly.
          </p>
        </section>

        {/* POST /v1/scans */}
        <section id="post-scans">
          <h2 className="font-fraunces text-2xl font-[600] text-on-surface mt-12 mb-4 scroll-mt-24">
            POST /v1/scans
          </h2>
          <p className="text-body-md text-on-surface-variant mb-4 leading-relaxed">
            Submit a smart contract for security scanning.
          </p>

          <h3 className="font-fraunces text-lg font-[600] text-on-surface mt-6 mb-3">
            Request
          </h3>
          <CodeBlock
            language="json"
            code={`POST /v1/scans
Authorization: Bearer YOUR_API_KEY
Content-Type: application/json

{
  "name": "My Protocol",
  "chain": "evm",
  "contract_code": "pragma solidity ^0.8.0;\\n...",
  "format": "json"
}`}
          />

          <h3 className="font-fraunces text-lg font-[600] text-on-surface mt-6 mb-3">
            Request Fields
          </h3>
          <div className="overflow-x-auto mt-4">
            <table className="w-full border border-outline-variant rounded text-body-md">
              <thead>
                <tr className="border-b border-outline-variant">
                  <th className="text-left p-3 bg-surface-container-low font-[600] text-on-surface">Field</th>
                  <th className="text-left p-3 bg-surface-container-low font-[600] text-on-surface">Type</th>
                  <th className="text-left p-3 bg-surface-container-low font-[600] text-on-surface">Description</th>
                </tr>
              </thead>
              <tbody>
                <tr className="border-b border-outline-variant">
                  <td className="p-3"><code className="font-mono text-xs">name</code></td>
                  <td className="p-3"><code className="font-mono text-xs">string</code></td>
                  <td className="p-3 text-on-surface-variant">Human-readable scan name</td>
                </tr>
                <tr className="border-b border-outline-variant">
                  <td className="p-3"><code className="font-mono text-xs">chain</code></td>
                  <td className="p-3"><code className="font-mono text-xs">string</code></td>
                  <td className="p-3 text-on-surface-variant">evm, solana, or move</td>
                </tr>
                <tr className="border-b border-outline-variant">
                  <td className="p-3"><code className="font-mono text-xs">contract_code</code></td>
                  <td className="p-3"><code className="font-mono text-xs">string</code></td>
                  <td className="p-3 text-on-surface-variant">Full source code of contract</td>
                </tr>
                <tr>
                  <td className="p-3"><code className="font-mono text-xs">format</code></td>
                  <td className="p-3"><code className="font-mono text-xs">string</code></td>
                  <td className="p-3 text-on-surface-variant">json, html, or pdf (default: json)</td>
                </tr>
              </tbody>
            </table>
          </div>

          <h3 className="font-fraunces text-lg font-[600] text-on-surface mt-6 mb-3">
            Response (202 Accepted)
          </h3>
          <CodeBlock
            language="json"
            code={`{
  "scan_id": "scan_1a2b3c4d5e6f7g8h",
  "status": "pending",
  "created_at": "2024-06-15T10:30:00Z",
  "polling_url": "/v1/scans/scan_1a2b3c4d5e6f7g8h"
}`}
          />
        </section>

        {/* GET /v1/scans/:id */}
        <section id="get-scan">
          <h2 className="font-fraunces text-2xl font-[600] text-on-surface mt-12 mb-4 scroll-mt-24">
            GET /v1/scans/:id
          </h2>
          <p className="text-body-md text-on-surface-variant mb-4 leading-relaxed">
            Poll for scan results. Use this to check if the scan is complete and retrieve the report.
          </p>

          <h3 className="font-fraunces text-lg font-[600] text-on-surface mt-6 mb-3">
            Request
          </h3>
          <CodeBlock
            language="bash"
            code={`GET /v1/scans/scan_1a2b3c4d5e6f7g8h
Authorization: Bearer YOUR_API_KEY`}
          />

          <h3 className="font-fraunces text-lg font-[600] text-on-surface mt-6 mb-3">
            Response — In Progress (200 OK)
          </h3>
          <CodeBlock
            language="json"
            code={`{
  "scan_id": "scan_1a2b3c4d5e6f7g8h",
  "status": "scanning",
  "created_at": "2024-06-15T10:30:00Z",
  "progress": 65
}`}
          />

          <h3 className="font-fraunces text-lg font-[600] text-on-surface mt-6 mb-3">
            Response — Complete (200 OK)
          </h3>
          <CodeBlock
            language="json"
            code={`{
  "scan_id": "scan_1a2b3c4d5e6f7g8h",
  "status": "complete",
  "created_at": "2024-06-15T10:30:00Z",
  "completed_at": "2024-06-15T10:45:23Z",
  "findings": {
    "critical": 2,
    "high": 5,
    "medium": 3,
    "low": 1,
    "info": 0
  },
  "report_url": "/v1/scans/scan_1a2b3c4d5e6f7g8h/report",
  "report_format": "json"
}`}
          />

          <h3 className="font-fraunces text-lg font-[600] text-on-surface mt-6 mb-3">
            Response — Error (400/500)
          </h3>
          <CodeBlock
            language="json"
            code={`{
  "scan_id": "scan_1a2b3c4d5e6f7g8h",
  "status": "error",
  "error": "Invalid contract syntax",
  "error_details": "Unexpected token at line 15, column 8"
}`}
          />

          <h3 className="font-fraunces text-lg font-[600] text-on-surface mt-6 mb-3">
            Polling Recommendations
          </h3>
          <ul className="list-disc list-inside space-y-2 text-body-md text-on-surface-variant">
            <li>Poll every 2-3 seconds for typical scans (5-15 minutes)</li>
            <li>Implement exponential backoff after 10 failed polls</li>
            <li>Set a maximum timeout of 30 minutes per scan</li>
            <li>Store scan_id for auditing and historical analysis</li>
          </ul>
        </section>

        {/* Rate Limits */}
        <section id="rate-limits">
          <h2 className="font-fraunces text-2xl font-[600] text-on-surface mt-12 mb-4 scroll-mt-24">
            Rate Limits
          </h2>
          <p className="text-body-md text-on-surface-variant mb-4 leading-relaxed">
            API requests are rate limited per API key to ensure fair service availability:
          </p>

          <div className="overflow-x-auto mt-4">
            <table className="w-full border border-outline-variant rounded text-body-md">
              <thead>
                <tr className="border-b border-outline-variant">
                  <th className="text-left p-3 bg-surface-container-low font-[600] text-on-surface">Plan</th>
                  <th className="text-left p-3 bg-surface-container-low font-[600] text-on-surface">Requests/Hour</th>
                  <th className="text-left p-3 bg-surface-container-low font-[600] text-on-surface">Concurrent Scans</th>
                </tr>
              </thead>
              <tbody>
                <tr className="border-b border-outline-variant">
                  <td className="p-3">Starter</td>
                  <td className="p-3 text-on-surface-variant">10</td>
                  <td className="p-3 text-on-surface-variant">1</td>
                </tr>
                <tr className="border-b border-outline-variant">
                  <td className="p-3">Pro</td>
                  <td className="p-3 text-on-surface-variant">100</td>
                  <td className="p-3 text-on-surface-variant">5</td>
                </tr>
                <tr>
                  <td className="p-3">Enterprise</td>
                  <td className="p-3 text-on-surface-variant">Unlimited</td>
                  <td className="p-3 text-on-surface-variant">Unlimited</td>
                </tr>
              </tbody>
            </table>
          </div>

          <h3 className="font-fraunces text-lg font-[600] text-on-surface mt-6 mb-3">
            Rate Limit Headers
          </h3>
          <p className="text-body-md text-on-surface-variant mb-4 leading-relaxed">
            Every response includes rate limit information:
          </p>
          <CodeBlock
            language="text"
            code={`X-RateLimit-Limit: 100
X-RateLimit-Remaining: 87
X-RateLimit-Reset: 1623771600`}
          />
        </section>
      </article>
    </DocsShell>
  )
}
