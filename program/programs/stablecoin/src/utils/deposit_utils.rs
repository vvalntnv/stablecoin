use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};
use anchor_spl::{
    token_2022::{mint_to, MintTo, Token2022},
    token_interface::{Mint, TokenAccount},
};

use crate::constants::MINT_SEED;

pub fn deposit_collateral<'info>(
    amount_to_deposit: u64,
    depositor: &Signer<'info>,
    reserve_account: &SystemAccount<'info>,
    system_program: &Program<'info, System>,
) -> Result<()> {
    transfer(
        CpiContext::new(
            system_program.to_account_info(),
            Transfer {
                from: depositor.to_account_info(),
                to: reserve_account.to_account_info(),
            },
        ),
        amount_to_deposit,
    )?;

    Ok(())
}

pub fn mint_tokens<'info>(
    mint: &InterfaceAccount<'info, Mint>,
    mint_bump: u8,
    tokens_to_mint: u64,
    benefitiary_account: &InterfaceAccount<'info, TokenAccount>,
    token_program: &Program<'info, Token2022>,
) -> Result<()> {
    let signer_seeds: &[&[&[u8]]] = &[&[MINT_SEED, &[mint_bump]]];
    mint_to(
        CpiContext::new_with_signer(
            token_program.to_account_info(),
            MintTo {
                mint: mint.to_account_info(),
                to: benefitiary_account.to_account_info(),
                authority: mint.to_account_info(),
            },
            signer_seeds,
        ),
        tokens_to_mint,
    )?;

    Ok(())
}
