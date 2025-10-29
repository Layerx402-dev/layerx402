"use client"

import { useEffect, useRef, useState } from "react"

const stats = [
  {
    value: "$22K+",
    description: "x402 volume processed, real money flowing through HTTP",
  },
  {
    value: "99.99%",
    description: "historical uptime for rock-solid infrastructure",
  },
  {
    value: "45+",
    description: "active developers building on Layerx402",
  },
  {
    value: "13K+",
    description: "instant verifications, seamless x402 transactions",
  }
]

export function StatsSection() {
  const canvasRef = useRef<HTMLCanvasElement>(null)
  const sectionRef = useRef<HTMLDivElement>(null)
  const [isVisible, setIsVisible] = useState(false)

  useEffect(() => {
    const observer = new IntersectionObserver(
      (entries) => {
        entries.forEach((entry) => {
          if (entry.isIntersecting) {
            setIsVisible(true)
          }
        })
      },
      { threshold: 0.2 },
    )

    if (sectionRef.current) observer.observe(sectionRef.current)

    return () => observer.disconnect()
  }, [])

  useEffect(() => {
    const canvas = canvasRef.current
    if (!canvas) return

    const ctx = canvas.getContext('2d')
    if (!ctx) return

    // Set canvas size
    const updateCanvasSize = () => {
      const dpr = window.devicePixelRatio || 1
      const rect = canvas.getBoundingClientRect()
      canvas.width = rect.width * dpr
      canvas.height = rect.height * dpr
      ctx.scale(dpr, dpr)
      canvas.style.width = rect.width + 'px'
      canvas.style.height = rect.height + 'px'
    }

    updateCanvasSize()
    window.addEventListener('resize', updateCanvasSize)

    // Globe animation
    const centerX = canvas.width / (window.devicePixelRatio || 1) / 2
    const centerY = canvas.height / (window.devicePixelRatio || 1) / 2
    const radius = Math.min(centerX, centerY) * 1

    let rotation = 0

    const drawGlobe = () => {
      ctx.clearRect(0, 0, canvas.width, canvas.height)

      // Draw dots in a globe pattern
      const dots = 2500
      for (let i = 0; i < dots; i++) {
        const phi = Math.acos(-1 + (2 * i) / dots)
        const theta = Math.sqrt(dots * Math.PI) * phi + rotation

        const x = centerX + radius * Math.cos(theta) * Math.sin(phi)
        const y = centerY + radius * Math.sin(theta) * Math.sin(phi)
        const z = Math.cos(phi)

        if (z > 0) {
          const size = z * 1.5
          const opacity = z * 0.8
          ctx.fillStyle = `rgba(168, 238, 55, ${opacity})`
          ctx.beginPath()
          ctx.arc(x, y, size, 0, Math.PI * 2)
          ctx.fill()
        }
      }

      rotation += 0.002
      requestAnimationFrame(drawGlobe)
    }

    drawGlobe()

    return () => {
      window.removeEventListener('resize', updateCanvasSize)
    }
  }, [])

  return (
    <section
      ref={sectionRef}
      className="relative overflow-hidden py-24 bg-background"
    >
      <div className="container mx-auto px-4">
        <div className="grid lg:grid-cols-2 gap-16 items-center max-w-7xl mx-auto">
          {/* Left side - Content */}
          <div className="space-y-12">
            {/* Header */}
            <div className="space-y-6">
              <div className="text-sm font-semibold tracking-wide text-white/75 uppercase">
                Innovation in Action
              </div>
              <h2 className="text-5xl lg:text-6xl pb-2 font-bold bg-gradient-to-r from-primary to-chart-2 bg-clip-text text-transparent">
                Powering the future of x402 payments
              </h2>
              <p className="text-lg text-white/60 leading-relaxed max-w-xl -mt-2">
                Countless developers, AI agents, and forward-thinking businesses are already building the future with Layerx402.
                Experience crypto payments at blazing speed.
              </p>
            </div>

            {/* Stats */}
            <div className="grid grid-cols-2 gap-x-12 gap-y-10 pt-8">
              {stats.map((stat, index) => (
                <div
                  key={index}
                  className="relative pl-6 border-l-2 border-primary/80"
                  style={{
                    opacity: isVisible ? 1 : 0,
                    transform: isVisible ? 'translateY(0)' : 'translateY(20px)',
                    transition: `all 0.6s ease-out ${index * 150}ms`
                  }}
                >
                  <div className="text-3xl lg:text-4xl font-bold text-white mb-2">
                    {stat.value}
                  </div>
                  <div className="text-sm text-white/60 leading-relaxed">
                    {stat.description}
                  </div>
                </div>
              ))}
            </div>
          </div>

          {/* Right side - Globe visualization */}
          <div className="relative h-[500px] lg:h-[600px] flex items-center justify-center">
            <canvas
              ref={canvasRef}
              className="w-full h-full"
            />
          </div>
        </div>
      </div>
    </section>
  )
}
