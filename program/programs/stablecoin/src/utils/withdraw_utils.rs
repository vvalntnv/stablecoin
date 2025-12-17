use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};
use anchor_spl::token_2022::{burn, Burn, Token2022};
use anchor_spl::token_interface::{Mint, TokenAccount};

use crate::constants::RESERVE_ACCOUNT_SEED;

pub fn withdraw_collateral<'info>(
    amount_to_withdraw: u64,
    depositor: &Signer<'info>,
    reserve_account: &SystemAccount<'info>,
    reserve_account_bump: u8,
    system_program: &Program<'info, System>,
) -> Result<()> {
    let signer_seeds: &[&[&[u8]]] = &[&[RESERVE_ACCOUNT_SEED, &[reserve_account_bump]]];

    transfer(
        CpiContext::new_with_signer(
            system_program.to_account_info(),
            Transfer {
                from: reserve_account.to_account_info(),
                to: depositor.to_account_info(),
            },
            signer_seeds,
        ),
        amount_to_withdraw,
    )?;

    Ok(())
}

pub fn burn_tokens<'info>(
    mint: &InterfaceAccount<'info, Mint>,
    depositor: &Signer<'info>,
    tokens_to_burn: u64,
    token_account: &InterfaceAccount<'info, TokenAccount>,
    token_program: &Program<'info, Token2022>,
) -> Result<()> {
    burn(
        CpiContext::new(
            token_program.to_account_info(),
            Burn {
                mint: mint.to_account_info(),
                from: token_account.to_account_info(),
                authority: depositor.to_account_info(),
            },
        ),
        tokens_to_burn,
    )?;

    Ok(())
}
