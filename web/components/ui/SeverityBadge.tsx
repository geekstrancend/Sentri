import clsx from 'clsx'

type SeverityLevel = 'critical' | 'high' | 'medium' | 'low'

interface SeverityBadgeProps {
  level: SeverityLevel
  count?: number
  label?: string
  className?: string
}

const severityShorthand: Record<SeverityLevel, string> = {
  critical: 'C',
  high: 'H',
  medium: 'M',
  low: 'L',
}

const severityLabels: Record<SeverityLevel, string> = {
  critical: 'CRITICAL',
  high: 'HIGH RISK',
  medium: 'MEDIUM',
  low: 'LOW RISK',
}

export function SeverityBadge({
  level,
  count,
  label,
  className,
}: SeverityBadgeProps) {
  const displayText = count !== undefined 
    ? `${count}${severityShorthand[level]}`
    : label || severityLabels[level]

  const colorClasses: Record<SeverityLevel, string> = {
    critical: 'bg-critical-bg border-critical-border text-critical',
    high: 'bg-high-bg border-high-border text-high',
    medium: 'bg-medium-bg border-medium-border text-medium',
    low: 'bg-low-bg border-low-border text-low',
  }

  return (
    <span
      className={clsx(
        'text-label-sm px-2 py-0.5 border rounded whitespace-nowrap inline-block',
        colorClasses[level],
        className,
      )}
    >
      {displayText}
    </span>
  )
}
