use anchor_lang::prelude::*;
use anchor_spl::token::spl_token::instruction::AuthorityType::MintTokens;
use anchor_spl::token::{set_authority, SetAuthority, Token};

use crate::{error::ProgramErrors, MintAccount};

#[derive(Accounts)]
pub struct MintAuthorityAccount<'info> {
    #[account(mut)]
    pub mint_authority: Signer<'info>,

    #[account(mut)]
    pub mint_account: Account<'info, MintAccount>,

    // we are assign this new_authority account as the mint-authority
    /// CHECK : This is fine, we are just using the account's key
    pub new_authority: Option<AccountInfo<'info>>,

    pub token_program: Program<'info, Token>,
}

impl<'info> MintAuthorityAccount<'info> {
    pub fn set_mint_authority(&mut self, new_authority: Option<Pubkey>) -> Result<()> {
        require!(
            self.mint_account.mint_authority == Some(self.mint_authority.key()),
            ProgramErrors::InvalidAuthority
        );

        let cpi_accounts = SetAuthority {
            current_authority: self.mint_authority.to_account_info(),
            account_or_mint: self.mint_account.to_account_info(),
        };

        let cpi_context = CpiContext::new(self.token_program.to_account_info(), cpi_accounts);
        set_authority(cpi_context, MintTokens, new_authority)?;

        Ok(())
    }
}
