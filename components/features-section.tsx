"use client"

import { Badge } from "@/components/ui/badge"
import { Zap, Shield, Code, BarChart3, Cpu, Globe, ArrowRight } from "lucide-react"
import { useEffect, useRef, useState } from "react"

const features = [
  {
    icon: Zap,
    title: "Lightning-Fast Verification",
    description: "Experience the future of payments with sub-50ms x402 verification. Our hyper-optimized infrastructure processes transactions at unprecedented speeds, delivering the seamless experience your users demand.",
    metric: "< 50ms",
    badge: "Ultra Fast",
    color: "from-yellow-500/20 to-orange-500/20",
    borderColor: "border-yellow-500/30",
    hoverColor: "group-hover:text-yellow-500",
    iconBg: "bg-yellow-500/10",
    iconColor: "text-yellow-500",
    accentColor: "bg-yellow-500"
  },
  {
    icon: Shield,
    title: "Unbreakable Reliability",
    description: "Built for the big leagues with 99.99% uptime SLA. Redundant infrastructure, intelligent failover, and 24/7 monitoring ensure your x402 payments never sleep.",
    metric: "99.99%",
    badge: "Secure",
    color: "from-green-500/20 to-emerald-500/20",
    borderColor: "border-green-500/30",
    hoverColor: "group-hover:text-green-500",
    iconBg: "bg-green-500/10",
    iconColor: "text-green-500",
    accentColor: "bg-green-500"
  },
  {
    icon: Code,
    title: "Developer-First APIs",
    description: "Elegantly simple REST & WebSocket APIs that just work. Go from zero to accepting x402 crypto payments in minutes with our intuitive developer experience.",
    metric: "5 min",
    badge: "Easy Setup",
    color: "from-blue-500/20 to-cyan-500/20",
    borderColor: "border-blue-500/30",
    hoverColor: "group-hover:text-blue-500",
    iconBg: "bg-blue-500/10",
    iconColor: "text-blue-500",
    accentColor: "bg-blue-500"
  },
  {
    icon: BarChart3,
    title: "Real-Time Everything",
    description: "Live WebSocket streams put you in complete control. Watch every x402 payment, settlement, and event unfold in real-time. No delays, no compromises.",
    metric: "Real-time",
    badge: "Live",
    color: "from-purple-500/20 to-pink-500/20",
    borderColor: "border-purple-500/30",
    hoverColor: "group-hover:text-purple-500",
    iconBg: "bg-purple-500/10",
    iconColor: "text-purple-500",
    accentColor: "bg-purple-500"
  },
  {
    icon: Cpu,
    title: "True Multi-Chain Power",
    description: "One API, unlimited chains. Accept x402 payments on Solana, Ethereum, and beyond. Build once, deploy everywhere with our unified protocol.",
    metric: "Multi-Chain",
    badge: "Flexible",
    color: "from-indigo-500/20 to-purple-500/20",
    borderColor: "border-indigo-500/30",
    hoverColor: "group-hover:text-indigo-500",
    iconBg: "bg-indigo-500/10",
    iconColor: "text-indigo-500",
    accentColor: "bg-indigo-500"
  },
  {
    icon: Globe,
    title: "Globally Distributed",
    description: "Planet-scale infrastructure spanning North America, Europe, and Asia-Pacific. Deliver blazing-fast x402 payment experiences to users anywhere on Earth.",
    metric: "Global",
    badge: "Worldwide",
    color: "from-teal-500/20 to-green-500/20",
    borderColor: "border-teal-500/30",
    hoverColor: "group-hover:text-teal-500",
    iconBg: "bg-teal-500/10",
    iconColor: "text-teal-500",
    accentColor: "bg-teal-500"
  },
]

