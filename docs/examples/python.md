# Python Examples

Complete examples for integrating Layerx402 API with Python.

## Installation

```bash
pip install requests solana python-dotenv websocket-client
```

## Lightning API Examples

### Execute Buy Trade

```python
import requests
import os
from dotenv import load_dotenv

load_dotenv()

def execute_buy_trade(public_key: str, mint: str, amount: float, slippage: float = 10):
    """Execute a buy trade using the Lightning API"""

    url = 'https://api.layerx402.dev/trade/buy'
    headers = {
        'Content-Type': 'application/json',
        'Authorization': f'Bearer {os.getenv("TURNPIKE_API_KEY")}'
    }
    payload = {
        'publicKey': public_key,
        'mint': mint,
        'amount': amount,
        'slippage': slippage,
        'priorityFee': 0.0001
    }

    try:
        response = requests.post(url, headers=headers, json=payload)
        response.raise_for_status()

        data = response.json()
        print(f'Trade successful!')
        print(f'Signature: {data["signature"]}')
        print(f'Tokens received: {data["tokensReceived"]}')
        print(f'Effective price: {data["effectivePrice"]}')

        return data
    except requests.exceptions.RequestException as e:
        print(f'Error executing trade: {e}')
        if hasattr(e.response, 'json'):
            print(e.response.json())
        raise

# Usage
execute_buy_trade(
    public_key='YOUR_WALLET_PUBLIC_KEY',
    mint='EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v',
    amount=0.01
)
```

### Get Token Information

```python
def get_token_info(mint: str) -> dict:
    """Get detailed information about a token"""

    url = f'https://api.layerx402.dev/token/info/{mint}'
    headers = {
        'Authorization': f'Bearer {os.getenv("TURNPIKE_API_KEY")}'
    }

    response = requests.get(url, headers=headers)
    response.raise_for_status()

    data = response.json()
    print(f'{data["symbol"]}: ${data["price"]}')
    print(f'Market Cap: ${data["marketCap"]:,}')
    print(f'24h Volume: ${data["volume24h"]:,}')

    return data

# Usage
token_info = get_token_info('EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v')
```

### Get Portfolio

```python
def get_portfolio(public_key: str) -> dict:
    """Get all token holdings for a wallet"""

    url = f'https://api.layerx402.dev/portfolio/{public_key}'
    headers = {
        'Authorization': f'Bearer {os.getenv("TURNPIKE_API_KEY")}'
    }

    response = requests.get(url, headers=headers)
    response.raise_for_status()

    data = response.json()
    print(f'Total Value: ${data["totalValueUsd"]:,.2f}')
    print(f'SOL Balance: {data["solBalance"]}')
    print(f'Tokens: {len(data["tokens"])}')

    for token in data['tokens']:
        print(f'  {token["symbol"]}: {token["balance"]} (${token["valueUsd"]:,.2f})')

    return data

# Usage
portfolio = get_portfolio('YOUR_WALLET_PUBLIC_KEY')
```

## Local Transaction API Examples

### Build and Sign Transaction

```python
from solana.rpc.api import Client
from solana.transaction import Transaction
from solders.keypair import Keypair
import base64

def build_and_sign_trade(wallet: Keypair, mint: str, amount: float):
    """Build, sign, and send a transaction locally"""

    # 1. Build transaction via API
    url = 'https://api.layerx402.dev/transaction/buy'
    headers = {
        'Content-Type': 'application/json',
        'Authorization': f'Bearer {os.getenv("TURNPIKE_API_KEY")}'
    }
    payload = {
        'publicKey': str(wallet.pubkey()),
        'mint': mint,
        'amount': amount,
        'slippage': 10
    }

    response = requests.post(url, headers=headers, json=payload)
    response.raise_for_status()
    data = response.json()

    # 2. Deserialize transaction
    transaction_bytes = base64.b64decode(data['transaction'])
    transaction = Transaction.deserialize(transaction_bytes)

    # 3. Connect to RPC
    client = Client('https://api.mainnet-beta.solana.com')

    # 4. Sign transaction
    transaction.sign(wallet)

    # 5. Send transaction
    result = client.send_raw_transaction(transaction.serialize())
    signature = result['result']

    print(f'Transaction sent: {signature}')

    # 6. Confirm transaction
    client.confirm_transaction(signature)
    print(f'Transaction confirmed')

    return signature

# Usage
# Load your wallet (securely)
secret_key_bytes = base64.b64decode(os.getenv('WALLET_PRIVATE_KEY'))
wallet = Keypair.from_bytes(secret_key_bytes)

signature = build_and_sign_trade(
    wallet=wallet,
    mint='TOKEN_MINT_ADDRESS',
    amount=0.01
)
```

