use anchor_lang::prelude::*;

use crate::{error::ProgramErrors, MintAccount, TokenAccount};

#[derive(Accounts)]
pub struct MintTo<'info> {
    #[account(mut)]
    pub mint_authority: Signer<'info>,

    #[account(mut)]
    pub mint_account: Account<'info, MintAccount>,

    // account where the actual token are stored
    #[account(
        mut,
        seeds = [b"token_account",mint_account.key().as_ref(), token_account.owner.key().as_ref()],
        bump = token_account.bump,
    )]
    pub token_account: Account<'info, TokenAccount>,
}

impl<'info> MintTo<'info> {
    // mint the tokens into the mint account
    pub fn mint_to(&mut self, amount: u64) -> Result<()> {
        // first the mint account need to be initialized
        require!(
            self.mint_account.is_initialized,
            ProgramErrors::UninitializedAccount
        );

        require!(
            self.token_account.is_initialized,
            ProgramErrors::UninitializedAccount
        );

        // as the mint_authority is optional in the mint_account so,
        // first we need to check whether it is exist or not then we need to compare the keys
        if let Some(mint_authority) = self.mint_account.mint_authority {
            require_keys_eq!(
                *self.mint_authority.key,
                mint_authority,
                ProgramErrors::InvalidAuthority
            );
        } else {
            return Err(ProgramErrors::AuthorityDoesNotExist.into());
        };

        // adding the amount to the token account and the adding it to the total supply
        self.token_account.amount = self.token_account.amount.saturating_add(amount);
        self.mint_account.supply = self.mint_account.supply.saturating_add(amount);

        Ok(())
    }
}
