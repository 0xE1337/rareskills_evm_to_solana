use anchor_lang::prelude::*;

declare_id!("AS83Z3aG3iZDhG2Ar6BTM7obBZcMx9GkRNtqpC5BXQAV");

#[program]
pub mod func_test {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let u = get_a_num();
        msg!("{}", u);
        Ok(())
    }
}

fn get_a_num() -> u64 {
    2
}

#[derive(Accounts)]
pub struct Initialize {}