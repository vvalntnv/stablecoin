use crate::{
    constants::{
        CONFIG_SEED, LIQUIDATION_BONUS, LIQUIDATION_THRESHOLD, MINT_SEED, MIN_HEALTH_FACTOR,
    },
    state::Config,
};
use anchor_lang::prelude::*;
use anchor_spl::{token_2022::Token2022, token_interface::Mint};

pub fn process(ctx: Context<InitializeConfigAccount>) -> Result<()> {
    *ctx.accounts.config_account = Config {
        auth: ctx.accounts.authority.key(),
        mint: ctx.accounts.mint_account.key(),
        liquidation_threshold: LIQUIDATION_THRESHOLD,
        liquidation_bonus: LIQUIDATION_BONUS,
        min_health_factor: MIN_HEALTH_FACTOR,
        bump: ctx.bumps.config_account,
        bump_mint_account: ctx.bumps.mint_account,
    };

    Ok(())
}

#[derive(Accounts)]
pub struct InitializeConfigAccount<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        seeds = [MINT_SEED],
        bump,
        mint::decimals = 6,
        mint::authority = authority.key(),
        mint::freeze_authority = authority.key())
    ]
    pub mint_account: InterfaceAccount<'info, Mint>,

    #[account(
        init,
        payer = authority,
        seeds = [CONFIG_SEED],
        bump,
        space = 8 + Config::INIT_SPACE
    )]
    pub config_account: Account<'info, Config>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token2022>,
}
