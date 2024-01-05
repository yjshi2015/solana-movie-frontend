use solana_program::{program_error::ProgramError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ReviewError {
    // Error 0
    #[error("Account not initialized yet")]
    UninitializedAccount,

    // Error 1
    #[error("PDA derived does not equal PDA passed in")]
    InvalidPDA,

    // Error 2
    #[error("Input data exceeds max length")]
    InvalidDataLength,

    // Error 3
    #[error("Rating greater than 5 or less than 1")]
    InvalidRating,
}

impl From<ReviewError> for ProgramError {
    fn from(e: ReviewError) -> Self {
        // Rust 中枚举的变体默认从0开始编号
        ProgramError::Custom(e as u32)
    }
}