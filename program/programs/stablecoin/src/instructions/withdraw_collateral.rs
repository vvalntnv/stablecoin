use crate::constants::RESERVE_ACCOUNT_SEED;
use crate::errors::StablecoinError;
use crate::utils::{assert_account_is_healthy, burn_tokens, withdraw_collateral};
use crate::{
    constants::{COLLAT_SEED, CONFIG_SEED, MINT_SEED},
    state::Config,
};
use anchor_lang::prelude::*;
use anchor_spl::token_2022::Token2022;
use anchor_spl::token_interface::{Mint, TokenAccount};
use pyth_solana_receiver_sdk::price_update::PriceUpdateV2;

use crate::state::Collateral;

pub fn process(
    ctx: Context<WithdrawCollateral>,
    collateral_to_withdraw: u64,
    tokens_to_burn: u64,
) -> Result<()> {
    let oracle = &ctx.accounts.oracle;
    let config = &ctx.accounts.config_account;
    let collateral_account = &mut ctx.accounts.collateral_account;

    let depositor = &ctx.accounts.depositor;
    let token_account = &ctx.accounts.token_account;

    let reserve_account = &ctx.accounts.reserve_account;
    let reserve_account_bump = collateral_account.reserve_account_bump;

    let mint_account = &ctx.accounts.mint;

    let token_program = &ctx.accounts.token_program;
    let system_program = &ctx.accounts.system_program;

    if collateral_account.reserve_amount < collateral_to_withdraw {
        return err!(StablecoinError::InvalidCollateralRequest);
    }

    collateral_account.reserve_amount = collateral_account
        .reserve_amount
        .checked_sub(collateral_to_withdraw)
        .unwrap();

    collateral_account.tokens_minted = collateral_account
        .tokens_minted
        .checked_sub(tokens_to_burn)
        .unwrap();

    withdraw_collateral(
        collateral_to_withdraw,
        depositor,
        reserve_account,
        reserve_account_bump,
        system_program,
    )?;

    burn_tokens(
        mint_account,
        depositor,
        tokens_to_burn,
        token_account,
        token_program,
    )?;

    assert_account_is_healthy(oracle, collateral_account, config)?;

    Ok(())
}

#[derive(Accounts)]
pub struct WithdrawCollateral<'info> {
    #[account(mut)]
    pub depositor: Signer<'info>,

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
        seeds = [CONFIG_SEED],
        bump = config_account.bump,
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
        associated_token::mint = mint,
        associated_token::authority = depositor,
        associated_token::token_program = token_program
    )]
    pub token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [RESERVE_ACCOUNT_SEED, depositor.key().as_ref()],
        bump = collateral_account.reserve_account_bump
    )]
    pub reserve_account: SystemAccount<'info>,

    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,

    pub oracle: Account<'info, PriceUpdateV2>,
}
