# Rust Examples

Complete examples for integrating Layerx402 API with Rust.

## Dependencies

Add to `Cargo.toml`:

```toml
[dependencies]
reqwest = { version = "0.11", features = ["json"] }
tokio = { version = "1", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
solana-client = "1.16"
solana-sdk = "1.16"
base64 = "0.21"
anyhow = "1.0"
```

## Lightning API Examples

### Execute Buy Trade

```rust
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize)]
struct BuyTradeRequest {
    #[serde(rename = "publicKey")]
    public_key: String,
    mint: String,
    amount: f64,
    slippage: f64,
    #[serde(rename = "priorityFee")]
    priority_fee: f64,
}

#[derive(Deserialize, Debug)]
struct BuyTradeResponse {
    success: bool,
    signature: String,
    #[serde(rename = "tokensReceived")]
    tokens_received: u64,
    #[serde(rename = "effectivePrice")]
    effective_price: f64,
}

async fn execute_buy_trade(
    public_key: &str,
    mint: &str,
    amount: f64,
) -> Result<BuyTradeResponse, Box<dyn std::error::Error>> {
    let client = Client::new();
    let api_key = env::var("TURNPIKE_API_KEY")?;

    let request = BuyTradeRequest {
        public_key: public_key.to_string(),
        mint: mint.to_string(),
        amount,
        slippage: 10.0,
        priority_fee: 0.0001,
    };

    let response = client
        .post("https://api.layerx402.dev/trade/buy")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&request)
        .send()
        .await?;

    let data: BuyTradeResponse = response.json().await?;

    println!("Trade successful!");
    println!("Signature: {}", data.signature);
    println!("Tokens received: {}", data.tokens_received);
    println!("Effective price: {}", data.effective_price);

    Ok(data)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let result = execute_buy_trade(
        "YOUR_WALLET_PUBLIC_KEY",
        "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
        0.01,
    )
    .await?;

    println!("Result: {:?}", result);

    Ok(())
}
```

### Get Token Information

```rust
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct TokenInfo {
    mint: String,
    symbol: String,
    name: String,
    decimals: u8,
    price: f64,
    #[serde(rename = "marketCap")]
    market_cap: f64,
    #[serde(rename = "volume24h")]
    volume_24h: f64,
}

async fn get_token_info(mint: &str) -> Result<TokenInfo, Box<dyn std::error::Error>> {
    let client = Client::new();
    let api_key = env::var("TURNPIKE_API_KEY")?;

    let url = format!("https://api.layerx402.dev/token/info/{}", mint);

    let response = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", api_key))
        .send()
        .await?;

    let info: TokenInfo = response.json().await?;

    println!("{}: ${}", info.symbol, info.price);
    println!("Market Cap: ${}", info.market_cap);

    Ok(info)
}
```

### Get Portfolio

```rust
#[derive(Deserialize, Debug)]
struct Portfolio {
    #[serde(rename = "publicKey")]
    public_key: String,
    #[serde(rename = "totalValueUsd")]
    total_value_usd: f64,
    #[serde(rename = "solBalance")]
    sol_balance: f64,
    tokens: Vec<TokenHolding>,
}

#[derive(Deserialize, Debug)]
struct TokenHolding {
    mint: String,
    symbol: String,
    balance: f64,
    #[serde(rename = "valueUsd")]
    value_usd: f64,
}

async fn get_portfolio(public_key: &str) -> Result<Portfolio, Box<dyn std::error::Error>> {
    let client = Client::new();
    let api_key = env::var("TURNPIKE_API_KEY")?;

    let url = format!("https://api.layerx402.dev/portfolio/{}", public_key);

    let response = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", api_key))
        .send()
        .await?;

    let portfolio: Portfolio = response.json().await?;

    println!("Total Value: ${:.2}", portfolio.total_value_usd);
    println!("SOL Balance: {}", portfolio.sol_balance);
    println!("Tokens: {}", portfolio.tokens.len());

    for token in &portfolio.tokens {
        println!("  {}: {} (${:.2})", token.symbol, token.balance, token.value_usd);
    }

    Ok(portfolio)
}
```

