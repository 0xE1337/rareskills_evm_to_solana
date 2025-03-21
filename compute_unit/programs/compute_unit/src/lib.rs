use anchor_lang::prelude::*;

declare_id!("DQMfGr3N4pjmfLWJtbrbo4DLyh7BsUBAL2ky63zx3Lcs");

#[program]
pub mod compute_unit {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let mut a = Vec::new();
        a.push(1);
        a.push(2);
        a.push(3);
        a.push(4);
        a.push(5);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
