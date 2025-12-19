use std::cmp::min;

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
    constants::{BPS_MAX_VALUE, RESERVE_ACCOUNT_SEED},
    errors::StablecoinError,
    state::{Collateral, Config},
    utils::{burn_tokens, get_reserve_value_in_usd},
};

pub struct LiquidationData<'a, 'info> {
    pub collateral: &'a mut Account<'info, Collateral>,
    pub reserve_account: &'a SystemAccount<'info>,
    pub liquidator: &'a Signer<'info>,
    pub liquidator_token_account: &'a InterfaceAccount<'info, TokenAccount>,
    pub depositor_token_account: &'a InterfaceAccount<'info, TokenAccount>,
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
    let collateral = liquidation_data.collateral;
    let tokens_to_burn_with_bonus = calculate_liquidation_with_bonus(
        tokens_to_burn,
        liquidation_data.config.liquidation_bonus as u64,
    )?;

    let real_total_collateral = liquidation_data.reserve_account.lamports();
    collateral.reserve_amount = real_total_collateral;

    let total_collateral = collateral.reserve_amount;
    let collateral_total_worth =
        get_reserve_value_in_usd(liquidation_data.oracle, total_collateral)?;

    let collateral_to_release = total_collateral
        .checked_mul(tokens_to_burn_with_bonus)
        .ok_or(StablecoinError::MathOverflow)?
        .checked_div(collateral_total_worth)
        .ok_or(StablecoinError::MathOverflow)?;

    let collateral_to_release = min(collateral_to_release, total_collateral);

    burn_tokens(
        liquidation_data.mint,
        liquidation_data.liquidator,
        tokens_to_burn,
        liquidation_data.liquidator_token_account,
        liquidation_data.token_program,
    )?;

    let depositor_key = collateral.depositor.clone();
    let signer_seeds: &[&[&[u8]]] = &[&[RESERVE_ACCOUNT_SEED, depositor_key.as_ref()]];

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

    collateral.reserve_amount = collateral
        .reserve_amount
        .checked_sub(collateral_to_release)
        .unwrap_or(0);

    collateral.tokens_minted = collateral
        .tokens_minted
        .checked_sub(tokens_to_burn)
        .ok_or(StablecoinError::MathOverflow)?;

    if collateral.reserve_amount == 0 && collateral.tokens_minted > 0 {
        collateral.tokens_minted = 0;

        let rent_lamports = liquidation_data.reserve_account.lamports();
        transfer(
            CpiContext::new_with_signer(
                liquidation_data.system_program.to_account_info(),
                Transfer {
                    from: liquidation_data.reserve_account.to_account_info(),
                    to: liquidation_data.liquidator.to_account_info(),
                },
                signer_seeds,
            ),
            rent_lamports,
        )?;

        liquidation_data.reserve_account.assign(&System::id());
    }

    Ok(())
}

pub fn calculate_liquidation_with_bonus(
    tokens_to_burn: u64,
    liquidation_bonus_percentage: u64,
) -> Result<u64> {
    // Base Points System, where the precentage is scaled by 100
    // 100% = 10_000 BPS

    let liquidation_bonus_bps = liquidation_bonus_percentage
        .checked_mul(100)
        .ok_or(StablecoinError::MathOverflow)?;

    let multiplier = liquidation_bonus_bps
        .checked_add(BPS_MAX_VALUE)
        .ok_or(StablecoinError::MathOverflow)?;

    let bonus_amount = (tokens_to_burn as u128)
        .checked_mul(multiplier as u128)
        .ok_or(StablecoinError::MathOverflow)?
        .checked_div(BPS_MAX_VALUE as u128)
        .ok_or(StablecoinError::MathOverflow)?;

    bonus_amount
        .try_into()
        .map_err(|_| StablecoinError::MathOverflow.into())
}
