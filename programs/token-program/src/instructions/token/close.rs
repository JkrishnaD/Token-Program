use anchor_lang::prelude::*;

use crate::{error::ProgramErrors, MintAccount, TokenAccount};

#[derive(Accounts)]
pub struct Close<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        has_one = owner,
        seeds = [b"token_account", token_account.mint.key().as_ref(), owner.key().as_ref()],
        bump = token_account.bump,
        close = owner
    )]
    pub token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"mint_account", mint_account.key().as_ref()],
        bump = mint_account.bump,
    )]
    pub mint_account: Account<'info, MintAccount>,
}

impl<'info> Close<'info> {
    pub fn close_account(&mut self) -> Result<()> {
        require!(!self.token_account.is_frozen, ProgramErrors::FrozenAccount);

        if self.token_account.amount > 0 {
            let amount = self.token_account.amount;
            self.burn_tokens(amount)?;
        };

        Ok(())
    }
    pub fn burn_tokens(&mut self, amount: u64) -> Result<()> {
        // decrease the token account's amount
        self.token_account.amount = self
            .token_account
            .amount
            .checked_sub(amount)
            .ok_or(ProgramErrors::InsufficientBalance)?;

        // decrease the mint account's supply
        self.mint_account.supply = self
            .mint_account
            .supply
            .checked_sub(amount)
            .ok_or(ProgramErrors::InsufficientBalance)?;

        Ok(())
    }
}
