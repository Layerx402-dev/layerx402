# Lightning API

The Lightning API is our HTTPS-based payment API designed for quick integration with the x402 protocol. Send a request and we handle everything: payment verification, transaction building, signing, and broadcasting.

## Overview

**Base URL**: `https://api.layerx402.dev`

**Authentication**: Bearer token in `Authorization` header

**Rate Limits**:
- Standard: 100 requests/minute
- Premium: 1,000 requests/minute

## Endpoints

### Send Payment

Execute a payment over the x402 protocol. This endpoint handles the buyer flow, allowing you to programmatically send payments to access protected resources.

**Endpoint**: `POST /payment/send`

**Request Body**:

| Parameter | Type | Required | Description |
| --- | --- | --- | --- |
| `walletAddress` | string | Yes | Your wallet address (EVM or Solana format) |
| `recipientAddress` | string | Yes | Payment recipient's wallet address |
| `amount` | number | Yes | Payment amount in native currency (min: 0.001, max: 100) |
| `resource` | string | No | Protected resource URL being accessed |
| `metadata` | object | No | Additional payment metadata (description, reference ID, etc.) |
| `priorityFee` | number | No | Network priority fee (default: auto, range: 0.00001-0.01) |

**Example Request**:

```bash
curl -X POST https://api.layerx402.dev/payment/send \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer YOUR_API_KEY" \
  -d '{
    "walletAddress": "7BgBvyjrZX8YTqjkKrfbSx9X8QP4NDaP1hj4VKMjqA5s",
    "recipientAddress": "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
    "amount": 0.001,
    "resource": "https://api.example.com/protected-endpoint",
    "metadata": {
      "description": "API access payment",
      "referenceId": "req_12345"
    },
    "priorityFee": 0.0001
  }'
```

**Success Response** (200):

```json
{
  "success": true,
  "paymentId": "pay_5j7sK8K2YaFvS2hE3YgY4z",
  "signature": "5j7sK8K2YaFvS2hE3YgY4zRtV3pGq1Y7zxP8c9X3Qa6",
  "amount": 0.001,
  "recipient": "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
  "status": "confirmed",
  "blockHeight": 123456789,
  "timestamp": 1697234567890,
  "estimatedConfirmation": "2-5 seconds",
  "paymentProof": "0x7f8a3b2c..."
}
```

---

### Receive Payment

Verify and process incoming x402 payments. This endpoint handles the seller flow, validating payment proofs and confirming receipt of funds for protected resources.

**Endpoint**: `POST /payment/receive`

**Request Body**:

| Parameter | Type | Required | Description |
| --- | --- | --- | --- |
| `walletAddress` | string | Yes | Your receiving wallet address |
| `paymentProof` | string | Yes | Cryptographic payment proof (from X-PAYMENT header) |
| `expectedAmount` | number | Yes | Expected payment amount |
| `resource` | string | No | Protected resource being accessed |
| `facilitatorUrl` | string | No | Facilitator URL for payment verification (default: x402 network) |
| `timeout` | number | No | Verification timeout in seconds (default: 30) |

**Example Request**:

```bash
curl -X POST https://api.layerx402.dev/payment/receive \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer YOUR_API_KEY" \
  -d '{
    "walletAddress": "7BgBvyjrZX8YTqjkKrfbSx9X8QP4NDaP1hj4VKMjqA5s",
    "paymentProof": "0x7f8a3b2c1d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b",
    "expectedAmount": 0.001,
    "resource": "/api/protected-endpoint",
    "timeout": 30
  }'
```

**Success Response** (200):

```json
{
  "success": true,
  "verified": true,
  "paymentId": "pay_3k9mN7PqRtV8sW2xY",
  "signature": "3k9mN7PqRtV8sW2xY1zC4aB6nH5jL8fG9eD3cF2qA7s",
  "amountReceived": 0.001,
  "sender": "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
  "status": "confirmed",
  "blockHeight": 123456790,
  "timestamp": 1697234567890,
  "verificationTime": "45ms"
}
```

---

### Get Payment Status

Retrieve the status and details of a specific payment transaction.

**Endpoint**: `GET /payment/status/{paymentId}`

**Parameters**:

| Parameter | Type | Required | Description |
| --- | --- | --- | --- |
| `paymentId` | string | Yes | Payment transaction ID (path parameter) |

