use anchor_lang::prelude::*;

declare_id!("GAjxiEpSVe9z66dvghFu1qD3BFBUYJvYGMVfMuF3Qe5r");

#[program]
pub mod hello_world {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Hello, world!");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
