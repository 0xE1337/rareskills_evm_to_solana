use anchor_lang::prelude::*;
use anchor_lang::solana_program::sysvar::{instructions, fees::Fees, recent_blockhashes::RecentBlockhashes};

declare_id!("7R6mHJKegD4xPDcxKKwF5VkN7Xc5vUzaznGBmzk3ikxw");

#[program]
pub mod sysvars {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, number: u32) -> Result<()> {
        let arr = [ctx.accounts.instruction_sysvar.clone()];
    
        let account_info_iter = &mut arr.iter();
    
        let instructions_sysvar_account = next_account_info(account_info_iter)?;
    
        let instruction_details =
            instructions::load_instruction_at_checked(0, instructions_sysvar_account)?;
    
        msg!(
            "Instruction details of this transaction: {:?}",
            instruction_details
        );
        msg!("Number is: {}", number);
    
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    /// CHECK: 
    pub stake_history: AccountInfo<'info>, 
    /// CHECK: 
    pub recent_blockhashes: AccountInfo<'info>,
    /// CHECK: 
    pub instruction_sysvar: AccountInfo<'info>,
}