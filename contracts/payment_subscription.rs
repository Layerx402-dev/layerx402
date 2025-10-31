use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod payment_subscription {
    use super::*;

    /// Create a subscription plan
    pub fn create_plan(
        ctx: Context<CreatePlan>,
        name: String,
        price: u64,
        interval_seconds: i64,
        max_subscribers: Option<u64>,
    ) -> Result<()> {
        let plan = &mut ctx.accounts.plan;
        let clock = Clock::get()?;

        require!(price > 0, SubscriptionError::InvalidPrice);
        require!(interval_seconds > 0, SubscriptionError::InvalidInterval);
        require!(name.len() <= 64, SubscriptionError::NameTooLong);

        plan.merchant = ctx.accounts.merchant.key();
        plan.name = name.clone();
        plan.price = price;
        plan.interval_seconds = interval_seconds;
        plan.max_subscribers = max_subscribers;
        plan.active_subscribers = 0;
        plan.total_subscriptions = 0;
        plan.total_revenue = 0;
        plan.active = true;
        plan.created_at = clock.unix_timestamp;
        plan.bump = *ctx.bumps.get("plan").unwrap();

        msg!("Subscription plan created: {}", name);
        msg!("  Price: {} lamports", price);
        msg!("  Interval: {} seconds", interval_seconds);

        Ok(())
    }

    /// Subscribe to a plan
    pub fn subscribe(ctx: Context<Subscribe>) -> Result<()> {
        let subscription = &mut ctx.accounts.subscription;
        let plan = &mut ctx.accounts.plan;
        let clock = Clock::get()?;

        require!(plan.active, SubscriptionError::PlanInactive);

        if let Some(max) = plan.max_subscribers {
            require!(
                plan.active_subscribers < max,
                SubscriptionError::MaxSubscribersReached
            );
        }

        subscription.subscriber = ctx.accounts.subscriber.key();
        subscription.plan = plan.key();
        subscription.merchant = plan.merchant;
        subscription.price = plan.price;
        subscription.interval_seconds = plan.interval_seconds;
        subscription.start_time = clock.unix_timestamp;
        subscription.next_billing = clock.unix_timestamp + plan.interval_seconds;
        subscription.last_payment = None;
        subscription.failed_payments = 0;
        subscription.total_paid = 0;
        subscription.status = SubscriptionStatus::Active;
        subscription.auto_renew = true;
        subscription.bump = *ctx.bumps.get("subscription").unwrap();

        plan.active_subscribers += 1;
        plan.total_subscriptions += 1;

        msg!("Subscribed to plan: {}", plan.name);
        msg!("  Next billing: {}", subscription.next_billing);

        Ok(())
    }

    /// Process subscription payment
    pub fn process_payment(ctx: Context<ProcessPayment>) -> Result<()> {
        let subscription = &mut ctx.accounts.subscription;
        let plan = &mut ctx.accounts.plan;
        let clock = Clock::get()?;

        require!(
            subscription.status == SubscriptionStatus::Active,
            SubscriptionError::InvalidSubscriptionStatus
        );

        require!(
            clock.unix_timestamp >= subscription.next_billing,
            SubscriptionError::PaymentNotDue
        );

        // Transfer payment from subscriber to merchant
        let transfer_ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.subscriber.key(),
            &ctx.accounts.merchant.key(),
            subscription.price,
        );

        anchor_lang::solana_program::program::invoke(
            &transfer_ix,
            &[
                ctx.accounts.subscriber.to_account_info(),
                ctx.accounts.merchant.to_account_info(),
            ],
        )?;

        subscription.last_payment = Some(clock.unix_timestamp);
        subscription.next_billing += subscription.interval_seconds;
        subscription.total_paid += subscription.price;
        subscription.failed_payments = 0;

        plan.total_revenue += subscription.price;

        msg!("Subscription payment processed: {} lamports", subscription.price);
        msg!("  Next billing: {}", subscription.next_billing);

        Ok(())
    }

    /// Mark payment as failed
    pub fn mark_failed_payment(ctx: Context<MarkFailedPayment>) -> Result<()> {
        let subscription = &mut ctx.accounts.subscription;

        require!(
            subscription.status == SubscriptionStatus::Active,
            SubscriptionError::InvalidSubscriptionStatus
        );

        subscription.failed_payments += 1;

        // Suspend after 3 failed payments
        if subscription.failed_payments >= 3 {
            subscription.status = SubscriptionStatus::Suspended;
            ctx.accounts.plan.active_subscribers -= 1;
            msg!("Subscription suspended due to failed payments");
        } else {
            msg!("Failed payment recorded: {}/3", subscription.failed_payments);
        }

        Ok(())
    }

    /// Cancel subscription
    pub fn cancel_subscription(ctx: Context<CancelSubscription>) -> Result<()> {
        let subscription = &mut ctx.accounts.subscription;
        let plan = &mut ctx.accounts.plan;

        require!(
            ctx.accounts.authority.key() == subscription.subscriber
                || ctx.accounts.authority.key() == plan.merchant,
            SubscriptionError::Unauthorized
        );

        require!(
            subscription.status == SubscriptionStatus::Active
                || subscription.status == SubscriptionStatus::Suspended,
            SubscriptionError::AlreadyCancelled
        );

        if subscription.status == SubscriptionStatus::Active {
            plan.active_subscribers -= 1;
        }

        subscription.status = SubscriptionStatus::Cancelled;
        subscription.auto_renew = false;

        msg!("Subscription cancelled");

        Ok(())
    }

    /// Reactivate suspended subscription
    pub fn reactivate_subscription(ctx: Context<ReactivateSubscription>) -> Result<()> {
        let subscription = &mut ctx.accounts.subscription;
        let plan = &mut ctx.accounts.plan;

        require!(
            ctx.accounts.subscriber.key() == subscription.subscriber,
            SubscriptionError::Unauthorized
        );

        require!(
            subscription.status == SubscriptionStatus::Suspended,
            SubscriptionError::InvalidSubscriptionStatus
        );

        if let Some(max) = plan.max_subscribers {
            require!(
                plan.active_subscribers < max,
                SubscriptionError::MaxSubscribersReached
            );
        }

        subscription.status = SubscriptionStatus::Active;
        subscription.failed_payments = 0;
        plan.active_subscribers += 1;

        msg!("Subscription reactivated");

        Ok(())
    }

    /// Update subscription auto-renew
    pub fn update_auto_renew(ctx: Context<UpdateAutoRenew>, auto_renew: bool) -> Result<()> {
        let subscription = &mut ctx.accounts.subscription;

        require!(
            ctx.accounts.subscriber.key() == subscription.subscriber,
            SubscriptionError::Unauthorized
        );

        subscription.auto_renew = auto_renew;

        msg!("Auto-renew updated: {}", auto_renew);

        Ok(())
    }

    /// Update plan price
    pub fn update_plan_price(ctx: Context<UpdatePlanPrice>, new_price: u64) -> Result<()> {
        let plan = &mut ctx.accounts.plan;

        require!(
            ctx.accounts.merchant.key() == plan.merchant,
            SubscriptionError::Unauthorized
        );

        require!(new_price > 0, SubscriptionError::InvalidPrice);

        let old_price = plan.price;
        plan.price = new_price;

        msg!("Plan price updated: {} -> {} lamports", old_price, new_price);

        Ok(())
    }

    /// Deactivate plan
    pub fn deactivate_plan(ctx: Context<DeactivatePlan>) -> Result<()> {
        let plan = &mut ctx.accounts.plan;

        require!(
            ctx.accounts.merchant.key() == plan.merchant,
            SubscriptionError::Unauthorized
        );

        plan.active = false;

        msg!("Plan deactivated");

        Ok(())
    }

    /// Get subscription info
    pub fn get_subscription_info(ctx: Context<GetSubscriptionInfo>) -> Result<()> {
        let subscription = &ctx.accounts.subscription;
        let clock = Clock::get()?;

        let days_remaining = if subscription.status == SubscriptionStatus::Active {
            (subscription.next_billing - clock.unix_timestamp) / 86400
        } else {
            0
        };

        msg!("Subscription Info:");
        msg!("  Status: {:?}", subscription.status);
        msg!("  Price: {} lamports", subscription.price);
        msg!("  Next billing: {}", subscription.next_billing);
        msg!("  Days remaining: {}", days_remaining);
        msg!("  Total paid: {} lamports", subscription.total_paid);
        msg!("  Failed payments: {}", subscription.failed_payments);

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(name: String)]
pub struct CreatePlan<'info> {
    #[account(
        init,
        payer = merchant,
        space = 8 + Plan::INIT_SPACE,
        seeds = [b"plan", merchant.key().as_ref(), name.as_bytes()],
        bump
    )]
    pub plan: Account<'info, Plan>,

    #[account(mut)]
    pub merchant: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Subscribe<'info> {
    #[account(
        init,
        payer = subscriber,
        space = 8 + Subscription::INIT_SPACE,
        seeds = [b"subscription", subscriber.key().as_ref(), plan.key().as_ref()],
        bump
    )]
    pub subscription: Account<'info, Subscription>,

    #[account(mut)]
    pub plan: Account<'info, Plan>,

    #[account(mut)]
    pub subscriber: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ProcessPayment<'info> {
    #[account(mut)]
    pub subscription: Account<'info, Subscription>,

    #[account(mut)]
    pub plan: Account<'info, Plan>,

    #[account(mut)]
    pub subscriber: Signer<'info>,

    /// CHECK: Merchant receiving payment
    #[account(mut)]
    pub merchant: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct MarkFailedPayment<'info> {
    #[account(mut)]
    pub subscription: Account<'info, Subscription>,

    #[account(mut)]
    pub plan: Account<'info, Plan>,

    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct CancelSubscription<'info> {
    #[account(mut)]
    pub subscription: Account<'info, Subscription>,

    #[account(mut)]
    pub plan: Account<'info, Plan>,

    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct ReactivateSubscription<'info> {
    #[account(mut)]
    pub subscription: Account<'info, Subscription>,

    #[account(mut)]
    pub plan: Account<'info, Plan>,

    pub subscriber: Signer<'info>,
}