**Example Request**:

```bash
curl -X GET https://api.layerx402.dev/payment/status/pay_5j7sK8K2YaFvS2hE3YgY4z \
  -H "Authorization: Bearer YOUR_API_KEY"
```

**Success Response** (200):

```json
{
  "paymentId": "pay_5j7sK8K2YaFvS2hE3YgY4z",
  "signature": "5j7sK8K2YaFvS2hE3YgY4zRtV3pGq1Y7zxP8c9X3Qa6",
  "status": "confirmed",
  "amount": 0.001,
  "sender": "7BgBvyjrZX8YTqjkKrfbSx9X8QP4NDaP1hj4VKMjqA5s",
  "recipient": "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
  "resource": "https://api.example.com/protected-endpoint",
  "blockHeight": 123456789,
  "confirmations": 32,
  "timestamp": 1697234567890,
  "metadata": {
    "description": "API access payment",
    "referenceId": "req_12345"
  }
}
```

---

### Get Payment History

Get payment transaction history for a specific wallet address. Returns both sent and received payments.

**Endpoint**: `GET /payment/history/{walletAddress}`

**Parameters**:

| Parameter | Type | Required | Description |
| --- | --- | --- | --- |
| `walletAddress` | string | Yes | Wallet address (path parameter) |
| `type` | string | No | Filter by type: "sent", "received", or "all" (default: "all") |
| `limit` | number | No | Number of results (default: 50, max: 500) |
| `offset` | number | No | Pagination offset (default: 0) |

**Example Request**:

```bash
curl -X GET "https://api.layerx402.dev/payment/history/7BgBvyjrZX8YTqjkKrfbSx9X8QP4NDaP1hj4VKMjqA5s?type=all&limit=10" \
  -H "Authorization: Bearer YOUR_API_KEY"
```

**Success Response** (200):

```json
{
  "walletAddress": "7BgBvyjrZX8YTqjkKrfbSx9X8QP4NDaP1hj4VKMjqA5s",
  "total": 245,
  "limit": 10,
  "offset": 0,
  "payments": [
    {
      "paymentId": "pay_5j7sK8K2YaFvS2hE3YgY4z",
      "type": "sent",
      "amount": 0.001,
      "recipient": "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
      "resource": "https://api.example.com/protected-endpoint",
      "status": "confirmed",
      "timestamp": 1697234567890
    },
    {
      "paymentId": "pay_3k9mN7PqRtV8sW2xY",
      "type": "received",
      "amount": 0.005,
      "sender": "9XdT2Km8AaBjRnVyS3fG4wEpQxHnB5cY8zLjP1qR7vN",
      "resource": "/api/my-protected-resource",
      "status": "confirmed",
      "timestamp": 1697234567800
    }
  ]
}
```

## Error Responses

All endpoints may return these common errors:

### 400 Bad Request

Invalid parameters:

```json
{
  "success": false,
  "error": {
    "code": "INVALID_PARAMETERS",
    "message": "Invalid request parameters",
    "details": {
      "amount": "Must be between 0.001 and 100"
    }
  }
}
```

### 401 Unauthorized

```json
{
  "success": false,
  "error": {
    "code": "UNAUTHORIZED",
    "message": "Invalid or missing API key"
  }
}
```

### 429 Rate Limit

```json
{
  "success": false,
  "error": {
    "code": "RATE_LIMIT_EXCEEDED",
    "message": "Rate limit exceeded. Please retry after 30 seconds.",
    "retryAfter": 30
  }
}
```

## Common Error Codes

| Code | Description | Solution |
| --- | --- | --- |
| `INSUFFICIENT_BALANCE` | Not enough funds for payment + fees | Add more funds to wallet |
| `INVALID_PAYMENT_PROOF` | Payment proof invalid or expired | Retry payment with fresh proof |
| `PAYMENT_VERIFICATION_FAILED` | Facilitator could not verify payment | Check payment status and retry |
| `INVALID_RECIPIENT` | Recipient address invalid or doesn't exist | Verify recipient address |
| `RATE_LIMIT` | Too many requests | Slow down requests |

See [Error Handling](../error-handling.md) for complete error documentation.

## Code Examples

### JavaScript/TypeScript

