# Error Handling

Understanding how to handle errors properly is crucial for building robust applications with the Layerx402 API.

## Error Response Format

All errors follow a consistent JSON structure:

```json
{
  "success": false,
  "error": {
    "code": "ERROR_CODE",
    "message": "Human-readable error description",
    "details": {
      "additionalInfo": "contextual information"
    }
  },
  "timestamp": 1697234567890
}
```

## HTTP Status Codes

| Status Code | Meaning | Common Causes |
| --- | --- | --- |
| 400 | Bad Request | Invalid parameters, malformed JSON |
| 401 | Unauthorized | Missing or invalid API key |
| 403 | Forbidden | Insufficient permissions |
| 404 | Not Found | Resource doesn't exist |
| 429 | Too Many Requests | Rate limit exceeded |
| 500 | Internal Server Error | Server-side issue |
| 503 | Service Unavailable | Temporary service disruption |

## Common Error Codes

### Authentication Errors

#### UNAUTHORIZED

```json
{
  "success": false,
  "error": {
    "code": "UNAUTHORIZED",
    "message": "Invalid or missing API key"
  }
}
```

**Solution**: Verify your API key is correct and included in the Authorization header.

#### FORBIDDEN

```json
{
  "success": false,
  "error": {
    "code": "FORBIDDEN",
    "message": "Insufficient permissions for this resource"
  }
}
```

**Solution**: Upgrade your plan to access this endpoint.

### Validation Errors

#### INVALID_PARAMETERS

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

**Solution**: Check the details object for specific parameter issues.

#### INVALID_PUBLIC_KEY

```json
{
  "success": false,
  "error": {
    "code": "INVALID_PUBLIC_KEY",
    "message": "Public key is not a valid Solana address"
  }
}
```

**Solution**: Ensure you're providing a valid base58-encoded Solana public key.

#### INVALID_MINT

```json
{
  "success": false,
  "error": {
    "code": "INVALID_MINT",
    "message": "Token mint address does not exist or is invalid"
  }
}
```

**Solution**: Verify the token mint address is correct and the token exists.

### Trading Errors

#### INSUFFICIENT_BALANCE

```json
{
  "success": false,
  "error": {
    "code": "INSUFFICIENT_BALANCE",
    "message": "Insufficient SOL balance for transaction",
    "details": {
      "required": 0.01,
      "available": 0.005,
      "includesFees": true
    }
  }
}
```

**Solution**: Ensure your wallet has enough SOL to cover the transaction amount plus fees.

#### SLIPPAGE_EXCEEDED

```json
{
  "success": false,
  "error": {
    "code": "SLIPPAGE_EXCEEDED",
    "message": "Price moved beyond acceptable slippage",
    "details": {
      "requestedSlippage": 10,
      "actualSlippage": 15.2,
      "expectedPrice": 0.00001,
      "currentPrice": 0.0000115
    }
  }
}
```

**Solution**: Increase slippage tolerance or retry the transaction when market is less volatile.

#### INSUFFICIENT_LIQUIDITY

```json
{
  "success": false,
  "error": {
    "code": "INSUFFICIENT_LIQUIDITY",
    "message": "Not enough liquidity to execute this trade",
    "details": {
      "requestedAmount": 100,
      "availableLiquidity": 50
    }
  }
}
```

**Solution**: Reduce trade size or wait for liquidity to improve.

### Rate Limiting

#### RATE_LIMIT_EXCEEDED

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

**Solution**: Implement exponential backoff and respect the `retryAfter` value.

### Network Errors

#### NETWORK_ERROR

```json
{
  "success": false,
  "error": {
    "code": "NETWORK_ERROR",
    "message": "Failed to connect to Solana network",
    "details": {
      "rpcEndpoint": "https://...",
      "retryable": true
    }
  }
}
```

**Solution**: Retry the request. If the issue persists, check the status page.

