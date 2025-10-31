use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod payment_rewards {
    use super::*;

    /// Initialize rewards program
    pub fn initialize_program(
        ctx: Context<InitializeProgram>,
        points_per_lamport: u64,
        redemption_rate: u64,
    ) -> Result<()> {
        let program = &mut ctx.accounts.program;

        require!(points_per_lamport > 0, RewardsError::InvalidRate);
        require!(redemption_rate > 0, RewardsError::InvalidRate);

        program.authority = ctx.accounts.authority.key();
        program.points_per_lamport = points_per_lamport;
        program.redemption_rate = redemption_rate;
        program.total_points_issued = 0;
        program.total_points_redeemed = 0;
        program.total_members = 0;
        program.active = true;
        program.bump = *ctx.bumps.get("program").unwrap();

        msg!("Rewards program initialized");
        msg!("  Rate: {} points per lamport", points_per_lamport);
        msg!("  Redemption: {} points = 1 lamport", redemption_rate);

        Ok(())
    }

    /// Create rewards member account
    pub fn create_member(ctx: Context<CreateMember>) -> Result<()> {
        let member = &mut ctx.accounts.member;
        let program = &mut ctx.accounts.program;
        let clock = Clock::get()?;

        require!(program.active, RewardsError::ProgramInactive);

        member.user = ctx.accounts.user.key();
        member.points_balance = 0;
        member.lifetime_points_earned = 0;
        member.lifetime_points_redeemed = 0;
        member.tier = MemberTier::Bronze;
        member.total_payments = 0;
        member.total_spent = 0;
        member.joined_at = clock.unix_timestamp;
        member.last_activity = clock.unix_timestamp;
        member.bump = *ctx.bumps.get("member").unwrap();

        program.total_members += 1;

        msg!("Rewards member created for {}", ctx.accounts.user.key());

        Ok(())
    }

    /// Award points for payment
    pub fn award_points(
        ctx: Context<AwardPoints>,
        payment_amount: u64,
    ) -> Result<()> {
        let member = &mut ctx.accounts.member;
        let program = &mut ctx.accounts.program;
        let clock = Clock::get()?;

        require!(program.active, RewardsError::ProgramInactive);
        require!(payment_amount > 0, RewardsError::InvalidAmount);

        // Calculate base points
        let base_points = payment_amount * program.points_per_lamport;

        // Apply tier multiplier
        let multiplier = match member.tier {
            MemberTier::Bronze => 10,  // 1.0x
            MemberTier::Silver => 15,  // 1.5x
            MemberTier::Gold => 20,    // 2.0x
            MemberTier::Platinum => 30, // 3.0x
        };

        let points = (base_points * multiplier) / 10;

        member.points_balance += points;
        member.lifetime_points_earned += points;
        member.total_payments += 1;
        member.total_spent += payment_amount;
        member.last_activity = clock.unix_timestamp;

        program.total_points_issued += points;

        // Check for tier upgrade
        check_tier_upgrade(member)?;

        msg!("Awarded {} points for payment of {} lamports", points, payment_amount);
        msg!("  Tier: {:?} ({}x multiplier)", member.tier, multiplier as f64 / 10.0);
        msg!("  New balance: {} points", member.points_balance);

        Ok(())
    }

    /// Redeem points for SOL
    pub fn redeem_points(
        ctx: Context<RedeemPoints>,
        points_to_redeem: u64,
    ) -> Result<()> {
        let member = &mut ctx.accounts.member;
        let program = &mut ctx.accounts.program;
        let clock = Clock::get()?;

        require!(program.active, RewardsError::ProgramInactive);
        require!(points_to_redeem > 0, RewardsError::InvalidAmount);
        require!(
            member.points_balance >= points_to_redeem,
            RewardsError::InsufficientPoints
        );

        // Calculate SOL value
        let sol_value = points_to_redeem / program.redemption_rate;
        require!(sol_value > 0, RewardsError::AmountTooSmall);

        // Transfer SOL from pool to user
        **ctx.accounts.reward_pool.try_borrow_mut_lamports()? -= sol_value;
        **ctx.accounts.user.try_borrow_mut_lamports()? += sol_value;

        member.points_balance -= points_to_redeem;
        member.lifetime_points_redeemed += points_to_redeem;
        member.last_activity = clock.unix_timestamp;

        program.total_points_redeemed += points_to_redeem;

        msg!("Redeemed {} points for {} lamports", points_to_redeem, sol_value);
        msg!("  Remaining balance: {} points", member.points_balance);

        Ok(())
    }

    /// Award bonus points
    pub fn award_bonus(
        ctx: Context<AwardBonus>,
        points: u64,
        reason: String,
    ) -> Result<()> {
        let member = &mut ctx.accounts.member;
        let program = &mut ctx.accounts.program;
        let clock = Clock::get()?;

        require!(
            ctx.accounts.authority.key() == program.authority,
            RewardsError::Unauthorized
        );

        require!(points > 0, RewardsError::InvalidAmount);
        require!(reason.len() <= 128, RewardsError::ReasonTooLong);

        member.points_balance += points;
        member.lifetime_points_earned += points;
        member.last_activity = clock.unix_timestamp;

        program.total_points_issued += points;

        check_tier_upgrade(member)?;

        msg!("Awarded bonus {} points: {}", points, reason);

        Ok(())
    }

    /// Transfer points between members
    pub fn transfer_points(
        ctx: Context<TransferPoints>,
        points: u64,
    ) -> Result<()> {
        let from_member = &mut ctx.accounts.from_member;
        let to_member = &mut ctx.accounts.to_member;
        let clock = Clock::get()?;

        require!(points > 0, RewardsError::InvalidAmount);
        require!(
            from_member.points_balance >= points,
            RewardsError::InsufficientPoints
        );

        from_member.points_balance -= points;
        to_member.points_balance += points;

        from_member.last_activity = clock.unix_timestamp;
        to_member.last_activity = clock.unix_timestamp;

        msg!("Transferred {} points", points);

        Ok(())
    }

    /// Set member tier manually
    pub fn set_tier(
        ctx: Context<SetTier>,
        new_tier: MemberTier,
    ) -> Result<()> {
        let member = &mut ctx.accounts.member;
        let program = &ctx.accounts.program;

        require!(
            ctx.accounts.authority.key() == program.authority,
            RewardsError::Unauthorized
        );

        let old_tier = member.tier;
        member.tier = new_tier;

        msg!("Tier updated: {:?} -> {:?}", old_tier, new_tier);

        Ok(())
    }

    /// Update program rates
    pub fn update_rates(
        ctx: Context<UpdateRates>,
        points_per_lamport: Option<u64>,
        redemption_rate: Option<u64>,
    ) -> Result<()> {
        let program = &mut ctx.accounts.program;

        require!(
            ctx.accounts.authority.key() == program.authority,
            RewardsError::Unauthorized
        );

        if let Some(rate) = points_per_lamport {
            require!(rate > 0, RewardsError::InvalidRate);
            program.points_per_lamport = rate;
            msg!("Points per lamport updated: {}", rate);
        }

        if let Some(rate) = redemption_rate {
            require!(rate > 0, RewardsError::InvalidRate);
            program.redemption_rate = rate;
            msg!("Redemption rate updated: {}", rate);
        }

        Ok(())
    }

    /// Get member stats
    pub fn get_member_stats(ctx: Context<GetMemberStats>) -> Result<()> {
        let member = &ctx.accounts.member;
        let program = &ctx.accounts.program;

        let sol_value = member.points_balance / program.redemption_rate;

        msg!("Member Stats:");
        msg!("  Tier: {:?}", member.tier);
        msg!("  Points balance: {}", member.points_balance);
        msg!("  SOL value: {} lamports", sol_value);
        msg!("  Lifetime earned: {} points", member.lifetime_points_earned);
        msg!("  Lifetime redeemed: {} points", member.lifetime_points_redeemed);
        msg!("  Total payments: {}", member.total_payments);
        msg!("  Total spent: {} lamports", member.total_spent);

        Ok(())
    }
}

