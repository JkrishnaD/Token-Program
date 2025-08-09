#![allow(unexpected_cfgs)]
#![allow(deprecated)]
pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("5rGzLLT482QKhnXdFqf9g2UpZDLsBCXjKH7R9Q3bkbRC");

#[program]
pub mod token_program {
    use super::*;

    // instruction to initialize a mint account
    pub fn initialize_mint(
        ctx: Context<InitializeMint>,
        supply: u64,
        decimals: u8,
        mint_authority: Option<Pubkey>,
        freeze_authority: Option<Pubkey>,
    ) -> Result<()> {
        ctx.accounts.init_mint(
            supply,
            decimals,
            mint_authority,
            freeze_authority,
            &ctx.bumps,
        )
    }

    // instruction to initialize a token account
    pub fn init_token_account(ctx: Context<InitTokenAccount>) -> Result<()> {
        ctx.accounts.init_token_account(&ctx.bumps)
    }

    // instruction to mint tokens to a token account
    pub fn mint_to(ctx: Context<MintTo>, amount: u64) -> Result<()> {
        ctx.accounts.mint_to(amount)
    }

    // if needed we can change the mint authority if it already exist
    pub fn set_mint_authority(
        ctx: Context<SetMintAuthority>,
        mint_authority: Option<Pubkey>,
    ) -> Result<()> {
        ctx.accounts.set_mint_authority(mint_authority)
    }

    // transfering the tokens from one token account to another
    pub fn token_transfer(ctx: Context<TokenTransfer>, amount: u64) -> Result<()> {
        ctx.accounts.transfer(amount)
    }

    // delegate authority to another account
    pub fn delegate(ctx: Context<Delegate>, delegate: Pubkey, amount: u64) -> Result<()> {
        ctx.accounts.delegate_authority(delegate, amount)
    }

    // transfering the delegate amount by the delegate authority
    pub fn delegate_tranfer(ctx: Context<DelegateTransfer>, amount: u64) -> Result<()> {
        ctx.accounts.transfer(amount)
    }

    // revoke the delegate authority
    pub fn revoke(ctx: Context<Delegate>) -> Result<()> {
        ctx.accounts.revoke_delegate()
    }

    // freeze or thaw the token account
    pub fn freeze(ctx: Context<Freeze>) -> Result<()> {
        ctx.accounts.freeze_account()
    }

    pub fn thaw(ctx: Context<Freeze>) -> Result<()> {
        ctx.accounts.thaw_account()
    }

    // burn the tokens from a token account
    pub fn burn_tokens(ctx: Context<Burn>, burn_amount: u64) -> Result<()> {
        ctx.accounts.burn_tokens(burn_amount)
    }

    // close the token account
    pub fn close_account(ctx: Context<Close>) -> Result<()> {
        ctx.accounts.close_account()
    }
}
