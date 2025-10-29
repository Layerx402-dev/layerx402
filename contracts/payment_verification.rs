use anchor_lang::prelude::*;
use anchor_lang::solana_program::keccak;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod payment_verification {
    use super::*;

    /// Initialize the payment verifier with authority
    pub fn initialize_verifier(ctx: Context<InitializeVerifier>) -> Result<()> {
        let verifier = &mut ctx.accounts.verifier;

        verifier.authority = ctx.accounts.authority.key();
        verifier.total_verifications = 0;
        verifier.total_volume = 0;
        verifier.bump = *ctx.bumps.get("verifier").unwrap();

        msg!("Payment verifier initialized");

        Ok(())
    }

    /// Verify a payment proof and record it on-chain
    pub fn verify_payment(
        ctx: Context<VerifyPayment>,
        payment_proof: String,
        amount: u64,
        recipient: Pubkey,
        network: String,
        transaction_signature: String,
    ) -> Result<()> {
        let payment = &mut ctx.accounts.payment;
        let verifier = &mut ctx.accounts.verifier;
        let clock = Clock::get()?;

        require!(amount > 0, VerificationError::InvalidAmount);

        // Generate payment hash from proof
        let payment_hash = keccak::hash(payment_proof.as_bytes());

        payment.payment_hash = payment_hash.to_bytes();
        payment.payment_proof = payment_proof.clone();
        payment.amount = amount;
        payment.payer = ctx.accounts.payer.key();
        payment.recipient = recipient;
        payment.network = network.clone();
        payment.transaction_signature = transaction_signature.clone();
        payment.verified_at = clock.unix_timestamp;
        payment.status = PaymentStatus::Verified;
        payment.bump = *ctx.bumps.get("payment").unwrap();

        // Update verifier stats
        verifier.total_verifications += 1;
        verifier.total_volume += amount;

        msg!("Payment verified:");
        msg!("  Amount: {} lamports", amount);
        msg!("  Recipient: {}", recipient);
        msg!("  Network: {}", network);
        msg!("  Transaction: {}", transaction_signature);
        msg!("  Hash: {:?}", payment_hash.to_bytes());

        Ok(())
    }

    /// Mark a payment as settled
    pub fn settle_payment(ctx: Context<SettlePayment>) -> Result<()> {
        let payment = &mut ctx.accounts.payment;
        let clock = Clock::get()?;

        require!(
            payment.status == PaymentStatus::Verified,
            VerificationError::InvalidPaymentStatus
        );

        require!(
            ctx.accounts.authority.key() == ctx.accounts.verifier.authority,
            VerificationError::Unauthorized
        );

        payment.status = PaymentStatus::Settled;
        payment.settled_at = Some(clock.unix_timestamp);

        msg!("Payment settled: {}", payment.transaction_signature);

        Ok(())
    }

    /// Dispute a payment (only by recipient or authority)
    pub fn dispute_payment(
        ctx: Context<DisputePayment>,
        reason: String,
    ) -> Result<()> {
        let payment = &mut ctx.accounts.payment;
        let clock = Clock::get()?;

        require!(
            payment.status == PaymentStatus::Verified || payment.status == PaymentStatus::Settled,
            VerificationError::InvalidPaymentStatus
        );

        require!(
            ctx.accounts.disputer.key() == payment.recipient
                || ctx.accounts.disputer.key() == ctx.accounts.verifier.authority,
            VerificationError::Unauthorized
        );

        payment.status = PaymentStatus::Disputed;
        payment.dispute_reason = Some(reason.clone());
        payment.disputed_at = Some(clock.unix_timestamp);

        msg!("Payment disputed: {}", reason);

        Ok(())
    }

    /// Resolve a disputed payment (only by authority)
    pub fn resolve_dispute(
        ctx: Context<ResolveDispute>,
        resolution: DisputeResolution,
    ) -> Result<()> {
        let payment = &mut ctx.accounts.payment;
        let clock = Clock::get()?;

        require!(
            payment.status == PaymentStatus::Disputed,
            VerificationError::InvalidPaymentStatus
        );

        require!(
            ctx.accounts.authority.key() == ctx.accounts.verifier.authority,
            VerificationError::Unauthorized
        );

        payment.status = match resolution {
            DisputeResolution::ApprovePayment => PaymentStatus::Settled,
            DisputeResolution::RefundPayer => PaymentStatus::Refunded,
        };

        payment.resolved_at = Some(clock.unix_timestamp);

        msg!("Dispute resolved: {:?}", resolution);

        Ok(())
    }

    /// Query payment verification status
    pub fn get_payment_status(ctx: Context<GetPaymentStatus>) -> Result<()> {
        let payment = &ctx.accounts.payment;

        msg!("Payment Status:");
        msg!("  Status: {:?}", payment.status);
        msg!("  Amount: {} lamports", payment.amount);
        msg!("  Payer: {}", payment.payer);
        msg!("  Recipient: {}", payment.recipient);
        msg!("  Verified at: {}", payment.verified_at);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeVerifier<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + Verifier::INIT_SPACE,
        seeds = [b"verifier"],
        bump
    )]
    pub verifier: Account<'info, Verifier>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(payment_proof: String)]
