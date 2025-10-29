use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_instruction;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod payment_settlement {
    use super::*;

    /// Initialize the settlement pool
    pub fn initialize_pool(
        ctx: Context<InitializePool>,
        fee_percentage: u16, // Basis points (e.g., 100 = 1%)
    ) -> Result<()> {
        let pool = &mut ctx.accounts.pool;

        require!(fee_percentage <= 1000, SettlementError::InvalidFeePercentage); // Max 10%

        pool.authority = ctx.accounts.authority.key();
        pool.total_settled = 0;
        pool.total_fees_collected = 0;
        pool.fee_percentage = fee_percentage;
        pool.active_settlements = 0;
        pool.bump = *ctx.bumps.get("pool").unwrap();

        msg!("Settlement pool initialized with {}% fee", fee_percentage as f64 / 100.0);

        Ok(())
    }

    /// Create a new settlement for a payment
    pub fn create_settlement(
        ctx: Context<CreateSettlement>,
        amount: u64,
        recipient: Pubkey,
        payment_id: String,
    ) -> Result<()> {
        let settlement = &mut ctx.accounts.settlement;
        let pool = &mut ctx.accounts.pool;
        let clock = Clock::get()?;

        require!(amount > 0, SettlementError::InvalidAmount);

        // Calculate fee
        let fee = (amount as u128 * pool.fee_percentage as u128 / 10000) as u64;
        let net_amount = amount.checked_sub(fee).ok_or(SettlementError::InvalidAmount)?;

        settlement.payment_id = payment_id.clone();
        settlement.payer = ctx.accounts.payer.key();
        settlement.recipient = recipient;
        settlement.gross_amount = amount;
        settlement.fee_amount = fee;
        settlement.net_amount = net_amount;
        settlement.status = SettlementStatus::Pending;
        settlement.created_at = clock.unix_timestamp;
        settlement.settled_at = None;
        settlement.bump = *ctx.bumps.get("settlement").unwrap();

        pool.active_settlements += 1;

        msg!("Settlement created:");
        msg!("  Payment ID: {}", payment_id);
        msg!("  Gross: {} lamports", amount);
        msg!("  Fee: {} lamports", fee);
        msg!("  Net: {} lamports", net_amount);

        Ok(())
    }

    /// Process the settlement and transfer funds
    pub fn process_settlement(ctx: Context<ProcessSettlement>) -> Result<()> {
        let settlement = &mut ctx.accounts.settlement;
        let pool = &mut ctx.accounts.pool;
        let clock = Clock::get()?;

        require!(
            settlement.status == SettlementStatus::Pending,
            SettlementError::InvalidSettlementStatus
        );

        require!(
            ctx.accounts.payer.key() == settlement.payer,
            SettlementError::Unauthorized
        );

        // Transfer gross amount from payer to settlement pool
        let transfer_to_pool = system_instruction::transfer(
            &ctx.accounts.payer.key(),
            &ctx.accounts.pool.key(),
            settlement.gross_amount,
        );

        anchor_lang::solana_program::program::invoke(
            &transfer_to_pool,
            &[
                ctx.accounts.payer.to_account_info(),
                ctx.accounts.pool.to_account_info(),
            ],
        )?;

        // Transfer net amount from pool to recipient
        **ctx.accounts.pool.to_account_info().try_borrow_mut_lamports()? -= settlement.net_amount;
        **ctx.accounts.recipient.try_borrow_mut_lamports()? += settlement.net_amount;

        settlement.status = SettlementStatus::Completed;
        settlement.settled_at = Some(clock.unix_timestamp);

        pool.total_settled += settlement.net_amount;
        pool.total_fees_collected += settlement.fee_amount;
        pool.active_settlements -= 1;

        msg!("Settlement processed:");
        msg!("  Transferred {} lamports to recipient", settlement.net_amount);
        msg!("  Fee collected: {} lamports", settlement.fee_amount);

        Ok(())
    }

    /// Batch process multiple settlements
    pub fn batch_process_settlements(
        ctx: Context<BatchProcessSettlements>,
        settlement_count: u8,
    ) -> Result<()> {
        require!(settlement_count > 0, SettlementError::InvalidBatchSize);
        require!(settlement_count <= 10, SettlementError::InvalidBatchSize); // Max 10 per batch

        let pool = &mut ctx.accounts.pool;

        msg!("Batch processing {} settlements", settlement_count);

        // Note: In a real implementation, you would iterate through remaining accounts
        // and process each settlement. This is a simplified version.

        pool.active_settlements = pool.active_settlements.saturating_sub(settlement_count as u64);

        Ok(())
    }

    /// Cancel a pending settlement
    pub fn cancel_settlement(ctx: Context<CancelSettlement>) -> Result<()> {
        let settlement = &mut ctx.accounts.settlement;
        let pool = &mut ctx.accounts.pool;

        require!(
            settlement.status == SettlementStatus::Pending,
            SettlementError::InvalidSettlementStatus
        );

        require!(
            ctx.accounts.authority.key() == settlement.payer
                || ctx.accounts.authority.key() == pool.authority,
            SettlementError::Unauthorized
        );

        settlement.status = SettlementStatus::Cancelled;

        pool.active_settlements -= 1;

        msg!("Settlement cancelled: {}", settlement.payment_id);

        Ok(())
    }

    /// Withdraw collected fees (authority only)
    pub fn withdraw_fees(
        ctx: Context<WithdrawFees>,
        amount: u64,
    ) -> Result<()> {
        let pool = &mut ctx.accounts.pool;

        require!(
            ctx.accounts.authority.key() == pool.authority,
            SettlementError::Unauthorized
        );

        require!(amount > 0, SettlementError::InvalidAmount);

        // Transfer fees from pool to authority
        **ctx.accounts.pool.to_account_info().try_borrow_mut_lamports()? -= amount;
        **ctx.accounts.authority.try_borrow_mut_lamports()? += amount;

        msg!("Withdrew {} lamports in fees", amount);

        Ok(())
    }

    /// Update fee percentage (authority only)
    pub fn update_fee_percentage(
        ctx: Context<UpdateFeePercentage>,
        new_fee_percentage: u16,
    ) -> Result<()> {
        let pool = &mut ctx.accounts.pool;

        require!(
            ctx.accounts.authority.key() == pool.authority,
            SettlementError::Unauthorized
        );

        require!(new_fee_percentage <= 1000, SettlementError::InvalidFeePercentage);

        let old_fee = pool.fee_percentage;
        pool.fee_percentage = new_fee_percentage;

        msg!("Fee updated from {}% to {}%",
             old_fee as f64 / 100.0,
             new_fee_percentage as f64 / 100.0);

        Ok(())
    }

    /// Get pool statistics
    pub fn get_pool_stats(ctx: Context<GetPoolStats>) -> Result<()> {
        let pool = &ctx.accounts.pool;

        msg!("Settlement Pool Statistics:");
        msg!("  Total settled: {} lamports", pool.total_settled);
        msg!("  Total fees collected: {} lamports", pool.total_fees_collected);
        msg!("  Active settlements: {}", pool.active_settlements);
        msg!("  Fee percentage: {}%", pool.fee_percentage as f64 / 100.0);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializePool<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + SettlementPool::INIT_SPACE,
        seeds = [b"pool"],
        bump
    )]
    pub pool: Account<'info, SettlementPool>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(payment_id: String)]