## WebSocket Examples

### Basic WebSocket Connection

```python
import websocket
import json
import threading

class Layerx402Stream:
    def __init__(self, api_key: str):
        self.api_key = api_key
        self.ws = None

    def on_message(self, ws, message):
        data = json.loads(message)

        if data['type'] == 'trade':
            print(f'Trade: {data["side"]} {data["amount"]} @ ${data["price"]}')
        elif data['type'] == 'newToken':
            print(f'New token: {data["symbol"]} - {data["name"]}')

    def on_error(self, ws, error):
        print(f'Error: {error}')

    def on_close(self, ws, close_status_code, close_msg):
        print('Connection closed')

    def on_open(self, ws):
        print('Connected to Layerx402 WebSocket')

        # Authenticate
        ws.send(json.dumps({
            'method': 'authenticate',
            'apiKey': self.api_key
        }))

        # Subscribe to trades
        ws.send(json.dumps({
            'method': 'subscribe',
            'keys': ['trades', 'newTokens']
        }))

    def connect(self):
        websocket.enableTrace(False)
        self.ws = websocket.WebSocketApp(
            'wss://layerx402.dev/api/data',
            on_open=self.on_open,
            on_message=self.on_message,
            on_error=self.on_error,
            on_close=self.on_close
        )

        # Run in background thread
        wst = threading.Thread(target=self.ws.run_forever)
        wst.daemon = True
        wst.start()

    def disconnect(self):
        if self.ws:
            self.ws.close()

# Usage
stream = Layerx402Stream(os.getenv('TURNPIKE_API_KEY'))
stream.connect()

# Keep running
import time
try:
    while True:
        time.sleep(1)
except KeyboardInterrupt:
    stream.disconnect()
```

### Async WebSocket with Websockets Library

```python
import asyncio
import websockets
import json

async def stream_trades(api_key: str):
    uri = 'wss://layerx402.dev/api/data'

    async with websockets.connect(uri) as websocket:
        # Authenticate
        await websocket.send(json.dumps({
            'method': 'authenticate',
            'apiKey': api_key
        }))

        # Subscribe
        await websocket.send(json.dumps({
            'method': 'subscribe',
            'keys': ['trades']
        }))

        # Listen for messages
        async for message in websocket:
            data = json.loads(message)

            if data['type'] == 'trade':
                print(f'{data["side"]} {data["amount"]} @ ${data["price"]}')

# Usage
asyncio.run(stream_trades(os.getenv('TURNPIKE_API_KEY')))
```

## Complete Trading Bot Example

