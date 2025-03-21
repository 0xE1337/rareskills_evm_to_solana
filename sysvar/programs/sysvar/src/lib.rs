use anchor_lang::{prelude::*, solana_program::sysvar::recent_blockhashes::RecentBlockhashes};

// replace program id
declare_id!("99kAR22er5EWujcyc8u7RdLah1me37Je6D8jNoudqC38");

#[program]
pub mod sysvar {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        // RECENT BLOCK HASHES
        let arr = [ctx.accounts.recent_blockhashes.clone()];
        let accounts_iter = &mut arr.iter();
        let sh_sysvar_info = next_account_info(accounts_iter)?;
        let recent_blockhashes = RecentBlockhashes::from_account_info(sh_sysvar_info)?;
        let data = recent_blockhashes.last().unwrap();

        msg!("The recent block hash is: {:?}", data.blockhash);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    /// CHECK: readonly
    pub recent_blockhashes: AccountInfo<'info>,
}