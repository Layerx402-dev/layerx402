# WebSocket API

Stream real-time x402 payment data, transaction events, and network activity via high-performance WebSocket connections.

## Overview

**WebSocket URL**: `wss://layerx402.dev/api/data`

**Connection Limits**:
- Standard: 5 concurrent connections, 50 messages/second
- Premium: 20 concurrent connections, 200 messages/second

## Connecting

### JavaScript/TypeScript

```javascript
const ws = new WebSocket('wss://layerx402.dev/api/data');

ws.onopen = () => {
  console.log('Connected to Layerx402 WebSocket');

  // Subscribe to payments
  ws.send(JSON.stringify({
    method: 'subscribe',
    keys: ['payments']
  }));
};

ws.onmessage = (event) => {
  const data = JSON.parse(event.data);
  console.log('Received:', data);
};

ws.onerror = (error) => {
  console.error('WebSocket error:', error);
};

ws.onclose = (event) => {
  console.log('Connection closed:', event.code, event.reason);
};
```

### Python

```python
import websocket
import json

def on_message(ws, message):
    data = json.loads(message)
    print('Received:', data)

def on_error(ws, error):
    print('Error:', error)

def on_close(ws, close_status_code, close_msg):
    print('Connection closed')

def on_open(ws):
    print('Connected to Layerx402 WebSocket')

    # Subscribe to payments
    ws.send(json.dumps({
        'method': 'subscribe',
        'keys': ['payments']
    }))

ws = websocket.WebSocketApp(
    'wss://layerx402.dev/api/data',
    on_open=on_open,
    on_message=on_message,
    on_error=on_error,
    on_close=on_close
)

ws.run_forever()
```

## Authentication

For WebSocket connections, include your API key in the initial connection message:

```javascript
ws.onopen = () => {
  // Authenticate
  ws.send(JSON.stringify({
    method: 'authenticate',
    apiKey: 'YOUR_API_KEY'
  }));
};
```

You'll receive a confirmation:

```json
{
  "type": "authenticated",
  "userId": "user_123",
  "plan": "premium"
}
```

## Subscription Methods

### Subscribe to Channels

```javascript
ws.send(JSON.stringify({
  method: 'subscribe',
  keys: ['payments', 'verifications']
}));
```

**Available Channels**:
- `payments` - All payment transactions across the network
- `verifications` - Payment verification events
- `settlements` - Payment settlement confirmations
- `{WALLET_ADDRESS}` - Events for a specific wallet address

**Success Response**:

```json
{
  "type": "subscribed",
  "keys": ["payments", "verifications"],
  "timestamp": 1697234567890
}
```

### Subscribe to Specific Wallet

```javascript
ws.send(JSON.stringify({
  method: 'subscribe',
  keys: ['7BgBvyjrZX8YTqjkKrfbSx9X8QP4NDaP1hj4VKMjqA5s']
}));
```

### Unsubscribe

```javascript
ws.send(JSON.stringify({
  method: 'unsubscribe',
  keys: ['payments']
}));
```

## Event Types

### Payment Event

Emitted when a payment transaction occurs on the x402 network.

```json
{
  "type": "payment",
  "paymentId": "pay_5j7sK8K2YaFvS2hE3YgY4z",
  "signature": "5j7sK8K2YaFvS2hE3YgY4zRtV3pGq1Y7zxP8c9X3Qa6",
  "sender": "7BgBvyjrZX8YTqjkKrfbSx9X8QP4NDaP1hj4VKMjqA5s",
  "recipient": "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
  "amount": 0.001,
  "resource": "https://api.example.com/protected-endpoint",
  "status": "confirmed",
  "blockHeight": 123456789,
  "timestamp": 1697234567890,
  "direction": "sent",
  "metadata": {
    "description": "API access payment",
    "referenceId": "req_12345"
  }
}
```

### Verification Event

Emitted when a payment is verified by a facilitator.

```json
{
  "type": "verification",
  "paymentId": "pay_5j7sK8K2YaFvS2hE3YgY4z",
  "verified": true,
  "verifiedBy": "facilitator.x402.org",
  "verificationTime": "45ms",
  "timestamp": 1697234567890,
  "recipient": "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
  "amount": 0.001
}
```

### Settlement Event

Emitted when a payment is settled and confirmed on-chain.

```json
{
  "type": "settlement",
  "paymentId": "pay_5j7sK8K2YaFvS2hE3YgY4z",
  "signature": "5j7sK8K2YaFvS2hE3YgY4zRtV3pGq1Y7zxP8c9X3Qa6",
  "status": "settled",
  "confirmations": 32,
  "blockHeight": 123456789,
  "finalAmount": 0.001,
  "fees": 0.00001,
  "timestamp": 1697234567890
}
```

### Error Event

```json
{
  "type": "error",
  "code": "SUBSCRIPTION_FAILED",
  "message": "Invalid subscription key",
  "timestamp": 1697234567890
}
```

## Advanced Usage

### Reconnection Logic

