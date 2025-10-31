use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod payment_dispute {
    use super::*;

    /// Initialize dispute resolution system
    pub fn initialize_resolver(
        ctx: Context<InitializeResolver>,
        arbitrators: Vec<Pubkey>,
    ) -> Result<()> {
        let resolver = &mut ctx.accounts.resolver;

        require!(
            arbitrators.len() > 0 && arbitrators.len() <= 10,
            DisputeError::InvalidArbitratorCount
        );

        resolver.authority = ctx.accounts.authority.key();
        resolver.arbitrators = arbitrators;
        resolver.total_disputes = 0;
        resolver.resolved_disputes = 0;
        resolver.bump = *ctx.bumps.get("resolver").unwrap();

        msg!("Dispute resolver initialized with {} arbitrators", resolver.arbitrators.len());

        Ok(())
    }

    /// Create a new dispute
    pub fn create_dispute(
        ctx: Context<CreateDispute>,
        payment_id: String,
        amount: u64,
        reason: String,
        evidence: String,
    ) -> Result<()> {
        let dispute = &mut ctx.accounts.dispute;
        let resolver = &mut ctx.accounts.resolver;
        let clock = Clock::get()?;

        require!(amount > 0, DisputeError::InvalidAmount);
        require!(reason.len() > 0 && reason.len() <= 256, DisputeError::InvalidReason);
        require!(evidence.len() <= 1024, DisputeError::EvidenceTooLong);

        dispute.payment_id = payment_id.clone();
        dispute.claimant = ctx.accounts.claimant.key();
        dispute.respondent = ctx.accounts.respondent.key();
        dispute.amount = amount;
        dispute.reason = reason.clone();
        dispute.evidence = vec![Evidence {
            submitter: ctx.accounts.claimant.key(),
            content: evidence,
            submitted_at: clock.unix_timestamp,
        }];
        dispute.status = DisputeStatus::Open;
        dispute.votes_for_claimant = 0;
        dispute.votes_for_respondent = 0;
        dispute.assigned_arbitrators = Vec::new();
        dispute.created_at = clock.unix_timestamp;
        dispute.resolved_at = None;
        dispute.resolution = None;
        dispute.bump = *ctx.bumps.get("dispute").unwrap();

        resolver.total_disputes += 1;

        msg!("Dispute created for payment: {}", payment_id);
        msg!("  Amount: {} lamports", amount);
        msg!("  Reason: {}", reason);

        Ok(())
    }

    /// Submit additional evidence
    pub fn submit_evidence(
        ctx: Context<SubmitEvidence>,
        evidence_content: String,
    ) -> Result<()> {
        let dispute = &mut ctx.accounts.dispute;
        let clock = Clock::get()?;

        require!(
            dispute.status == DisputeStatus::Open || dispute.status == DisputeStatus::UnderReview,
            DisputeError::InvalidDisputeStatus
        );

        require!(
            ctx.accounts.submitter.key() == dispute.claimant
                || ctx.accounts.submitter.key() == dispute.respondent,
            DisputeError::Unauthorized
        );

        require!(
            evidence_content.len() <= 1024,
            DisputeError::EvidenceTooLong
        );

        dispute.evidence.push(Evidence {
            submitter: ctx.accounts.submitter.key(),
            content: evidence_content,
            submitted_at: clock.unix_timestamp,
        });

        msg!("Evidence submitted by {}", ctx.accounts.submitter.key());

        Ok(())
    }

    /// Assign arbitrators to dispute
    pub fn assign_arbitrators(
        ctx: Context<AssignArbitrators>,
        arbitrator_indices: Vec<u8>,
    ) -> Result<()> {
        let dispute = &mut ctx.accounts.dispute;
        let resolver = &ctx.accounts.resolver;

        require!(
            ctx.accounts.authority.key() == resolver.authority,
            DisputeError::Unauthorized
        );

        require!(
            dispute.status == DisputeStatus::Open,
            DisputeError::InvalidDisputeStatus
        );

        require!(
            arbitrator_indices.len() >= 3 && arbitrator_indices.len() % 2 == 1,
            DisputeError::InvalidArbitratorCount
        );

        // Verify arbitrator indices
        for &idx in &arbitrator_indices {
            require!(
                (idx as usize) < resolver.arbitrators.len(),
                DisputeError::InvalidArbitratorIndex
            );
        }

        dispute.assigned_arbitrators = arbitrator_indices
            .iter()
            .map(|&idx| resolver.arbitrators[idx as usize])
            .collect();

        dispute.status = DisputeStatus::UnderReview;

        msg!("Assigned {} arbitrators to dispute", dispute.assigned_arbitrators.len());

        Ok(())
    }

    /// Cast arbitrator vote
    pub fn cast_vote(
        ctx: Context<CastVote>,
        vote_for_claimant: bool,
    ) -> Result<()> {
        let dispute = &mut ctx.accounts.dispute;

        require!(
            dispute.status == DisputeStatus::UnderReview,
            DisputeError::InvalidDisputeStatus
        );

        require!(
            dispute.assigned_arbitrators.contains(&ctx.accounts.arbitrator.key()),
            DisputeError::NotAssignedArbitrator
        );

        if vote_for_claimant {
            dispute.votes_for_claimant += 1;
            msg!("Vote cast for claimant");
        } else {
            dispute.votes_for_respondent += 1;
            msg!("Vote cast for respondent");
        }

        // Check if voting is complete
        let total_votes = dispute.votes_for_claimant + dispute.votes_for_respondent;
        let required_votes = dispute.assigned_arbitrators.len() as u8;

        if total_votes >= required_votes {
            // Majority wins
            if dispute.votes_for_claimant > dispute.votes_for_respondent {
                dispute.status = DisputeStatus::ResolvedForClaimant;
                msg!("Dispute resolved in favor of claimant");
            } else {
                dispute.status = DisputeStatus::ResolvedForRespondent;
                msg!("Dispute resolved in favor of respondent");
            }
        }

        Ok(())
    }

    /// Finalize dispute resolution
    pub fn finalize_dispute(
        ctx: Context<FinalizeDispute>,
        resolution_notes: String,
    ) -> Result<()> {
        let dispute = &mut ctx.accounts.dispute;
        let resolver = &mut ctx.accounts.resolver;
        let clock = Clock::get()?;

        require!(
            ctx.accounts.authority.key() == resolver.authority,
            DisputeError::Unauthorized
        );

        require!(
            dispute.status == DisputeStatus::ResolvedForClaimant
                || dispute.status == DisputeStatus::ResolvedForRespondent,
            DisputeError::InvalidDisputeStatus
        );

        // Transfer funds based on resolution
        if dispute.status == DisputeStatus::ResolvedForClaimant {
            // Transfer to claimant
            **ctx.accounts.escrow.try_borrow_mut_lamports()? -= dispute.amount;
            **ctx.accounts.claimant.try_borrow_mut_lamports()? += dispute.amount;
            msg!("Funds transferred to claimant");
        } else {
            // Transfer to respondent
            **ctx.accounts.escrow.try_borrow_mut_lamports()? -= dispute.amount;
            **ctx.accounts.respondent.try_borrow_mut_lamports()? += dispute.amount;
            msg!("Funds transferred to respondent");
        }

        dispute.resolved_at = Some(clock.unix_timestamp);
        dispute.resolution = Some(resolution_notes);
        dispute.status = DisputeStatus::Finalized;

        resolver.resolved_disputes += 1;

        msg!("Dispute finalized");

        Ok(())
    }

    /// Appeal a dispute decision
    pub fn appeal_dispute(
        ctx: Context<AppealDispute>,
        appeal_reason: String,
    ) -> Result<()> {
        let dispute = &mut ctx.accounts.dispute;
        let clock = Clock::get()?;

        require!(
            dispute.status == DisputeStatus::ResolvedForClaimant
                || dispute.status == DisputeStatus::ResolvedForRespondent,
            DisputeError::InvalidDisputeStatus
        );

        require!(
            ctx.accounts.appellant.key() == dispute.claimant
                || ctx.accounts.appellant.key() == dispute.respondent,
            DisputeError::Unauthorized
        );

        // Check appeal window (7 days)
        let appeal_deadline = dispute.created_at + (7 * 86400);
        require!(
            clock.unix_timestamp <= appeal_deadline,
            DisputeError::AppealWindowExpired
        );

        dispute.status = DisputeStatus::Appealed;
        dispute.evidence.push(Evidence {
            submitter: ctx.accounts.appellant.key(),
            content: format!("APPEAL: {}", appeal_reason),
            submitted_at: clock.unix_timestamp,
        });

        msg!("Dispute appealed");

        Ok(())
    }

    /// Get dispute info
    pub fn get_dispute_info(ctx: Context<GetDisputeInfo>) -> Result<()> {
        let dispute = &ctx.accounts.dispute;

        msg!("Dispute Info:");
        msg!("  Payment ID: {}", dispute.payment_id);
        msg!("  Status: {:?}", dispute.status);
        msg!("  Amount: {} lamports", dispute.amount);
        msg!("  Claimant: {}", dispute.claimant);
        msg!("  Respondent: {}", dispute.respondent);
        msg!("  Votes - Claimant: {}, Respondent: {}",
             dispute.votes_for_claimant,
             dispute.votes_for_respondent);
        msg!("  Evidence count: {}", dispute.evidence.len());

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeResolver<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + Resolver::INIT_SPACE,
        seeds = [b"resolver"],
        bump
    )]
    pub resolver: Account<'info, Resolver>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(payment_id: String)]
