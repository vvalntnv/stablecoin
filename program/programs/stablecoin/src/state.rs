//  here is where the state will live
//  The requriements are
//      - Collateral Account - where the user will deposit their collateral
//          - here we have to track: who deposited, what is his token account, how much it is
//          deposited. Its asset_account (SOL /it can be any other reserve token/). The
//          reserve_balance. How many tokens have been minted (of our stablecoin)
//      - Config Account - Holds the overal information for the stablecoin account?

use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace, Debug)]
pub struct Collateral {
    pub depositor: Pubkey,
    pub token_account: Pubkey, // the place where our stablecoin will be staked
    pub reserve_account: Pubkey,
    pub reserve_amount: u64,
    pub tokens_minted: u64,
    pub bump: u8,
    pub reserve_account_bump: u8,
    pub is_initialized: bool,
}

#[account]
#[derive(InitSpace, Debug)]
pub struct Config {
    pub auth: Pubkey,
    pub mint: Pubkey,
    /// This value is measured in percentage
    pub liquidation_threshold: u8,
    /// Again in percentage, how much should be payed to the liquidator
    pub liquidation_bonus: u8,
    pub min_health_factor: u8,
    pub bump: u8,
    pub bump_mint_account: u8,
}