export function FeaturesSection() {
  const headerRef = useRef<HTMLDivElement>(null)
  const cardsRef = useRef<HTMLDivElement>(null)
  const [hoveredCard, setHoveredCard] = useState<number | null>(null)

  useEffect(() => {
    const observer = new IntersectionObserver(
      (entries) => {
        entries.forEach((entry) => {
          if (entry.isIntersecting) {
            entry.target.classList.add("animate-in")
          }
        })
      },
      { threshold: 0.1 },
    )

    if (headerRef.current) observer.observe(headerRef.current)
    if (cardsRef.current) observer.observe(cardsRef.current)

    return () => observer.disconnect()
  }, [])

  return (
    <section id="features" className="relative overflow-hidden">
      <div className="relative container mx-auto px-4 py-24">
        <div className="mx-auto max-w-7xl">
          <div ref={headerRef} className="fade-up mb-20 text-center">
            <h2 className="mb-6 text-balance text-5xl font-bold tracking-tight">
              Next-Gen x402 Infrastructure
              <br />
              <span className="bg-gradient-to-r from-primary to-chart-2 bg-clip-text text-transparent">Built Different</span>
            </h2>
            <p className="text-xl text-muted-foreground max-w-3xl mx-auto leading-relaxed">
              The most powerful x402 payment infrastructure on the planet. Uncompromising performance,
              bulletproof reliability, and global scale. This is crypto payments evolved.
            </p>
          </div>

          <div ref={cardsRef} className="stagger-in grid gap-6 md:grid-cols-2 lg:grid-cols-3">
            {features.map((feature, i) => {
              // Different layouts for variety
              const isHorizontal = true
              const hasTopAccent = false

              return (
                <div
                  key={i}
                  className={`group relative overflow-hidden border bg-card/50 backdrop-blur-sm transition-all duration-500 hover:shadow-2xl hover:shadow-primary/20 hover:scale-[1.02] border-l-4 ${feature.borderColor}`}
                  style={{ animationDelay: `${i * 100}ms` }}
                  onMouseEnter={() => setHoveredCard(i)}
                  onMouseLeave={() => setHoveredCard(null)}
                >
                  {/* Gradient Background on hover */}
                  <div className={`absolute inset-0 bg-gradient-to-br ${feature.color} opacity-0 transition-opacity duration-500 group-hover:opacity-100`} />

                  {/* Content */}
                  <div className={`relative p-8 flex flex-col`}>
                    {/* Header section with icon and badge */}
                    <div className={`flex items-start justify-between gap-4 mb-6`}>
                      <div className={`${feature.iconBg} p-4 transition-all duration-500 group-hover:scale-110 group-hover:rotate-6`}>
                        <feature.icon className={`h-8 w-8 ${feature.iconColor}`} />
                      </div>

                      <div className={`flex ${isHorizontal ? 'flex-col items-end' : 'flex-row items-start'} gap-2`}>
                        <Badge variant="outline" className={`${feature.iconColor} border-current`}>
                          {feature.badge}
                        </Badge>
                        <div className={`text-2xl font-bold ${feature.iconColor}`}>
                          {feature.metric}
                        </div>
                      </div>
                    </div>

                    {/* Title */}
                    <h3 className={`text-2xl font-bold mb-3 ${feature.hoverColor} transition-colors duration-300`}>
                      {feature.title}
                    </h3>

                    {/* Description */}
                    <p className="text-muted-foreground leading-relaxed mb-4 group-hover:text-foreground/80 transition-colors duration-300">
                      {feature.description}
                    </p>

                    {/* Learn more arrow that appears on hover */}
                    <a href="https://docs.layerx402.dev" className="flex items-center gap-2 text-sm font-semibold opacity-0 group-hover:opacity-100 transition-opacity duration-300">
                      <span className={feature.iconColor}>Learn more</span>
                      <ArrowRight className={`h-4 w-4 ${feature.iconColor} transition-transform group-hover:translate-x-1`} />
                    </a>
                  </div>
                </div>
              )
            })}
          </div>

          {/* Bottom CTA Section */}
          <div className="mt-20 text-center">
            <div className="inline-flex items-center gap-6 rounded-2xl border border-primary/20 bg-gradient-to-r from-primary/5 to-chart-2/5 px-6 py-4 shadow backdrop-blur-lg">
              <div className="text-left">
                <div className="font-semibold text-lg">Ready to build the future?</div>
                <div className="text-sm text-muted-foreground">Ship x402 payments in minutes, not months</div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </section>
  )
}
