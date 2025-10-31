use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod payment_streaming {
    use super::*;

    /// Create a new payment stream
    pub fn create_stream(
        ctx: Context<CreateStream>,
        recipient: Pubkey,
        amount_per_second: u64,
        start_time: i64,
        end_time: i64,
        cliff_time: Option<i64>,
    ) -> Result<()> {
        let stream = &mut ctx.accounts.stream;
        let clock = Clock::get()?;

        require!(amount_per_second > 0, StreamError::InvalidAmount);
        require!(end_time > start_time, StreamError::InvalidTimeRange);
        require!(
            start_time >= clock.unix_timestamp,
            StreamError::InvalidStartTime
        );

        if let Some(cliff) = cliff_time {
            require!(
                cliff > start_time && cliff < end_time,
                StreamError::InvalidCliffTime
            );
        }

        let total_duration = end_time - start_time;
        let total_amount = (amount_per_second as u128 * total_duration as u128) as u64;

        stream.sender = ctx.accounts.sender.key();
        stream.recipient = recipient;
        stream.amount_per_second = amount_per_second;
        stream.total_amount = total_amount;
        stream.withdrawn_amount = 0;
        stream.start_time = start_time;
        stream.end_time = end_time;
        stream.cliff_time = cliff_time;
        stream.last_withdrawal = start_time;
        stream.status = StreamStatus::Active;
        stream.paused = false;
        stream.created_at = clock.unix_timestamp;
        stream.bump = *ctx.bumps.get("stream").unwrap();

        msg!("Stream created:");
        msg!("  Total amount: {} lamports", total_amount);
        msg!("  Rate: {} lamports/second", amount_per_second);
        msg!("  Duration: {} seconds", total_duration);

        Ok(())
    }

    /// Fund the stream with SOL
    pub fn fund_stream(ctx: Context<FundStream>) -> Result<()> {
        let stream = &ctx.accounts.stream;

        require!(
            stream.status == StreamStatus::Active,
            StreamError::InvalidStreamStatus
        );

        // Transfer SOL to stream PDA
        let transfer_ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.sender.key(),
            &ctx.accounts.stream.key(),
            stream.total_amount,
        );

        anchor_lang::solana_program::program::invoke(
            &transfer_ix,
            &[
                ctx.accounts.sender.to_account_info(),
                ctx.accounts.stream.to_account_info(),
            ],
        )?;

        msg!("Stream funded with {} lamports", stream.total_amount);

        Ok(())
    }

    /// Withdraw available streamed funds
    pub fn withdraw_stream(ctx: Context<WithdrawStream>) -> Result<()> {
        let stream = &mut ctx.accounts.stream;
        let clock = Clock::get()?;

        require!(
            stream.status == StreamStatus::Active,
            StreamError::InvalidStreamStatus
        );

        require!(!stream.paused, StreamError::StreamPaused);

        require!(
            ctx.accounts.recipient.key() == stream.recipient,
            StreamError::Unauthorized
        );

        // Check if cliff period has passed
        if let Some(cliff) = stream.cliff_time {
            require!(
                clock.unix_timestamp >= cliff,
                StreamError::CliffNotReached
            );
        }

        // Calculate withdrawable amount
        let current_time = std::cmp::min(clock.unix_timestamp, stream.end_time);
        let elapsed = current_time - stream.start_time;
        let total_vested = (stream.amount_per_second as u128 * elapsed as u128) as u64;
        let withdrawable = total_vested.saturating_sub(stream.withdrawn_amount);

        require!(withdrawable > 0, StreamError::NothingToWithdraw);

        // Transfer to recipient
        **ctx.accounts.stream.to_account_info().try_borrow_mut_lamports()? -= withdrawable;
        **ctx.accounts.recipient.try_borrow_mut_lamports()? += withdrawable;

        stream.withdrawn_amount += withdrawable;
        stream.last_withdrawal = clock.unix_timestamp;

        if stream.withdrawn_amount >= stream.total_amount {
            stream.status = StreamStatus::Completed;
        }

        msg!("Withdrew {} lamports", withdrawable);
        msg!("Total withdrawn: {}/{}", stream.withdrawn_amount, stream.total_amount);

        Ok(())
    }

    /// Pause the stream (sender only)
    pub fn pause_stream(ctx: Context<PauseStream>) -> Result<()> {
        let stream = &mut ctx.accounts.stream;

        require!(
            ctx.accounts.sender.key() == stream.sender,
            StreamError::Unauthorized
        );

        require!(
            stream.status == StreamStatus::Active,
            StreamError::InvalidStreamStatus
        );

        require!(!stream.paused, StreamError::AlreadyPaused);

        stream.paused = true;

        msg!("Stream paused");

        Ok(())
    }

    /// Resume a paused stream (sender only)
    pub fn resume_stream(ctx: Context<ResumeStream>) -> Result<()> {
        let stream = &mut ctx.accounts.stream;

        require!(
            ctx.accounts.sender.key() == stream.sender,
            StreamError::Unauthorized
        );

        require!(stream.paused, StreamError::StreamNotPaused);

        stream.paused = false;

        msg!("Stream resumed");

        Ok(())
    }

    /// Cancel the stream and refund remaining balance
    pub fn cancel_stream(ctx: Context<CancelStream>) -> Result<()> {
        let stream = &mut ctx.accounts.stream;
        let clock = Clock::get()?;

        require!(
            ctx.accounts.sender.key() == stream.sender
                || ctx.accounts.sender.key() == stream.recipient,
            StreamError::Unauthorized
        );

        require!(
            stream.status == StreamStatus::Active,
            StreamError::InvalidStreamStatus
        );

        // Calculate amounts
        let current_time = std::cmp::min(clock.unix_timestamp, stream.end_time);
        let elapsed = current_time - stream.start_time;
        let total_vested = (stream.amount_per_second as u128 * elapsed as u128) as u64;
        let owed_to_recipient = total_vested.saturating_sub(stream.withdrawn_amount);
        let refund_to_sender = stream.total_amount.saturating_sub(total_vested);

        // Transfer owed amount to recipient
        if owed_to_recipient > 0 {
            **ctx.accounts.stream.to_account_info().try_borrow_mut_lamports()? -= owed_to_recipient;
            **ctx.accounts.recipient.try_borrow_mut_lamports()? += owed_to_recipient;
        }

        // Refund remaining to sender
        if refund_to_sender > 0 {
            **ctx.accounts.stream.to_account_info().try_borrow_mut_lamports()? -= refund_to_sender;
            **ctx.accounts.sender.try_borrow_mut_lamports()? += refund_to_sender;
        }

        stream.withdrawn_amount += owed_to_recipient;
        stream.status = StreamStatus::Cancelled;

        msg!("Stream cancelled:");
        msg!("  Paid to recipient: {} lamports", owed_to_recipient);
        msg!("  Refunded to sender: {} lamports", refund_to_sender);

        Ok(())
    }

    /// Get stream info
    pub fn get_stream_info(ctx: Context<GetStreamInfo>) -> Result<()> {
        let stream = &ctx.accounts.stream;
        let clock = Clock::get()?;

        let current_time = std::cmp::min(clock.unix_timestamp, stream.end_time);
        let elapsed = current_time - stream.start_time;
        let total_vested = (stream.amount_per_second as u128 * elapsed as u128) as u64;
        let withdrawable = total_vested.saturating_sub(stream.withdrawn_amount);

        msg!("Stream Info:");
        msg!("  Status: {:?}", stream.status);
        msg!("  Paused: {}", stream.paused);
        msg!("  Total: {} lamports", stream.total_amount);
        msg!("  Withdrawn: {} lamports", stream.withdrawn_amount);
        msg!("  Withdrawable: {} lamports", withdrawable);
        msg!("  Rate: {} lamports/second", stream.amount_per_second);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateStream<'info> {
    #[account(
        init,
        payer = sender,
        space = 8 + Stream::INIT_SPACE,
        seeds = [b"stream", sender.key().as_ref()],
        bump
    )]
    pub stream: Account<'info, Stream>,

    #[account(mut)]
    pub sender: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct FundStream<'info> {
    #[account(mut)]
    pub stream: Account<'info, Stream>,

    #[account(mut)]
    pub sender: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct WithdrawStream<'info> {
    #[account(
        mut,
        seeds = [b"stream", stream.sender.as_ref()],
        bump = stream.bump
    )]
    pub stream: Account<'info, Stream>,

    /// CHECK: Recipient receiving the stream
    #[account(mut)]
    pub recipient: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct PauseStream<'info> {
    #[account(mut)]
    pub stream: Account<'info, Stream>,

    pub sender: Signer<'info>,
}

