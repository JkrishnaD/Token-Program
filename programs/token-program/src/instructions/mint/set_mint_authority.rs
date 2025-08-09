use anchor_lang::prelude::*;

use crate::{error::ProgramErrors, MintAccount};

#[derive(Accounts)]
pub struct SetMintAuthority<'info> {
    #[account(mut)]
    pub mint_authority: Signer<'info>,

    #[account(mut)]
    pub mint_account: Account<'info, MintAccount>,

    // we are assign this new_authority account as the mint-authority
    /// CHECK : This is fine, we are just using the account's key
    pub new_authority: Option<AccountInfo<'info>>,
}

impl<'info> SetMintAuthority<'info> {
    pub fn set_mint_authority(&mut self, new_authority: Option<Pubkey>) -> Result<()> {
        // checking for the mint authority
        // if that exist then we can allow the mint authority to change the mint authority
        if let Some(owner) = self.mint_account.mint_authority {
            // if the mint authority is set then we need to check whether the mint authority is same as the signer
            require_keys_eq!(
                *self.mint_authority.key,
                owner,
                ProgramErrors::InvalidAuthority
            );
        } else {
            // if the mint authority is not set then we can set the new authority
            return Err(ProgramErrors::AuthorityDoesNotExist.into());
        }

        // if the new authority is None then we are setting it to None
        // here we can able to pass the none because if the mint_authority want no one to ever mint again
        if let Some(new_auth) = new_authority {
            self.mint_account.mint_authority = Some(new_auth);
        } else {
            self.mint_account.mint_authority = None;
        };

        Ok(())
    }
}
