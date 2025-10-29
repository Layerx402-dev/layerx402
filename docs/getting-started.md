# Getting Started

This guide will help you integrate x402 payments using Layerx402's infrastructure in minutes.

## Prerequisites

Before you begin, ensure you have:

- Basic understanding of HTTP APIs
- A wallet address to receive x402 payments
- Familiarity with the [x402 protocol](https://x402.gitbook.io/x402) (helpful but not required)

## What You'll Build

In this guide, you'll learn to:

1. Get your Layerx402 API key
2. Verify an x402 payment
3. Settle a verified payment
4. Monitor payments in real-time

## Installation

No SDK installation required! Layerx402 works with standard HTTP requests and WebSocket connections.

However, if you want to integrate x402 into your application, you may want these libraries:

{% tabs %}
{% tab title="JavaScript/TypeScript" %}
```bash
npm install axios  # or use fetch
npm install ws     # for WebSocket support
```
{% endtab %}

{% tab title="Python" %}
```bash
pip install requests
pip install websocket-client
```
{% endtab %}

{% tab title="Rust" %}
```bash
cargo add reqwest
cargo add tokio-tungstenite  # for WebSocket support
```
{% endtab %}
{% endtabs %}

## Get Your API Key

1. Visit [layerx402.dev](https://layerx402.dev)
2. Sign up for an account
3. Navigate to the API Keys section in your dashboard
4. Generate a new API key
5. Store your API key securely (never commit it to version control!)

## Your First x402 Payment Verification

Here's how to verify an x402 payment:

{% tabs %}
{% tab title="cURL" %}
```bash
curl -X POST https://api.layerx402.dev/v1/payments/verify \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer YOUR_API_KEY" \
  -d '{
    "payment_proof": "BASE64_ENCODED_PAYMENT_PROOF",
    "amount": 1000000,
    "recipient": "YOUR_WALLET_ADDRESS",
    "network": "solana"
  }'
```
{% endtab %}

{% tab title="JavaScript" %}
```javascript
const response = await fetch('https://api.layerx402.dev/v1/payments/verify', {
  method: 'POST',
  headers: {
    'Content-Type': 'application/json',
    'Authorization': 'Bearer YOUR_API_KEY'
  },
  body: JSON.stringify({
    payment_proof: 'BASE64_ENCODED_PAYMENT_PROOF',
    amount: 1000000,
    recipient: 'YOUR_WALLET_ADDRESS',
    network: 'solana'
  })
});

const data = await response.json();
console.log('Payment verified:', data);
```
{% endtab %}

{% tab title="Python" %}
```python
import requests

response = requests.post(
    'https://api.layerx402.dev/v1/payments/verify',
    headers={
        'Content-Type': 'application/json',
        'Authorization': 'Bearer YOUR_API_KEY'
    },
    json={
        'payment_proof': 'BASE64_ENCODED_PAYMENT_PROOF',
        'amount': 1000000,
        'recipient': 'YOUR_WALLET_ADDRESS',
        'network': 'solana'
    }
)

data = response.json()
print('Payment verified:', data)
```
{% endtab %}
{% endtabs %}

## Settle a Verified Payment

After verifying a payment, settle it to complete the transaction:

```bash
curl -X POST https://api.layerx402.dev/v1/payments/settle \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer YOUR_API_KEY" \
  -d '{
    "payment_id": "pay_abc123xyz"
  }'
```

## Complete Integration Example

Here's a complete example of handling an x402 payment request:

```javascript
// Your API server receives a 402 Payment Required request
app.get('/api/protected-resource', async (req, res) => {
  const paymentProof = req.headers['x-payment-proof'];

  if (!paymentProof) {
    // No payment provided, return 402
    return res.status(402).json({
      error: 'Payment Required',
      amount: 1000000,
      recipient: 'YOUR_WALLET_ADDRESS',
      network: 'solana'
    });
  }

  try {
    // Verify the payment with Layerx402
    const verification = await fetch('https://api.layerx402.dev/v1/payments/verify', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'Authorization': 'Bearer YOUR_API_KEY'
      },
      body: JSON.stringify({
        payment_proof: paymentProof,
        amount: 1000000,
        recipient: 'YOUR_WALLET_ADDRESS',
        network: 'solana'
      })
    });

    const result = await verification.json();

    if (result.verified) {
      // Settle the payment
      await fetch('https://api.layerx402.dev/v1/payments/settle', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'Authorization': 'Bearer YOUR_API_KEY'
        },
        body: JSON.stringify({
          payment_id: result.payment_id
        })
      });

      // Payment verified and settled, return the resource
      return res.json({ data: 'Your protected resource' });
    } else {
      return res.status(402).json({ error: 'Invalid payment' });
    }
  } catch (error) {
    return res.status(500).json({ error: 'Payment verification failed' });
  }
});
```

## Next Steps

- Explore the [Payment Verification API](api-reference/payment-verification-api.md) for detailed documentation
- Learn about [WebSocket streaming](api-reference/websocket-api.md) for real-time payment monitoring
- Check out [code examples](examples/) for your preferred language
- Read the [x402 protocol docs](https://x402.gitbook.io/x402) for deeper understanding

## Best Practices

1. **Secure Your API Key** - Never expose your API key in client-side code or public repositories
2. **Validate Payment Amounts** - Always check that the payment amount matches your expected price
3. **Handle Errors Gracefully** - Implement proper error handling for network issues and API errors
4. **Monitor Payments** - Use WebSocket streams to track payment status in real-time
5. **Respect Rate Limits** - Monitor your usage and upgrade your plan if needed
6. **Test Thoroughly** - Test with small amounts on testnets before going to production

## Understanding the x402 Flow

1. **Client requests a resource** → Your server responds with `402 Payment Required` including payment details
2. **Client creates payment** → Client constructs and signs an x402 payment transaction
3. **Client sends payment proof** → Client includes payment proof in subsequent request
4. **Server verifies payment** → Your server uses Layerx402 to verify the payment
5. **Server settles payment** → After verification, settle the payment with Layerx402
6. **Server returns resource** → Payment complete, return the requested resource

With Layerx402, steps 4 and 5 are handled in milliseconds with our high-performance infrastructure!
