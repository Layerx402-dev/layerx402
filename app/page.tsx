'use client'

import { Header } from "@/components/header"
import { HeroSection } from "@/components/hero-section"
import { FeaturesSection } from "@/components/features-section"
import { StatsSection } from "@/components/stats-section"
import { Footer } from "@/components/footer"

export default function Home() {
  return (
    <div className="relative min-h-screen bg-background">
      {/* Image Background */}
      <div
        className="fixed inset-0 z-0 bg-cover bg-center bg-no-repeat"
        style={{ backgroundImage: 'url(/background.jpg)' }}
      >
        {/* White overlay with 50% opacity */}
        <div className="absolute inset-0 bg-black/65" />
      </div>

      {/* Content */}
      <div className="relative z-20">
        <Header />
        <main>
          {/* Hero - Full width, centered */}
          <HeroSection />

          {/* Alternating full-width sections with diagonal dividers */}
          <div className="relative">
            {/* Stats Section - Light background */}
            <StatsSection />

            {/* Trading Interface - Transparent */}
            {/* <TradingInterface /> */}


            {/* Features Section - Light background */}
            <FeaturesSection />
          </div>
        </main>
        <Footer />
      </div>
    </div>
  )
}
