'use client'

import { useEffect } from 'react'

export default function DocsRedirect() {
  useEffect(() => {
    window.location.href = 'https://docs.layerx402.dev'
  }, [])

  return (
    <div className="flex min-h-screen items-center justify-center">
      <p className="text-lg">Redirecting to documentation...</p>
    </div>
  )
}