#[derive(Accounts)]
pub struct ResumeStream<'info> {
    #[account(mut)]
    pub stream: Account<'info, Stream>,

    pub sender: Signer<'info>,
}

#[derive(Accounts)]
pub struct CancelStream<'info> {
    #[account(mut)]
    pub stream: Account<'info, Stream>,

    #[account(mut)]
    pub sender: Signer<'info>,

    /// CHECK: Recipient of remaining funds
    #[account(mut)]
    pub recipient: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct GetStreamInfo<'info> {
    pub stream: Account<'info, Stream>,
}

#[account]
#[derive(InitSpace)]
pub struct Stream {
    pub sender: Pubkey,              // 32 bytes
    pub recipient: Pubkey,           // 32 bytes
    pub amount_per_second: u64,      // 8 bytes
    pub total_amount: u64,           // 8 bytes
    pub withdrawn_amount: u64,       // 8 bytes
    pub start_time: i64,             // 8 bytes
    pub end_time: i64,               // 8 bytes
    pub cliff_time: Option<i64>,     // 9 bytes
    pub last_withdrawal: i64,        // 8 bytes
    pub status: StreamStatus,        // 1 byte
    pub paused: bool,                // 1 byte
    pub created_at: i64,             // 8 bytes
    pub bump: u8,                    // 1 byte
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, InitSpace)]
pub enum StreamStatus {
    Active,
    Completed,
    Cancelled,
}

#[error_code]
pub enum StreamError {
    #[msg("Invalid amount")]
    InvalidAmount,

    #[msg("Invalid time range")]
    InvalidTimeRange,

    #[msg("Invalid start time")]
    InvalidStartTime,

    #[msg("Invalid cliff time")]
    InvalidCliffTime,

    #[msg("Invalid stream status")]
    InvalidStreamStatus,

    #[msg("Unauthorized")]
    Unauthorized,

    #[msg("Stream is paused")]
    StreamPaused,

    #[msg("Stream is not paused")]
    StreamNotPaused,

    #[msg("Already paused")]
    AlreadyPaused,

    #[msg("Cliff period not reached")]
    CliffNotReached,

    #[msg("Nothing to withdraw")]
    NothingToWithdraw,
}
