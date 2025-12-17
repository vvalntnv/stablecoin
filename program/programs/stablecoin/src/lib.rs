mod constants;
mod errors;
mod instructions;
mod state;
mod utils;

use anchor_lang::prelude::*;
use instructions::*;

declare_id!("EtWRBmDRdDXPDQeVd1ToT4aHgRj26qugVmu1G2MV5TWK");

#[program]
pub mod stablecoin {
    use super::*;

    pub fn initialize_config(ctx: Context<InitializeConfigAccount>) -> Result<()> {
        instructions::admin::initialize_config::process(ctx)
    }

    pub fn update_config(
        ctx: Context<UpdateConfContext>,
        min_health_factor: Option<u8>,
        liquidity_threshold: Option<u8>,
        liquidity_bonus: Option<u8>,
    ) -> Result<()> {
        instructions::admin::update_config::process(
            ctx,
            min_health_factor,
            liquidity_threshold,
            liquidity_bonus,
        )
    }

    pub fn deposit_collateral_and_mint_tokens(
        ctx: Context<DepositCollateralAndMintTokens>,
        amount_to_mint: u64,
        amount_deposited: u64,
    ) -> Result<()> {
        instructions::deposit_collateral::process(ctx, amount_to_mint, amount_deposited)
    }
}
