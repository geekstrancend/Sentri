import { ReactNode } from 'react'
import clsx from 'clsx'
import { Loader2 } from 'lucide-react'

type ButtonVariant = 'primary' | 'secondary' | 'ghost' | 'signal'

interface ButtonProps extends React.ButtonHTMLAttributes<HTMLButtonElement> {
  variant?: ButtonVariant
  size?: 'sm' | 'md' | 'lg'
  children: ReactNode
  icon?: ReactNode
  iconPosition?: 'left' | 'right'
  fullWidth?: boolean
  loading?: boolean
}

const variantStyles: Record<ButtonVariant, string> = {
  // Indigo brand action — the single primary CTA per view.
  primary:
    'bg-indigo text-surface-container-lowest font-[600] border border-transparent hover:bg-indigo-bright hover:shadow-glow active:translate-y-px',
  // Quiet, bordered — secondary actions.
  secondary:
    'bg-surface-container-low/60 border border-outline-variant text-on-surface hover:border-indigo/60 hover:bg-surface-container active:translate-y-px',
  // Text-only — tertiary.
  ghost: 'bg-transparent border border-transparent text-on-surface-variant hover:text-on-surface hover:bg-surface-container-low',
  // Green "signal" — success/run/verified affirmative actions.
  signal:
    'bg-signal text-surface-container-lowest font-[600] border border-transparent hover:bg-signal-bright hover:shadow-glow-signal active:translate-y-px',
}

const sizeStyles: Record<string, string> = {
  sm: 'px-3 py-1.5 text-[0.8125rem] rounded-lg gap-1.5',
  md: 'px-4 py-2.5 text-sm rounded-lg gap-2',
  lg: 'px-6 py-3 text-[0.9375rem] rounded-xl gap-2',
}

export function Button({
  variant = 'primary',
  size = 'md',
  children,
  icon,
  iconPosition = 'left',
  fullWidth = false,
  loading = false,
  disabled,
  className,
  ...props
}: ButtonProps) {
  const iconEl = loading ? <Loader2 size={16} className="animate-spin" /> : icon
  const showLeft = iconEl && (iconPosition === 'left' || loading)
  const showRight = icon && iconPosition === 'right' && !loading

  return (
    <button
      className={clsx(
        'relative inline-flex items-center justify-center whitespace-nowrap font-medium',
        'transition-[background-color,border-color,color,box-shadow,transform] duration-150 ease-out',
        'focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo',
        'disabled:opacity-40 disabled:cursor-not-allowed disabled:pointer-events-none disabled:shadow-none',
        variantStyles[variant],
        sizeStyles[size],
        fullWidth && 'w-full',
        className,
      )}
      disabled={disabled || loading}
      aria-busy={loading || undefined}
      {...props}
    >
      {showLeft && <span className="flex items-center">{iconEl}</span>}
      {children}
      {showRight && <span className="flex items-center">{icon}</span>}
    </button>
  )
}