```javascript
class Layerx402WebSocket {
  constructor(apiKey) {
    this.apiKey = apiKey;
    this.ws = null;
    this.reconnectDelay = 1000;
    this.maxReconnectDelay = 30000;
    this.subscriptions = new Set();
  }

  connect() {
    this.ws = new WebSocket('wss://layerx402.dev/api/data');

    this.ws.onopen = () => {
      console.log('Connected');
      this.reconnectDelay = 1000;

      // Authenticate
      this.send({
        method: 'authenticate',
        apiKey: this.apiKey
      });

      // Resubscribe to previous subscriptions
      if (this.subscriptions.size > 0) {
        this.send({
          method: 'subscribe',
          keys: Array.from(this.subscriptions)
        });
      }
    };

    this.ws.onmessage = (event) => {
      const data = JSON.parse(event.data);
      this.handleMessage(data);
    };

    this.ws.onerror = (error) => {
      console.error('WebSocket error:', error);
    };

    this.ws.onclose = () => {
      console.log('Connection closed, reconnecting...');
      setTimeout(() => {
        this.reconnectDelay = Math.min(
          this.reconnectDelay * 2,
          this.maxReconnectDelay
        );
        this.connect();
      }, this.reconnectDelay);
    };
  }

  subscribe(keys) {
    keys.forEach(key => this.subscriptions.add(key));

    if (this.ws?.readyState === WebSocket.OPEN) {
      this.send({
        method: 'subscribe',
        keys
      });
    }
  }

  unsubscribe(keys) {
    keys.forEach(key => this.subscriptions.delete(key));

    if (this.ws?.readyState === WebSocket.OPEN) {
      this.send({
        method: 'unsubscribe',
        keys
      });
    }
  }

  send(data) {
    if (this.ws?.readyState === WebSocket.OPEN) {
      this.ws.send(JSON.stringify(data));
    }
  }

  handleMessage(data) {
    switch (data.type) {
      case 'payment':
        this.onPayment(data);
        break;
      case 'verification':
        this.onVerification(data);
        break;
      case 'settlement':
        this.onSettlement(data);
        break;
      case 'error':
        this.onError(data);
        break;
    }
  }

  onPayment(data) {
    // Override this method
    console.log('Payment:', data);
  }

  onVerification(data) {
    // Override this method
    console.log('Verification:', data);
  }

  onSettlement(data) {
    // Override this method
    console.log('Settlement:', data);
  }

  onError(data) {
    // Override this method
    console.error('Error:', data);
  }

  disconnect() {
    if (this.ws) {
      this.ws.close();
      this.ws = null;
    }
  }
}

// Usage
const ws = new Layerx402WebSocket('YOUR_API_KEY');
ws.connect();
ws.subscribe(['payments', 'verifications']);

ws.onPayment = (data) => {
  console.log('Payment detected:', data);
};

ws.onVerification = (data) => {
  console.log('Payment verified:', data);
};
```

### React Hook Example

```typescript
import { useEffect, useRef, useState } from 'react';

interface Payment {
  type: 'payment';
  paymentId: string;
  signature: string;
  sender: string;
  recipient: string;
  amount: number;
  resource: string;
  status: string;
  timestamp: number;
  direction: 'sent' | 'received';
}

export function useLayerx402WebSocket(apiKey: string) {
  const ws = useRef<WebSocket | null>(null);
  const [payments, setPayments] = useState<Payment[]>([]);
  const [connected, setConnected] = useState(false);

  useEffect(() => {
    ws.current = new WebSocket('wss://layerx402.dev/api/data');

    ws.current.onopen = () => {
      setConnected(true);

      ws.current?.send(JSON.stringify({
        method: 'authenticate',
        apiKey
      }));

      ws.current?.send(JSON.stringify({
        method: 'subscribe',
        keys: ['payments']
      }));
    };

    ws.current.onmessage = (event) => {
      const data = JSON.parse(event.data);

      if (data.type === 'payment') {
        setPayments(prev => [data, ...prev].slice(0, 100));
      }
    };

    ws.current.onclose = () => {
      setConnected(false);
    };

    return () => {
      ws.current?.close();
    };
  }, [apiKey]);

  const subscribe = (keys: string[]) => {
    if (ws.current?.readyState === WebSocket.OPEN) {
      ws.current.send(JSON.stringify({
        method: 'subscribe',
        keys
      }));
    }
  };

  return { payments, connected, subscribe };
}
```

## Best Practices

1. **Implement Reconnection**: Handle connection drops gracefully
2. **Authenticate Early**: Send authentication immediately after connecting
3. **Manage Subscriptions**: Track subscriptions for reconnection
4. **Handle Errors**: Monitor error events and log them
5. **Rate Limiting**: Respect message rate limits
6. **Heartbeat**: Implement ping/pong for connection health
7. **Clean Disconnection**: Close connections properly when done

## Heartbeat/Ping-Pong

Keep the connection alive with periodic pings:

```javascript
let pingInterval;

ws.onopen = () => {
  // Send ping every 30 seconds
  pingInterval = setInterval(() => {
    if (ws.readyState === WebSocket.OPEN) {
      ws.send(JSON.stringify({ method: 'ping' }));
    }
  }, 30000);
};

ws.onmessage = (event) => {
  const data = JSON.parse(event.data);

  if (data.type === 'pong') {
    console.log('Pong received');
  }
};

ws.onclose = () => {
  clearInterval(pingInterval);
};
```

## Error Codes

| Code | Description |
| --- | --- |
| `AUTHENTICATION_FAILED` | Invalid API key |
| `SUBSCRIPTION_FAILED` | Invalid subscription key |
| `RATE_LIMIT_EXCEEDED` | Too many messages |
| `CONNECTION_LIMIT` | Too many concurrent connections |

## Next Steps

- View [Lightning API](lightning-api.md) for payment endpoints
- Explore [x402 Protocol Documentation](https://x402.gitbook.io/x402/)
- Check [Examples](../examples/) for complete implementations
- Learn about [Error Handling](../error-handling.md)
