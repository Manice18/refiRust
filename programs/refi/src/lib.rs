use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer as SplTransfer};
use solana_program::system_instruction;
pub mod constant;
pub mod states;
use crate::{constant::*, states::*};

// This is your program's public key and it will update
// automatically when you build the project.
declare_id!("AEodbPbcpUUZd9vH4o1EcTH5GNRFeKBDBbFrtTxZYxg6");

#[program]
pub mod refi_prog {
    use super::*;
    pub fn initialize_ngo(ctx: Context<InitializeNgo>) -> Result<()> {
        //Initialize NGO profile Account with default data
        let ngo_profile = &mut ctx.accounts.ngo_profile;
        ngo_profile.authority = ctx.accounts.authority.key();
        ngo_profile.projects_made = 0;
        ngo_profile.investor_count = 0;
        Ok(())
    }

    pub fn initialize_investor(ctx: Context<InitializeInvestor>) -> Result<()> {
        //Initialize Investor profile Account with default data
        let investor_profile = &mut ctx.accounts.investor_profile;
        investor_profile.authority = ctx.accounts.authority.key();
        investor_profile.projects_invested = 0;
        Ok(())
    }

    pub fn add_ngo(
        ctx: Context<AddNgo>,
        name_of_ngo: String,
        date_of_ngo_started: String,
    ) -> Result<()> {
        let ngo_account = &mut ctx.accounts.ngo_account;

        ngo_account.authority = ctx.accounts.authority.key();
        ngo_account.name_of_ngo = name_of_ngo;
        ngo_account.date_of_ngo_started = date_of_ngo_started;

        Ok(())
    }

    pub fn add_investor(
        ctx: Context<AddInvestor>,
        name_of_investor: String,
        type_of_investor: String,
    ) -> Result<()> {
        let investor_account = &mut ctx.accounts.investor_account;

        investor_account.authority = ctx.accounts.authority.key();
        investor_account.name_of_investor = name_of_investor;
        investor_account.type_of_investor = type_of_investor;

        Ok(())
    }

    pub fn add_project(
        ctx: Context<AddProject>,
        idx: u8,
        name_of_project: String,
        start_year: String,
        project_lead: String,
        location: String,
        category: String,
        description: String,
        funding_raised: String,
        project_image: String,
        carbon_captured: u8,
    ) -> Result<()> {
        let ngo_profile = &mut ctx.accounts.ngo_profile;
        let ngo_project = &mut ctx.accounts.ngo_project;

        ngo_project.authority = ctx.accounts.authority.key();
        ngo_project.idx = idx;
        ngo_project.name_of_project = name_of_project;
        ngo_project.start_year = start_year;
        ngo_project.project_lead = project_lead;
        ngo_project.location = location;
        ngo_project.category = category;
        ngo_project.description = description;
        ngo_project.funding_raised = funding_raised;
        ngo_project.project_image = project_image;
        ngo_project.carbon_captured = carbon_captured;

        ngo_profile.projects_made = ngo_profile.projects_made.checked_add(1).unwrap();

        Ok(())
    }

    pub fn transfer_lamports(ctx: Context<TransferLamports>, amount: u64) -> Result<()> {
        let from_account = &ctx.accounts.from;
        let to_account = &ctx.accounts.to;

        // Create the transfer instruction
        let transfer_instruction =
            system_instruction::transfer(from_account.key, to_account.key, amount * 1000000000);

        // Invoke the transfer instruction
        anchor_lang::solana_program::program::invoke_signed(
            &transfer_instruction,
            &[
                from_account.to_account_info(),
                to_account.clone(),
                ctx.accounts.system_program.to_account_info(),
            ],
            &[],
        )?;

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
        seeds = [NGO_TAG, authority.key().as_ref()],
        bump,
        payer = authority,
        space = 42
    )]
    pub ngo_profile: Box<Account<'info, NgoProfile>>,

    pub system_program: Program<'info, System>,
}

// Add Details to the Account
#[derive(Accounts)]
pub struct AddNgo<'info> {
    #[account(
        init,
        seeds = [NAME_TAG ,authority.key().as_ref()],
        bump,
        payer =authority,
        space = 560,
    )]
    pub ngo_account: Box<Account<'info, NgoAccount>>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

//
#[derive(Accounts)]
pub struct AddProject<'info> {
    #[account(
        mut,
        seeds= [NGO_TAG,authority.key().as_ref()],
        bump,
        has_one = authority,
    )]
    pub ngo_profile: Box<Account<'info, NgoProfile>>,

    #[account(
        init,
        seeds = [NEWPROJECT_TAG ,authority.key().as_ref(),&[ngo_profile.projects_made]],
        bump,
        payer =authority,
        space = 3882,
    )]
    pub ngo_project: Box<Account<'info, NgoProject>>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

// Create a pda context for Investor
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

#[derive(Accounts)]
pub struct AddInvestor<'info> {
    #[account(
        init,
        seeds = [INVEST_TAG ,authority.key().as_ref()],
        bump,
        payer =authority,
        space = 560,
    )]
    pub investor_account: Box<Account<'info, InvestorAccount>>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

//transferSOl
#[derive(Accounts)]
pub struct TransferLamports<'info> {
    #[account(mut)]
    pub from: Signer<'info>,
    #[account(mut)]
    pub to: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}
