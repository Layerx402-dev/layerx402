use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod payment_multisig {
    use super::*;

    /// Create a multisig wallet
    pub fn create_multisig(
        ctx: Context<CreateMultisig>,
        owners: Vec<Pubkey>,
        threshold: u8,
    ) -> Result<()> {
        let multisig = &mut ctx.accounts.multisig;

        require!(owners.len() > 0 && owners.len() <= 10, MultisigError::InvalidOwnerCount);
        require!(threshold > 0, MultisigError::InvalidThreshold);
        require!(
            (threshold as usize) <= owners.len(),
            MultisigError::ThresholdTooHigh
        );

        // Check for duplicate owners
        let mut sorted_owners = owners.clone();
        sorted_owners.sort();
        for i in 1..sorted_owners.len() {
            require!(
                sorted_owners[i] != sorted_owners[i - 1],
                MultisigError::DuplicateOwner
            );
        }

        multisig.owners = owners.clone();
        multisig.threshold = threshold;
        multisig.transaction_count = 0;
        multisig.bump = *ctx.bumps.get("multisig").unwrap();

        msg!("Multisig created with {} owners, threshold: {}", owners.len(), threshold);

        Ok(())
    }

    /// Create a transaction proposal
    pub fn create_transaction(
        ctx: Context<CreateTransaction>,
        recipient: Pubkey,
        amount: u64,
        memo: String,
    ) -> Result<()> {
        let transaction = &mut ctx.accounts.transaction;
        let multisig = &mut ctx.accounts.multisig;
        let clock = Clock::get()?;

        require!(
            multisig.owners.contains(&ctx.accounts.proposer.key()),
            MultisigError::NotAnOwner
        );

        require!(amount > 0, MultisigError::InvalidAmount);
        require!(memo.len() <= 256, MultisigError::MemoTooLong);

        transaction.multisig = multisig.key();
        transaction.transaction_id = multisig.transaction_count;
        transaction.proposer = ctx.accounts.proposer.key();
        transaction.recipient = recipient;
        transaction.amount = amount;
        transaction.memo = memo.clone();
        transaction.approvals = vec![ctx.accounts.proposer.key()]; // Proposer auto-approves
        transaction.rejections = Vec::new();
        transaction.status = TransactionStatus::Pending;
        transaction.created_at = clock.unix_timestamp;
        transaction.executed_at = None;
        transaction.bump = *ctx.bumps.get("transaction").unwrap();

        multisig.transaction_count += 1;

        msg!("Transaction #{} created", transaction.transaction_id);
        msg!("  Recipient: {}", recipient);
        msg!("  Amount: {} lamports", amount);
        msg!("  Memo: {}", memo);

        Ok(())
    }

    /// Approve a transaction
    pub fn approve_transaction(ctx: Context<ApproveTransaction>) -> Result<()> {
        let transaction = &mut ctx.accounts.transaction;
        let multisig = &ctx.accounts.multisig;

        require!(
            multisig.owners.contains(&ctx.accounts.owner.key()),
            MultisigError::NotAnOwner
        );

        require!(
            transaction.status == TransactionStatus::Pending,
            MultisigError::InvalidTransactionStatus
        );

        require!(
            !transaction.approvals.contains(&ctx.accounts.owner.key()),
            MultisigError::AlreadyApproved
        );

        require!(
            !transaction.rejections.contains(&ctx.accounts.owner.key()),
            MultisigError::AlreadyRejected
        );

        transaction.approvals.push(ctx.accounts.owner.key());

        msg!("Transaction #{} approved by {}", transaction.transaction_id, ctx.accounts.owner.key());
        msg!("  Approvals: {}/{}", transaction.approvals.len(), multisig.threshold);

        // Check if threshold is met
        if transaction.approvals.len() >= multisig.threshold as usize {
            transaction.status = TransactionStatus::Approved;
            msg!("Transaction approved! Ready for execution");
        }

        Ok(())
    }

    /// Reject a transaction
    pub fn reject_transaction(ctx: Context<RejectTransaction>) -> Result<()> {
        let transaction = &mut ctx.accounts.transaction;
        let multisig = &ctx.accounts.multisig;

        require!(
            multisig.owners.contains(&ctx.accounts.owner.key()),
            MultisigError::NotAnOwner
        );

        require!(
            transaction.status == TransactionStatus::Pending ||
            transaction.status == TransactionStatus::Approved,
            MultisigError::InvalidTransactionStatus
        );

        require!(
            !transaction.rejections.contains(&ctx.accounts.owner.key()),
            MultisigError::AlreadyRejected
        );

        // Remove approval if exists
        transaction.approvals.retain(|&owner| owner != ctx.accounts.owner.key());
        transaction.rejections.push(ctx.accounts.owner.key());

        msg!("Transaction #{} rejected by {}", transaction.transaction_id, ctx.accounts.owner.key());

        // Calculate if transaction can still pass
        let max_possible_approvals = multisig.owners.len() - transaction.rejections.len();
        if max_possible_approvals < multisig.threshold as usize {
            transaction.status = TransactionStatus::Rejected;
            msg!("Transaction rejected! Cannot reach threshold");
        } else if transaction.approvals.len() < multisig.threshold as usize {
            transaction.status = TransactionStatus::Pending;
        }

        Ok(())
    }

    /// Execute an approved transaction
    pub fn execute_transaction(ctx: Context<ExecuteTransaction>) -> Result<()> {
        let transaction = &mut ctx.accounts.transaction;
        let multisig = &ctx.accounts.multisig;
        let clock = Clock::get()?;

        require!(
            multisig.owners.contains(&ctx.accounts.executor.key()),
            MultisigError::NotAnOwner
        );

        require!(
            transaction.status == TransactionStatus::Approved,
            MultisigError::NotApproved
        );

        // Transfer funds from multisig to recipient
        **ctx.accounts.multisig.to_account_info().try_borrow_mut_lamports()? -= transaction.amount;
        **ctx.accounts.recipient.try_borrow_mut_lamports()? += transaction.amount;

        transaction.status = TransactionStatus::Executed;
        transaction.executed_at = Some(clock.unix_timestamp);

        msg!("Transaction #{} executed", transaction.transaction_id);
        msg!("  Transferred {} lamports to {}", transaction.amount, transaction.recipient);

        Ok(())
    }

    /// Cancel a pending transaction (proposer only)
    pub fn cancel_transaction(ctx: Context<CancelTransaction>) -> Result<()> {
        let transaction = &mut ctx.accounts.transaction;

        require!(
            ctx.accounts.proposer.key() == transaction.proposer,
            MultisigError::NotProposer
        );

        require!(
            transaction.status == TransactionStatus::Pending ||
            transaction.status == TransactionStatus::Approved,
            MultisigError::InvalidTransactionStatus
        );

        transaction.status = TransactionStatus::Cancelled;

        msg!("Transaction #{} cancelled", transaction.transaction_id);

        Ok(())
    }

    /// Add owner to multisig (requires threshold approval)
    pub fn add_owner(
        ctx: Context<AddOwner>,
        new_owner: Pubkey,
    ) -> Result<()> {
        let multisig = &mut ctx.accounts.multisig;

        require!(
            multisig.owners.len() < 10,
            MultisigError::TooManyOwners
        );

        require!(
            !multisig.owners.contains(&new_owner),
            MultisigError::AlreadyOwner
        );

        multisig.owners.push(new_owner);

        msg!("Owner added: {}", new_owner);
        msg!("  Total owners: {}", multisig.owners.len());

        Ok(())
    }

    /// Remove owner from multisig (requires threshold approval)
    pub fn remove_owner(
        ctx: Context<RemoveOwner>,
        owner_to_remove: Pubkey,
    ) -> Result<()> {
        let multisig = &mut ctx.accounts.multisig;

        require!(
            multisig.owners.contains(&owner_to_remove),
            MultisigError::NotAnOwner
        );

        require!(
            multisig.owners.len() > 1,
            MultisigError::CannotRemoveLastOwner
        );

        multisig.owners.retain(|&owner| owner != owner_to_remove);

        // Adjust threshold if necessary
        if (multisig.threshold as usize) > multisig.owners.len() {
            multisig.threshold = multisig.owners.len() as u8;
            msg!("Threshold adjusted to {}", multisig.threshold);
        }

        msg!("Owner removed: {}", owner_to_remove);
        msg!("  Remaining owners: {}", multisig.owners.len());

        Ok(())
    }

    /// Change threshold (requires threshold approval)
    pub fn change_threshold(
        ctx: Context<ChangeThreshold>,
        new_threshold: u8,
    ) -> Result<()> {
        let multisig = &mut ctx.accounts.multisig;

        require!(new_threshold > 0, MultisigError::InvalidThreshold);
        require!(
            (new_threshold as usize) <= multisig.owners.len(),
            MultisigError::ThresholdTooHigh
        );

        let old_threshold = multisig.threshold;
        multisig.threshold = new_threshold;

        msg!("Threshold changed: {} -> {}", old_threshold, new_threshold);

        Ok(())
    }

    /// Get multisig info
    pub fn get_multisig_info(ctx: Context<GetMultisigInfo>) -> Result<()> {
        let multisig = &ctx.accounts.multisig;

        msg!("Multisig Info:");
        msg!("  Owners: {}", multisig.owners.len());
        msg!("  Threshold: {}", multisig.threshold);
        msg!("  Total transactions: {}", multisig.transaction_count);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateMultisig<'info> {
    #[account(
        init,
        payer = creator,
        space = 8 + Multisig::INIT_SPACE,
        seeds = [b"multisig", creator.key().as_ref()],
        bump
    )]
    pub multisig: Account<'info, Multisig>,

    #[account(mut)]
    pub creator: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateTransaction<'info> {
    #[account(
        init,
        payer = proposer,
        space = 8 + Transaction::INIT_SPACE,
        seeds = [b"transaction", multisig.key().as_ref(), &multisig.transaction_count.to_le_bytes()],
        bump
    )]
    pub transaction: Account<'info, Transaction>,

    #[account(mut)]
    pub multisig: Account<'info, Multisig>,

    #[account(mut)]
    pub proposer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ApproveTransaction<'info> {
    #[account(mut)]
    pub transaction: Account<'info, Transaction>,

    pub multisig: Account<'info, Multisig>,

    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct RejectTransaction<'info> {
    #[account(mut)]
    pub transaction: Account<'info, Transaction>,

    pub multisig: Account<'info, Multisig>,

    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct ExecuteTransaction<'info> {
    #[account(mut)]
    pub transaction: Account<'info, Transaction>,

    #[account(mut)]
    pub multisig: Account<'info, Multisig>,

    /// CHECK: Recipient receiving funds
    #[account(mut)]
    pub recipient: AccountInfo<'info>,

    pub executor: Signer<'info>,
}

