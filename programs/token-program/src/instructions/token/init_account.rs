use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount as SplTokenaccount},
};

use crate::{MintAccount, TokenAccount};

#[derive(Accounts)]
pub struct InitAccount<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    // the account which holds the mint
    #[account(
        seeds = [b"mint_account", user.key().as_ref()],
        bump = mint_account.bump
    )]
    pub mint_account: Account<'info, MintAccount>,

    // the mint tokens
    pub spl_mint: Account<'info, Mint>,

    // the account which stores the token of the user which are issues in the token account
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = spl_mint,
        associated_token::authority = user,
    )]
    pub token_ata: Account<'info, SplTokenaccount>,

    // the token account which is used to store the user's token data
    #[account(
        init,
        payer = user,
        space = TokenAccount::INIT_SPACE,
        seeds = [b"token_account",spl_mint.key().as_ref(),user.key().as_ref()],
        bump
    )]
    pub token_account: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> InitAccount<'info> {
    pub fn init_token_account(&mut self) -> Result<()> {
        self.token_account.set_inner(TokenAccount {
            amount: 0,
            mint: self.mint_account.key(),
            owner: self.user.key(),
            bump: self.token_account.bump,
            is_initialized: true,
            is_frozen: false,
            delegate: None,
            delegate_amount: 0,
            close_authority: None,
        });
        Ok(())
    }
}
