use anchor_lang::prelude::*;

use crate::{error::ProgramErrors, MintAccount, TokenAccount};

#[derive(Accounts)]
pub struct Burn<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(mut)]
    pub mint_account: Account<'info, MintAccount>,

    #[account(
        mut,
        has_one = owner,
        seeds = [b"token_account", token_account.mint.key().as_ref(), owner.key().as_ref()],
        bump = token_account.bump,
    )]
    pub token_account: Account<'info, TokenAccount>,
}

impl<'info> Burn<'info> {
    pub fn burn_tokens(&mut self, amount: u64) -> Result<()> {
        // check if the token account is frozen
        require!(!self.token_account.is_frozen, ProgramErrors::FrozenAccount);

        //comparing the mints in both the accounts
        require!(
            self.token_account.mint == self.mint_account.key(),
            ProgramErrors::MintMismatch
        );

        // check if the amount to burn is greater than zero
        require!(amount > 0, ProgramErrors::InvalidAmount);

        // check if the token account has enough balance to burn
        require!(
            self.token_account.amount >= amount,
            ProgramErrors::InsufficientBalance
        );

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

    // the function helps to burn the tokens which are assigned to the delegate
    pub fn delegate_burn(&mut self, delegate: Pubkey, amount: u64) -> Result<()> {
        // check if the token account is frozen
        require!(!self.token_account.is_frozen, ProgramErrors::FrozenAccount);

        // check if the delegate is set
        require!(
            self.token_account.delegate == Some(delegate),
            ProgramErrors::InvalidDelegate
        );

        // check if the delegate amount is sufficient
        require!(
            self.token_account.delegate_amount >= amount,
            ProgramErrors::InsufficientDelegateAmount
        );

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

        // decrease the delegate amount
        self.token_account.delegate_amount = self
            .token_account
            .delegate_amount
            .checked_sub(amount)
            .ok_or(ProgramErrors::InsufficientDelegateAmount)?;

        Ok(())
    }
}
