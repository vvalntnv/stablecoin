use crate::{
    constants::{COLLAT_SEED, CONFIG_SEED, MINT_SEED, RESERVE_ACCOUNT_SEED},
    state::{Collateral, Config},
    utils::{deposit_collateral, mint_tokens, price_utils::assert_account_is_healthy},
};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_2022::{approve, Approve, Token2022},
    token_interface::{Mint, TokenAccount},
};
use pyth_solana_receiver_sdk::price_update::PriceUpdateV2;

pub fn process(
    ctx: Context<DepositCollateralAndMintTokens>,
    amount_to_mint: u64,
    amount_deposited: u64,
) -> Result<()> {
    let depositor = &ctx.accounts.depositor;
    let collateral_account = &mut ctx.accounts.collateral_account;
    let token_program = &ctx.accounts.token_program;
    let system_program = &ctx.accounts.system_program;
    let mint = &ctx.accounts.mint;
    let token_account = &ctx.accounts.token_account;
    let reserve_account = &ctx.accounts.reserve_account;
    let config_account = &ctx.accounts.config_account;
    let oracle = &ctx.accounts.oracle;

    if !collateral_account.is_initialized {
        collateral_account.reserve_account = ctx.accounts.reserve_account.key();
        collateral_account.token_account = ctx.accounts.token_account.key();
        collateral_account.bump = ctx.bumps.collateral_account;
        collateral_account.reserve_account_bump = ctx.bumps.reserve_account;

        collateral_account.reserve_amount = 0;

        collateral_account.is_initialized = true;
    }

    collateral_account.reserve_amount = collateral_account
        .reserve_amount
        .checked_add(amount_deposited)
        .unwrap();

    collateral_account.tokens_minted = collateral_account
        .tokens_minted
        .checked_add(amount_to_mint)
        .unwrap();

    assert_account_is_healthy(oracle, collateral_account, config_account)?;

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
