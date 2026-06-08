'use client'

interface TerminalProps {
  title?: string
  output: Array<{
    prefix?: string
    text: string
    type?: 'info' | 'scan' | 'critical' | 'high' | 'done'
  }>
}

export function Terminal({ title = 'sentri-cli --scan ./contracts/Vault.sol', output }: TerminalProps) {
  const getPrefixStyles = (type?: string) => {
    const baseStyles = 'inline-block px-1.5 py-0.5 rounded-xs font-[600] text-label-sm'
    
    switch (type) {
      case 'critical':
        return `${baseStyles} bg-critical text-white`
      case 'high':
        return `${baseStyles} bg-high text-surface`
      case 'scan':
      case 'info':
      case 'done':
        return 'text-outline'
      default:
        return ''
    }
  }

  const getTextColor = (type?: string) => {
    switch (type) {
      case 'done':
        return 'text-low'
      default:
        return 'text-outline'
    }
  }

  return (
    <div className="bg-surface-container-lowest border border-outline-variant rounded-lg overflow-hidden">
      {/* Title Bar */}
      <div className="bg-surface-container-low h-8 px-4 flex items-center justify-between border-b border-outline-variant">
        <div className="flex gap-1.5">
          <div className="w-1.5 h-1.5 rounded-full bg-critical" />
          <div className="w-1.5 h-1.5 rounded-full bg-high" />
          <div className="w-1.5 h-1.5 rounded-full bg-low" />
        </div>
        <span className="text-outline text-label-sm text-center flex-1">{title}</span>
        <div className="w-12" />
      </div>

      {/* Content */}
      <div className="p-5 font-mono text-code-block leading-5">
        {output.map((line, idx) => (
          <div key={idx} className="flex gap-2 mb-1">
            {line.prefix && (
              <span className={getPrefixStyles(line.type)}>
                [{line.prefix}]
              </span>
            )}
            <span className={getTextColor(line.type)}>{line.text}</span>
            {idx === output.length - 1 && (
              <span className="animate-blink-cursor">▊</span>
            )}
          </div>
        ))}
      </div>
    </div>
  )
}
