'use client'

import { useState } from 'react'
import { Copy, Check } from 'lucide-react'
import clsx from 'clsx'

interface CodeBlockProps {
  code: string
  language?: string
  highlightLines?: number[]
}

export function CodeBlock({ code, language, highlightLines = [] }: CodeBlockProps) {
  const [copied, setCopied] = useState(false)

  const handleCopy = async () => {
    await navigator.clipboard.writeText(code)
    setCopied(true)
    setTimeout(() => setCopied(false), 2000)
  }

  const lines = code.split('\n')

  return (
    <div className="bg-surface-container-lowest border border-outline-variant rounded overflow-hidden">
      {language && (
        <div className="bg-surface-container-low h-9 px-4 flex items-center justify-between border-b border-outline-variant">
          <div className="flex gap-1.5">
            <div className="w-1.5 h-1.5 rounded-full bg-critical" />
            <div className="w-1.5 h-1.5 rounded-full bg-high" />
            <div className="w-1.5 h-1.5 rounded-full bg-low" />
          </div>
          <div className="flex items-center gap-3">
            <span className="text-label-sm bg-surface-container border border-outline-variant px-2 py-0.5 rounded-full text-outline">
              {language}
            </span>
            <button
              onClick={handleCopy}
              className="text-outline hover:text-on-surface transition-colors p-1.5 -m-0.5"
              aria-label={copied ? 'Copied to clipboard' : 'Copy code'}
              title="Copy code"
            >
              {copied ? (
                <Check size={16} />
              ) : (
                <Copy size={16} />
              )}
            </button>
          </div>
        </div>
      )}
      <div className="overflow-x-auto">
        <pre className="p-5 font-mono text-code-block leading-5 text-on-surface-variant">
          {lines.map((line, idx) => (
            <div
              key={idx}
              className={clsx(
                'transition-colors',
                highlightLines.includes(idx + 1) &&
                  'bg-critical/8 border-l-2 border-critical pl-4 -ml-5',
              )}
            >
              {line}
            </div>
          ))}
        </pre>
      </div>
    </div>
  )
}