## Local Transaction API Examples

### Build and Sign Transaction

```rust
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};

#[derive(Deserialize)]
struct TransactionResponse {
    transaction: String,
    #[serde(rename = "lastValidBlockHeight")]
    last_valid_block_height: u64,
}

async fn build_and_sign_trade(
    wallet: &Keypair,
    mint: &str,
    amount: f64,
) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();
    let api_key = env::var("TURNPIKE_API_KEY")?;

    // 1. Build transaction via API
    let request = serde_json::json!({
        "publicKey": wallet.pubkey().to_string(),
        "mint": mint,
        "amount": amount,
        "slippage": 10.0
    });

    let response = client
        .post("https://api.layerx402.dev/transaction/buy")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&request)
        .send()
        .await?;

    let data: TransactionResponse = response.json().await?;

    // 2. Deserialize transaction
    let transaction_bytes = BASE64.decode(&data.transaction)?;
    let mut transaction: Transaction = bincode::deserialize(&transaction_bytes)?;

    // 3. Connect to RPC
    let rpc_client = RpcClient::new("https://api.mainnet-beta.solana.com".to_string());

    // 4. Sign transaction
    let recent_blockhash = rpc_client.get_latest_blockhash()?;
    transaction.sign(&[wallet], recent_blockhash);

    // 5. Send transaction
    let signature = rpc_client.send_and_confirm_transaction(&transaction)?;

    println!("Transaction sent: {}", signature);

    Ok(signature.to_string())
}
```

## WebSocket Examples

### Basic WebSocket Connection

Add to `Cargo.toml`:

```toml
tokio-tungstenite = "0.20"
futures-util = "0.3"
```

```rust
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use futures_util::{StreamExt, SinkExt};
use serde_json::json;

async fn stream_trades(api_key: String) -> Result<(), Box<dyn std::error::Error>> {
    let url = "wss://layerx402.dev/api/data";
    let (ws_stream, _) = connect_async(url).await?;
    let (mut write, mut read) = ws_stream.split();

    // Authenticate
    write.send(Message::Text(
        json!({
            "method": "authenticate",
            "apiKey": api_key
        })
        .to_string()
    )).await?;

    // Subscribe to trades
    write.send(Message::Text(
        json!({
            "method": "subscribe",
            "keys": ["trades"]
        })
        .to_string()
    )).await?;

    // Listen for messages
    while let Some(msg) = read.next().await {
        match msg? {
            Message::Text(text) => {
                let data: serde_json::Value = serde_json::from_str(&text)?;

                if data["type"] == "trade" {
                    println!(
                        "Trade: {} {} @ ${}",
                        data["side"],
                        data["amount"],
                        data["price"]
                    );
                }
            }
            Message::Close(_) => break,
            _ => {}
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = env::var("TURNPIKE_API_KEY")?;
    stream_trades(api_key).await?;
    Ok(())
}
```

## Complete Trading Bot Example

