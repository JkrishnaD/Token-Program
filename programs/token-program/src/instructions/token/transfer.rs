use anchor_lang::prelude::*;

use crate::{error::ProgramErrors, MintAccount, TokenAccount};

#[derive(Accounts)]
pub struct TokenTransfer<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    pub mint: Account<'info, MintAccount>,
    #[account(
        mut,
        has_one = owner
    )]
    pub from_ata: Account<'info, TokenAccount>,

    #[account(mut)]
    pub to_ata: Account<'info, TokenAccount>,
}

impl<'info> TokenTransfer<'info> {
    pub fn transfer(&mut self, amount: u64) -> Result<()> {
        let from = &mut self.from_ata;
        let to = &mut self.to_ata;
        let mint = &self.mint;

        if from.amount < amount {
            return Err(ProgramError::InsufficientFunds.into());
        }

        // ensuring both the from and to accounts are from the same mint
        require!(from.mint == mint.key(), ProgramErrors::MintMismatch);
        require!(to.mint == mint.key(), ProgramErrors::MintMismatch);

        // ensuring both the accounts are not frozen
        require!(from.is_frozen == false, ProgramErrors::FrozenAccount);
        require!(to.is_frozen == false, ProgramErrors::FrozenAccount);

        // ensuring the owner of the from account is the same as the signer
        // if not we can't perform the transaction
        require!(
            from.owner == self.owner.key(),
            ProgramErrors::InvalidAuthority
        );

        // checking if the delegate is authorised to do the transaction or not
        if let Some(delegate) = from.delegate.as_ref() {
            // checking the delegate is the owner of the account or not
            require_keys_eq!(*delegate, self.owner.key(), ProgramErrors::InvalidAuthority);
            require!(
                from.delegate_amount >= amount,
                ProgramErrors::InsufficientFunds
            );

            // if the delegate is performing the transtion then we need to deduct
            // the amount from the delegate amount
            from.delegate_amount -= amount;
            if from.delegate_amount == 0 {
                from.delegate = None;
            }
        } else {
            // if there is no delegate then owner need to the from account owner if not
            // we can't perform the transaction
            require_keys_eq!(
                from.owner,
                self.owner.key(),
                ProgramErrors::InvalidAuthority
            );
        }

        if from.amount < amount {
            return Err(ProgramErrors::InsufficientFunds.into());
        }

        from.amount -= amount;
        to.amount += amount;

        Ok(())
    }
}