// Helper function
fn check_tier_upgrade(member: &mut Member) -> Result<()> {
    let new_tier = if member.total_spent >= 100_000_000_000 {
        // 100 SOL
        MemberTier::Platinum
    } else if member.total_spent >= 50_000_000_000 {
        // 50 SOL
        MemberTier::Gold
    } else if member.total_spent >= 10_000_000_000 {
        // 10 SOL
        MemberTier::Silver
    } else {
        MemberTier::Bronze
    };

    if new_tier != member.tier {
        let old_tier = member.tier;
        member.tier = new_tier;
        msg!("Tier upgraded: {:?} -> {:?}", old_tier, new_tier);
    }

    Ok(())
}

#[derive(Accounts)]
pub struct InitializeProgram<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + RewardsProgram::INIT_SPACE,
        seeds = [b"rewards_program"],
        bump
    )]
    pub program: Account<'info, RewardsProgram>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateMember<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + Member::INIT_SPACE,
        seeds = [b"member", user.key().as_ref()],
        bump
    )]
    pub member: Account<'info, Member>,

    #[account(
        mut,
        seeds = [b"rewards_program"],
        bump = program.bump
    )]
    pub program: Account<'info, RewardsProgram>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AwardPoints<'info> {
    #[account(
        mut,
        seeds = [b"member", member.user.as_ref()],
        bump = member.bump
    )]
    pub member: Account<'info, Member>,

    #[account(
        mut,
        seeds = [b"rewards_program"],
        bump = program.bump
    )]
    pub program: Account<'info, RewardsProgram>,

    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct RedeemPoints<'info> {
    #[account(
        mut,
        seeds = [b"member", user.key().as_ref()],
        bump = member.bump
    )]
    pub member: Account<'info, Member>,

    #[account(
        seeds = [b"rewards_program"],
        bump = program.bump
    )]
    pub program: Account<'info, RewardsProgram>,

    /// CHECK: Reward pool holding funds
    #[account(mut)]
    pub reward_pool: AccountInfo<'info>,

    #[account(mut)]
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct AwardBonus<'info> {
    #[account(mut)]
    pub member: Account<'info, Member>,

    #[account(
        mut,
        seeds = [b"rewards_program"],
        bump = program.bump
    )]
    pub program: Account<'info, RewardsProgram>,

    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct TransferPoints<'info> {
    #[account(mut)]
    pub from_member: Account<'info, Member>,

    #[account(mut)]
    pub to_member: Account<'info, Member>,

    pub from_user: Signer<'info>,
}

