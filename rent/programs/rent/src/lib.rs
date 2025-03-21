use anchor_lang::prelude::*;
use anchor_lang::solana_program::rent as rent_module;

declare_id!("DT7Bxyyx24bbiNvmSuC9EZ1Z9SxFPbjzSyC3xnGYnPVc");

#[program]
pub mod rent {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let base_cost = rent_module::ACCOUNT_STORAGE_OVERHEAD as f64
            * rent_module::DEFAULT_LAMPORTS_PER_BYTE_YEAR as f64
            * rent_module::DEFAULT_EXEMPTION_THRESHOLD;
        msg!("cost to create an empty account: {}", base_cost); // 890880
    
        let total_cost = base_cost + 32.0 * rent_module::DEFAULT_LAMPORTS_PER_BYTE_YEAR as f64
            * rent_module::DEFAULT_EXEMPTION_THRESHOLD;
        msg!("cost to create a 32 byte account: {}", total_cost); // 1113600
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
