'use client'

import { useState } from 'react'
import { Download, Share2, AlertCircle, CheckCircle } from 'lucide-react'
import { AppShell } from '@/components/layout/AppShell'
import { Button } from '@/components/ui/Button'
import { SeverityBadge } from '@/components/ui/SeverityBadge'

interface Finding {
  id: string
  severity: 'critical' | 'high' | 'medium' | 'low'
  title: string
  description: string
  location: string
  recommendation: string
  codeSnippet?: string
}

export default function ReportDetailPage({ params }: { params: { id: string } }) {
  const reportId = params.id
  const [findings] = useState<Finding[]>([
    {
      id: '1',
      severity: 'critical',
      title: 'Reentrancy Vulnerability',
      description: 'The contract is vulnerable to reentrancy attacks in the withdraw function.',
      location: 'Vault.sol:42',
      recommendation: 'Use checks-effects-interactions pattern or add mutex lock.',
      codeSnippet: `function withdraw(uint256 amount) external {
  uint256 balance = balances[msg.sender];
  require(amount <= balance);
  (bool success, ) = msg.sender.call{value: amount}("");
  require(success);
  balances[msg.sender] -= amount;
}`,
    },
    {
      id: '2',
      severity: 'high',
      title: 'Unchecked Return Value',
      description: 'Return value of transfer call is not checked.',
      location: 'Token.sol:108',
      recommendation: 'Check the return value or use safeTransfer from OpenZeppelin.',
      codeSnippet: `token.transfer(recipient, amount);`,
    },
    {
      id: '3',
      severity: 'medium',
      title: 'Missing Input Validation',
      description: 'Function does not validate user input parameters.',
      location: 'Pool.sol:55',
      recommendation: 'Add require statements to validate all input parameters.',
      codeSnippet: `function setFee(uint256 newFee) external onlyOwner {
  fee = newFee;
}`,
    },
  ])

  const stats = {
    critical: 1,
    high: 1,
    medium: 1,
    low: 0,
  }

  return (
    <AppShell currentPage="audits" onNewScan={() => {}}>
      <div className="max-w-5xl mx-auto p-6 space-y-8">
        {/* Header */}
        <div className="flex items-start justify-between">
          <div>
            <h1 className="text-4xl font-[700] text-on-surface font-fraunces mb-2">
              Security Audit Report
            </h1>
            <p className="text-outline">
              Report ID: <span className="font-mono text-on-surface-variant">{reportId}</span>
            </p>
            <p className="text-outline mt-1">
              Generated: {new Date().toLocaleString()}
            </p>
          </div>
          <div className="flex gap-2">
            <Button variant="secondary" icon={<Share2 size={16} />}>
              Share
            </Button>
            <Button variant="secondary" icon={<Download size={16} />}>
              Download PDF
            </Button>
          </div>
        </div>

        {/* Summary Stats */}
        <div className="grid grid-cols-4 gap-4">
          {[
            { label: 'CRITICAL', count: stats.critical, color: 'bg-critical' },
            { label: 'HIGH', count: stats.high, color: 'bg-high' },
            { label: 'MEDIUM', count: stats.medium, color: 'bg-medium' },
            { label: 'LOW', count: stats.low, color: 'bg-low' },
          ].map((item) => (
            <div
              key={item.label}
              className="bg-surface-container border border-outline-variant rounded-lg p-4 text-center"
            >
              <div className="text-4xl font-[700] text-on-surface mb-2">
                {item.count}
              </div>
              <div className="text-sm text-outline">{item.label}</div>
            </div>
          ))}
        </div>

        {/* Executive Summary */}
        <div className="bg-surface-container-low border border-outline-variant rounded-lg p-6">
          <h2 className="text-2xl font-[700] text-on-surface font-fraunces mb-4">
            Executive Summary
          </h2>
          <p className="text-on-surface-variant leading-relaxed mb-4">
            This security audit identified 3 vulnerabilities in the smart contract, including 1 critical issue
            that requires immediate attention before deployment to mainnet. The critical vulnerability poses a
            significant risk of fund loss through reentrancy attacks.
          </p>
          <div className="bg-critical/10 border border-critical/30 rounded-lg p-4 flex gap-3 items-start">
            <AlertCircle size={20} className="text-critical flex-shrink-0 mt-0.5" />
            <div>
              <h3 className="font-[600] text-critical">Recommendation</h3>
              <p className="text-sm text-on-surface-variant mt-1">
                Do not deploy to mainnet until all critical and high-severity vulnerabilities are resolved.
              </p>
            </div>
          </div>
        </div>

        {/* Detailed Findings */}
        <div className="space-y-6">
          <h2 className="text-2xl font-[700] text-on-surface font-fraunces">
            Detailed Findings
          </h2>

          {findings.map((finding) => (
            <div
              key={finding.id}
              className="bg-surface-container-low border border-outline-variant rounded-lg p-6 space-y-4"
            >
              <div className="flex items-start gap-4">
                <SeverityBadge level={finding.severity} />
                <div className="flex-1">
                  <h3 className="text-lg font-[700] text-on-surface">{finding.title}</h3>
                  <p className="text-sm text-outline-variant mt-1">{finding.location}</p>
                </div>
              </div>

              <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
                <div>
                  <h4 className="font-[600] text-on-surface mb-2">Description</h4>
                  <p className="text-on-surface-variant">{finding.description}</p>
                </div>
                <div>
                  <h4 className="font-[600] text-on-surface mb-2">Recommendation</h4>
                  <p className="text-on-surface-variant">{finding.recommendation}</p>
                </div>
              </div>

              {finding.codeSnippet && (
                <div>
                  <h4 className="font-[600] text-on-surface mb-2">Affected Code</h4>
                  <pre className="bg-surface p-4 rounded-lg border border-outline-variant overflow-x-auto text-xs">
                    <code className="text-on-surface-variant">{finding.codeSnippet}</code>
                  </pre>
                </div>
              )}
            </div>
          ))}
        </div>

        {/* Best Practices */}
        <div className="bg-surface-container-low border border-outline-variant rounded-lg p-6 space-y-4">
          <h2 className="text-2xl font-[700] text-on-surface font-fraunces">
            Best Practices & Recommendations
          </h2>
          <ul className="space-y-3">
            {[
              'Implement comprehensive unit and integration tests',
              'Use established patterns from OpenZeppelin contracts',
              'Consider formal verification for critical functions',
              'Implement event logging for all state changes',
              'Add pause mechanisms for emergency situations',
            ].map((rec, idx) => (
              <li key={idx} className="flex gap-3 items-start">
                <CheckCircle size={20} className="text-low flex-shrink-0 mt-0.5" />
                <span className="text-on-surface-variant">{rec}</span>
              </li>
            ))}
          </ul>
        </div>

        {/* Footer */}
        <div className="border-t border-outline-variant pt-6 text-center text-outline-variant text-sm">
          <p>This audit was performed using Sentri's AI-powered analysis with 1500+ security invariants.</p>
          <p className="mt-2">For questions or clarifications, contact our security team at security@sentri.dev</p>
        </div>
      </div>
    </AppShell>
  )
}
