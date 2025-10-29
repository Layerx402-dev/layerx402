# Layerx402 API Documentation

Welcome to Layerx402! Build with the x402 payment protocol using our high-performance infrastructure layer.

## What is Layerx402?

Layerx402 is a production-grade API infrastructure for the [x402 protocol](https://www.x402.org/) - the open standard for HTTP-native payments. We handle the complexity of x402 payment verification, settlement, and monitoring so you can focus on building great products.

## What is x402?

[x402](https://x402.gitbook.io/x402) is an open payment standard enabling services to charge for API access directly over HTTP using the `402 Payment Required` status code. It enables:

- **Account-free payments** - No registration, email, or OAuth required
- **Machine-to-machine payments** - Perfect for AI agents and autonomous systems
- **Micropayments** - Enable usage-based monetization at any scale
- **Zero fees** - No traditional payment processor overhead
- **Blockchain agnostic** - Works with any blockchain or token

## Key Features

- **Sub-50ms Payment Verification** - Ultra-fast x402 payment processing
- **Production-Ready APIs** - RESTful and WebSocket interfaces
- **Multi-Chain Support** - Accept payments on any blockchain supported by x402
- **Real-time Monitoring** - Track payments and settlements as they happen
- **99.99% Uptime** - Enterprise-grade reliability
- **Global Infrastructure** - Low-latency worldwide

## Quick Start

### 1. Get your API key

Sign up at [layerx402.dev](https://layerx402.dev) to get your API key.

### 2. Verify an x402 payment

```bash
curl -X POST https://api.layerx402.dev/v1/payments/verify \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer YOUR_API_KEY" \
  -d '{
    "payment_proof": "...",
    "amount": 1000000,
    "recipient": "YOUR_WALLET_ADDRESS"
  }'
```

### 3. Settle the payment

```bash
curl -X POST https://api.layerx402.dev/v1/payments/settle \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer YOUR_API_KEY" \
  -d '{
    "payment_id": "payment_abc123"
  }'
```

## Documentation Structure

- [Getting Started](getting-started.md) - Set up your API key and integrate x402
- [API Reference](api-reference/) - Complete API documentation
  - [Payment Verification API](api-reference/payment-verification-api.md) - Verify x402 payments
  - [Payment Settlement API](api-reference/payment-settlement-api.md) - Settle verified payments
  - [WebSocket API](api-reference/websocket-api.md) - Real-time payment events
- [Authentication](authentication.md) - API authentication guide
- [Rate Limits](rate-limits.md) - Understanding rate limits and quotas
- [Error Handling](error-handling.md) - Common errors and solutions
- [Examples](examples/) - Code examples in multiple languages

## x402 Resources

- **x402 Protocol**: [x402.org](https://www.x402.org/)
- **x402 Documentation**: [x402.gitbook.io](https://x402.gitbook.io/x402)
- **x402 on Twitter**: [@x402protocol](https://x.com/x402protocol)

## Support

- **Documentation Issues**: [GitHub Issues](https://github.com/layerx402/docs/issues)
- **Twitter**: [@layerx402](https://x.com/layerx402)
- **Website**: [layerx402.dev](https://layerx402.dev)
- **Email**: support@layerx402.dev