```rust
use std::collections::HashMap;
use tokio::sync::mpsc;

#[derive(Debug)]
struct PriceUpdate {
    mint: String,
    price: f64,
}

struct TradingBot {
    api_key: String,
    public_key: String,
    targets: HashMap<String, f64>,
}

impl TradingBot {
    fn new(api_key: String, public_key: String) -> Self {
        Self {
            api_key,
            public_key,
            targets: HashMap::new(),
        }
    }

    fn add_target(&mut self, mint: String, target_price: f64) {
        self.targets.insert(mint, target_price);
    }

    async fn execute_buy(&self, mint: &str, amount: f64) -> Result<String, Box<dyn std::error::Error>> {
        let client = Client::new();

        let request = serde_json::json!({
            "publicKey": self.public_key,
            "mint": mint,
            "amount": amount,
            "slippage": 15.0
        });

        let response = client
            .post("https://api.layerx402.dev/trade/buy")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request)
            .send()
            .await?;

        let data: BuyTradeResponse = response.json().await?;
        println!("Buy executed: {}", data.signature);

        Ok(data.signature)
    }

    async fn handle_price_update(&mut self, update: PriceUpdate) {
        if let Some(&target_price) = self.targets.get(&update.mint) {
            if update.price <= target_price {
                println!("Target price reached for {}!", update.mint);

                if let Ok(_) = self.execute_buy(&update.mint, 0.01).await {
                    self.targets.remove(&update.mint);
                }
            }
        }
    }

    async fn start(mut self) -> Result<(), Box<dyn std::error::Error>> {
        let url = "wss://layerx402.dev/api/data";
        let (ws_stream, _) = connect_async(url).await?;
        let (mut write, mut read) = ws_stream.split();

        // Authenticate
        write.send(Message::Text(
            json!({
                "method": "authenticate",
                "apiKey": self.api_key
            })
            .to_string()
        )).await?;

        // Subscribe
        write.send(Message::Text(
            json!({
                "method": "subscribe",
                "keys": ["priceUpdates"]
            })
            .to_string()
        )).await?;

        // Process messages
        while let Some(msg) = read.next().await {
            if let Ok(Message::Text(text)) = msg {
                let data: serde_json::Value = serde_json::from_str(&text)?;

                if data["type"] == "priceUpdate" {
                    let update = PriceUpdate {
                        mint: data["mint"].as_str().unwrap().to_string(),
                        price: data["price"].as_f64().unwrap(),
                    };

                    self.handle_price_update(update).await;
                }
            }
        }

        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut bot = TradingBot::new(
        env::var("TURNPIKE_API_KEY")?,
        "YOUR_WALLET_PUBLIC_KEY".to_string(),
    );

    bot.add_target("TOKEN_MINT_1".to_string(), 0.00001);
    bot.add_target("TOKEN_MINT_2".to_string(), 0.00005);

    bot.start().await?;

    Ok(())
}
```

## Error Handling Example

```rust
use anyhow::{Context, Result};
use thiserror::Error;

#[derive(Error, Debug, Deserialize)]
#[error("{message}")]
struct APIError {
    code: String,
    message: String,
    #[serde(default)]
    details: serde_json::Value,
}

async fn execute_trade_with_retry(
    mut params: serde_json::Value,
    max_retries: u32,
) -> Result<BuyTradeResponse> {
    let client = Client::new();
    let api_key = env::var("TURNPIKE_API_KEY")?;

    for attempt in 0..max_retries {
        let response = client
            .post("https://api.layerx402.dev/trade/buy")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", api_key))
            .json(&params)
            .send()
            .await?;

        if response.status().is_success() {
            return Ok(response.json().await?);
        }

        let error: serde_json::Value = response.json().await?;
        let api_error: APIError = serde_json::from_value(error["error"].clone())?;

        match api_error.code.as_str() {
            "RATE_LIMIT_EXCEEDED" => {
                let wait_time = 2u64.pow(attempt);
                println!("Rate limited, waiting {}s...", wait_time);
                tokio::time::sleep(tokio::time::Duration::from_secs(wait_time)).await;
                continue;
            }
            "SLIPPAGE_EXCEEDED" if attempt < max_retries - 1 => {
                if let Some(slippage) = params["slippage"].as_f64() {
                    params["slippage"] = serde_json::json!(slippage * 1.5);
                    println!("Increasing slippage to {}", params["slippage"]);
                    continue;
                }
            }
            _ => return Err(anyhow::anyhow!("{}", api_error)),
        }
    }

    Err(anyhow::anyhow!("Failed after {} attempts", max_retries))
}
```
