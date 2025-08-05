use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct MintAccount {
    pub mint_authority: Option<Pubkey>,   // who owns the mint
    pub freeze_authority: Option<Pubkey>, // who can freeze the account
    pub decimals: u8,                     // mint token decimals
    pub supply: u64,                      // the total supply of the account
    pub is_initialized: bool,             // flag to ensure the account is initialized or not
}
