# Layerx402 Solana Smart Contracts

This directory contains Solana smart contracts (programs) for the Layerx402 x402 payment infrastructure, built using the Anchor framework.

## 📋 Contracts Overview

### 1. Payment Escrow (`payment_escrow.rs`)

Handles escrow functionality for x402 payments, holding SOL until payment verification is complete.

**Features:**
- Create and fund escrow accounts
- Release funds to recipient after verification
- Automatic refunds on expiry
- Cancel and reclaim funds
- Secure PDA-based architecture

**Instructions:**
- `initialize_escrow` - Create a new escrow account
- `fund_escrow` - Deposit SOL into escrow
- `release_escrow` - Release funds to recipient (with payment proof)
- `refund_escrow` - Refund to payer if expired/cancelled
- `close_escrow` - Close account and reclaim rent

### 2. Payment Verification (`payment_verification.rs`)

On-chain verification system for x402 payment proofs with dispute resolution.

**Features:**
- Cryptographic payment proof verification (keccak hash)
- On-chain payment records
- Settlement tracking
- Dispute and resolution system
- Global verifier statistics

**Instructions:**
- `initialize_verifier` - Setup the verification authority
- `verify_payment` - Verify and record a payment on-chain
- `settle_payment` - Mark payment as settled
- `dispute_payment` - Raise a dispute
- `resolve_dispute` - Resolve disputes (authority only)
- `get_payment_status` - Query payment details

### 3. Payment Settlement (`payment_settlement.rs`)

Manages payment settlements with configurable fees and batch processing.

**Features:**
- Settlement pool with fee collection
- Configurable fee percentages
- Batch settlement processing
- Fee withdrawal (authority only)
- Comprehensive settlement statistics

**Instructions:**
- `initialize_pool` - Create settlement pool
- `create_settlement` - Initialize a new settlement
- `process_settlement` - Execute settlement and transfer funds
- `batch_process_settlements` - Process multiple settlements
- `cancel_settlement` - Cancel pending settlement
- `withdraw_fees` - Withdraw collected fees
- `update_fee_percentage` - Update platform fees
- `get_pool_stats` - Get pool statistics

## 🚀 Prerequisites

1. **Install Rust:**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. **Install Solana CLI:**
```bash
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
```

3. **Install Anchor:**
```bash
cargo install --git https://github.com/coral-xyz/anchor avm --locked --force
avm install latest
avm use latest
```

4. **Set up Solana wallet:**
```bash
solana-keygen new
solana config set --url devnet  # or mainnet-beta
```

## 🛠️ Build & Deploy

### Project Setup

1. **Initialize Anchor workspace:**
```bash
cd contracts
anchor init layerx402-contracts --no-git
```

2. **Copy contract files:**
```bash
cp payment_*.rs layerx402-contracts/programs/layerx402-contracts/src/
```

3. **Update `lib.rs`:**
```rust
pub mod payment_escrow;
pub mod payment_verification;
pub mod payment_settlement;
```

### Build Contracts

```bash
cd layerx402-contracts
anchor build
```

### Deploy to Devnet

```bash
# Configure for devnet
solana config set --url devnet

# Airdrop SOL for deployment (devnet only)
solana airdrop 2

# Deploy
anchor deploy

# Note the program IDs and update declare_id! in each contract
```

### Deploy to Mainnet

```bash
# Configure for mainnet
solana config set --url mainnet-beta

# Deploy (requires sufficient SOL)
anchor deploy
```

## 📝 Usage Examples

### Example 1: Create and Process Payment Escrow

```typescript
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";

const program = anchor.workspace.PaymentEscrow;

// Initialize escrow
const escrowPDA = await program.methods
  .initializeEscrow(
    new anchor.BN(1000000000), // 1 SOL in lamports
    recipientPublicKey,
    Math.floor(Date.now() / 1000) + 3600 // 1 hour expiry
  )
  .accounts({
    escrow: escrowAccount,
    payer: payerPublicKey,
    systemProgram: anchor.web3.SystemProgram.programId,
  })
  .rpc();

// Fund escrow
await program.methods
  .fundEscrow()
  .accounts({
    escrow: escrowAccount,
    payer: payerPublicKey,
    systemProgram: anchor.web3.SystemProgram.programId,
  })
  .rpc();

// Release after verification
await program.methods
  .releaseEscrow("BASE64_PAYMENT_PROOF")
  .accounts({
    escrow: escrowAccount,
    recipient: recipientPublicKey,
    authority: authorityPublicKey,
  })
  .rpc();
```

