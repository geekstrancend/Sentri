'use client'

import { X, Download, BarChart3 } from 'lucide-react'
import { Button } from './Button'
import { SeverityBadge } from './SeverityBadge'

interface SampleReportModalProps {
  isOpen: boolean
  onClose: () => void
}

export function SampleReportModal({ isOpen, onClose }: SampleReportModalProps) {
  if (!isOpen) return null

  return (
    <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50 p-4">
      <div className="bg-surface rounded-xl shadow-2xl w-full max-w-2xl max-h-[90vh] overflow-y-auto">
        {/* Close button */}
        <div className="sticky top-0 flex justify-between items-center px-6 py-4 border-b border-outline-variant bg-surface">
          <div className="flex items-center gap-2">
            <BarChart3 className="w-5 h-5 text-secondary" />
            <h2 className="text-xl font-bold text-on-surface">Sample Audit Report</h2>
          </div>
          <button
            onClick={onClose}
            className="p-1 hover:bg-outline-variant rounded-lg transition"
          >
            <X className="w-5 h-5 text-on-surface" />
          </button>
        </div>

        <div className="px-6 py-6 space-y-6">
          {/* Report Header */}
          <div>
            <div className="flex justify-between items-start mb-4">
              <div>
                <p className="text-label-sm text-outline mb-2">SAMPLE AUDIT</p>
                <h3 className="font-fraunces text-2xl font-[600] text-on-surface">
                  VaultToken ERC-4626
                </h3>
              </div>
              <div className="text-right text-xs text-on-surface-variant">
                <p>Contract Version 2.1.0</p>
                <p>Scan Date: Dec 15, 2024</p>
              </div>
            </div>
          </div>

          {/* Severity Grid */}
          <div className="grid grid-cols-4 gap-2 bg-surface-container rounded-lg p-4">
            <div className="text-center">
              <div className="font-fraunces text-3xl font-[600] text-critical mb-1">3</div>
              <div className="text-label-xs text-outline">CRITICAL</div>
            </div>
            <div className="text-center">
              <div className="font-fraunces text-3xl font-[600] text-high mb-1">5</div>
              <div className="text-label-xs text-outline">HIGH</div>
            </div>
            <div className="text-center">
              <div className="font-fraunces text-3xl font-[600] text-medium mb-1">8</div>
              <div className="text-label-xs text-outline">MEDIUM</div>
            </div>
            <div className="text-center">
              <div className="font-fraunces text-3xl font-[600] text-low mb-1">12</div>
              <div className="text-label-xs text-outline">LOW</div>
            </div>
          </div>

          {/* Key Findings */}
          <div>
            <h4 className="font-fraunces text-lg font-[600] text-on-surface mb-3">Key Findings</h4>
            <div className="space-y-3">
              {[
                {
                  severity: 'critical',
                  title: 'Reentrancy in deposit()',
                  description: 'The contract transfers funds before updating state, allowing reentrancy attacks.',
                },
                {
                  severity: 'high',
                  title: 'Unchecked external call',
                  description: 'No validation of return value from underlying token transfer.',
                },
                {
                  severity: 'medium',
                  title: 'Integer overflow potential',
                  description: 'totalSupply calculation could overflow in extreme scenarios.',
                },
              ].map((finding, idx) => (
                <div key={idx} className="flex gap-3 p-3 bg-surface-container rounded-lg">
                  <div className="flex-shrink-0 pt-1">
                    <SeverityBadge level={finding.severity as any} />
                  </div>
                  <div>
                    <h5 className="font-[600] text-on-surface mb-1">{finding.title}</h5>
                    <p className="text-sm text-on-surface-variant">{finding.description}</p>
                  </div>
                </div>
              ))}
            </div>
          </div>

          {/* Recommendations */}
          <div>
            <h4 className="font-fraunces text-lg font-[600] text-on-surface mb-3">Recommendations</h4>
            <div className="space-y-2 text-sm text-on-surface-variant">
              <div className="flex gap-2">
                <span className="text-secondary flex-shrink-0">✓</span>
                <p>Use Checks-Effects-Interactions pattern to prevent reentrancy</p>
              </div>
              <div className="flex gap-2">
                <span className="text-secondary flex-shrink-0">✓</span>
                <p>Implement OpenZeppelin's ReentrancyGuard for additional protection</p>
              </div>
              <div className="flex gap-2">
                <span className="text-secondary flex-shrink-0">✓</span>
                <p>Add require() statements to validate all external call return values</p>
              </div>
            </div>
          </div>

          {/* Scan Details */}
          <div className="bg-surface-container rounded-lg p-4">
            <h4 className="text-sm font-[600] text-on-surface mb-3">Scan Details</h4>
            <div className="grid grid-cols-2 gap-4 text-xs">
              <div>
                <span className="text-on-surface-variant">Lines Analyzed</span>
                <p className="font-[600] text-on-surface">2,847</p>
              </div>
              <div>
                <span className="text-on-surface-variant">Functions Checked</span>
                <p className="font-[600] text-on-surface">34</p>
              </div>
              <div>
                <span className="text-on-surface-variant">Invariants Applied</span>
                <p className="font-[600] text-on-surface">1,402</p>
              </div>
              <div>
                <span className="text-on-surface-variant">Scan Duration</span>
                <p className="font-[600] text-on-surface">12.3 seconds</p>
              </div>
            </div>
          </div>

          {/* Action Buttons */}
          <div className="flex gap-3 pt-4 border-t border-outline-variant">
            <Button variant="secondary" fullWidth className="gap-2">
              <Download size={16} />
              Download Full PDF
            </Button>
            <Button variant="primary" fullWidth onClick={onClose}>
              Close
            </Button>
          </div>
        </div>
      </div>
    </div>
  )
}
