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

  const colorMap: Record<SeverityLevel, { bg: string; border: string; text: string }> = {
    critical: {
      bg: 'rgba(239, 68, 68, 0.12)',
      border: 'rgba(239, 68, 68, 0.4)',
      text: '#EF4444',
    },
    high: {
      bg: 'rgba(251, 191, 36, 0.12)',
      border: 'rgba(251, 191, 36, 0.4)',
      text: '#FBBF24',
    },
    medium: {
      bg: 'rgba(129, 140, 248, 0.12)',
      border: 'rgba(129, 140, 248, 0.4)',
      text: '#818CF8',
    },
    low: {
      bg: 'rgba(74, 222, 128, 0.12)',
      border: 'rgba(74, 222, 128, 0.4)',
      text: '#4ADE80',
    },
  }

  const colors = colorMap[level]

  return (
    <span
      className={clsx(
        'text-label-sm px-2 py-0.5 border rounded whitespace-nowrap inline-block',
        className,
      )}
      style={{
        backgroundColor: colors.bg,
        borderColor: colors.border,
        color: colors.text,
      }}
    >
      {displayText}
    </span>
  )
}
