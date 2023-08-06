use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct NgoProfile {
    pub authority: Pubkey, // takes space of 32
    pub name_org: u8,      // takes space of 1
}

#[account]
#[derive(Default)]
pub struct InvestorProfile {
    pub authority: Pubkey, // takes space of 32
    pub name_investor: u8, // takes space of 1
}

#[account]
#[derive(Default)]
pub struct CommunityProfile {
    pub authority: Pubkey,     // takes space of 32
    pub number_of_invests: u8, // takes space of 1
}
