import Image from "next/image"

export function Footer() {
  return (
    <footer className="border-t border-border bg-navy-very-pale">
      <div className="container mx-auto px-4 py-12">
        <div className="grid gap-20 md:grid-cols-4">
          <div>
            <div className="mb-4 flex items-center gap-4">
              <Image src="/logo.png" alt="Logo" width={50} height={50} />
              <span className="text-2xl text-primary font-light">Layerx402</span>
            </div>
            <p className="text-sm text-muted-foreground">
              High-performance infrastructure for the x402 payment protocol. Making crypto payments as simple as HTTP.
            </p>
          </div>

          <div>
            <h3 className="mb-4 font-semibold text-foreground">Product</h3>
            <ul className="space-y-2 text-sm text-muted-foreground">
              <li>
                <a href="/#features" className="hover:text-primary transition-colors">
                  Features
                </a>
              </li>
              <li>
                <a href="/status" className="hover:text-primary transition-colors">
                  API Status
                </a>
              </li>
            </ul>
          </div>

          <div>
            <h3 className="mb-4 font-semibold text-foreground">Developers</h3>
            <ul className="space-y-2 text-sm text-muted-foreground">
              <li>
                <a href="https://docs.layerx402.dev" target="_blank" rel="noopener noreferrer" className="hover:text-primary transition-colors">
                  Documentation
                </a>
              </li>
              <li>
                <a href="https://docs.layerx402.dev/api-reference/api-reference" target="_blank" rel="noopener noreferrer" className="hover:text-primary transition-colors">
                  API Reference
                </a>
              </li>
              <li>
                <a href="https://github.com/" className="hover:text-primary transition-colors">
                  GitHub
                </a>
              </li>
            </ul>
          </div>

          <div>
            <h3 className="mb-4 font-semibold text-foreground">Community</h3>
            <ul className="space-y-2 text-sm text-muted-foreground">
              <li>
                <a href="https://x.com/layerx402" className="hover:text-primary transition-colors">
                  X
                </a>
              </li>
              <li>
                <a href="https://t.me" className="hover:text-primary transition-colors">
                  Telegram
                </a>
              </li>
              <li>
                <a href="https://discord.com/" className="hover:text-primary transition-colors">
                  Discord
                </a>
              </li>
            </ul>
          </div>
        </div>

        <div className="mt-12 border-t border-border pt-8 text-center text-sm text-muted-foreground">
          <p>© 2025 Layerx402. All rights reserved.</p>
        </div>
      </div>
    </footer>
  )
}
