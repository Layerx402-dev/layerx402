"use client"

import { Card } from "@/components/ui/card"
import { Badge } from "@/components/ui/badge"
import { TrendingUp, TrendingDown, Activity, BarChart3, Users, Zap, DollarSign, Target, Clock } from "lucide-react"
import { useEffect, useRef, useState } from "react"

interface Trade {
  id: string
  type: "buy" | "sell"
  ticker: string
  amount: string
  price: string
  time: string
  wallet: string
}

// Pool of fake tickers and data
const TICKERS = [
  "PEPE", "BONK", "WIF", "BOOK", "MYRO", "POPCAT", "MEW", "MOTHER", 
  "BODEN", "TREMP", "JUPE", "SLERF", "ORCA", "RAY", "JUPITER", "DRIFT"
]

const WALLETS = [
  "7x...2k9", "Bm...4fG", "9K...8tL", "Cx...1pR", "5H...9mN", 
  "Qw...7vB", "8D...3sF", "Pz...6hJ", "4R...5nC", "Lm...2wX"
]

function generateRandomTrade(): Trade {
  const type = Math.random() > 0.6 ? "buy" : "sell"
  const ticker = TICKERS[Math.floor(Math.random() * TICKERS.length)]
  const amount = (Math.random() * 5 + 0.1).toFixed(1)
  const basePrice = Math.random() * 0.1 + 0.001
  const price = `$${basePrice.toFixed(6)}`
  const wallet = WALLETS[Math.floor(Math.random() * WALLETS.length)]
  
  return {
    id: Math.random().toString(36).substr(2, 9),
    type,
    ticker,
    amount: `${amount} SOL`,
    price,
    time: "now",
    wallet
  }
}

