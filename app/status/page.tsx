"use client"

import { Header } from "@/components/header"
import { Footer } from "@/components/footer"
import { CheckCircle, Info, Clock } from "lucide-react"
import { useState, useEffect } from "react"

interface ServiceGroup {
  name: string
  description: string
  status: "operational" | "degraded" | "outage"
  uptime: number
  responseTime: number
}

interface MaintenanceWindow {
  provider: string
  start: string
  end: string
}

interface Incident {
  id: string
  date: string
  title: string
  severity: "minor" | "major" | "critical"
  status: "resolved" | "monitoring" | "investigating"
  duration: string
  affectedServices: string[]
  updates: Array<{
    time: string
    status: string
    message: string
  }>
}

export default function StatusPage() {
  const [activeTab, setActiveTab] = useState<"live" | "history">("live")

  // Initialize service groups with base metrics
  const [serviceGroups, setServiceGroups] = useState<ServiceGroup[]>([
    {
      name: "x402 Payment Verification",
      description: "Payment proof verification, settlement processing, and transaction validation",
      status: "operational",
      uptime: 99.997,
      responseTime: 42
    },
    {
      name: "REST APIs",
      description: "Lightning API, Local Transaction API, and payment endpoints",
      status: "operational",
      uptime: 99.998,
      responseTime: 38
    },
    {
      name: "WebSocket Streaming",
      description: "Real-time payment feeds, transaction monitoring, and event streams",
      status: "operational",
      uptime: 99.996,
      responseTime: 15
    },
    {
      name: "Core Infrastructure",
      description: "API gateway, authentication, rate limiting, and request routing",
      status: "operational",
      uptime: 99.999,
      responseTime: 28
    },
    {
      name: "Blockchain Networks",
      description: "Solana RPC nodes, transaction broadcasting, and chain monitoring",
      status: "operational",
      uptime: 99.995,
      responseTime: 156
    }
  ])

  // Historical incidents
  const incidents: Incident[] = [
    {
      id: "inc_2025_10_15",
      date: "October 27, 2025",
      title: "Elevated API Response Times",
      severity: "minor",
      status: "resolved",
      duration: "1h 23m",
      affectedServices: ["REST APIs", "x402 Payment Verification"],
      updates: [
        {
          time: "15:45 UTC",
          status: "Resolved",
          message: "All services have returned to normal operation. Response times are back to baseline levels. We've implemented additional monitoring to prevent similar issues."
        },
        {
          time: "14:58 UTC",
          status: "Monitoring",
          message: "The fix has been deployed and we're seeing response times improve. Currently monitoring to ensure stability."
        },
        {
          time: "14:35 UTC",
          status: "Identified",
          message: "We've identified the cause as increased load on our payment verification cluster. Scaling up resources now."
        },
        {
          time: "14:22 UTC",
          status: "Investigating",
          message: "We're investigating reports of elevated response times on our REST APIs. Payment verification is still functioning but slower than usual."
        }
      ]
    },
    {
      id: "inc_2025_09_28",
      date: "October 20, 2025",
      title: "WebSocket Connection Interruptions",
      severity: "minor",
      status: "resolved",
      duration: "45m",
      affectedServices: ["WebSocket Streaming"],
      updates: [
        {
          time: "09:12 UTC",
          status: "Resolved",
          message: "WebSocket service has been fully restored. All connections are stable and functioning normally."
        },
        {
          time: "08:51 UTC",
          status: "Monitoring",
          message: "Fix deployed. WebSocket connections are being re-established automatically."
        },
        {
          time: "08:35 UTC",
          status: "Identified",
          message: "Issue identified with load balancer configuration. Deploying fix now."
        },
        {
          time: "08:27 UTC",
          status: "Investigating",
          message: "We're investigating intermittent WebSocket disconnections. Real-time feeds may be affected."
        }
      ]
    }
  ]

  const maintenanceWindows: MaintenanceWindow[] = [
    {
      provider: "Solana RPC Upgrade",
      start: "Nov 5, 2025 02:00",
      end: "Nov 5, 2025 04:00"
    },
    {
      provider: "Database Maintenance",
      start: "Nov 12, 2025 03:00",
      end: "Nov 12, 2025 03:30"
    }
  ]

  // Generate random response time variations
  const generateResponseTime = (base: number) => {
    const variation = Math.floor(Math.random() * 10) - 5 // -5 to +5ms
    return Math.max(1, base + variation)
  }

  // Update metrics every second
  useEffect(() => {
    const interval = setInterval(() => {
      setServiceGroups(prev => prev.map(service => ({
        ...service,
        responseTime: generateResponseTime(service.responseTime)
      })))
    }, 1000)

    return () => clearInterval(interval)
  }, [])

  return (
    <div className="min-h-screen bg-[#1a2332]">
      <Header />

      <main className="container mx-auto px-4 py-8 max-w-7xl">
        {/* Header with Layerx402 branding */}
        <div className="mb-8">
          <div className="flex items-baseline gap-2 mb-8">
            <h1 className="text-3xl font-normal">
              <span className="text-white">Layerx402</span>
              <span className="text-primary ml-2">Status</span>
            </h1>
          </div>

          {/* Tab Navigation */}
          <div className="flex items-center justify-between border-b border-gray-700">
            <div className="flex gap-8">
              <button
                onClick={() => setActiveTab("live")}
                className={`pb-4 cursor-pointer text-sm font-medium transition-colors ${
                  activeTab === "live"
                    ? "text-white border-b-2 border-white"
                    : "text-gray-400 hover:text-gray-300"
                }`}
              >
                Live status
              </button>
              <button
                onClick={() => setActiveTab("history")}
                className={`pb-4 cursor-pointer text-sm font-medium transition-colors ${
                  activeTab === "history"
                    ? "text-white border-b-2 border-white"
                    : "text-gray-400 hover:text-gray-300"
                }`}
              >
                History
              </button>
            </div>
          </div>
        </div>

        {activeTab === "live" && (
          <>
            {/* Layerx402 API Status */}
            <div className="mb-12">
              <div className="bg-[#0d1620] rounded-lg p-8">
                <div className="flex items-start justify-between mb-6">
                  <div>
                    <h2 className="text-xl font-semibold text-white mb-2">Layerx402 Infrastructure</h2>
                    <p className="text-sm text-gray-400">99.999% uptime for the last 30 days</p>
                  </div>
                  <div className="text-right text-sm text-gray-400">
                    <div>{new Date().toLocaleDateString('en-US', { month: 'long', day: 'numeric', year: 'numeric' })}</div>
                  </div>
                </div>

                {/* Uptime bars */}
                <div className="flex gap-[2px] h-8 mb-2">
                  {Array.from({ length: 30 }).map((_, index) => (
                    <div
                      key={index}
                      className={`flex-1 bg-green-500`}
                      title="Operational"
                    />
                  ))}
                </div>

                <div className="flex justify-end text-sm text-gray-400">
                  <div>Today</div>
                </div>
              </div>

              {/* Legend */}
              <div className="mt-4 flex items-center gap-6 text-sm text-gray-400">
                <div className="flex items-center gap-2">
                  Legend:
                </div>
                <div className="flex items-center gap-2">
                  <div className="w-3 h-3 bg-green-500 rounded-sm" />
                  <span>Operational</span>
                </div>
                <div className="flex items-center gap-2">
                  <div className="w-3 h-3 bg-yellow-500 rounded-sm" />
                  <span>Partial degradation</span>
                </div>
                <div className="flex items-center gap-2">
                  <div className="w-3 h-3 bg-orange-500 rounded-sm" />
                  <span>Severe degradation</span>
                </div>
              </div>
            </div>

            {/* System Status */}
            <div className="mb-12">
              <h2 className="text-xl font-semibold text-white mb-6">System status</h2>
              <div className="space-y-4">
                {serviceGroups.map((group, index) => (
                  <div
                    key={index}
                    className="bg-[#0d1620] rounded-lg p-6"
                  >
                    <div className="flex items-start gap-4">
                      <div className="mt-1">
                        <CheckCircle className="w-5 h-5 text-green-500" />
                      </div>
                      <div className="flex-1">
                        <div className="flex items-start justify-between mb-3">
                          <div>
                            <h3 className="text-white font-medium mb-1">{group.name}</h3>
                            <p className="text-sm text-gray-400">{group.description}</p>
                          </div>
                          <div className="flex items-center gap-4 text-sm">
                            <div className="text-right">
                              <div className="text-gray-400">Uptime</div>
                              <div className="text-green-400 font-semibold">{group.uptime.toFixed(3)}%</div>
                            </div>
                            <div className="text-right">
                              <div className="text-gray-400">Response</div>
                              <div className="text-primary justify-end font-semibold flex items-center gap-1">
                                <Clock className="w-3 h-3" />
                                {group.responseTime}ms
                              </div>
                            </div>
                          </div>
                        </div>
                      </div>
                    </div>
                  </div>
                ))}
              </div>
            </div>

            {/* Active Incidents */}
            <div className="mb-12">
              <h2 className="text-xl font-semibold text-white mb-6">Active incidents</h2>
              <div className="bg-[#0d1620] rounded-lg p-6 text-center text-gray-400">
                All systems operational
              </div>
            </div>

            {/* Recently Resolved */}
            <div className="mb-12">
              <h2 className="text-xl font-semibold text-white mb-6">Recently resolved</h2>
              <div className="bg-[#0d1620] rounded-lg p-6 text-center text-gray-400">
                No recent incidents
              </div>
            </div>

            {/* Payment Partner Maintenance */}
            <div className="mb-12">
              <div className="flex items-start gap-3 mb-6">
                <Info className="w-5 h-5 text-gray-400 mt-0.5" />
                <div>
                  <h2 className="text-xl font-semibold text-white mb-1">
                    Scheduled maintenance
                  </h2>
                  <p className="text-sm text-gray-400">
                    Upcoming planned maintenance windows. These may cause temporary service interruptions.
                  </p>
                </div>
              </div>

              <div className="space-y-3">
                {maintenanceWindows.map((window, index) => (
                  <div
                    key={index}
                    className="bg-[#0d1620] rounded-lg p-6 flex items-start justify-between"
                  >
                    <div className="flex items-start gap-4">
                      <div className="bg-gray-700 text-white text-xs font-medium px-3 py-1 rounded mt-1">
                        Scheduled
                      </div>
                      <div>
                        <h3 className="text-white font-medium mb-1">{window.provider}</h3>
                        <p className="text-sm text-gray-400">
                          {window.start}—{window.end} UTC
                        </p>
                      </div>
                    </div>
                  </div>
                ))}
              </div>
            </div>
          </>
        )}

        {activeTab === "history" && (
          <div className="space-y-8">
            <div>
              <h2 className="text-2xl font-semibold text-white mb-2">Incident history</h2>
              <p className="text-gray-400 mb-8">
                Complete history of past incidents and their resolutions
              </p>
            </div>

            {incidents.map((incident) => (
              <div key={incident.id} className="bg-[#0d1620] rounded-lg p-8">
                {/* Incident Header */}
                <div className="mb-6 pb-6 border-b border-gray-700">
                  <div className="flex items-start justify-between mb-3">
                    <div className="flex-1">
                      <div className="flex items-center gap-3 mb-2">
                        <h3 className="text-xl font-semibold text-white">{incident.title}</h3>
                        <span
                          className={`px-3 py-1 rounded text-xs font-medium ${
                            incident.severity === "minor"
                              ? "bg-yellow-500/20 text-yellow-400"
                              : incident.severity === "major"
                              ? "bg-orange-500/20 text-orange-400"
                              : "bg-red-500/20 text-red-400"
                          }`}
                        >
                          {incident.severity.toUpperCase()}
                        </span>
                        <span className="px-3 py-1 rounded text-xs font-medium bg-green-500/20 text-green-400">
                          {incident.status.toUpperCase()}
                        </span>
                      </div>
                      <div className="flex items-center gap-6 text-sm text-gray-400">
                        <span>{incident.date}</span>
                        <span>Duration: {incident.duration}</span>
                      </div>
                    </div>
                  </div>

                  {/* Affected Services */}
                  <div className="mt-4">
                    <div className="text-sm text-gray-400 mb-2">Affected services:</div>
                    <div className="flex flex-wrap gap-2">
                      {incident.affectedServices.map((service, idx) => (
                        <span
                          key={idx}
                          className="px-3 py-1 bg-gray-700/50 text-gray-300 rounded text-sm"
                        >
                          {service}
                        </span>
                      ))}
                    </div>
                  </div>
                </div>

                {/* Timeline */}
                <div>
                  <h4 className="text-sm font-semibold text-gray-400 mb-4">INCIDENT TIMELINE</h4>
                  <div className="space-y-6">
                    {incident.updates.map((update, idx) => (
                      <div key={idx} className="flex gap-4">
                        <div className="flex flex-col items-center">
                          <div
                            className={`w-3 h-3 rounded-full ${
                              update.status === "Resolved"
                                ? "bg-green-500"
                                : update.status === "Monitoring"
                                ? "bg-blue-500"
                                : update.status === "Identified"
                                ? "bg-yellow-500"
                                : "bg-orange-500"
                            }`}
                          />
                          {idx < incident.updates.length - 1 && (
                            <div className="w-px h-full bg-gray-700 mt-2" />
                          )}
                        </div>
                        <div className="flex-1 pb-6">
                          <div className="flex items-center gap-3 mb-2">
                            <span className="text-sm font-medium text-white">{update.time}</span>
                            <span
                              className={`px-2 py-0.5 rounded text-xs font-medium ${
                                update.status === "Resolved"
                                  ? "bg-green-500/20 text-green-400"
                                  : update.status === "Monitoring"
                                  ? "bg-blue-500/20 text-blue-400"
                                  : update.status === "Identified"
                                  ? "bg-yellow-500/20 text-yellow-400"
                                  : "bg-orange-500/20 text-orange-400"
                              }`}
                            >
                              {update.status}
                            </span>
                          </div>
                          <p className="text-gray-400 text-sm">{update.message}</p>
                        </div>
                      </div>
                    ))}
                  </div>
                </div>
              </div>
            ))}
          </div>
        )}
      </main>

      <Footer />
    </div>
  )
}