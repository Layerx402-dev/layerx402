# Local Transaction API

The Local Transaction API provides maximum security and control for x402 payments. We build the payment transaction for you, but you sign and broadcast it using your own wallet and RPC endpoint.

## Overview

**Base URL**: `https://api.layerx402.dev`

**Flow**:
1. Request a payment transaction from Layerx402
2. Receive base64-encoded unsigned transaction
3. Sign the transaction with your wallet
4. Broadcast to blockchain using your RPC endpoint

## Why Use Local Transactions?

- **Full Control**: You sign and send payment transactions yourself
- **Custom RPC**: Use your own blockchain RPC infrastructure
- **Maximum Security**: Private keys never leave your system
- **Transaction Transparency**: Inspect payment transactions before signing
- **Institutional Grade**: Meets compliance requirements for payment processing
- **Multi-Chain Support**: Works with EVM and Solana chains

## Endpoints

### Build Send Payment Transaction

Build an unsigned transaction for sending a payment over the x402 protocol.

**Endpoint**: `POST /transaction/send`

**Request Body**:

| Parameter | Type | Required | Description |
| --- | --- | --- | --- |
| `walletAddress` | string | Yes | Your wallet address (EVM or Solana format) |
| `recipientAddress` | string | Yes | Payment recipient's wallet address |
| `amount` | number | Yes | Payment amount in native currency |
| `resource` | string | No | Protected resource URL being accessed |
| `metadata` | object | No | Additional payment metadata |
| `priorityFee` | number | No | Network priority fee (default: auto) |

**Example Request**:

```bash
curl -X POST https://api.layerx402.dev/transaction/send \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer YOUR_API_KEY" \
  -d '{
    "walletAddress": "7BgBvyjrZX8YTqjkKrfbSx9X8QP4NDaP1hj4VKMjqA5s",
    "recipientAddress": "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
    "amount": 0.001,
    "resource": "https://api.example.com/protected-endpoint",
    "metadata": {
      "description": "API access payment"
    },
    "priorityFee": 0.0001
  }'
```

**Success Response** (200):

```json
{
  "transaction": "BASE64_ENCODED_TRANSACTION_DATA",
  "lastValidBlockHeight": 123456789,
  "paymentProof": "0x7f8a3b2c...",
  "expiresAt": 1697234627890,
  "estimatedConfirmation": "2-5 seconds"
}
```

---

### Build Receive Payment Transaction

Build an unsigned transaction for receiving and verifying a payment on the seller side.

**Endpoint**: `POST /transaction/receive`

**Request Body**:

| Parameter | Type | Required | Description |
| --- | --- | --- | --- |
| `walletAddress` | string | Yes | Your receiving wallet address |
| `paymentProof` | string | Yes | Cryptographic payment proof from buyer |
| `expectedAmount` | number | Yes | Expected payment amount |
| `resource` | string | No | Protected resource being accessed |
| `facilitatorUrl` | string | No | Facilitator URL for verification |
| `priorityFee` | number | No | Network priority fee |

**Example Request**:

```bash
curl -X POST https://api.layerx402.dev/transaction/receive \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer YOUR_API_KEY" \
  -d '{
    "walletAddress": "7BgBvyjrZX8YTqjkKrfbSx9X8QP4NDaP1hj4VKMjqA5s",
    "paymentProof": "0x7f8a3b2c1d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b",
    "expectedAmount": 0.001,
    "resource": "/api/protected-endpoint"
  }'
```

**Success Response** (200):

```json
{
  "transaction": "BASE64_ENCODED_TRANSACTION_DATA",
  "lastValidBlockHeight": 123456789,
  "verified": true,
  "paymentId": "pay_3k9mN7PqRtV8sW2xY",
  "expiresAt": 1697234627890
}
```

## Signing and Sending

After receiving the unsigned transaction, you need to sign and broadcast it.

### JavaScript/TypeScript Example

```javascript
import { Connection, Transaction, Keypair } from '@solana/web3.js';

async function signAndSendTransaction(
  base64Transaction: string,
  wallet: Keypair,
  rpcEndpoint: string
) {
  // Create connection to your RPC
  const connection = new Connection(rpcEndpoint);

  // Deserialize the transaction
  const transaction = Transaction.from(
    Buffer.from(base64Transaction, 'base64')
  );

  // Sign the transaction with your wallet
  transaction.sign(wallet);

  // Send the transaction
  const signature = await connection.sendRawTransaction(
    transaction.serialize()
  );

  console.log('Transaction sent:', signature);

  // Wait for confirmation
  const confirmation = await connection.confirmTransaction(signature);

  return {
    signature,
    confirmation
  };
}

// Usage
async function executeSendPayment() {
  // 1. Get unsigned payment transaction from Layerx402
  const response = await fetch('https://api.layerx402.dev/transaction/send', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      'Authorization': `Bearer ${process.env.LAYERX402_API_KEY}`
    },
    body: JSON.stringify({
      walletAddress: wallet.publicKey.toBase58(),
      recipientAddress: 'RECIPIENT_WALLET_ADDRESS',
      amount: 0.001,
      resource: 'https://api.example.com/protected-endpoint',
      metadata: {
        description: 'API access payment'
      }
    })
  });

  const data = await response.json();

  // 2. Sign and send
  const result = await signAndSendTransaction(
    data.transaction,
    wallet,
    'https://api.mainnet-beta.solana.com'
  );

  console.log('Payment completed:', result);
  console.log('Payment proof:', data.paymentProof);
}
```

### Using with Wallet Adapter (Browser)

