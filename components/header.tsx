"use client"

import Image from "next/image"
import Link from "next/link"
import { Button } from "@/components/ui/button"
import { Menu, X } from "lucide-react"
import { useState } from "react"

export function Header() {
  const [mobileMenuOpen, setMobileMenuOpen] = useState(false)

  return (
    <header className="sticky top-0 z-50">
      <nav className="container mx-auto flex h-20 items-center justify-between px-4">
        <Link href="/" className="flex items-center gap-4">
          <Image src="/logo.png" alt="Logo" width={50} height={50} />
          <span className="text-2xl text-primary font-light">Layerx402</span>
        </Link>

        <div className="absolute w-[400px] left-half-200 hidden items-center gap-8 md:flex justify-center">
          <a
            href="/#features"
            className="text-sm font-medium text-foreground transition-colors duration-300 hover:text-primary"
          >
            Features
          </a>
          <a href="/status" className="text-sm font-medium text-foreground transition-colors duration-300 hover:text-primary">
            Status
          </a>
          <a href="https://docs.layerx402.dev" className="text-sm font-medium text-foreground transition-colors duration-300 hover:text-primary">
            Docs
          </a>
          <a href="https://github.com/Layerx402-dev/layerx402" className="text-sm font-medium text-foreground transition-colors duration-300 hover:text-primary">
            Github
          </a>
        </div>

        <div className="hidden items-center gap-3 md:flex">
          <a href="https://docs.layerx402.dev" target="_blank" rel="noopener noreferrer">
          <Button size="sm" className="">
            Get Started
          </Button>
          </a>
        </div>

        <button className="md:hidden text-foreground" onClick={() => setMobileMenuOpen(!mobileMenuOpen)}>
          {mobileMenuOpen ? <X className="h-6 w-6" /> : <Menu className="h-6 w-6" />}
        </button>
      </nav>

      {mobileMenuOpen && (
        <div className="border-t border-border bg-background p-4 md:hidden">
          <div className="flex flex-col gap-4">
            <a href="#features" className="text-sm font-medium">
              Features
            </a>
            <a href="https://docs.layerx402.dev" target="_blank" rel="noopener noreferrer" className="text-sm font-medium">
              Docs
            </a>
            <a href="/status" className="text-sm font-medium">
              Status
            </a>
            <a href="https://docs.layerx402.dev/api-reference/api-reference" className="text-sm font-medium">
              API
            </a>
            <a href="https://docs.layerx402.dev" target="_blank" rel="noopener noreferrer">
              <Button size="sm" className="w-full">
                Get started
              </Button>
            </a>
          </div>
        </div>
      )}
    </header>
  )
}