#### TRANSACTION_FAILED

```json
{
  "success": false,
  "error": {
    "code": "TRANSACTION_FAILED",
    "message": "Transaction simulation failed",
    "details": {
      "logs": ["Program log: Error: ...", "..."],
      "retryable": false
    }
  }
}
```

**Solution**: Review the logs for specific failure reasons. May require parameter adjustments.

## Implementing Error Handling

### JavaScript/TypeScript Example

```javascript
async function executeTrade(params) {
  try {
    const response = await fetch('https://api.layerx402.dev/trade/buy', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${process.env.TURNPIKE_API_KEY}`
      },
      body: JSON.stringify(params)
    });

    const data = await response.json();

    if (!response.ok) {
      throw new APIError(data.error, response.status);
    }

    return data;
  } catch (error) {
    if (error instanceof APIError) {
      switch (error.code) {
        case 'INSUFFICIENT_BALANCE':
          console.error('Not enough SOL:', error.details);
          break;
        case 'SLIPPAGE_EXCEEDED':
          console.warn('Slippage too high, retrying with higher tolerance...');
          return executeTrade({ ...params, slippage: params.slippage * 1.5 });
        case 'RATE_LIMIT_EXCEEDED':
          console.log('Rate limited, waiting...');
          await new Promise(resolve => setTimeout(resolve, error.retryAfter * 1000));
          return executeTrade(params);
        default:
          console.error('API Error:', error);
      }
    } else {
      console.error('Network error:', error);
    }
    throw error;
  }
}

class APIError extends Error {
  constructor(error, status) {
    super(error.message);
    this.code = error.code;
    this.details = error.details;
    this.status = status;
    this.retryAfter = error.retryAfter;
  }
}
```

### Python Example

```python
import requests
import time
from typing import Dict, Any

class Layerx402APIError(Exception):
    def __init__(self, error_data: Dict[str, Any], status_code: int):
        self.code = error_data.get('code')
        self.message = error_data.get('message')
        self.details = error_data.get('details', {})
        self.status_code = status_code
        self.retry_after = error_data.get('retryAfter', 0)
        super().__init__(self.message)

def execute_trade(params: Dict[str, Any], max_retries: int = 3):
    for attempt in range(max_retries):
        try:
            response = requests.post(
                'https://api.layerx402.dev/trade/buy',
                headers={
                    'Content-Type': 'application/json',
                    'Authorization': f'Bearer {os.getenv("TURNPIKE_API_KEY")}'
                },
                json=params
            )

            data = response.json()

            if not response.ok:
                raise Layerx402APIError(data['error'], response.status_code)

            return data

        except Layerx402APIError as e:
            if e.code == 'RATE_LIMIT_EXCEEDED':
                time.sleep(e.retry_after)
                continue
            elif e.code == 'SLIPPAGE_EXCEEDED' and attempt < max_retries - 1:
                params['slippage'] *= 1.5
                continue
            else:
                raise
        except requests.RequestException as e:
            if attempt < max_retries - 1:
                time.sleep(2 ** attempt)
                continue
            raise

    raise Exception(f'Failed after {max_retries} attempts')
```

## Best Practices

1. **Always Check Response Status**: Don't assume requests succeeded
2. **Handle Rate Limits Gracefully**: Implement exponential backoff
3. **Log Errors**: Keep detailed logs for debugging
4. **Retry Transient Errors**: Network issues and rate limits are retryable
5. **Don't Retry Non-Retryable Errors**: Invalid parameters won't succeed on retry
6. **Monitor Error Rates**: High error rates may indicate issues with your integration
7. **Use Timeouts**: Don't let requests hang indefinitely

## Getting Help

If you encounter persistent errors:

1. Check the [Status Page](https://layerx402.dev/status) for system issues
2. Review your code against the [examples](examples/)
3. Contact support with error details and request IDs
4. Join our community on [Discord](#) for help from other developers
