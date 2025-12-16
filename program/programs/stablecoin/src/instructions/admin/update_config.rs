use crate::constants::CONFIG_SEED;
use anchor_lang::prelude::*;

use crate::state::Config;

pub fn process(
    ctx: Context<UpdateConfContext>,
    min_health_factor: Option<u8>,
    liquidity_threshold: Option<u8>,
    liquidity_bonus: Option<u8>,
) -> Result<()> {
    let config = &mut ctx.accounts.config;

    if let Some(mhf) = min_health_factor {
        config.min_health_factor = mhf;
    }

    if let Some(liq_thrsh) = liquidity_threshold {
        config.liquidation_threshold = liq_thrsh;
    }

    if let Some(liq_bonus) = liquidity_bonus {
        config.liquidation_bonus = liq_bonus;
    }

    Ok(())
}

#[derive(Accounts)]
pub struct UpdateConfContext<'info> {
    #[account(
        mut,
        seeds = [CONFIG_SEED],
        bump = config.bump,
    )]
    pub config: Account<'info, Config>,
}
