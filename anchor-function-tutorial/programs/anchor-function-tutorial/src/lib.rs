use anchor_lang::prelude::*;

declare_id!("2NiEZQfvDcWVNX6dp5ikS31XkstNR51pPHGNNuB5djD2");

#[program]
pub mod anchor_function_tutorial {
    use super::*;

    pub fn function_a(ctx: Context<NonEmptyAccountExample>) -> Result<()> {
        msg!("Function A called");
        Ok(())
    }

    pub fn function_b(ctx: Context<Empty>, first_arg: u64) -> Result<()> {
        msg!("Function B with arg {}", first_arg);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct NonEmptyAccountExample<'info> {
    signer: Signer<'info>,
    another_signer: Signer<'info>,
}

#[derive(Accounts)]
pub struct Empty {}