#[derive(Accounts)]
pub struct CancelTransaction<'info> {
    #[account(mut)]
    pub transaction: Account<'info, Transaction>,

    pub proposer: Signer<'info>,
}

#[derive(Accounts)]
pub struct AddOwner<'info> {
    #[account(mut)]
    pub multisig: Account<'info, Multisig>,

    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct RemoveOwner<'info> {
    #[account(mut)]
    pub multisig: Account<'info, Multisig>,

    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct ChangeThreshold<'info> {
    #[account(mut)]
    pub multisig: Account<'info, Multisig>,

    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct GetMultisigInfo<'info> {
    pub multisig: Account<'info, Multisig>,
}

#[account]
#[derive(InitSpace)]
pub struct Multisig {
    #[max_len(10)]
    pub owners: Vec<Pubkey>,            // 4 + (32 * 10) bytes
    pub threshold: u8,                  // 1 byte
    pub transaction_count: u64,         // 8 bytes
    pub bump: u8,                       // 1 byte
}

#[account]
#[derive(InitSpace)]
pub struct Transaction {
    pub multisig: Pubkey,               // 32 bytes
    pub transaction_id: u64,            // 8 bytes
    pub proposer: Pubkey,               // 32 bytes
    pub recipient: Pubkey,              // 32 bytes
    pub amount: u64,                    // 8 bytes
    #[max_len(256)]
    pub memo: String,                   // 4 + 256 bytes
    #[max_len(10)]
    pub approvals: Vec<Pubkey>,         // 4 + (32 * 10) bytes
    #[max_len(10)]
    pub rejections: Vec<Pubkey>,        // 4 + (32 * 10) bytes
    pub status: TransactionStatus,      // 1 byte
    pub created_at: i64,                // 8 bytes
    pub executed_at: Option<i64>,       // 9 bytes
    pub bump: u8,                       // 1 byte
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, InitSpace)]
pub enum TransactionStatus {
    Pending,
    Approved,
    Rejected,
    Executed,
    Cancelled,
}

#[error_code]
pub enum MultisigError {
    #[msg("Invalid owner count")]
    InvalidOwnerCount,

    #[msg("Invalid threshold")]
    InvalidThreshold,

    #[msg("Threshold too high")]
    ThresholdTooHigh,

    #[msg("Duplicate owner")]
    DuplicateOwner,

    #[msg("Not an owner")]
    NotAnOwner,

    #[msg("Invalid amount")]
    InvalidAmount,

    #[msg("Memo too long")]
    MemoTooLong,

    #[msg("Invalid transaction status")]
    InvalidTransactionStatus,

    #[msg("Already approved")]
    AlreadyApproved,

    #[msg("Already rejected")]
    AlreadyRejected,

    #[msg("Transaction not approved")]
    NotApproved,

    #[msg("Not the proposer")]
    NotProposer,

    #[msg("Too many owners")]
    TooManyOwners,

    #[msg("Already an owner")]
    AlreadyOwner,

    #[msg("Cannot remove last owner")]
    CannotRemoveLastOwner,
}