#[derive(Accounts)]
pub struct SetTier<'info> {
    #[account(mut)]
    pub member: Account<'info, Member>,

    #[account(
        seeds = [b"rewards_program"],
        bump = program.bump
    )]
    pub program: Account<'info, RewardsProgram>,

    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct UpdateRates<'info> {
    #[account(
        mut,
        seeds = [b"rewards_program"],
        bump = program.bump
    )]
    pub program: Account<'info, RewardsProgram>,

    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct GetMemberStats<'info> {
    pub member: Account<'info, Member>,
    pub program: Account<'info, RewardsProgram>,
}

#[account]
#[derive(InitSpace)]
pub struct RewardsProgram {
    pub authority: Pubkey,               // 32 bytes
    pub points_per_lamport: u64,         // 8 bytes
    pub redemption_rate: u64,            // 8 bytes
    pub total_points_issued: u64,        // 8 bytes
    pub total_points_redeemed: u64,      // 8 bytes
    pub total_members: u64,              // 8 bytes
    pub active: bool,                    // 1 byte
    pub bump: u8,                        // 1 byte
}

#[account]
#[derive(InitSpace)]
pub struct Member {
    pub user: Pubkey,                    // 32 bytes
    pub points_balance: u64,             // 8 bytes
    pub lifetime_points_earned: u64,     // 8 bytes
    pub lifetime_points_redeemed: u64,   // 8 bytes
    pub tier: MemberTier,                // 1 byte
    pub total_payments: u64,             // 8 bytes
    pub total_spent: u64,                // 8 bytes
    pub joined_at: i64,                  // 8 bytes
    pub last_activity: i64,              // 8 bytes
    pub bump: u8,                        // 1 byte
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, InitSpace)]
pub enum MemberTier {
    Bronze,
    Silver,
    Gold,
    Platinum,
}

#[error_code]
pub enum RewardsError {
    #[msg("Invalid rate")]
    InvalidRate,

    #[msg("Program is inactive")]
    ProgramInactive,

    #[msg("Invalid amount")]
    InvalidAmount,

    #[msg("Insufficient points")]
    InsufficientPoints,

    #[msg("Amount too small")]
    AmountTooSmall,

    #[msg("Unauthorized")]
    Unauthorized,

    #[msg("Reason too long")]
    ReasonTooLong,
}
