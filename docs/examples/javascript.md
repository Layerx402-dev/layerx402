# JavaScript/TypeScript Examples

Complete examples for integrating Layerx402 API with JavaScript and TypeScript.

## Installation

```bash
npm install @solana/web3.js
# Optional: for WebSocket reconnection
npm install ws
```

## Lightning API Examples

### Execute Buy Trade

```typescript
import fetch from 'node-fetch';

interface BuyTradeParams {
  publicKey: string;
  mint: string;
  amount: number;
  slippage?: number;
  priorityFee?: number;
}

async function executeBuyTrade(params: BuyTradeParams) {
  try {
    const response = await fetch('https://api.layerx402.dev/trade/buy', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${process.env.TURNPIKE_API_KEY}`
      },
      body: JSON.stringify({
        ...params,
        slippage: params.slippage || 10,
        priorityFee: params.priorityFee || 0.0001
      })
    });

    if (!response.ok) {
      const error = await response.json();
      throw new Error(`Trade failed: ${error.error.message}`);
    }

    const data = await response.json();
    console.log('Trade successful!');
    console.log('Signature:', data.signature);
    console.log('Tokens received:', data.tokensReceived);
    console.log('Effective price:', data.effectivePrice);

    return data;
  } catch (error) {
    console.error('Error executing trade:', error);
    throw error;
  }
}

// Usage
executeBuyTrade({
  publicKey: 'YOUR_WALLET_PUBLIC_KEY',
  mint: 'EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v',
  amount: 0.01
});
```

### Get Token Information

```typescript
async function getTokenInfo(mint: string) {
  const response = await fetch(
    `https://api.layerx402.dev/token/info/${mint}`,
    {
      headers: {
        'Authorization': `Bearer ${process.env.TURNPIKE_API_KEY}`
      }
    }
  );

  if (!response.ok) {
    throw new Error('Failed to fetch token info');
  }

  const data = await response.json();
  return data;
}

// Usage
const tokenInfo = await getTokenInfo('EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v');
console.log(`${tokenInfo.symbol}: $${tokenInfo.price}`);
```

### Get Portfolio

```typescript
async function getPortfolio(publicKey: string) {
  const response = await fetch(
    `https://api.layerx402.dev/portfolio/${publicKey}`,
    {
      headers: {
        'Authorization': `Bearer ${process.env.TURNPIKE_API_KEY}`
      }
    }
  );

  const data = await response.json();
  console.log('Total value:', data.totalValueUsd);
  console.log('Tokens:', data.tokens.length);

  return data;
}
```

## Local Transaction API Examples

### Build and Sign Transaction

```typescript
import { Connection, Transaction, Keypair } from '@solana/web3.js';
import * as bs58 from 'bs58';

async function buildAndSignTrade() {
  // Your wallet (load from secure storage)
  const secretKey = bs58.decode(process.env.WALLET_PRIVATE_KEY);
  const wallet = Keypair.fromSecretKey(secretKey);

  // 1. Build transaction via API
  const response = await fetch('https://api.layerx402.dev/transaction/buy', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      'Authorization': `Bearer ${process.env.TURNPIKE_API_KEY}`
    },
    body: JSON.stringify({
      publicKey: wallet.publicKey.toBase58(),
      mint: 'TOKEN_MINT_ADDRESS',
      amount: 0.01,
      slippage: 10
    })
  });

  const data = await response.json();

  // 2. Deserialize transaction
  const transaction = Transaction.from(
    Buffer.from(data.transaction, 'base64')
  );

  // 3. Connect to RPC
  const connection = new Connection('https://api.mainnet-beta.solana.com');

  // 4. Sign transaction
  transaction.sign(wallet);

  // 5. Send transaction
  const signature = await connection.sendRawTransaction(
    transaction.serialize()
  );

  console.log('Transaction sent:', signature);

  // 6. Confirm transaction
  const confirmation = await connection.confirmTransaction(signature);
  console.log('Transaction confirmed:', confirmation);

  return signature;
}
```

### Browser Wallet Integration

```typescript
import { useWallet, useConnection } from '@solana/wallet-adapter-react';
import { Transaction } from '@solana/web3.js';

