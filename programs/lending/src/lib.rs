#![allow(unexpected_cfgs, deprecated)]
use anchor_lang::prelude::*;
use instructions::*;

mod instructions;
mod state;
declare_id!("CGcqN95YmRuWobDvG1YANh3XE8gtecVfvTnKrBNatD3y");

#[program]
pub mod lending {
    use super::*;

    pub fn init_bank(
        ctx: Context<InitBank>,
        liquadation_threshold: u64,
        max_ltv: u64,
    ) -> Result<()> {
        ctx.accounts.init_bank(liquadation_threshold, max_ltv)
    }

    pub fn init_user(ctx: Context<InitUser>, usdc_address: Pubkey) -> Result<()> {
        ctx.accounts.init_user(usdc_address)
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        ctx.accounts.deposit_asset(amount)
    }
}
