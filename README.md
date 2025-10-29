# Layerx402

High-performance infrastructure layer for the x402 protocol. Build, integrate, and scale x402-powered applications with blazing-fast APIs and developer-first tools.

## What is x402?

[x402](https://www.x402.org/) is an open payment standard enabling services to charge for API access directly over HTTP using the `402 Payment Required` status code. It enables frictionless, account-free payments for APIs, AI agents, and digital services.

## Features

- **Sub-50ms Latency** - Ultra-fast x402 payment verification and settlement
- **Developer-First API** - Simple REST and WebSocket APIs for x402 integration
- **Multi-Chain Support** - Work with any blockchain supported by x402 protocol
- **Real-time Payment Streams** - Monitor x402 transactions and settlements live
- **99.99% Uptime** - Enterprise-grade reliability for production workloads
- **Zero Integration Friction** - Start accepting x402 payments in minutes

## Documentation

Complete API documentation is available in the `docs/` directory and can be published to GitBook.

**View Documentation**: See [docs/README.md](docs/README.md) for the full API reference.

### Quick Links

- [Getting Started](docs/getting-started.md)
- [API Reference](docs/api-reference/)
- [Authentication](docs/authentication.md)
- [Examples](docs/examples/)
- [FAQ](docs/resources/faq.md)

## Development

This is a Next.js 15 application with:

- **Framework**: Next.js 15 (App Router)
- **Styling**: Tailwind CSS
- **UI Components**: shadcn/ui
- **Language**: TypeScript

### Setup

```bash
# Install dependencies
pnpm install

# Run development server
pnpm dev

# Build for production
pnpm build

# Start production server
pnpm start
```

### Environment Variables

Create a `.env.local` file:

```env
# Add your environment variables here
NEXT_PUBLIC_APP_URL=http://localhost:3000
```

## Project Structure

```
.
├── app/                # Next.js app directory
│   ├── api/           # API routes
│   ├── docs/          # Documentation page (redirects to GitBook)
│   ├── status/        # Status monitoring page
│   └── page.tsx       # Home page
├── components/        # React components
├── docs/             # GitBook documentation
│   ├── api-reference/ # API documentation
│   ├── examples/      # Code examples
│   └── resources/     # FAQs, support, changelog
├── hooks/            # Custom React hooks
└── lib/              # Utility functions
```

## GitBook Documentation

The `docs/` directory contains complete API documentation structured for GitBook:

### Publishing to GitBook

1. **Connect Repository to GitBook**:
   - Go to [GitBook](https://www.gitbook.com/)
   - Create a new space
   - Connect your GitHub repository
   - Point to the `docs/` directory

2. **GitBook Configuration**:
   The documentation includes:
   - `.gitbook.yaml` - GitBook configuration
   - `SUMMARY.md` - Table of contents
   - Markdown files organized by topic

3. **Auto-sync**:
   GitBook will automatically sync with your repository on push.

### Local Documentation

You can also view the documentation locally:

```bash
cd docs
# Use any markdown viewer or GitBook CLI
```

## Deployment

Deploy to Vercel:

```bash
vercel
```

Or push to your main branch for automatic deployment.

## License

Proprietary - All rights reserved

## Support

- **Website**: [layerx402.dev](https://layerx402.dev)
- **Twitter**: [@layerx402](https://x.com/layerx402)
- **Email**: support@layerx402.dev
