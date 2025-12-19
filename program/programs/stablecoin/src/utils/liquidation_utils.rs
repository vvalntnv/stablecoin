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

pub struct LiquidationData<'a, 'info> {
    pub collateral: &'a Account<'info, Collateral>,
    pub reserve_account: &'a SystemAccount<'info>,
    pub liquidator: &'a Signer<'info>,
    pub liquidator_token_account: &'a InterfaceAccount<'info, TokenAccount>,
    pub oracle: &'a Account<'info, PriceUpdateV2>,
    pub config: &'a Account<'info, Config>,
    pub token_program: &'a Program<'info, Token2022>,
    pub system_program: &'a Program<'info, System>,
    pub mint: &'a InterfaceAccount<'info, Mint>,
}

pub fn liquidate_collateral<'a, 'info>(
    liquidation_data: LiquidationData<'a, 'info>,
    tokens_to_burn: u64,
) -> Result<()> {
    // what needs to happen here
    // A liquidator comes and wants to liquidate some of the tokens
    // he gives some amount of our tokens to us and in return we give him the collateral
    // that corresponds to that amount of tokens + a bonus for liquidating

    let total_collateral = liquidation_data.collateral.reserve_amount;
    let collateral_total_worth =
        get_reserve_value_in_usd(liquidation_data.oracle, total_collateral)?;

    // could be zero if amount is too small
    let liquidation_bonus = tokens_to_burn
        .checked_mul(liquidation_data.config.liquidation_bonus as u64)
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
        liquidation_data.mint,
        liquidation_data.liquidator,
        tokens_to_burn,
        liquidation_data.liquidator_token_account,
        liquidation_data.token_program,
    )?;

    let signer_seeds: &[&[&[u8]]] = &[&[
        RESERVE_ACCOUNT_SEED,
        liquidation_data.collateral.depositor.as_ref(),
    ]];

    transfer(
        CpiContext::new_with_signer(
            liquidation_data.system_program.to_account_info(),
            Transfer {
                from: liquidation_data.reserve_account.to_account_info(),
                to: liquidation_data.liquidator.to_account_info(),
            },
            signer_seeds,
        ),
        collateral_to_release,
    )?;

    Ok(())
}