```python
import requests
import websocket
import json
import time
from typing import Dict, Any

class TradingBot:
    def __init__(self, api_key: str, public_key: str):
        self.api_key = api_key
        self.public_key = public_key
        self.ws = None
        self.target_tokens = {}  # mint -> target_price

    def add_target(self, mint: str, target_price: float):
        """Add a token to watch"""
        self.target_tokens[mint] = target_price

    def execute_buy(self, mint: str, amount: float):
        """Execute a buy trade"""
        try:
            response = requests.post(
                'https://api.layerx402.dev/trade/buy',
                headers={
                    'Content-Type': 'application/json',
                    'Authorization': f'Bearer {self.api_key}'
                },
                json={
                    'publicKey': self.public_key,
                    'mint': mint,
                    'amount': amount,
                    'slippage': 15
                }
            )
            response.raise_for_status()
            data = response.json()
            print(f'Buy executed: {data["signature"]}')
            return data
        except Exception as e:
            print(f'Buy failed: {e}')
            return None

    def on_price_update(self, data: Dict[str, Any]):
        """Handle price update events"""
        mint = data['mint']

        if mint in self.target_tokens:
            target_price = self.target_tokens[mint]
            current_price = data['price']

            if current_price <= target_price:
                print(f'Target price reached for {mint}!')
                self.execute_buy(mint, 0.01)
                # Remove from targets after buying
                del self.target_tokens[mint]

    def on_message(self, ws, message):
        data = json.loads(message)

        if data['type'] == 'priceUpdate':
            self.on_price_update(data)

    def on_open(self, ws):
        print('Bot connected')

        # Authenticate
        ws.send(json.dumps({
            'method': 'authenticate',
            'apiKey': self.api_key
        }))

        # Subscribe to price updates
        ws.send(json.dumps({
            'method': 'subscribe',
            'keys': ['priceUpdates']
        }))

    def start(self):
        """Start the trading bot"""
        self.ws = websocket.WebSocketApp(
            'wss://layerx402.dev/api/data',
            on_open=self.on_open,
            on_message=self.on_message
        )

        self.ws.run_forever()

# Usage
bot = TradingBot(
    api_key=os.getenv('TURNPIKE_API_KEY'),
    public_key='YOUR_WALLET_PUBLIC_KEY'
)

# Add tokens to watch
bot.add_target('TOKEN_MINT_1', 0.00001)
bot.add_target('TOKEN_MINT_2', 0.00005)

# Start bot
bot.start()
```

## Error Handling Example

```python
import time
from typing import Optional

class Layerx402APIError(Exception):
    def __init__(self, error_data: dict, status_code: int):
        self.code = error_data.get('code')
        self.message = error_data.get('message')
        self.details = error_data.get('details', {})
        self.status_code = status_code
        super().__init__(self.message)

def execute_trade_with_retry(
    params: dict,
    max_retries: int = 3
) -> Optional[dict]:
    """Execute trade with automatic retry logic"""

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

            if not response.ok:
                error_data = response.json()
                raise Layerx402APIError(error_data['error'], response.status_code)

            return response.json()

        except Layerx402APIError as e:
            if e.code == 'RATE_LIMIT_EXCEEDED':
                wait_time = 2 ** attempt
                print(f'Rate limited, waiting {wait_time}s...')
                time.sleep(wait_time)
                continue
            elif e.code == 'SLIPPAGE_EXCEEDED' and attempt < max_retries - 1:
                params['slippage'] *= 1.5
                print(f'Slippage exceeded, increasing to {params["slippage"]}%')
                continue
            else:
                print(f'Error: {e.code} - {e.message}')
                raise

        except requests.RequestException as e:
            if attempt < max_retries - 1:
                time.sleep(2 ** attempt)
                continue
            raise

    raise Exception(f'Failed after {max_retries} attempts')

# Usage
result = execute_trade_with_retry({
    'publicKey': 'YOUR_WALLET_PUBLIC_KEY',
    'mint': 'TOKEN_MINT_ADDRESS',
    'amount': 0.01,
    'slippage': 10
})
```

## Async/Await Examples

```python
import asyncio
import aiohttp

async def async_execute_trade(public_key: str, mint: str, amount: float):
    """Execute trade asynchronously"""

    url = 'https://api.layerx402.dev/trade/buy'
    headers = {
        'Content-Type': 'application/json',
        'Authorization': f'Bearer {os.getenv("TURNPIKE_API_KEY")}'
    }
    payload = {
        'publicKey': public_key,
        'mint': mint,
        'amount': amount,
        'slippage': 10
    }

    async with aiohttp.ClientSession() as session:
        async with session.post(url, headers=headers, json=payload) as response:
            data = await response.json()
            return data

async def main():
    # Execute multiple trades concurrently
    trades = [
        async_execute_trade('PUBLIC_KEY', 'MINT_1', 0.01),
        async_execute_trade('PUBLIC_KEY', 'MINT_2', 0.01),
        async_execute_trade('PUBLIC_KEY', 'MINT_3', 0.01)
    ]

    results = await asyncio.gather(*trades, return_exceptions=True)

    for result in results:
        if isinstance(result, Exception):
            print(f'Trade failed: {result}')
        else:
            print(f'Trade successful: {result["signature"]}')

# Run
asyncio.run(main())
```
