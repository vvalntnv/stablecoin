use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};
use anchor_spl::{
    token_2022::Token2022,
    token_interface::{Mint, TokenAccount},
};

use crate::{
    state::{Collateral, Config},
    utils::burn_tokens,
};

pub fn liquidate_collateral<'info>(
    liquidator: &Signer<'info>,
    depositor: &SystemAccount<'info>,
    token_account: &InterfaceAccount<'info, TokenAccount>,
    mint: &InterfaceAccount<'info, Mint>,
    reserve_account: &SystemAccount<'info>,
    token_program: &Program<'info, Token2022>,
    system_program: &Program<'info, System>,
    config: &Account<'info, Config>,
) -> Result<()> {
    let burn_amount = token_account.amount;

    burn_tokens(mint, liquidator, burn_amount, token_account, token_program)?;

    let liquidation_bonus = burn_amount * config.liquidation_bonus as u64 / 100;
    let liquidation_lamports = burn_amount + liquidation_bonus;

    transfer(
        CpiContext::new(
            system_program.to_account_info(),
            Transfer {
                from: reserve_account.to_account_info(),
                to: liquidator.to_account_info(),
            },
        ),
        liquidation_lamports,
    )?;

    // TODO: Remove depositor from the reserve and use the reserve as a global reserve, not per
    // user? | Otherwise, just transfer the reserve amount to a wallet account, where the protocol
    // can benefit from liquidation
    let leftover = config.protocol_liquidation_fee as u64 * reserve_account.lamports() / 100;

    transfer(
        CpiContext::new(
            system_program.to_account_info(),
            Transfer {
                from: reserve_account.to_account_info(),
                to: depositor.to_account_info(),
            },
        ),
        leftover,
    )?;

    // this should happen at the end of the operation
    Ok(())
}
