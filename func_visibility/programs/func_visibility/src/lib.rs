use anchor_lang::prelude::*;
pub mod calculate;

declare_id!("9aGCxUbBH57U9hvLEYirajuh2Kxyycs9DSN6dNYeoE5y");

#[program]
pub mod func_visibility {
    use super::*;

    pub fn add_two_numbers(_ctx: Context<Initialize>, x: u64, y: u64) -> Result<()> {
        let result = calculate::add(x, y);
        msg!("{} + {} = {}", x, y, result);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}