```javascript
const axios = require('axios');

async function sendPayment(walletAddress, recipientAddress, amount, resource) {
  try {
    const response = await axios.post(
      'https://api.layerx402.dev/payment/send',
      {
        walletAddress,
        recipientAddress,
        amount,
        resource,
        metadata: {
          description: 'API access payment',
          referenceId: `req_${Date.now()}`
        },
        priorityFee: 0.0001
      },
      {
        headers: {
          'Authorization': `Bearer ${process.env.LAYERX402_API_KEY}`,
          'Content-Type': 'application/json'
        }
      }
    );

    console.log('Payment successful:', response.data);
    return response.data;
  } catch (error) {
    console.error('Payment failed:', error.response?.data || error.message);
    throw error;
  }
}

// Example: Verify received payment
async function verifyPayment(walletAddress, paymentProof, expectedAmount) {
  try {
    const response = await axios.post(
      'https://api.layerx402.dev/payment/receive',
      {
        walletAddress,
        paymentProof,
        expectedAmount
      },
      {
        headers: {
          'Authorization': `Bearer ${process.env.LAYERX402_API_KEY}`,
          'Content-Type': 'application/json'
        }
      }
    );

    return response.data.verified;
  } catch (error) {
    console.error('Verification failed:', error.response?.data || error.message);
    return false;
  }
}
```

### Python

```python
import requests
import os
from typing import Dict, Any

def send_payment(wallet_address: str, recipient_address: str, amount: float, resource: str) -> Dict[str, Any]:
    """Send a payment over the x402 protocol."""
    url = 'https://api.layerx402.dev/payment/send'
    headers = {
        'Authorization': f'Bearer {os.getenv("LAYERX402_API_KEY")}',
        'Content-Type': 'application/json'
    }
    payload = {
        'walletAddress': wallet_address,
        'recipientAddress': recipient_address,
        'amount': amount,
        'resource': resource,
        'metadata': {
            'description': 'API access payment',
            'referenceId': f'req_{int(time.time())}'
        },
        'priorityFee': 0.0001
    }

    try:
        response = requests.post(url, headers=headers, json=payload)
        response.raise_for_status()
        print('Payment successful:', response.json())
        return response.json()
    except requests.exceptions.RequestException as e:
        print('Payment failed:', e)
        raise

def verify_payment(wallet_address: str, payment_proof: str, expected_amount: float) -> bool:
    """Verify a received payment."""
    url = 'https://api.layerx402.dev/payment/receive'
    headers = {
        'Authorization': f'Bearer {os.getenv("LAYERX402_API_KEY")}',
        'Content-Type': 'application/json'
    }
    payload = {
        'walletAddress': wallet_address,
        'paymentProof': payment_proof,
        'expectedAmount': expected_amount
    }

    try:
        response = requests.post(url, headers=headers, json=payload)
        response.raise_for_status()
        return response.json().get('verified', False)
    except requests.exceptions.RequestException as e:
        print('Verification failed:', e)
        return False
```

## Best Practices

1. **Always Handle Errors**: Implement robust error handling for all payment requests
2. **Verify Payment Proofs**: Always validate payment proofs on the seller side before granting access
3. **Monitor Transactions**: Track payment signatures and verify confirmations on-chain
4. **Respect Rate Limits**: Implement exponential backoff strategies for retries
5. **Secure API Keys**: Never expose keys in client-side code or public repositories
6. **Test with Small Amounts**: Validate integration with minimal payment amounts first
7. **Use Metadata**: Include descriptive metadata for easier payment tracking and reconciliation
8. **Handle 402 Responses**: Implement proper HTTP 402 Payment Required handling in your client
9. **Set Appropriate Timeouts**: Configure reasonable timeout values for payment verification
10. **Cache Payment Proofs**: Store payment proofs temporarily for retry scenarios

## x402 Protocol Integration

Layerx402 is designed to work seamlessly with the x402 protocol. For direct x402 integration:

- **Buyers**: Use client libraries like `x402-fetch` or `x402-axios` to automatically handle 402 responses
- **Sellers**: Implement x402 middleware for your framework (Express, Next.js, FastAPI, etc.)
- **Learn More**: Visit [x402.org](https://x402.org) for protocol documentation and specifications

## Next Steps

- Explore [x402 Protocol Documentation](https://x402.gitbook.io/x402/)
- Set up [WebSocket streaming](websocket-api.md) for real-time payment monitoring
- View [code examples](../examples/) for your language