pub struct CreateSettlement<'info> {
    #[account(
        init,
        payer = payer,
        space = 8 + Settlement::INIT_SPACE,
        seeds = [b"settlement", payer.key().as_ref(), payment_id.as_bytes()],
        bump
    )]
    pub settlement: Account<'info, Settlement>,

    #[account(
        mut,
        seeds = [b"pool"],
        bump = pool.bump
    )]
    pub pool: Account<'info, SettlementPool>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ProcessSettlement<'info> {
    #[account(mut)]
    pub settlement: Account<'info, Settlement>,

    #[account(
        mut,
        seeds = [b"pool"],
        bump = pool.bump
    )]
    pub pool: Account<'info, SettlementPool>,

    #[account(mut)]
    pub payer: Signer<'info>,

    /// CHECK: Recipient account receiving the settlement
    #[account(mut)]
    pub recipient: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct BatchProcessSettlements<'info> {
    #[account(
        mut,
        seeds = [b"pool"],
        bump = pool.bump
    )]
    pub pool: Account<'info, SettlementPool>,

    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct CancelSettlement<'info> {
    #[account(mut)]
    pub settlement: Account<'info, Settlement>,

    #[account(
        mut,
        seeds = [b"pool"],
        bump = pool.bump
    )]
    pub pool: Account<'info, SettlementPool>,

    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct WithdrawFees<'info> {
    #[account(
        mut,
        seeds = [b"pool"],
        bump = pool.bump
    )]
    pub pool: Account<'info, SettlementPool>,

    #[account(mut)]
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct UpdateFeePercentage<'info> {
    #[account(
        mut,
        seeds = [b"pool"],
        bump = pool.bump
    )]
    pub pool: Account<'info, SettlementPool>,

    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct GetPoolStats<'info> {
    pub pool: Account<'info, SettlementPool>,
}

#[account]
#[derive(InitSpace)]
pub struct SettlementPool {
    pub authority: Pubkey,           // 32 bytes
    pub total_settled: u64,          // 8 bytes
    pub total_fees_collected: u64,   // 8 bytes
    pub fee_percentage: u16,         // 2 bytes (basis points)
    pub active_settlements: u64,     // 8 bytes
    pub bump: u8,                    // 1 byte
}

#[account]
#[derive(InitSpace)]
pub struct Settlement {
    #[max_len(64)]
    pub payment_id: String,          // 4 + 64 bytes
    pub payer: Pubkey,               // 32 bytes
    pub recipient: Pubkey,           // 32 bytes
    pub gross_amount: u64,           // 8 bytes
    pub fee_amount: u64,             // 8 bytes
    pub net_amount: u64,             // 8 bytes
    pub status: SettlementStatus,    // 1 byte
    pub created_at: i64,             // 8 bytes
    pub settled_at: Option<i64>,     // 9 bytes
    pub bump: u8,                    // 1 byte
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, InitSpace)]
pub enum SettlementStatus {
    Pending,
    Completed,
    Cancelled,
    Failed,
}

#[error_code]
pub enum SettlementError {
    #[msg("Invalid amount")]
    InvalidAmount,

    #[msg("Invalid fee percentage")]
    InvalidFeePercentage,

    #[msg("Invalid settlement status")]
    InvalidSettlementStatus,

    #[msg("Unauthorized")]
    Unauthorized,

    #[msg("Invalid batch size")]
    InvalidBatchSize,

    #[msg("Insufficient funds")]
    InsufficientFunds,
}
