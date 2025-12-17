use anchor_lang::prelude::*;

#[error_code]
pub enum StablecoinError {
    #[msg("Price is invalid bro")]
    InvalidPrice,

    #[msg("insufficient collateral")]
    InsufficientCollateral,

    #[msg("Requested Collateral is more than the deposited collateral")]
    InvalidCollateralRequest,
}