function TradingComponent() {
  const { publicKey, signTransaction } = useWallet();
  const { connection } = useConnection();

  const executeTrade = async () => {
    if (!publicKey || !signTransaction) {
      throw new Error('Wallet not connected');
    }

    // Build transaction
    const response = await fetch('https://api.layerx402.dev/transaction/buy', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${process.env.NEXT_PUBLIC_TURNPIKE_API_KEY}`
      },
      body: JSON.stringify({
        publicKey: publicKey.toBase58(),
        mint: 'TOKEN_MINT_ADDRESS',
        amount: 0.01,
        slippage: 10
      })
    });

    const data = await response.json();

    // Deserialize and sign
    const transaction = Transaction.from(
      Buffer.from(data.transaction, 'base64')
    );

    const signedTx = await signTransaction(transaction);

    // Send
    const signature = await connection.sendRawTransaction(
      signedTx.serialize()
    );

    // Confirm
    await connection.confirmTransaction(signature);

    return signature;
  };

  return (
    <button onClick={executeTrade} disabled={!publicKey}>
      Execute Trade
    </button>
  );
}
```

## WebSocket Examples

### Basic WebSocket Connection

```typescript
class Layerx402Stream {
  private ws: WebSocket | null = null;
  private apiKey: string;

  constructor(apiKey: string) {
    this.apiKey = apiKey;
  }

  connect() {
    this.ws = new WebSocket('wss://layerx402.dev/api/data');

    this.ws.onopen = () => {
      console.log('Connected');

      // Authenticate
      this.send({
        method: 'authenticate',
        apiKey: this.apiKey
      });

      // Subscribe to trades
      this.send({
        method: 'subscribe',
        keys: ['trades']
      });
    };

    this.ws.onmessage = (event) => {
      const data = JSON.parse(event.data);
      this.handleMessage(data);
    };

    this.ws.onerror = (error) => {
      console.error('Error:', error);
    };

    this.ws.onclose = () => {
      console.log('Disconnected');
      // Reconnect after 5 seconds
      setTimeout(() => this.connect(), 5000);
    };
  }

  private send(data: any) {
    if (this.ws?.readyState === WebSocket.OPEN) {
      this.ws.send(JSON.stringify(data));
    }
  }

  private handleMessage(data: any) {
    if (data.type === 'trade') {
      console.log('Trade:', {
        mint: data.mint,
        side: data.side,
        amount: data.amount,
        price: data.price
      });
    }
  }

  disconnect() {
    this.ws?.close();
  }
}

// Usage
const stream = new Layerx402Stream(process.env.TURNPIKE_API_KEY);
stream.connect();
```

### React Hook for WebSocket

```typescript
import { useEffect, useState } from 'react';

interface Trade {
  mint: string;
  signature: string;
  amount: number;
  price: number;
  side: 'buy' | 'sell';
  timestamp: number;
}

export function useLayerx402Trades(apiKey: string) {
  const [trades, setTrades] = useState<Trade[]>([]);
  const [connected, setConnected] = useState(false);

  useEffect(() => {
    const ws = new WebSocket('wss://layerx402.dev/api/data');

    ws.onopen = () => {
      setConnected(true);

      ws.send(JSON.stringify({
        method: 'authenticate',
        apiKey
      }));

      ws.send(JSON.stringify({
        method: 'subscribe',
        keys: ['trades']
      }));
    };

    ws.onmessage = (event) => {
      const data = JSON.parse(event.data);

      if (data.type === 'trade') {
        setTrades(prev => [data, ...prev].slice(0, 50));
      }
    };

    ws.onclose = () => {
      setConnected(false);
    };

    return () => ws.close();
  }, [apiKey]);

  return { trades, connected };
}

// Usage in component
function TradeFeed() {
  const { trades, connected } = useLayerx402Trades(
    process.env.NEXT_PUBLIC_TURNPIKE_API_KEY
  );

  return (
    <div>
      <div>Status: {connected ? 'Connected' : 'Disconnected'}</div>
      <ul>
        {trades.map(trade => (
          <li key={trade.signature}>
            {trade.side} {trade.amount} @ ${trade.price}
          </li>
        ))}
      </ul>
    </div>
  );
}
```

## Complete Trading Bot Example

```typescript
import { Connection, Keypair } from '@solana/web3.js';

class TradingBot {
  private apiKey: string;
  private wallet: Keypair;
  private ws: WebSocket | null = null;

  constructor(apiKey: string, wallet: Keypair) {
    this.apiKey = apiKey;
    this.wallet = wallet;
  }

  async start() {
    // Connect to WebSocket for price monitoring
    this.ws = new WebSocket('wss://layerx402.dev/api/data');

    this.ws.onopen = () => {
      this.ws?.send(JSON.stringify({
        method: 'authenticate',
        apiKey: this.apiKey
      }));

      this.ws?.send(JSON.stringify({
        method: 'subscribe',
        keys: ['priceUpdates']
      }));
    };

    this.ws.onmessage = (event) => {
      const data = JSON.parse(event.data);

      if (data.type === 'priceUpdate') {
        this.onPriceUpdate(data);
      }
    };
  }

  private async onPriceUpdate(data: any) {
    // Simple strategy: buy on 5% dip
    if (data.priceChange < -5) {
      console.log('Price dipped, buying...');
      await this.executeBuy(data.mint, 0.01);
    }
  }

  private async executeBuy(mint: string, amount: number) {
    try {
      const response = await fetch('https://api.layerx402.dev/trade/buy', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'Authorization': `Bearer ${this.apiKey}`
        },
        body: JSON.stringify({
          publicKey: this.wallet.publicKey.toBase58(),
          mint,
          amount,
          slippage: 15
        })
      });

      const data = await response.json();
      console.log('Buy executed:', data.signature);
    } catch (error) {
      console.error('Buy failed:', error);
    }
  }

  stop() {
    this.ws?.close();
  }
}

// Usage
const bot = new TradingBot(
  process.env.TURNPIKE_API_KEY,
  wallet
);

bot.start();
```

## Error Handling Example

```typescript
class APIError extends Error {
  code: string;
  details: any;

  constructor(error: any) {
    super(error.message);
    this.code = error.code;
    this.details = error.details;
  }
}

async function safeExecuteTrade(params: any, maxRetries = 3) {
  for (let attempt = 0; attempt < maxRetries; attempt++) {
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
        throw new APIError(data.error);
      }

      return data;
    } catch (error) {
      if (error instanceof APIError) {
        if (error.code === 'RATE_LIMIT_EXCEEDED') {
          await new Promise(resolve => setTimeout(resolve, 2000 * (attempt + 1)));
          continue;
        } else if (error.code === 'SLIPPAGE_EXCEEDED' && attempt < maxRetries - 1) {
          params.slippage *= 1.5;
          continue;
        }
      }
      throw error;
    }
  }
}
```