#[derive(Accounts)]
pub struct UpdateAutoRenew<'info> {
    #[account(mut)]
    pub subscription: Account<'info, Subscription>,

    pub subscriber: Signer<'info>,
}

#[derive(Accounts)]
pub struct UpdatePlanPrice<'info> {
    #[account(mut)]
    pub plan: Account<'info, Plan>,

    pub merchant: Signer<'info>,
}

#[derive(Accounts)]
pub struct DeactivatePlan<'info> {
    #[account(mut)]
    pub plan: Account<'info, Plan>,

    pub merchant: Signer<'info>,
}

#[derive(Accounts)]
pub struct GetSubscriptionInfo<'info> {
    pub subscription: Account<'info, Subscription>,
}

#[account]
#[derive(InitSpace)]
pub struct Plan {
    pub merchant: Pubkey,                // 32 bytes
    #[max_len(64)]
    pub name: String,                    // 4 + 64 bytes
    pub price: u64,                      // 8 bytes
    pub interval_seconds: i64,           // 8 bytes
    pub max_subscribers: Option<u64>,    // 9 bytes
    pub active_subscribers: u64,         // 8 bytes
    pub total_subscriptions: u64,        // 8 bytes
    pub total_revenue: u64,              // 8 bytes
    pub active: bool,                    // 1 byte
    pub created_at: i64,                 // 8 bytes
    pub bump: u8,                        // 1 byte
}