export function TradingInterface() {
  const sectionRef = useRef<HTMLDivElement>(null)
  const [isVisible, setIsVisible] = useState(false)
  const [currentVolume, setCurrentVolume] = useState(425.4)
  const [totalTrades, setTotalTrades] = useState(1.85)
  
  const [trades, setTrades] = useState<Trade[]>([
    { id: "1", type: "buy", ticker: "PEPE", amount: "1.2 SOL", price: "$0.000234", time: "2ms ago", wallet: "7x...2k9" },
    { id: "2", type: "sell", ticker: "BONK", amount: "0.8 SOL", price: "$0.000189", time: "5ms ago", wallet: "Bm...4fG" },
    { id: "3", type: "buy", ticker: "WIF", amount: "2.5 SOL", price: "$0.001235", time: "12ms ago", wallet: "9K...8tL" },
    { id: "4", type: "buy", ticker: "BOOK", amount: "0.5 SOL", price: "$0.000567", time: "18ms ago", wallet: "Cx...1pR" },
    { id: "5", type: "buy", ticker: "WIF", amount: "2.5 SOL", price: "$0.001235", time: "12ms ago", wallet: "9K...8tL" },
    { id: "6", type: "buy", ticker: "BOOK", amount: "0.5 SOL", price: "$0.000567", time: "18ms ago", wallet: "Cx...1pR" },
  ])

  useEffect(() => {
    const observer = new IntersectionObserver(
      (entries) => {
        entries.forEach((entry) => {
          if (entry.isIntersecting && !isVisible) {
            setIsVisible(true)
          }
        })
      },
      { threshold: 0.2 },
    )

    if (sectionRef.current) observer.observe(sectionRef.current)

    return () => observer.disconnect()
  }, [isVisible])

  // Simulate live data updates
  useEffect(() => {
    const interval = setInterval(() => {
      setCurrentVolume(prev => prev + (Math.random() - 0.5) * 10)
      setTotalTrades(prev => prev + Math.random() * 0.01)
    }, 3000)

    return () => clearInterval(interval)
  }, [])

  // Add new trades periodically
  useEffect(() => {
    const interval = setInterval(() => {
      const newTrade = generateRandomTrade()
      setTrades(prevTrades => {
        const updatedTrades = [newTrade, ...prevTrades].slice(0, 8) // Keep only latest 8 trades
        // Update timestamps for existing trades
        return updatedTrades.map((trade, index) => {
          if (index === 0) return trade
          const seconds = Math.floor(Math.random() * 30) + (index * 3)
          return {
            ...trade,
            time: `${seconds}ms ago`
          }
        })
      })
    }, 700 + Math.random() * 1000) // Random interval between 0.5-2 seconds

    return () => clearInterval(interval)
  }, [])

  return (
    <section id="trading" className="relative overflow-hidden py-24" ref={sectionRef}>
      <div className="relative container mx-auto px-4">
        <div className="mx-auto max-w-7xl">
          {/* Header */}
          <div className="mb-16">
            <div className="mb-6 inline-flex items-center gap-2 rounded-none border border-primary/20 bg-primary/10 px-4 py-2 text-sm font-medium text-primary">
              <BarChart3 className="h-4 w-4" />
              Live Network Activity
            </div>
            <h2 className="mb-6 text-5xl font-bold tracking-tight">
              <span className="">
                Real-Time Transaction
              </span>
              <br />
              <span className="bg-gradient-to-r from-primary via-chart-2 to-primary bg-clip-text text-transparent">
                Monitoring
              </span>
            </h2>
            <p className="text-xl text-muted-foreground max-w-3xl leading-relaxed">
              Stream live transaction data, monitor network activity, and gain instant visibility into
              on-chain operations with sub-second latency.
            </p>
          </div>

          {/* Main Trading Dashboard */}
          <div className="grid gap-8 lg:grid-cols-3">
            {/* Market Stats - Left Column */}
            <div className="lg:col-span-1 space-y-6">
              {/* Volume Card */}
              <Card className="group relative overflow-hidden border-0 bg-gradient-to-br from-card/20 to-card/40 backdrop-blur-sm transition-all duration-500 hover:shadow-2xl hover:shadow-green-500/10">
                <div className="absolute inset-0 bg-gradient-to-br from-green-500/5 to-emerald-500/10 opacity-80 transition-opacity duration-500 group-hover:opacity-100" />
                <div className="relative p-6">
                  <div className="mb-4 flex items-center justify-between">
                    <div className="flex h-12 w-12 items-center justify-center rounded-none bg-green-500/10 group-hover:bg-green-500/20 transition-colors duration-300">
                      <DollarSign className="h-6 w-6 text-green-500" />
                    </div>
                    <Badge className="bg-green-500/20 text-green-400 border-green-500/30">
                      <TrendingUp className="h-3 w-3 mr-1" />
                      +12.3%
                    </Badge>
                  </div>
                  <div className="space-y-2">
                    <h3 className="text-sm font-medium text-muted-foreground">24h Network Volume</h3>
                    <div className="text-3xl font-bold text-green-500">
                      ${currentVolume.toFixed(1)}M
                    </div>
                  </div>
                </div>
              </Card>

              {/* Success Rate */}
              <Card className="group relative overflow-hidden border-0 bg-gradient-to-br from-card/20 to-card/40 backdrop-blur-sm transition-all duration-500 hover:shadow-2xl hover:shadow-purple-500/10">
                <div className="absolute inset-0 bg-gradient-to-br from-purple-500/5 to-pink-500/10 opacity-80 transition-opacity duration-500 group-hover:opacity-100" />
                <div className="relative p-6">
                  <div className="mb-4 flex items-center justify-between">
                    <div className="flex h-12 w-12 items-center justify-center rounded-none bg-purple-500/10 group-hover:bg-purple-500/20 transition-colors duration-300">
                      <Target className="h-6 w-6 text-purple-500" />
                    </div>
                  </div>
                  <div className="space-y-2">
                    <h3 className="text-sm font-medium text-muted-foreground">Avg. Confirmation</h3>
                    <div className="text-3xl font-bold text-purple-500">237ms</div>
                  </div>
                </div>
              </Card>
            </div>

            {/* Live Trades Feed - Right Column */}
            <div className="lg:col-span-2">
              <Card className="relative overflow-hidden bg-gradient-to-br from-card/20 neon-border to-card/40 backdrop-blur-sm h-full">
                <div className="absolute inset-0 bg-gradient-to-br from-primary/2 to-chart-2/2" />
                <div className="relative px-6 pt-1">
                  <div className="mb-6 flex items-center justify-between">
                    <div className="flex items-center gap-3">
                      <div className="flex h-10 w-10 items-center justify-center rounded-none bg-primary/10">
                        <Activity className="h-5 w-5 text-primary animate-pulse" />
                      </div>
                      <div>
                        <h3 className="text-lg font-semibold">Live Transaction Feed</h3>
                        <p className="text-sm text-muted-foreground">Real-time on-chain activity</p>
                      </div>
                    </div>
                    <div className="flex items-center gap-2">
                      <div className="h-2 w-2 rounded-none bg-green-500 animate-pulse" />
                      <span className="text-sm font-medium text-green-500">Live</span>
                    </div>
                  </div>

                  {/* Trade Feed */}
                  <div className="space-y-2 max-h-96 overflow-hidden">
                    {trades.map((trade, i) => (
                      <div 
                        key={trade.id} 
                        className={`group flex items-center justify-between p-4 rounded-none transition-all duration-500 border ${
                          i === 0 
                            ? 'bg-gradient-to-r from-primary/10 to-chart-2/10 border-primary/20] shadow-lg' 
                            : 'bg-muted/20 border-border/30 hover:bg-muted/40 hover:border-primary/20'
                        }`}
                        style={{
                          animation: i === 0 ? 'fadeInScale 0.6s ease-out' : undefined,
                          transform: isVisible ? 'translateX(0)' : 'translateX(50px)',
                          opacity: isVisible ? 1 : 0,
                          transition: `all 0.6s ease-out ${i * 100}ms`
                        }}
                      >
                        <div className="flex items-center gap-4 flex-1">
                          <div className={`flex h-10 w-10 items-center justify-center rounded-none transition-all duration-300 ${
                            trade.type === "buy" 
                              ? 'bg-green-500/10 group-hover:bg-green-500/20' 
                              : 'bg-red-500/10 group-hover:bg-red-500/20'
                          }`}>
                            {trade.type === "buy" ? (
                              <TrendingUp className="h-5 w-5 text-green-500" />
                            ) : (
                              <TrendingDown className="h-5 w-5 text-red-500" />
                            )}
                          </div>
                          <div className="flex flex-col">
                            <div className="flex items-center gap-2 mb-1">
                              <Badge variant="outline" className="text-xs font-bold px-2 py-1">
                                ${trade.ticker}
                              </Badge>
                              <span className="text-sm font-medium">{trade.amount}</span>
                            </div>
                            <div className="flex items-center gap-2 text-xs text-muted-foreground">
                              <Clock className="h-3 w-3" />
                              <span>{trade.time}</span>
                              <span>•</span>
                              <span>{trade.wallet}</span>
                            </div>
                          </div>
                        </div>
                        <div className="text-right">
                          <div className="text-lg font-bold">{trade.price}</div>
                          <div className={`text-xs ${
                            trade.type === "buy" ? 'text-green-500' : 'text-red-500'
                          }`}>
                            {trade.type.toUpperCase()}
                          </div>
                        </div>
                      </div>
                    ))}
                  </div>
                </div>
              </Card>
            </div>
          </div>
        </div>
      </div>
    </section>
  )
}
