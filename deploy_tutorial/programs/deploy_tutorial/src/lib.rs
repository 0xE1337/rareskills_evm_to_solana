use anchor_lang::prelude::*;

declare_id!("2VQxLN7SMtkcuZvhBnLYRVbgK7Z38AmNTm5xwbSfu7Hy");

#[program]
pub mod deploy_tutorial {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}