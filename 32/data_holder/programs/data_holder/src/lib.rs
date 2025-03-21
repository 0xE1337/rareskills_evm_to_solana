use anchor_lang::prelude::*;
use std::mem::size_of;

declare_id!("B2ygCyDthArX1FqVqfzo2BuBBvAd1eSkMovagAQdrH8K");

#[program]
pub mod data_holder {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let storage = &mut ctx.accounts.storage;
        storage.x = 9;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = signer,
        space = size_of::<Storage>() + 8,
        seeds = [],
        bump
    )]
    pub storage: Account<'info, Storage>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Storage {
    pub x: u64,
}
