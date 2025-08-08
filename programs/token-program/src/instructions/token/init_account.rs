use anchor_lang::prelude::*;

use crate::{MintAccount, TokenAccount};

#[derive(Accounts)]
pub struct InitTokenAccount<'info> {
    // the one who is paying for the account creation
    #[account(mut)]
    pub payer: Signer<'info>,

    // the mint account that the token account will be associated with
    pub mint_account: Account<'info, MintAccount>,

    // the token account that will be created
    #[account(
        init,
        payer = payer,
        space = 8 + TokenAccount::INIT_SPACE,
        seeds = [b"token_account",mint_account.key().as_ref(),payer.key().as_ref()],
        bump
    )]
    pub token_account: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
}

impl<'info> InitTokenAccount<'info> {
    pub fn init_token_account(&mut self,bumps:&InitTokenAccountBumps) -> Result<()> {
        // creating the mint account
        self.token_account.set_inner(TokenAccount {
            amount: 0,
            mint: self.mint_account.key(),
            owner: self.payer.key(),
            bump: bumps.token_account,
            is_initialized: true,
            is_frozen: false,
            delegate: None,
            delegate_amount: 0,
            close_authority: None,
        });
        Ok(())
    }
}