pub struct CreateDispute<'info> {
    #[account(
        init,
        payer = claimant,
        space = 8 + Dispute::INIT_SPACE,
        seeds = [b"dispute", payment_id.as_bytes()],
        bump
    )]
    pub dispute: Account<'info, Dispute>,

    #[account(
        mut,
        seeds = [b"resolver"],
        bump = resolver.bump
    )]
    pub resolver: Account<'info, Resolver>,

    #[account(mut)]
    pub claimant: Signer<'info>,

    /// CHECK: Respondent in the dispute
    pub respondent: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SubmitEvidence<'info> {
    #[account(mut)]
    pub dispute: Account<'info, Dispute>,

    pub submitter: Signer<'info>,
}

#[derive(Accounts)]
pub struct AssignArbitrators<'info> {
    #[account(mut)]
    pub dispute: Account<'info, Dispute>,

    #[account(
        seeds = [b"resolver"],
        bump = resolver.bump
    )]
    pub resolver: Account<'info, Resolver>,

    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct CastVote<'info> {
    #[account(mut)]
    pub dispute: Account<'info, Dispute>,

    pub arbitrator: Signer<'info>,
}

#[derive(Accounts)]
pub struct FinalizeDispute<'info> {
    #[account(mut)]
    pub dispute: Account<'info, Dispute>,

    #[account(
        seeds = [b"resolver"],
        bump = resolver.bump
    )]
    pub resolver: Account<'info, Resolver>,

    /// CHECK: Escrow holding funds
    #[account(mut)]
    pub escrow: AccountInfo<'info>,

    /// CHECK: Claimant receiving funds
    #[account(mut)]
    pub claimant: AccountInfo<'info>,

    /// CHECK: Respondent receiving funds
    #[account(mut)]
    pub respondent: AccountInfo<'info>,

    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct AppealDispute<'info> {
    #[account(mut)]
    pub dispute: Account<'info, Dispute>,

    pub appellant: Signer<'info>,
}

