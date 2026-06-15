interface AsciiLogoProps {
  className?: string
  glow?: boolean
}

const LOGO = `  ███████╗███████╗███╗   ██╗████████╗██████╗ ██╗
  ██╔════╝██╔════╝████╗  ██║╚══██╔══╝██╔══██╗██║
  ███████╗█████╗  ██╔██╗ ██║   ██║   ██████╔╝██║
  ╚════██║██╔══╝  ██║╚██╗██║   ██║   ██╔══██╗██║
  ███████║███████╗██║ ╚████║   ██║   ██║  ██║██║
  ╚══════╝╚══════╝╚═╝  ╚═══╝   ╚═╝   ╚═╝  ╚═╝╚═╝`

export function AsciiLogo({ className, glow = false }: AsciiLogoProps) {
  const baseClasses = 'font-mono leading-[1.05] whitespace-pre select-none'
  const colorClasses = glow ? 'text-secondary drop-shadow-[0_0_24px_var(--secondary)]' : 'text-secondary'
  const finalClassName = `${baseClasses} ${colorClasses} ${className || ''}`

  return (
    <pre
      className={finalClassName}
      aria-label="Sentri"
      role="img"
    >
      {LOGO}
    </pre>
  )
}
