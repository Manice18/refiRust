use anchor_lang::prelude::*;
pub mod constant;
pub mod states;
use crate::{constant::*, states::*};

// This is your program's public key and it will update
// automatically when you build the project.
declare_id!("xKgysK9d4Aq9bJYzbQ4LoZNj5qr2rqPJQTcW8PfVLXm");

#[program]
pub mod refi_prog {
    use super::*;
    pub fn initialize_ngo(ctx: Context<InitializeNgo>) -> Result<()> {
        //Initialize NGO profile Account with default data
        let org_profile = &mut ctx.accounts.org_profile;
        org_profile.authority = ctx.accounts.authority.key();
        org_profile.name_org = 0;
        Ok(())
    }

    pub fn initialize_investor(ctx: Context<InitializeInvestor>) -> Result<()> {
        //Initialize Investor profile Account with default data
        let investor_profile = &mut ctx.accounts.investor_profile;
        investor_profile.authority = ctx.accounts.authority.key();
        investor_profile.name_investor = 0;
        Ok(())
    }

    pub fn initialize_community(ctx: Context<InitializeCommunity>) -> Result<()> {
        //Initialize Community profile Account with default data
        let community_profile = &mut ctx.accounts.community_profile;
        community_profile.authority = ctx.accounts.authority.key();
        community_profile.number_of_invests = 0;
        Ok(())
    }
}

// Create a pda context for NGO
#[derive(Accounts)]
pub struct InitializeNgo<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        seeds = [ORG_TAG, authority.key().as_ref()],
        bump,
        payer = authority,
        space = 41
    )]
    pub org_profile: Box<Account<'info, NgoProfile>>,

    pub system_program: Program<'info, System>,
}

// Create a pda context for Individual Investor
#[derive(Accounts)]
pub struct InitializeInvestor<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        seeds = [INVESTOR_TAG, authority.key().as_ref()],
        bump,
        payer = authority,
        space = 41
    )]
    pub investor_profile: Box<Account<'info, InvestorProfile>>,

    pub system_program: Program<'info, System>,
}

// Create a pda context for Community
#[derive(Accounts)]
pub struct InitializeCommunity<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        seeds = [COMMUNITY_TAG, authority.key().as_ref()],
        bump,
        payer = authority,
        space = 41
    )]
    pub community_profile: Box<Account<'info, CommunityProfile>>,

    pub system_program: Program<'info, System>,
}
