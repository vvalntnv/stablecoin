mod constants;
mod instructions;
mod state;

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
}
