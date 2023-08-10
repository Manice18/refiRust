use anchor_lang::prelude::*;

// NGO Profile
#[account]
#[derive(Default)]
pub struct NgoProfile {
    pub authority: Pubkey,  // takes space of 32
    pub projects_made: u8,  // takes space of 1
    pub investor_count: u8, //takes space of 1
}

#[account]
#[derive(Default)]
pub struct NgoAccount {
    pub authority: Pubkey,           // 32
    pub name_of_ngo: String,         // 4 +256
    pub date_of_ngo_started: String, // 4 +256
}

#[account]
#[derive(Default)]
pub struct NgoProject {
    pub authority: Pubkey,       //32
    pub idx: u8,                 //1
    pub name_of_project: String, //260
    pub start_year: String,      //260
    pub project_lead: String,    //260
    pub location: String,        //260
    pub category: String,        //260
    pub description: String,     //260
    pub funding_raised: String,  //260
    pub project_image: String,   //4 + 2048
    pub carbon_captured: u8,     //1
}

// Investor Profile
#[account]
#[derive(Default)]
pub struct InvestorProfile {
    pub authority: Pubkey,     // takes space of 32
    pub projects_invested: u8, // takes space of 1
}

#[account]
#[derive(Default)]
pub struct InvestorAccount {
    pub authority: Pubkey,        // 32
    pub name_of_investor: String, //256 + 4
    pub type_of_investor: String, // basically solo or orgnatization // 256+4
}
