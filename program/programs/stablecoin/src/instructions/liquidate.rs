use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_2022::Token2022,
    token_interface::{Mint, TokenAccount},
};
use pyth_solana_receiver_sdk::price_update::PriceUpdateV2;

use crate::{
    constants::{COLLAT_SEED, CONFIG_SEED, MINT_SEED, RESERVE_ACCOUNT_SEED},
    errors::StablecoinError,
    state::{Collateral, Config},
    utils::{assert_account_is_healthy, liquidate_collateral},
};
pub fn process(ctx: Context<LiquidateCollateral>) -> Result<()> {
    let liq_token_account = &ctx.accounts.token_account;
    let collateral_account = &mut ctx.accounts.collateral_account;

    let liquidator = &ctx.accounts.liquidator;
    let oracle = &ctx.accounts.oracle;
    let config = &ctx.accounts.config_account;

    let token_program = &ctx.accounts.token_program;
    let mint = &ctx.accounts.mint;

    let health_check = assert_account_is_healthy(oracle, collateral_account, config);

    match health_check {
        // 1. If the account is healthy, liquidation is illegal.
        Ok(_) => return err!(StablecoinError::CannotLiquidateHealthyAccount),

        // 2. If it's unhealthy because of insufficient collateral, proceed ().
        Err(Error::AnchorError(e))
            if e.error_name == StablecoinError::InsufficientCollateral.name() =>
        {
            liquidate_collateral(
                liquidator,
                liq_token_account,
                mint,
                collateral_account,
                token_program,
                config,
            )
        }

        // 3. If it's any other error, propagate that error.
        Err(e) => Err(e),
    }?;

    Ok(())
}

#[derive(Accounts)]
pub struct LiquidateCollateral<'info> {
    pub depositor: SystemAccount<'info>,

    #[account(
        mut,
        constraint = liquidator.key() != depositor.key()
    )]
    pub liquidator: Signer<'info>,

    #[account(
        seeds = [CONFIG_SEED],
        bump,
        has_one = mint
    )]
    pub config_account: Account<'info, Config>,

    #[account(
        mut,
        seeds = [MINT_SEED],
        bump = config_account.bump_mint_account
    )]
    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        seeds = [COLLAT_SEED, depositor.key().as_ref()],
        bump = collateral_account.bump,
        has_one = depositor,
        has_one = token_account,
        has_one = reserve_account
    )]
    pub collateral_account: Account<'info, Collateral>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = liquidator,
        associated_token::token_program = token_program,
    )]
    pub token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [RESERVE_ACCOUNT_SEED, depositor.key().as_ref()],
        bump
    )]
    pub reserve_account: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token2022>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub oracle: Account<'info, PriceUpdateV2>,
}
