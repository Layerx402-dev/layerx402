"use client"

import { useState } from "react"
import { Check, Copy } from "lucide-react"

type CodeTab = 'javascript' | 'curl' | 'python'

const codeExamples: Record<CodeTab, string> = {
  javascript: `const response = await fetch('https://api.layerx402.dev/v1/payments/verify', {
  method: 'POST',
  headers: {
    'Content-Type': 'application/json',
    'Authorization': 'Bearer YOUR_API_KEY'
  },
  body: JSON.stringify({
    payment_proof: 'BASE64_ENCODED_PAYMENT_PROOF',
    amount: 1000000,
    recipient: 'YOUR_WALLET_ADDRESS',
    network: 'solana'
  })
});

const data = await response.json();
console.log('Payment verified:', data);`,
  curl: `curl -X POST https://api.layerx402.dev/v1/payments/verify \\
  -H "Content-Type: application/json" \\
  -H "Authorization: Bearer YOUR_API_KEY" \\
  -d '{
    "payment_proof": "BASE64_ENCODED_PAYMENT_PROOF",
    "amount": 1000000,
    "recipient": "YOUR_WALLET_ADDRESS",
    "network": "solana"
  }'`,
  python: `import requests

response = requests.post(
    'https://api.layerx402.dev/v1/payments/verify',
    headers={
        'Content-Type': 'application/json',
        'Authorization': 'Bearer YOUR_API_KEY'
    },
    json={
        'payment_proof': 'BASE64_ENCODED_PAYMENT_PROOF',
        'amount': 1000000,
        'recipient': 'YOUR_WALLET_ADDRESS',
        'network': 'solana'
    }
)

data = response.json()
print('Payment verified:', data)`
}

export function CodeShowcase() {
  const [activeTab, setActiveTab] = useState<CodeTab>('javascript')
  const [copied, setCopied] = useState(false)

  const copyToClipboard = async () => {
    await navigator.clipboard.writeText(codeExamples[activeTab])
    setCopied(true)
    setTimeout(() => setCopied(false), 2000)
  }

  return (
    <section className="relative overflow-hidden py-24 bg-background">
      <div className="container mx-auto px-4">
        <div className="max-w-6xl mx-auto">
          {/* Header */}
          <div className="text-center mb-12">
            <h2 className="text-4xl lg:text-5xl font-bold mb-6">
              Verify Your First x402 Payment
            </h2>
            <p className="text-xl text-muted-foreground max-w-4xl mx-auto">
              Here's how to verify an x402 payment using Layerx402 using our powerful yet simple API:
            </p>
          </div>

          {/* Code Block */}
          <div className="bg-[#1a1a1a] rounded-lg overflow-hidden border border-primary/20 shadow-2xl">
            {/* Tabs */}
            <div className="flex items-center justify-between border-b border-white/10 bg-[#0d0d0d] px-6">
              <div className="flex gap-1">
                <button
                  onClick={() => setActiveTab('curl')}
                  className={`px-6 py-3 text-sm font-medium transition-colors ${
                    activeTab === 'curl'
                      ? 'text-white border-b-2 border-primary'
                      : 'text-gray-400 hover:text-gray-300'
                  }`}
                >
                  cURL
                </button>
                <button
                  onClick={() => setActiveTab('javascript')}
                  className={`px-6 py-3 text-sm font-medium transition-colors ${
                    activeTab === 'javascript'
                      ? 'text-white border-b-2 border-primary'
                      : 'text-gray-400 hover:text-gray-300'
                  }`}
                >
                  JavaScript
                </button>
                <button
                  onClick={() => setActiveTab('python')}
                  className={`px-6 py-3 text-sm font-medium transition-colors ${
                    activeTab === 'python'
                      ? 'text-white border-b-2 border-primary'
                      : 'text-gray-400 hover:text-gray-300'
                  }`}
                >
                  Python
                </button>
              </div>

              {/* Copy Button */}
              <button
                onClick={copyToClipboard}
                className="flex items-center gap-2 px-4 py-2 text-sm font-medium text-gray-300 hover:text-white transition-colors"
              >
                {copied ? (
                  <>
                    <Check className="h-4 w-4 text-green-500" />
                    <span className="text-green-500">Copied!</span>
                  </>
                ) : (
                  <>
                    <Copy className="h-4 w-4" />
                    <span>Copy</span>
                  </>
                )}
              </button>
            </div>

            {/* Code Content */}
            <div className="p-8 overflow-x-auto">
              <pre className="text-sm leading-relaxed">
                <code className="text-gray-300 font-mono">
                  {codeExamples[activeTab]}
                </code>
              </pre>
            </div>
          </div>

          {/* CTA */}
          <div className="mt-12 text-center">
            <a
              href="https://docs.layerx402.dev"
              target="_blank"
              rel="noopener noreferrer"
              className="inline-flex items-center gap-2 px-8 py-4 bg-primary text-primary-foreground font-semibold rounded-lg hover:bg-primary/90 transition-colors"
            >
              Read the full documentation
              <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M17 8l4 4m0 0l-4 4m4-4H3" />
              </svg>
            </a>
          </div>
        </div>
      </div>
    </section>
  )
}
