use anchor_lang::prelude::*;

use crate::{error::ProgramErrors, MintAccount, TokenAccount};

#[derive(Accounts)]
pub struct Delegate<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        has_one = owner,
        seeds = [b"token_account", token_account.mint.key().as_ref(), owner.key().as_ref()],
        bump = token_account.bump,
    )]
    pub token_account: Account<'info, TokenAccount>,
}

impl<'info> Delegate<'info> {
    pub fn delegate_authority(&mut self, delegate: Pubkey, amount: u64) -> Result<()> {
        // the amount which is on the delegate account should be less than or equal to the token account amount
        require!(
            self.token_account.amount >= amount,
            ProgramErrors::InsufficientBalance
        );

        // checking if the token account is frozen or not
        require!(!self.token_account.is_frozen, ProgramErrors::FrozenAccount);

        if amount == 0 {
            // if the amount is zero then we are removing the delegate
            self.token_account.delegate = None;
            self.token_account.delegate_amount = 0;
        } else {
            self.token_account.delegate = Some(delegate);
            self.token_account.delegate_amount = amount;
        }

        Ok(())
    }

    pub fn revoke_delegate(&mut self) -> Result<()> {
        require!(!self.token_account.is_frozen, ProgramErrors::FrozenAccount);

        // revoking the delegate means setting the delegate to None and delegate amount to 0
        self.token_account.delegate = None;
        self.token_account.delegate_amount = 0;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct DelegateTransfer<'info> {
    #[account(mut)]
    pub delegate_authority: Signer<'info>,

    #[account(mut)]
    pub mint_account: Account<'info, MintAccount>,

    #[account(
        mut,
        seeds = [b"token_account", mint_account.key().as_ref(), from.owner.key().as_ref()],
        bump = from.bump,
    )]
    pub from: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"token_account", mint_account.key().as_ref(), to.owner.key().as_ref()], 
        bump = to.bump,
        constraint = to.is_initialized @ ProgramErrors::UninitializedAccount,
    )]
    pub to: Account<'info, TokenAccount>,
}

impl<'info> DelegateTransfer<'info> {
    pub fn transfer(&mut self, amount: u64) -> Result<()> {
        // checking if the accounts are frozen or not
        require!(self.from.is_frozen == true, ProgramErrors::FrozenAccount);
        require!(self.to.is_frozen == false, ProgramErrors::FrozenAccount);

        // checking if the delegate is authorised to do the transaction or not
        require_keys_eq!(
            self.from.delegate.unwrap_or_default(),
            self.delegate_authority.key(),
            ProgramErrors::InvalidAuthority
        );

        // checking if the mint accounts are same or not
        require!(self.from.mint == self.to.mint, ProgramErrors::MintMismatch);

        // checking if the delegate amount is sufficient or not
        require!(
            self.from.delegate_amount >= amount,
            ProgramErrors::InsufficientFunds
        );

        // deducting the amount from the delegate amount
        self.from.delegate_amount -= amount;

        // if the delegate amount is zero then we are removing the delegate
        if self.from.delegate_amount == 0 {
            self.from.delegate = None;
        }

        // transferring the amount from token account to to account
        self.from.amount -= amount;
        self.to.amount += amount;

        Ok(())
    }
}
