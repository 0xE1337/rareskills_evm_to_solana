use anchor_lang::prelude::*;

declare_id!("9sMoqjMVfGwtydLC2BJxErgsd4bBjxDeR2YuwZL1hHLv");

#[program]
pub mod arithmetic {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, a: u64, b: u64, message: String) -> Result<()> {
        msg!("You said {:?}", message);
        msg!("You sent {} and {}", a, b);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}