### Example 2: Verify Payment

```typescript
const program = anchor.workspace.PaymentVerification;

// Initialize verifier (once)
await program.methods
  .initializeVerifier()
  .accounts({
    verifier: verifierPDA,
    authority: authorityPublicKey,
    systemProgram: anchor.web3.SystemProgram.programId,
  })
  .rpc();

// Verify a payment
await program.methods
  .verifyPayment(
    "BASE64_PAYMENT_PROOF",
    new anchor.BN(1000000000),
    recipientPublicKey,
    "solana",
    "5wHu...signature"
  )
  .accounts({
    payment: paymentPDA,
    verifier: verifierPDA,
    payer: payerPublicKey,
    systemProgram: anchor.web3.SystemProgram.programId,
  })
  .rpc();
```

### Example 3: Settlement Processing

```typescript
const program = anchor.workspace.PaymentSettlement;

// Initialize pool (once)
await program.methods
  .initializePool(100) // 1% fee
  .accounts({
    pool: poolPDA,
    authority: authorityPublicKey,
    systemProgram: anchor.web3.SystemProgram.programId,
  })
  .rpc();

// Create settlement
await program.methods
  .createSettlement(
    new anchor.BN(1000000000),
    recipientPublicKey,
    "payment_12345"
  )
  .accounts({
    settlement: settlementPDA,
    pool: poolPDA,
    payer: payerPublicKey,
    systemProgram: anchor.web3.SystemProgram.programId,
  })
  .rpc();

// Process settlement
await program.methods
  .processSettlement()
  .accounts({
    settlement: settlementPDA,
    pool: poolPDA,
    payer: payerPublicKey,
    recipient: recipientPublicKey,
    systemProgram: anchor.web3.SystemProgram.programId,
  })
  .rpc();
```

## 🧪 Testing

```bash
# Run all tests
anchor test

# Run specific test file
anchor test -- --test escrow

# Test on devnet
anchor test --provider.cluster devnet
```

## 📊 Contract Accounts & PDAs

### Payment Escrow
- **Escrow PDA**: `["escrow", payer.key()]`

### Payment Verification
- **Verifier PDA**: `["verifier"]`
- **Payment PDA**: `["payment", payer.key(), payment_proof.bytes()]`

### Payment Settlement
- **Pool PDA**: `["pool"]`
- **Settlement PDA**: `["settlement", payer.key(), payment_id.bytes()]`

## 🔒 Security Considerations

1. **Program Authority**: All contracts use PDA-based authority for security
2. **Time-based Controls**: Escrows have expiry timestamps to prevent locked funds
3. **Status Checks**: All state transitions validate current status
4. **Amount Validation**: All transfers validate amounts > 0
5. **Authorization**: Critical operations require proper authority signatures

## 🌐 Integration with Layerx402 API

These contracts are designed to work seamlessly with the Layerx402 REST API:

1. **API creates escrow** → Contract holds funds
2. **x402 payment received** → API calls verify_payment
3. **Verification complete** → Contract releases escrow
4. **Settlement initiated** → Contract processes and collects fees

## 📚 Additional Resources

- [Anchor Documentation](https://www.anchor-lang.com/)
- [Solana Documentation](https://docs.solana.com/)
- [x402 Protocol](https://www.x402.org/)
- [Layerx402 API Docs](https://docs.layerx402.dev)

## 🤝 Contributing

1. Fork the repository
2. Create your feature branch
3. Write tests for new features
4. Ensure all tests pass
5. Submit a pull request

## 📄 License

Proprietary - All rights reserved

## 🆘 Support

- **Documentation**: https://docs.layerx402.dev
- **Email**: support@layerx402.dev
- **Discord**: [Join our server](https://discord.gg/layerx402)

---

Built with ❤️ by the Layerx402 team
