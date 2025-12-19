use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};
use anchor_spl::{
    token_2022::Token2022,
    token_interface::{Mint, TokenAccount},
};
use pyth_solana_receiver_sdk::price_update::PriceUpdateV2;

use crate::{
    constants::RESERVE_ACCOUNT_SEED,
    errors::StablecoinError,
    state::{Collateral, Config},
    utils::{burn_tokens, get_reserve_value_in_usd},
};

pub fn liquidate_collateral<'info>(
    collateral: &Account<'info, Collateral>,
    reserve_account: &SystemAccount<'info>,
    liquidator: &Signer<'info>,
    liquidator_token_account: &InterfaceAccount<'info, TokenAccount>,
    oracle: &Account<'info, PriceUpdateV2>,
    config: &Account<'info, Config>,
    token_program: &Program<'info, Token2022>,
    system_program: &Program<'info, System>,
    mint: &InterfaceAccount<'info, Mint>,
    tokens_to_burn: u64,
) -> Result<()> {
    // what needs to happen here
    // A liquidator comes and wants to liquidate some of the tokens
    // he gives some amount of our tokens to us and in return we give him the collateral
    // that corresponds to that amount of tokens + a bonus for liquidating

    let total_collateral = collateral.reserve_amount;
    let collateral_total_worth = get_reserve_value_in_usd(oracle, total_collateral)?;

    // could be zero if amount is too small
    let liquidation_bonus = tokens_to_burn
        .checked_mul(config.liquidation_bonus as u64)
        .ok_or(StablecoinError::MathOverflow)?
        .checked_div(100)
        .ok_or(StablecoinError::MathOverflow)?;

    let burn_worth = tokens_to_burn + liquidation_bonus;

    let collateral_to_release = total_collateral
        .checked_mul(burn_worth)
        .ok_or(StablecoinError::MathOverflow)?
        .checked_div(collateral_total_worth)
        .ok_or(StablecoinError::MathOverflow)?;

    burn_tokens(
        mint,
        liquidator,
        tokens_to_burn,
        liquidator_token_account,
        token_program,
    )?;

    let signer_seeds: &[&[&[u8]]] = &[&[RESERVE_ACCOUNT_SEED, collateral.depositor.as_ref()]];

    transfer(
        CpiContext::new_with_signer(
            system_program.to_account_info(),
            Transfer {
                from: reserve_account.to_account_info(),
                to: liquidator.to_account_info(),
            },
            signer_seeds,
        ),
        collateral_to_release,
    )?;

    Ok(())
}