pub struct VerifyPayment<'info> {
    #[account(
        init,
        payer = payer,
        space = 8 + Payment::INIT_SPACE,
        seeds = [b"payment", payer.key().as_ref(), payment_proof.as_bytes()],
        bump
    )]
    pub payment: Account<'info, Payment>,

    #[account(
        mut,
        seeds = [b"verifier"],
        bump = verifier.bump
    )]
    pub verifier: Account<'info, Verifier>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SettlePayment<'info> {
    #[account(mut)]
    pub payment: Account<'info, Payment>,

    #[account(
        seeds = [b"verifier"],
        bump = verifier.bump
    )]
    pub verifier: Account<'info, Verifier>,

    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct DisputePayment<'info> {
    #[account(mut)]
    pub payment: Account<'info, Payment>,

    #[account(
        seeds = [b"verifier"],
        bump = verifier.bump
    )]
    pub verifier: Account<'info, Verifier>,

    pub disputer: Signer<'info>,
}

#[derive(Accounts)]
pub struct ResolveDispute<'info> {
    #[account(mut)]
    pub payment: Account<'info, Payment>,

    #[account(
        seeds = [b"verifier"],
        bump = verifier.bump
    )]
    pub verifier: Account<'info, Verifier>,

    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct GetPaymentStatus<'info> {
    pub payment: Account<'info, Payment>,
}

#[account]
#[derive(InitSpace)]
pub struct Verifier {
    pub authority: Pubkey,           // 32 bytes
    pub total_verifications: u64,    // 8 bytes
    pub total_volume: u64,           // 8 bytes
    pub bump: u8,                    // 1 byte
}

#[account]
#[derive(InitSpace)]
pub struct Payment {
    pub payment_hash: [u8; 32],      // 32 bytes
    #[max_len(512)]
    pub payment_proof: String,       // 4 + 512 bytes
    pub amount: u64,                 // 8 bytes
    pub payer: Pubkey,               // 32 bytes
    pub recipient: Pubkey,           // 32 bytes
    #[max_len(32)]
    pub network: String,             // 4 + 32 bytes
    #[max_len(128)]
    pub transaction_signature: String, // 4 + 128 bytes
    pub verified_at: i64,            // 8 bytes
    pub settled_at: Option<i64>,     // 9 bytes
    pub disputed_at: Option<i64>,    // 9 bytes
    pub resolved_at: Option<i64>,    // 9 bytes
    #[max_len(256)]
    pub dispute_reason: Option<String>, // 4 + 256 bytes
    pub status: PaymentStatus,       // 1 byte
    pub bump: u8,                    // 1 byte
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, InitSpace)]
pub enum PaymentStatus {
    Verified,
    Settled,
    Disputed,
    Refunded,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum DisputeResolution {
    ApprovePayment,
    RefundPayer,
}

#[error_code]
pub enum VerificationError {
    #[msg("Invalid amount")]
    InvalidAmount,

    #[msg("Invalid payment status")]
    InvalidPaymentStatus,

    #[msg("Unauthorized")]
    Unauthorized,

    #[msg("Payment already verified")]
    PaymentAlreadyVerified,
}
