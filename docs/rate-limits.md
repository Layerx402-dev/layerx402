# Rate Limits

Layerx402 implements rate limiting to ensure fair usage and maintain service quality for all users.

## Rate Limit Tiers

| Plan | Requests per Minute | Burst Limit |
| --- | --- | --- |
| Standard | 100 | 120 |
| Premium | 1,000 | 1,200 |
| Enterprise | Custom | Custom |

{% hint style="info" %}
**Burst Limit**: Short-term allowance for occasional traffic spikes above your base rate limit.
{% endhint %}

## Rate Limit Headers

Every API response includes rate limit information in the headers:

```
X-RateLimit-Limit: 100
X-RateLimit-Remaining: 87
X-RateLimit-Reset: 1697234567
```

| Header | Description |
| --- | --- |
| `X-RateLimit-Limit` | Maximum requests allowed in the current window |
| `X-RateLimit-Remaining` | Requests remaining in the current window |
| `X-RateLimit-Reset` | Unix timestamp when the rate limit resets |

## Handling Rate Limits

### Rate Limit Exceeded Response

When you exceed your rate limit, you'll receive a `429 Too Many Requests` response:

```json
{
  "success": false,
  "error": {
    "code": "RATE_LIMIT_EXCEEDED",
    "message": "Rate limit exceeded. Please retry after 30 seconds.",
    "retryAfter": 30
  },
  "timestamp": 1697234567890
}
```

### Best Practices

#### 1. Implement Exponential Backoff

```javascript
async function makeRequestWithBackoff(url, options, maxRetries = 3) {
  for (let i = 0; i < maxRetries; i++) {
    const response = await fetch(url, options);

    if (response.status === 429) {
      const retryAfter = response.headers.get('Retry-After') || Math.pow(2, i);
      await new Promise(resolve => setTimeout(resolve, retryAfter * 1000));
      continue;
    }

    return response;
  }

  throw new Error('Max retries exceeded');
}
```

#### 2. Monitor Rate Limit Headers

```javascript
function checkRateLimit(response) {
  const remaining = parseInt(response.headers.get('X-RateLimit-Remaining'));
  const limit = parseInt(response.headers.get('X-RateLimit-Limit'));

  if (remaining < limit * 0.1) {
    console.warn('Approaching rate limit:', remaining, 'requests remaining');
  }
}
```

#### 3. Batch Requests When Possible

Instead of making multiple individual requests, batch them when the API supports it:

```javascript
// Bad: Multiple requests
for (const mint of mints) {
  await getTokenInfo(mint);
}

// Good: Single batched request (if supported)
await getTokenInfoBatch(mints);
```

#### 4. Cache Responses

Cache API responses when data doesn't change frequently:

```javascript
const cache = new Map();

async function getTokenInfoCached(mint, ttl = 60000) {
  const cached = cache.get(mint);

  if (cached && Date.now() - cached.timestamp < ttl) {
    return cached.data;
  }

  const data = await getTokenInfo(mint);
  cache.set(mint, { data, timestamp: Date.now() });

  return data;
}
```

## WebSocket Rate Limits

WebSocket connections have separate rate limits:

| Plan | Concurrent Connections | Messages per Second |
| --- | --- | --- |
| Standard | 5 | 50 |
| Premium | 20 | 200 |
| Enterprise | Custom | Custom |

## Endpoint-Specific Limits

Some endpoints have additional rate limits:

### Trading Endpoints

- `/api/trade/buy` and `/api/trade/sell`: Limited to 10 trades/minute on Standard plan
- No additional limits on Premium and Enterprise plans

### Data Endpoints

- Token info and portfolio endpoints follow standard rate limits
- No endpoint-specific restrictions

## Upgrading Your Plan

If you consistently hit rate limits, consider upgrading:

1. Visit [layerx402.dev/pricing](https://layerx402.dev)
2. Compare plans and select one that fits your needs
3. Upgrade instantly with no downtime
4. Your new rate limits take effect immediately

## Monitoring Usage

Track your API usage in the dashboard:

1. Log in to [layerx402.dev](https://layerx402.dev)
2. Navigate to "API Usage"
3. View real-time metrics and historical data
4. Set up alerts for approaching limits

## Enterprise Plans

Need custom rate limits? Contact our sales team for enterprise pricing:

- Custom rate limits tailored to your needs
- Dedicated infrastructure
- Priority support
- SLA guarantees

Contact: [enterprise@layerx402.dev](mailto:enterprise@layerx402.dev)
