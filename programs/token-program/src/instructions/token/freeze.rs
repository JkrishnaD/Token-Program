use anchor_lang::prelude::*;

use crate::{error::ProgramErrors, TokenAccount};

#[derive(Accounts)]
pub struct Freeze<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(mut,
        has_one = owner,
        seeds = [b"token_account", token_account.mint.key().as_ref(), owner.key().as_ref()],
        bump = token_account.bump,
    )]
    pub token_account: Account<'info, TokenAccount>,
}

impl<'info> Freeze<'info> {
    pub fn freeze_account(&mut self) -> Result<()> {
        // checking if the token account is already frozen
        require!(self.token_account.is_frozen, ProgramErrors::AlreadyFrozen);

        // setting the token account as frozen
        self.token_account.is_frozen = true;
        Ok(())
    }

    pub fn unfreeze_account(&mut self) -> Result<()> {
        // checking if the token account is already unfrozen
        require!(!self.token_account.is_frozen, ProgramErrors::NotFrozenYet);

        // setting the token account as unfrozen
        self.token_account.is_frozen = false;
        Ok(())
    }
}
