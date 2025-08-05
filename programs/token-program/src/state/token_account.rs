use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct TokenAccount {
    pub mint: Pubkey,             // the token mint the which to account represents
    pub owner: Pubkey,            // the authority who owns the account
    pub amount: u64,              // the amount this account holds
    pub delegate: Option<Pubkey>, // to aurtorize the third-party
    pub delegate_amount: u64,     // the amount you delegate to the third-party
    pub is_frozen: bool,          // to freeze the account
    pub is_initialized: bool,     // flag to ensure the account is initialized or not
    pub close_authority: Option<Pubkey>, // the person who can close the account
    pub bump: u8,
}
// bump is used in the tokenAccount not in the mintAccount because here mintAccount is created by the keypair
// and it has the privateKey which can be controlled by a person
// but when it comes to the tokenAccount is created as a pda we need a bump to find it like
// a program create a uniques token account to a user using their wallet key and the seeds
// so we use the bumps in this token account not in the mint account
