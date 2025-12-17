use crate::{
    constants::{COLLAT_SEED, CONFIG_SEED, RESERVE_ACCOUNT_SEED},
    state::{Collateral, Config},
    utils::{deposit_collateral, mint_tokens},
};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_2022::Token2022,
    token_interface::{Mint, TokenAccount},
};
use pyth_solana_receiver_sdk::price_update::PriceUpdateV2;

pub fn process(
    ctx: Context<DepositCollateralAndMintTokens>,
    amount_to_mint: u64,
    amount_deposited: u64,
) -> Result<()> {
    let depositor = &ctx.accounts.depositor;
    let collateral = &mut ctx.accounts.collateral_account;
    let token_program = &ctx.accounts.token_program;
    let system_program = &ctx.accounts.system_program;
    let mint = &ctx.accounts.mint_account;
    let token_account = &ctx.accounts.token_account;
    let reserve_account = &ctx.accounts.reserve_account;
    let config_account = &ctx.accounts.config_account;

    if !collateral.is_initialized {
        collateral.reserve_account = ctx.accounts.reserve_account.key();
        collateral.token_account = ctx.accounts.token_account.key();
        collateral.bump = ctx.bumps.collateral_account;
        collateral.reserve_account_bump = ctx.bumps.reserve_account;

        collateral.reserve_amount = 0;

        collateral.is_initialized = true;
    }

    collateral.reserve_amount += amount_deposited;

    // here we will take the price from the oracle
    // we will get what is amount - (amount * (liq_thresh / 100)), where amount is
    // amount_deposited * price_by_the_oracle
    // and the result of that is the value (in dollars perhaps) that we need to mint. If our token
    // is 1:1 with the USD, we will need to mint excatly the result

    deposit_collateral(amount_deposited, depositor, reserve_account, system_program)?;

    mint_tokens(
        mint,
        config_account.bump_mint_account,
        amount_to_mint,
        token_account,
        token_program,
    )?;

    Ok(())
}

#[derive(Accounts)]
pub struct DepositCollateralAndMintTokens<'info> {
    #[account(mut)]
    pub depositor: Signer<'info>,

    #[account(
        seeds = [CONFIG_SEED],
        bump,
    )]
    pub config_account: Account<'info, Config>,

    #[account(mut)]
    pub mint_account: InterfaceAccount<'info, Mint>,

    #[account(
        init_if_needed,
        space = Collateral::INIT_SPACE,
        payer = depositor,
        seeds = [COLLAT_SEED, depositor.key().as_ref()],
        bump
    )]
    pub collateral_account: Account<'info, Collateral>,

    #[account(
        init_if_needed,
        payer = depositor,
        associated_token::mint = mint_account,
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
