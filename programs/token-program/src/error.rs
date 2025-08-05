use anchor_lang::prelude::*;

#[error_code]
pub enum ProgramErrors {
    #[msg("Custom error message")]
    CustomError,
    #[msg("Invalid mint authority")]
    InvalidAuthority
}
