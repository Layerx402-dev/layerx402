"use client"

import Link from "next/link"
import { Button } from "@/components/ui/button"
import { ArrowRight, Zap } from "lucide-react"
import { useEffect, useState } from "react"

export function HeroSection() {
  const [animateChart, setAnimateChart] = useState(false)

  useEffect(() => {
    const timer = setTimeout(() => setAnimateChart(true), 500)
    return () => clearTimeout(timer)
  }, [])

  return (
    <section className="relative min-h-[70vh] flex items-start">
      <div className="container mx-auto py-20">
        <div className="grid gap-16 lg:grid-cols-12 lg:gap-12 items-center">
          {/* Left Column - Main Content */}
          <div className="lg:col-span-7 space-y-8">
            {/* Badge */}
            <div className="inline-flex items-center gap-2 border-2 border-primary bg-primary/5 px-4 py-2 text-sm font-semibold text-primary">
              <Zap className="h-4 w-4" />
              The Future of Web Payments is Here
            </div>

            {/* Main Heading with unique layout */}
            <div className="space-y-4">
              <h1 className="text-6xl md:text-6xl font-bold leading-none tracking-tight">
                <span className="block text-foreground">The leading engine for</span>
                <span className="block text-primary">the x402 protocol.</span>
              </h1>
            </div>

            {/* Description */}
            <p className="text-lg text-white/70 max-w-2xl leading-relaxed">
              Revolutionary x402 infrastructure layer that turns HTTP into a payment rail. Accept lightning-fast crypto payments with zero friction - no wallets, no popups, just pure speed.
            </p>

            {/* CTAs */}
            <div className="flex flex-col sm:flex-row gap-4 pt-0">
              <a href="https://docs.layerx402.dev" target="_blank" rel="noopener noreferrer">
                <Button size="lg" className="group">
                  Start Building
                  <ArrowRight className="h-5 w-5 transition-transform group-hover:translate-x-1" />
                </Button>
              </a>
              <Link href="/status">
                <Button size="lg" variant="outline" className="">
                  View Status
                </Button>
              </Link>
            </div>
          </div>
        </div>
      </div>
    </section>
  )
}
