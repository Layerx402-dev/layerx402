# Frequently Asked Questions

## General

### What is Layerx402?

Layerx402 is a high-performance trading API for Solana tokens, offering sub-25ms execution times, institutional-grade security, and real-time market data streaming.

### How do I get started?

1. Sign up at [layerx402.dev](https://layerx402.dev)
2. Generate an API key from your dashboard
3. Choose your integration method (Lightning, Local, or WebSocket)
4. Start building with our [Getting Started guide](../getting-started.md)

### Which integration method should I use?

- **Lightning API**: Best for rapid development and automated trading bots
- **Local Transaction API**: Best for maximum security and institutional applications
- **WebSocket API**: Best for real-time data and monitoring

You can use multiple methods in the same application.

## Authentication & Security

### How do I keep my API key secure?

- Store API keys in environment variables, never in code
- Only use API keys on the server-side, never in client-side code
- Rotate keys regularly
- Never commit keys to version control

### Can I use the API from the browser?

You should never expose your API key in client-side JavaScript. Instead:
- Make API calls from your backend server
- Use the Local Transaction API to build transactions server-side
- Have users sign transactions in their browser wallet

### What happens if my API key is compromised?

Immediately:
1. Go to your dashboard
2. Delete the compromised key
3. Generate a new key
4. Update your application with the new key

## Rate Limits & Pricing

### What are the rate limits?

| Plan | Requests/Minute | WebSocket Connections |
| --- | --- | --- |
| Standard | 100 | 5 |
| Premium | 1,000 | 20 |
| Enterprise | Custom | Custom |

See [Rate Limits](../rate-limits.md) for details.

### How do I upgrade my plan?

Visit [layerx402.dev/pricing](https://layerx402.dev) and select your desired plan. Upgrades take effect immediately.

### What happens if I exceed my rate limit?

You'll receive a `429 Too Many Requests` response. Implement exponential backoff and retry logic. See [Error Handling](../error-handling.md).

## Trading

### What's the minimum trade amount?

- Minimum: 0.001 SOL
- Maximum: 100 SOL (can be increased for Enterprise plans)

### How is slippage calculated?

Slippage is the maximum acceptable price difference between when you submit the trade and when it executes. Set it as a percentage (e.g., 10 = 10%).

### What's the typical confirmation time?

Trades typically confirm in 2-5 seconds on Solana. You can monitor confirmation status using the transaction signature.

### Can I cancel a transaction?

Once submitted to the blockchain, transactions cannot be cancelled. However, transactions that haven't been processed yet will expire after their `lastValidBlockHeight`.

## WebSocket

### How do I handle disconnections?

Implement automatic reconnection with exponential backoff. See [WebSocket Examples](../examples/javascript.md#reconnection-logic).

### What's the difference between subscribing to 'trades' vs a specific token?

- `trades`: All trades across all tokens
- `{MINT_ADDRESS}`: Only events for that specific token

### Can I subscribe to multiple tokens?

Yes! Include multiple mint addresses in the `keys` array when subscribing.

## Local Transactions

### Why would I use local transactions?

Local transactions provide:
- Complete control over transaction signing
- Ability to use your own RPC endpoint
- Full transparency - you can inspect transactions before signing
- Maximum security - private keys never leave your system

### How long are transactions valid?

Transactions expire at the `lastValidBlockHeight` returned in the response, typically 60 seconds after creation. Send them promptly.

### Can I modify the transaction before signing?

While technically possible, we strongly recommend against it as modifications may cause the transaction to fail.

## Errors & Troubleshooting

### I'm getting "INSUFFICIENT_BALANCE" errors

Ensure your wallet has enough SOL to cover:
- The trade amount
- Transaction fees (typically 0.000005 SOL)
- Priority fees (if specified)

### I'm getting "SLIPPAGE_EXCEEDED" errors

The token price moved beyond your acceptable slippage. Try:
- Increasing your slippage tolerance
- Trading when the market is less volatile
- Using smaller trade amounts

### My transaction is stuck as "pending"

Solana transactions typically confirm in seconds. If pending for >30 seconds:
1. Check the transaction on a Solana explorer
2. Verify the transaction signature is correct
3. Check network congestion at [Status Page](https://layerx402.dev/status)

### I'm getting "INVALID_MINT" errors

- Verify the token mint address is correct
- Ensure the token exists on Solana mainnet
- Check if the token has sufficient liquidity

## Support

### How do I get help?

- **Documentation**: Search these docs
- **Status Page**: Check [layerx402.dev/status](https://layerx402.dev/status)
- **Twitter**: [@layerx402](https://x.com/layerx402)
- **Email**: support@layerx402.dev

### Is there a Discord or community?

Follow [@layerx402](https://x.com/layerx402) on Twitter for community updates and announcements.

### How do I report bugs?

Email support@layerx402.dev with:
- Detailed description of the issue
- Steps to reproduce
- Request IDs or transaction signatures
- Your plan tier

## Best Practices

### Should I implement retry logic?

Yes! Implement exponential backoff for:
- Rate limit errors (always retry)
- Network errors (retry with backoff)
- Slippage errors (retry with increased slippage)

Don't retry:
- Invalid parameters
- Insufficient balance
- Invalid mint addresses

### How should I handle transaction confirmations?

Always wait for transaction confirmation before considering a trade complete. Use the signature to query confirmation status.

### What's the best way to monitor my API usage?

Check your dashboard at [layerx402.dev/dashboard](https://layerx402.dev) for:
- Real-time usage metrics
- Historical data
- Rate limit consumption
- Cost analysis
