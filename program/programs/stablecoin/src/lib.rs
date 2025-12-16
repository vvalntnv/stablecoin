mod constants;
mod instructions;
mod state;

use anchor_lang::prelude::*;
use instructions::*;

declare_id!("EtWRBmDRdDXPDQeVd1ToT4aHgRj26qugVmu1G2MV5TWK");

#[program]
pub mod stablecoin {
    use super::*;

    pub fn initialize_config(ctx: Context<InitializeConfigAccount>) -> Result<()> {
        instructions::admin::initialize_config::process(ctx)
    }
}

#[derive(Accounts)]
pub struct Initialize {}
