"use client"

import { Card } from "@/components/ui/card"
import { Badge } from "@/components/ui/badge"
import { Zap, Shield, Code, BarChart3, Cpu, Globe } from "lucide-react"
import { useEffect, useRef, useState } from "react"

const features = [
  {
    icon: Zap,
    title: "Lightning-Fast Verification",
    description: "Experience the future of payments with sub-50ms x402 verification. Our hyper-optimized infrastructure processes transactions at unprecedented speeds, delivering the seamless experience your users demand.",
    metric: "< 50ms",
    badge: "Ultra Fast",
    color: "from-yellow-500/20 to-orange-500/20",
    hoverColor: "group-hover:text-yellow-500",
    iconBg: "bg-yellow-500/10",
    iconColor: "text-yellow-500"
  },
  {
    icon: Shield,
    title: "Unbreakable Reliability",
    description: "Built for the big leagues with 99.99% uptime SLA. Redundant infrastructure, intelligent failover, and 24/7 monitoring ensure your x402 payments never sleep.",
    metric: "99.99%",
    badge: "Secure",
    color: "from-green-500/20 to-emerald-500/20",
    hoverColor: "group-hover:text-green-500",
    iconBg: "bg-green-500/10",
    iconColor: "text-green-500"
  },
  {
    icon: Code,
    title: "Developer-First APIs",
    description: "Elegantly simple REST & WebSocket APIs that just work. Go from zero to accepting x402 crypto payments in minutes with our intuitive developer experience.",
    metric: "5 min",
    badge: "Easy Setup",
    color: "from-blue-500/20 to-cyan-500/20",
    hoverColor: "group-hover:text-blue-500",
    iconBg: "bg-blue-500/10",
    iconColor: "text-blue-500"
  },
  {
    icon: BarChart3,
    title: "Real-Time Everything",
    description: "Live WebSocket streams put you in complete control. Watch every x402 payment, settlement, and event unfold in real-time. No delays, no compromises.",
    metric: "Real-time",
    badge: "Live",
    color: "from-purple-500/20 to-pink-500/20",
    hoverColor: "group-hover:text-purple-500",
    iconBg: "bg-purple-500/10",
    iconColor: "text-purple-500"
  },
  {
    icon: Cpu,
    title: "True Multi-Chain Power",
    description: "One API, unlimited chains. Accept x402 payments on Solana, Ethereum, and beyond. Build once, deploy everywhere with our unified protocol.",
    metric: "Multi-Chain",
    badge: "Flexible",
    color: "from-indigo-500/20 to-purple-500/20",
    hoverColor: "group-hover:text-indigo-500",
    iconBg: "bg-indigo-500/10",
    iconColor: "text-indigo-500"
  },
  {
    icon: Globe,
    title: "Globally Distributed",
    description: "Planet-scale infrastructure spanning North America, Europe, and Asia-Pacific. Deliver blazing-fast x402 payment experiences to users anywhere on Earth.",
    metric: "Global",
    badge: "Worldwide",
    color: "from-teal-500/20 to-green-500/20",
    hoverColor: "group-hover:text-teal-500",
    iconBg: "bg-teal-500/10",
    iconColor: "text-teal-500"
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

          <div ref={cardsRef} className="stagger-in grid gap-8 md:grid-cols-2 lg:grid-cols-3">
            {features.map((feature, i) => (
              <Card
                key={i}
                className="group relative overflow-hidden border-0 bg-gradient-to-br from-card/50 to-card backdrop-blur-sm transition-all duration-500 hover:shadow-2xl hover:shadow-primary/10"
                style={{ animationDelay: `${i * 100}ms` }}
                onMouseEnter={() => setHoveredCard(i)}
                onMouseLeave={() => setHoveredCard(null)}
              >
                {/* Gradient Background */}
                <div className={`absolute inset-0 bg-gradient-to-br ${feature.color} opacity-0 transition-opacity duration-500 group-hover:opacity-100`} />
                
                {/* Content */}
                <div className="relative px-6 py-2">
                  <div className="mb-6 flex items-start justify-between">
                    <div className={`inline-flex h-14 w-14 items-center justify-center rounded-none ${feature.iconBg} transition-all duration-500 group-hover:scale-110 group-hover:shadow-lg`}>
                      <feature.icon className={`h-7 w-7 ${feature.iconColor} transition-all duration-500`} />
                    </div>
                  </div>
                  
                  <div className="mb-2">
                    <h3 className={`text-xl font-bold ${feature.hoverColor} transition-colors duration-300`}>
                      {feature.title}
                    </h3>
                  </div>
                  
                  <p className="text-muted-foreground leading-relaxed group-hover:text-foreground/80 transition-colors duration-300">
                    {feature.description}
                  </p>
                </div>
              </Card>
            ))}
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