#[derive(Accounts)]
pub struct GetDisputeInfo<'info> {
    pub dispute: Account<'info, Dispute>,
}

#[account]
#[derive(InitSpace)]
pub struct Resolver {
    pub authority: Pubkey,               // 32 bytes
    #[max_len(10)]
    pub arbitrators: Vec<Pubkey>,        // 4 + (32 * 10) bytes
    pub total_disputes: u64,             // 8 bytes
    pub resolved_disputes: u64,          // 8 bytes
    pub bump: u8,                        // 1 byte
}

#[account]
#[derive(InitSpace)]
pub struct Dispute {
    #[max_len(64)]
    pub payment_id: String,              // 4 + 64 bytes
    pub claimant: Pubkey,                // 32 bytes
    pub respondent: Pubkey,              // 32 bytes
    pub amount: u64,                     // 8 bytes
    #[max_len(256)]
    pub reason: String,                  // 4 + 256 bytes
    #[max_len(10)]
    pub evidence: Vec<Evidence>,         // Complex
    pub status: DisputeStatus,           // 1 byte
    pub votes_for_claimant: u8,          // 1 byte
    pub votes_for_respondent: u8,        // 1 byte
    #[max_len(10)]
    pub assigned_arbitrators: Vec<Pubkey>, // 4 + (32 * 10) bytes
    pub created_at: i64,                 // 8 bytes
    pub resolved_at: Option<i64>,        // 9 bytes
    #[max_len(512)]
    pub resolution: Option<String>,      // 4 + 512 bytes
    pub bump: u8,                        // 1 byte
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct Evidence {
    pub submitter: Pubkey,               // 32 bytes
    #[max_len(1024)]
    pub content: String,                 // 4 + 1024 bytes
    pub submitted_at: i64,               // 8 bytes
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, InitSpace)]
pub enum DisputeStatus {
    Open,
    UnderReview,
    ResolvedForClaimant,
    ResolvedForRespondent,
    Appealed,
    Finalized,
}

#[error_code]
pub enum DisputeError {
    #[msg("Invalid arbitrator count")]
    InvalidArbitratorCount,

    #[msg("Invalid amount")]
    InvalidAmount,

    #[msg("Invalid reason")]
    InvalidReason,

    #[msg("Evidence too long")]
    EvidenceTooLong,

    #[msg("Invalid dispute status")]
    InvalidDisputeStatus,

    #[msg("Unauthorized")]
    Unauthorized,

    #[msg("Invalid arbitrator index")]
    InvalidArbitratorIndex,

    #[msg("Not assigned as arbitrator")]
    NotAssignedArbitrator,

    #[msg("Appeal window expired")]
    AppealWindowExpired,
}
