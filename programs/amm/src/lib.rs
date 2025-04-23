#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;
pub mod instructions;
pub mod states;

declare_id!("2tD2D3Ld2nt4CCjQNfDB8S5N2iJBjyj3FWD4cMCVaidF");

#[program]
pub mod amm {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}
