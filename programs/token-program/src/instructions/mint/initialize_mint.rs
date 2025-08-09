use anchor_lang::prelude::*;

use crate::MintAccount;

#[derive(Accounts)]
pub struct InitializeMint<'info> {
    // the user who pay's for the account creation
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        payer = owner,
        space= 8 + MintAccount::INIT_SPACE,
        seeds = [b"mint_account", owner.key().as_ref()],
        bump
    )]
    pub mint_account: Account<'info, MintAccount>,

    pub system_program: Program<'info, System>,
}

impl<'info> InitializeMint<'info> {
    pub fn init_mint(
        &mut self,
        supply: u64,
        decimals: u8,
        mint_authority: Option<Pubkey>,
        freeze_authority: Option<Pubkey>,
        bumps: &InitializeMintBumps,
    ) -> Result<()> {
        self.mint_account.set_inner(MintAccount {
            mint_authority,
            freeze_authority,
            decimals,
            supply,
            is_initialized: true,
            bump: bumps.mint_account,
        });
        Ok(())
    }
}
