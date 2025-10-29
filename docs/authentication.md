# Authentication

All Layerx402 API requests require authentication using an API key.

## API Key Authentication

Include your API key in the `Authorization` header of every request:

```
Authorization: Bearer YOUR_API_KEY
```

### Example Request

```bash
curl -X GET https://api.layerx402.dev/token/info/MINT_ADDRESS \
  -H "Authorization: Bearer your_api_key_here"
```

## Obtaining an API Key

1. Sign up at [layerx402.dev](https://layerx402.dev)
2. Navigate to your dashboard
3. Go to the "API Keys" section
4. Click "Generate New Key"
5. Copy and securely store your API key

{% hint style="warning" %}
**Important**: Your API key is sensitive. Never share it publicly or commit it to version control.
{% endhint %}

## Security Best Practices

### Use Environment Variables

Store your API key in environment variables:

{% tabs %}
{% tab title="JavaScript/TypeScript" %}
```javascript
// .env file
TURNPIKE_API_KEY=your_api_key_here

// In your code
const apiKey = process.env.TURNPIKE_API_KEY;
```
{% endtab %}

{% tab title="Python" %}
```python
# .env file
TURNPIKE_API_KEY=your_api_key_here

# In your code
import os
api_key = os.getenv('TURNPIKE_API_KEY')
```
{% endtab %}
{% endtabs %}

### Server-Side Only

Never expose your API key in:

- Client-side JavaScript
- Mobile apps
- Public repositories
- Frontend code

Always make API calls from your backend server.

### Rotate Keys Regularly

Periodically rotate your API keys:

1. Generate a new API key
2. Update your application with the new key
3. Delete the old key once migration is complete

## Rate Limits by Plan

Different API keys have different rate limits based on your plan:

| Plan | Rate Limit |
| --- | --- |
| Standard | 100 requests/minute |
| Premium | 1,000 requests/minute |
| Enterprise | Custom limits |

See [Rate Limits](rate-limits.md) for more information.

## Testing Authentication

To verify your API key is working:

```bash
curl -X GET https://api.layerx402.dev/portfolio/YOUR_PUBLIC_KEY \
  -H "Authorization: Bearer YOUR_API_KEY"
```

A successful response indicates your API key is valid.

## Authentication Errors

### 401 Unauthorized

Your API key is missing or invalid:

```json
{
  "error": {
    "code": "UNAUTHORIZED",
    "message": "Invalid or missing API key"
  }
}
```

**Solution**: Check that you're including the correct API key in the Authorization header.

### 403 Forbidden

Your API key doesn't have permission for the requested resource:

```json
{
  "error": {
    "code": "FORBIDDEN",
    "message": "Insufficient permissions"
  }
}
```

**Solution**: Ensure your plan includes access to the endpoint you're trying to use.

## WebSocket Authentication

For WebSocket connections, authentication is handled differently. See [WebSocket API](api-reference/websocket-api.md#authentication) for details.
