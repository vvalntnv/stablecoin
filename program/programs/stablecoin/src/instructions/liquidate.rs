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
    utils::assert_account_is_healthy,
};
pub fn liquidate_account(ctx: Context<LiquidateCollateral>, amount_to_burn: u64) -> Result<()> {
    let oracle = &ctx.accounts.oracle;
    let collateral_account = &ctx.accounts.collateral_account;
    let config = &ctx.accounts.config_account;

    if let Err(err) = assert_account_is_healthy(oracle, collateral_account, config) {
        match err {
            Error::AnchorError(e) => e,
            Error::ProgramError(e) => todo!(),
        }
    } else {
        return err!(StablecoinError::CannotLiquidateHealthyAccount);
    }
}

#[derive(Accounts)]
pub struct LiquidateCollateral<'info> {
    #[account(mut)]
    pub depositor: Signer<'info>,

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
        init_if_needed,
        space = Collateral::INIT_SPACE,
        payer = depositor,
        seeds = [COLLAT_SEED, depositor.key().as_ref()],
        bump,
        has_one = depositor,
        has_one = token_account,
        has_one = reserve_account
    )]
    pub collateral_account: Account<'info, Collateral>,

    #[account(
        init_if_needed,
        payer = depositor,
        associated_token::mint = mint,
        associated_token::authority = depositor,
        associated_token::token_program = token_program
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
