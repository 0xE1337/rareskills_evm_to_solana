use anchor_lang::prelude::*;

declare_id!("J6pdvTB4EBfXGYHVsJdRmLXi9K8B4bwUDd5T8y9oAUBH");

const OWNER: &str = "5NhLjdFKocoRMqic9sqAe5TxLagJCoCBunzg51ioMYot";

#[program]
pub mod sender {
    use super::*;

    #[access_control(check(&ctx))]
    pub fn initialize(ctx: Context<OnlyOwner>) -> Result<()> {

        msg!("Holla, I'm the owner.");
        Ok(())
    }
}

fn check(ctx: &Context<OnlyOwner>) -> Result<()> {
    require_keys_eq!(
        ctx.accounts.signer_account.key(),
        OWNER.parse::<Pubkey>().unwrap(),
        OnlyOwnerError::NotOwner
    );

    Ok(())
}

#[derive(Accounts)]
pub struct OnlyOwner<'info> {
    signer_account: Signer<'info>,
}

#[error_code]
pub enum OnlyOwnerError {
    #[msg("Only owner can call this function!")]
    NotOwner,
}