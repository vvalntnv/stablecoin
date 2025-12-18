use anchor_lang::prelude::*;
use anchor_spl::associated_token::spl_associated_token_account::solana_program::native_token::LAMPORTS_PER_SOL;
use pyth_solana_receiver_sdk::price_update::{get_feed_id_from_hex, PriceUpdateV2};

use crate::{
    constants::{PRICE_MAX_AGE, SOLANA_USD_PRICE_FEED},
    errors::StablecoinError,
    state::{Collateral, Config},
};

/// Checks the healthfactor of an account
pub fn assert_account_is_healthy<'info>(
    oracle: &Account<'info, PriceUpdateV2>,
    collateral: &Collateral,
    config: &Config,
) -> Result<()> {
    // get the most recent USD price
    let threshold = config.liquidation_threshold as u64;
    let collateral_price = get_collateral_price_in_usd_using(oracle, collateral)?;

    let tokens_total = collateral.tokens_minted;

    // account is healthy
    if collateral_price >= 0 && tokens_total == 0 {
        return Ok(());
    }

    let liquidation_ratio = tokens_total
        .checked_mul(100) // multiply by a 100 to get the percentage in integer format
        .ok_or(StablecoinError::MathOverflow)?
        .checked_div(collateral_price)
        .ok_or(StablecoinError::MathOverflow)?;

    if liquidation_ratio > threshold {
        return err!(StablecoinError::InsufficientCollateral);
    }

    Ok(())
}

pub fn get_collateral_price_in_usd_using<'info>(
    oracle: &Account<'info, PriceUpdateV2>,
    collateral: &Collateral,
) -> Result<u64> {
    let feed_id = get_feed_id_from_hex(SOLANA_USD_PRICE_FEED)?;

    // here we get the raw price and we get the exponent
    let price_data = oracle.get_price_no_older_than(&Clock::get()?, PRICE_MAX_AGE, &feed_id)?;
    require!(price_data.price > 0, StablecoinError::InvalidPrice);

    // the collateral price is the deposited amount that we have in lamports
    // divided by 10^9 (because one lamport is just 1 billionth of a SOL) and that whole thing
    // multiplied by the price divided by 10^e, where e is the exponent of the price
    // That way, we get the exact amount of collateral value on the market currently
    // after simplification, the formula bellow appears:
    //
    //                    lamports * price_raw  ----------> Nominator
    // collat_price = -----------------------------
    //                        10^9 * 10^e       ----------> Denominator
    //                       [or 10^(9+e)]
    //
    // where e is the exponent that the oracle gave us!
    let exponent = price_data.exponent.abs() as u32;

    let nominator = (collateral.reserve_amount as u128)
        .checked_mul(price_data.price as u128)
        .ok_or(StablecoinError::MathOverflow)?;

    let denominator = (LAMPORTS_PER_SOL as u128)
        .checked_mul(10u128.pow(exponent))
        .ok_or(StablecoinError::MathOverflow)?;

    let collateral_price = nominator
        .checked_div(denominator)
        .ok_or(StablecoinError::MathOverflow)?;

    if collateral_price > u64::MAX as u128 {
        return err!(StablecoinError::MathOverflow);
    }

    Ok(collateral_price as u64)
}