```javascript
import { useWallet, useConnection } from '@solana/wallet-adapter-react';
import { Transaction } from '@solana/web3.js';

function PaymentComponent() {
  const { publicKey, signTransaction, sendTransaction } = useWallet();
  const { connection } = useConnection();

  const executePayment = async (recipientAddress, amount, resource) => {
    if (!publicKey || !signTransaction) {
      throw new Error('Wallet not connected');
    }

    // 1. Get unsigned payment transaction
    const response = await fetch('https://api.layerx402.dev/transaction/send', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${process.env.LAYERX402_API_KEY}`
      },
      body: JSON.stringify({
        walletAddress: publicKey.toBase58(),
        recipientAddress,
        amount,
        resource,
        metadata: {
          description: 'API access payment',
          timestamp: Date.now()
        }
      })
    });

    const data = await response.json();

    // 2. Deserialize transaction
    const transaction = Transaction.from(
      Buffer.from(data.transaction, 'base64')
    );

    // 3. Sign with wallet adapter
    const signedTransaction = await signTransaction(transaction);

    // 4. Send transaction
    const signature = await connection.sendRawTransaction(
      signedTransaction.serialize()
    );

    // 5. Confirm
    await connection.confirmTransaction(signature);

    console.log('Payment confirmed:', signature);
    console.log('Payment proof:', data.paymentProof);
    return { signature, paymentProof: data.paymentProof };
  };

  return (
    <button onClick={() => executePayment('RECIPIENT_ADDRESS', 0.001, '/api/endpoint')}>
      Send Payment
    </button>
  );
}
```

### Python Example

```python
from solana.rpc.api import Client
from solana.transaction import Transaction
from solders.keypair import Keypair
import base64
import requests

def sign_and_send_transaction(
    base64_transaction: str,
    wallet: Keypair,
    rpc_url: str
) -> str:
    # Create RPC client
    client = Client(rpc_url)

    # Deserialize transaction
    transaction_bytes = base64.b64decode(base64_transaction)
    transaction = Transaction.deserialize(transaction_bytes)

    # Sign transaction
    transaction.sign(wallet)

    # Send transaction
    result = client.send_raw_transaction(transaction.serialize())

    signature = result['result']
    print(f'Transaction sent: {signature}')

    # Wait for confirmation
    client.confirm_transaction(signature)

    return signature

# Usage
def execute_send_payment(wallet: Keypair):
    # 1. Get unsigned payment transaction
    response = requests.post(
        'https://api.layerx402.dev/transaction/send',
        headers={
            'Content-Type': 'application/json',
            'Authorization': f'Bearer {os.getenv("LAYERX402_API_KEY")}'
        },
        json={
            'walletAddress': str(wallet.pubkey()),
            'recipientAddress': 'RECIPIENT_WALLET_ADDRESS',
            'amount': 0.001,
            'resource': 'https://api.example.com/protected-endpoint',
            'metadata': {
                'description': 'API access payment'
            }
        }
    )

    data = response.json()

    # 2. Sign and send
    signature = sign_and_send_transaction(
        data['transaction'],
        wallet,
        'https://api.mainnet-beta.solana.com'
    )

    print(f'Payment completed: {signature}')
    print(f'Payment proof: {data["paymentProof"]}')
    return signature
```

### Rust Example

```rust
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};

async fn sign_and_send_transaction(
    base64_transaction: &str,
    wallet: &Keypair,
    rpc_url: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    // Create RPC client
    let client = RpcClient::new(rpc_url.to_string());

    // Deserialize transaction
    let transaction_bytes = BASE64.decode(base64_transaction)?;
    let mut transaction: Transaction = bincode::deserialize(&transaction_bytes)?;

    // Sign transaction
    transaction.sign(&[wallet], client.get_latest_blockhash()?);

    // Send transaction
    let signature = client.send_and_confirm_transaction(&transaction)?;

    println!("Transaction sent: {}", signature);

    Ok(signature.to_string())
}
```

## Transaction Expiration

Transactions have a limited validity period:

- **lastValidBlockHeight**: Block height after which transaction is invalid
- **expiresAt**: Unix timestamp when transaction expires (typically 60 seconds)

Always send transactions promptly after building them.

## Best Practices

1. **Verify Transaction Contents**: Inspect the transaction before signing
2. **Use Trusted RPC**: Use your own RPC or a trusted provider
3. **Handle Expiration**: Build and send transactions quickly
4. **Confirm Transactions**: Always wait for confirmation
5. **Store Private Keys Securely**: Never expose private keys
6. **Test Thoroughly**: Test with small amounts first

## Security Considerations

### Inspect Before Signing

```javascript
import { Transaction } from '@solana/web3.js';

function inspectTransaction(base64Tx: string) {
  const tx = Transaction.from(Buffer.from(base64Tx, 'base64'));

  console.log('Transaction details:');
  console.log('- Instructions:', tx.instructions.length);
  console.log('- Fee payer:', tx.feePayer?.toBase58());
  console.log('- Recent blockhash:', tx.recentBlockhash);

  // Inspect each instruction
  tx.instructions.forEach((ix, i) => {
    console.log(`Instruction ${i}:`, {
      programId: ix.programId.toBase58(),
      keys: ix.keys.length,
      data: ix.data.length
    });
  });
}
```

### Simulate Before Sending

```javascript
async function simulateTransaction(transaction: Transaction, connection: Connection) {
  const simulation = await connection.simulateTransaction(transaction);

  if (simulation.value.err) {
    console.error('Simulation failed:', simulation.value.err);
    throw new Error('Transaction would fail');
  }

  console.log('Simulation succeeded');
  console.log('Logs:', simulation.value.logs);

  return simulation;
}
```

## Error Handling

Same error codes as Lightning API apply. See [Error Handling](../error-handling.md).

## Next Steps

- View [WebSocket API](websocket-api.md) for real-time data
- Check [Examples](../examples/) for complete integration code
- Learn about [Error Handling](../error-handling.md)
