use anchor_lang::prelude::*;

#[error_code]
pub enum ProgramErrors {
    #[msg("Custom error message")]
    CustomError,
    #[msg("Invalid mint authority")]
    InvalidAuthority,
    #[msg("Invalid mints")]
    MintMismatch,
    #[msg("Accounts are frozen")]
    FrozenAccount,
    #[msg("Insufficient funds in the account")]
    InsufficientFunds,
}
