use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_instruction;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod payment_escrow {
    use super::*;

    /// Initialize a new payment escrow
    /// This creates an escrow account that holds SOL until payment is verified
    pub fn initialize_escrow(
        ctx: Context<InitializeEscrow>,
        amount: u64,
        recipient: Pubkey,
        expiry_timestamp: i64,
    ) -> Result<()> {
        let escrow = &mut ctx.accounts.escrow;
        let clock = Clock::get()?;

        require!(
            expiry_timestamp > clock.unix_timestamp,
            EscrowError::InvalidExpiryTimestamp
        );

        require!(amount > 0, EscrowError::InvalidAmount);

        escrow.payer = ctx.accounts.payer.key();
        escrow.recipient = recipient;
        escrow.amount = amount;
        escrow.status = EscrowStatus::Pending;
        escrow.created_at = clock.unix_timestamp;
        escrow.expiry_timestamp = expiry_timestamp;
        escrow.payment_proof = None;
        escrow.bump = *ctx.bumps.get("escrow").unwrap();

        msg!("Escrow initialized: {} SOL for {}", amount, recipient);

        Ok(())
    }

    /// Fund the escrow account with SOL
    pub fn fund_escrow(ctx: Context<FundEscrow>) -> Result<()> {
        let escrow = &ctx.accounts.escrow;

        require!(
            escrow.status == EscrowStatus::Pending,
            EscrowError::InvalidEscrowStatus
        );

        // Transfer SOL from payer to escrow PDA
        let transfer_instruction = system_instruction::transfer(
            &ctx.accounts.payer.key(),
            &ctx.accounts.escrow.key(),
            escrow.amount,
        );

        anchor_lang::solana_program::program::invoke(
            &transfer_instruction,
            &[
                ctx.accounts.payer.to_account_info(),
                ctx.accounts.escrow.to_account_info(),
            ],
        )?;

        msg!("Escrow funded with {} lamports", escrow.amount);

        Ok(())
    }

    /// Release funds to recipient after payment verification
    pub fn release_escrow(
        ctx: Context<ReleaseEscrow>,
        payment_proof: String,
    ) -> Result<()> {
        let escrow = &mut ctx.accounts.escrow;
        let clock = Clock::get()?;

        require!(
            escrow.status == EscrowStatus::Pending,
            EscrowError::InvalidEscrowStatus
        );

        require!(
            clock.unix_timestamp < escrow.expiry_timestamp,
            EscrowError::EscrowExpired
        );

        require!(
            ctx.accounts.authority.key() == escrow.payer
                || ctx.accounts.authority.key() == escrow.recipient,
            EscrowError::Unauthorized
        );

        // Transfer SOL from escrow to recipient
        **ctx.accounts.escrow.to_account_info().try_borrow_mut_lamports()? -= escrow.amount;
        **ctx.accounts.recipient.try_borrow_mut_lamports()? += escrow.amount;

        escrow.status = EscrowStatus::Released;
        escrow.payment_proof = Some(payment_proof.clone());

        msg!("Escrow released: {} lamports to {}", escrow.amount, escrow.recipient);
        msg!("Payment proof: {}", payment_proof);

        Ok(())
    }

    /// Refund escrow to payer if expired or cancelled
    pub fn refund_escrow(ctx: Context<RefundEscrow>) -> Result<()> {
        let escrow = &mut ctx.accounts.escrow;
        let clock = Clock::get()?;

        require!(
            escrow.status == EscrowStatus::Pending,
            EscrowError::InvalidEscrowStatus
        );

        // Allow refund if expired OR if payer wants to cancel
        let can_refund = clock.unix_timestamp >= escrow.expiry_timestamp
            || ctx.accounts.payer.key() == escrow.payer;

        require!(can_refund, EscrowError::CannotRefund);

        // Transfer SOL back to payer
        **ctx.accounts.escrow.to_account_info().try_borrow_mut_lamports()? -= escrow.amount;
        **ctx.accounts.payer.try_borrow_mut_lamports()? += escrow.amount;

        escrow.status = EscrowStatus::Refunded;

        msg!("Escrow refunded: {} lamports to payer", escrow.amount);

        Ok(())
    }

    /// Close the escrow account and reclaim rent
    pub fn close_escrow(ctx: Context<CloseEscrow>) -> Result<()> {
        let escrow = &ctx.accounts.escrow;

        require!(
            escrow.status == EscrowStatus::Released || escrow.status == EscrowStatus::Refunded,
            EscrowError::CannotCloseEscrow
        );

        msg!("Escrow account closed");

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeEscrow<'info> {
    #[account(
        init,
        payer = payer,
        space = 8 + Escrow::INIT_SPACE,
        seeds = [b"escrow", payer.key().as_ref()],
        bump
    )]
    pub escrow: Account<'info, Escrow>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct FundEscrow<'info> {
    #[account(mut)]
    pub escrow: Account<'info, Escrow>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ReleaseEscrow<'info> {
    #[account(
        mut,
        seeds = [b"escrow", escrow.payer.as_ref()],
        bump = escrow.bump,
    )]
    pub escrow: Account<'info, Escrow>,

    /// CHECK: This is the recipient receiving the funds
    #[account(mut)]
    pub recipient: AccountInfo<'info>,

    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct RefundEscrow<'info> {
    #[account(
        mut,
        seeds = [b"escrow", escrow.payer.as_ref()],
        bump = escrow.bump,
    )]
    pub escrow: Account<'info, Escrow>,

    #[account(mut)]
    pub payer: Signer<'info>,
}

#[derive(Accounts)]
pub struct CloseEscrow<'info> {
    #[account(
        mut,
        close = payer,
        seeds = [b"escrow", escrow.payer.as_ref()],
        bump = escrow.bump,
    )]
    pub escrow: Account<'info, Escrow>,

    #[account(mut)]
    pub payer: Signer<'info>,
}

#[account]
#[derive(InitSpace)]
pub struct Escrow {
    pub payer: Pubkey,          // 32 bytes
    pub recipient: Pubkey,      // 32 bytes
    pub amount: u64,            // 8 bytes
    pub status: EscrowStatus,   // 1 byte
    pub created_at: i64,        // 8 bytes
    pub expiry_timestamp: i64,  // 8 bytes
    #[max_len(256)]
    pub payment_proof: Option<String>, // 4 + 256 bytes
    pub bump: u8,               // 1 byte
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, InitSpace)]
pub enum EscrowStatus {
    Pending,
    Released,
    Refunded,
}

#[error_code]
pub enum EscrowError {
    #[msg("Invalid expiry timestamp")]
    InvalidExpiryTimestamp,

    #[msg("Invalid amount")]
    InvalidAmount,

    #[msg("Invalid escrow status")]
    InvalidEscrowStatus,

    #[msg("Escrow has expired")]
    EscrowExpired,

    #[msg("Unauthorized")]
    Unauthorized,

    #[msg("Cannot refund escrow")]
    CannotRefund,

    #[msg("Cannot close escrow")]
    CannotCloseEscrow,
}
