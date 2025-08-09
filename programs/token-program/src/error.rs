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
    #[msg("Account not initialized")]
    UninitializedAccount,
    #[msg("Authority Doesn't exist")]
    AuthorityDoesNotExist,
    #[msg("Insufficient balance in the account")]
    InsufficientBalance,
    #[msg("Already the account is frozen")]
    AlreadyFrozen,
    #[msg("Not frozen yet to unfreeze")]
    NotFrozenYet,
}
