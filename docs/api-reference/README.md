# API Reference

Layerx402 provides three powerful integration methods to suit different use cases and security requirements.

## Integration Methods

### Lightning API

The **Lightning API** is our HTTPS-based trading API designed for quick integration. Simply send a request, and we handle transaction building, signing, and broadcasting.

**Best for:**
- Rapid prototyping and development
- Applications where ease of use is priority
- Automated trading bots
- Quick market interactions

[View Lightning API Documentation →](lightning-api.md)

### Local Transaction API

The **Local Transaction API** gives you complete control. We build the transaction, you sign it with your own wallet and broadcast it through your preferred RPC endpoint.

**Best for:**
- Maximum security requirements
- Institutional applications
- Custom RPC infrastructure
- Full transaction transparency

[View Local Transaction API Documentation →](local-transaction-api.md)

### WebSocket API

The **WebSocket API** provides real-time streaming of market data, trades, and token events.

**Best for:**
- Real-time price monitoring
- Live trading dashboards
- Market analytics
- Event-driven applications

[View WebSocket API Documentation →](websocket-api.md)

## Base URL

All API endpoints are accessible via:

```
https://api.layerx402.dev
```

## Authentication

All requests require authentication via API key:

```
Authorization: Bearer YOUR_API_KEY
```

See [Authentication](../authentication.md) for details.

## Response Format

### Success Response

```json
{
  "success": true,
  "data": {
    // Response data
  },
  "timestamp": 1697234567890
}
```

### Error Response

```json
{
  "success": false,
  "error": {
    "code": "ERROR_CODE",
    "message": "Human-readable error message",
    "details": {}
  },
  "timestamp": 1697234567890
}
```

## Common Parameters

Many endpoints share common parameters:

| Parameter | Type | Description |
| --- | --- | --- |
| `publicKey` | string | Solana wallet public key (base58 format) |
| `mint` | string | Token mint address |
| `amount` | number | Amount for the transaction |
| `slippage` | number | Maximum slippage percentage (0.1-50) |
| `priorityFee` | number | Priority fee in SOL (optional, auto-calculated if not provided) |

## Rate Limits

| Plan | Requests/Minute |
| --- | --- |
| Standard | 100 |
| Premium | 1,000 |
| Enterprise | Custom |

See [Rate Limits](../rate-limits.md) for more information.

## SDKs and Libraries

While not required, we recommend using these community libraries:

- **JavaScript/TypeScript**: Direct fetch/axios integration
- **Python**: `requests` or `httpx`
- **Rust**: `reqwest`

See [Examples](../examples/) for integration code.
