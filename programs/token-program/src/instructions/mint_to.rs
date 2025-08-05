use anchor_lang::prelude::*;
use anchor_spl::token::{mint_to, Mint, MintTo as SplMintTo, Token, TokenAccount};

use crate::{error::ProgramErrors, MintAccount};

#[derive(Accounts)]
pub struct MintTo<'info> {
    #[account(mut)]
    pub mint_authority: Signer<'info>,

    #[account(mut)]
    pub mint_account: Account<'info, MintAccount>,

    // the token which we are trying to mint
    #[account(mut)]
    pub spl_mint: Account<'info, Mint>,

    // account where the actual token are stored
    #[account(mut)]
    pub mint_ata: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

impl<'info> MintTo<'info> {
    // mint the tokens into the mint account
    pub fn mint_to(&mut self, amount: u64) -> Result<()> {
        require!(
            self.mint_account.mint_authority == Some(self.mint_authority.key()),
            ProgramErrors::InvalidAuthority
        );

        let accounts = SplMintTo {
            authority: self.mint_account.to_account_info(),
            mint: self.spl_mint.to_account_info(),
            to: self.mint_ata.to_account_info(),
        };

        let ctx = CpiContext::new(self.token_program.to_account_info(), accounts);

        mint_to(ctx, amount)?;

        self.mint_account.supply = self.mint_account.supply.saturating_add(amount);
        Ok(())
    }
}
