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

    pub fn initialize(ctx: Context<InitializeMint>) -> Result<()> {
        Ok(())
    }

    pub fn token_transfer(ctx: Context<TokenTransfer>, amount: u64) -> Result<()> {
        ctx.accounts.transfer(amount)
    }
}