#[account]
#[derive(InitSpace)]
pub struct Subscription {
    pub subscriber: Pubkey,              // 32 bytes
    pub plan: Pubkey,                    // 32 bytes
    pub merchant: Pubkey,                // 32 bytes
    pub price: u64,                      // 8 bytes
    pub interval_seconds: i64,           // 8 bytes
    pub start_time: i64,                 // 8 bytes
    pub next_billing: i64,               // 8 bytes
    pub last_payment: Option<i64>,       // 9 bytes
    pub failed_payments: u8,             // 1 byte
    pub total_paid: u64,                 // 8 bytes
    pub status: SubscriptionStatus,      // 1 byte
    pub auto_renew: bool,                // 1 byte
    pub bump: u8,                        // 1 byte
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, InitSpace)]
pub enum SubscriptionStatus {
    Active,
    Suspended,
    Cancelled,
}

#[error_code]
pub enum SubscriptionError {
    #[msg("Invalid price")]
    InvalidPrice,

    #[msg("Invalid interval")]
    InvalidInterval,

    #[msg("Plan name too long")]
    NameTooLong,

    #[msg("Plan is inactive")]
    PlanInactive,

    #[msg("Max subscribers reached")]
    MaxSubscribersReached,

    #[msg("Invalid subscription status")]
    InvalidSubscriptionStatus,

    #[msg("Payment not due yet")]
    PaymentNotDue,

    #[msg("Unauthorized")]
    Unauthorized,

    #[msg("Subscription already cancelled")]
    AlreadyCancelled,
}
