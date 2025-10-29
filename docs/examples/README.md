# Examples

Complete code examples for integrating with the Layerx402 API in various programming languages.

## Quick Start Examples

### Simple Buy Trade

The simplest way to execute a buy trade:

{% tabs %}
{% tab title="JavaScript" %}
```javascript
const response = await fetch('https://api.layerx402.dev/trade/buy', {
  method: 'POST',
  headers: {
    'Content-Type': 'application/json',
    'Authorization': `Bearer ${process.env.TURNPIKE_API_KEY}`
  },
  body: JSON.stringify({
    publicKey: 'YOUR_WALLET_PUBLIC_KEY',
    mint: 'TOKEN_MINT_ADDRESS',
    amount: 0.01,
    slippage: 10
  })
});

const data = await response.json();
console.log('Trade successful:', data.signature);
```
{% endtab %}

{% tab title="Python" %}
```python
import requests
import os

response = requests.post(
    'https://api.layerx402.dev/trade/buy',
    headers={
        'Content-Type': 'application/json',
        'Authorization': f'Bearer {os.getenv("TURNPIKE_API_KEY")}'
    },
    json={
        'publicKey': 'YOUR_WALLET_PUBLIC_KEY',
        'mint': 'TOKEN_MINT_ADDRESS',
        'amount': 0.01,
        'slippage': 10
    }
)

data = response.json()
print(f'Trade successful: {data["signature"]}')
```
{% endtab %}

{% tab title="cURL" %}
```bash
curl -X POST https://api.layerx402.dev/trade/buy \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TURNPIKE_API_KEY" \
  -d '{
    "publicKey": "YOUR_WALLET_PUBLIC_KEY",
    "mint": "TOKEN_MINT_ADDRESS",
    "amount": 0.01,
    "slippage": 10
  }'
```
{% endtab %}
{% endtabs %}

## Language-Specific Guides

- [JavaScript/TypeScript](javascript.md) - Complete examples for Node.js and browser
- [Python](python.md) - Examples using requests and asyncio
- [Rust](rust.md) - Examples using reqwest and tokio

## Integration Patterns

### Trading Bot

Build an automated trading bot that monitors prices and executes trades:

```javascript
// See JavaScript examples for full implementation
```

### Portfolio Tracker

Track wallet holdings and values in real-time:

```javascript
// See JavaScript examples for full implementation
```

### WebSocket Monitor

Stream real-time market data:

```javascript
// See JavaScript examples for full implementation
```

## Best Practices

1. **Environment Variables**: Store API keys in environment variables
2. **Error Handling**: Always implement robust error handling
3. **Rate Limiting**: Respect rate limits with backoff strategies
4. **Testing**: Test with small amounts first
5. **Logging**: Keep detailed logs for debugging
6. **Monitoring**: Track transaction confirmations

## Next Steps

Choose your preferred language and dive into detailed examples:

- [JavaScript/TypeScript Examples →](javascript.md)
- [Python Examples →](python.md)
- [Rust Examples →](rust.md)
