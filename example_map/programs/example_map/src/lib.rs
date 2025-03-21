use anchor_lang::prelude::*;
use std::mem::size_of;

declare_id!("5GrC6Xf1hmwQMXrPoar1CKi8v3p4AFjkPmUj7Dsoa9YZ");

#[program]
pub mod example_map {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, key1: u64, key2: u64) -> Result<()> {
        Ok(())
    }

    pub fn set(ctx: Context<Set>, key1: u64, key2: u64, val: u64) -> Result<()> {
        ctx.accounts.val.value = val;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(key1: u64, key2: u64)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = signer,
        space = size_of::<Val>() + 8,
        seeds = [&key1.to_le_bytes(), &key2.to_le_bytes()],
        bump
    )]
    val: Account<'info, Val>,
    #[account(mut)]
    signer: Signer<'info>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(key1: u64, key2: u64)]
pub struct Set<'info> {
    #[account(mut, seeds = [&key1.to_le_bytes(), &key2.to_le_bytes()], bump)]
    val: Account<'info, Val>,
}

#[account]
pub struct Val {
    value: u64,
}