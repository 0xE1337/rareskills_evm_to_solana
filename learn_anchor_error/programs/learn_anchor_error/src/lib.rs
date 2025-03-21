use anchor_lang::prelude::*;

declare_id!("PhvCM3yNh1bkFHnhSZiNoJHy5HuNqb2u2WA9MKasF9N");

#[program]
pub mod learn_anchor_error {
    use super::*;


    pub fn limit_range(ctxThen : Context<LimitRange>, a: u64) -> Result<()> {
        require!(a >= 10, MyError::AisTooSmall);
        require!(a <= 100, MyError::AisTooBig);
        msg!("Result = {}", a);
        Ok(())
    }

    // NEW FUNCTION
    pub fn func(ctx: Context<LimitRange>) -> Result<()> {
        msg!("Will this print?");
        return err!(MyError::AlwaysErrors);
        // Ok(())
    }
}

#[derive(Accounts)]
pub struct LimitRange {}

#[error_code]
pub enum MyError {
    #[msg("a is too big")]
    AisTooBig,
    #[msg("a is too small")]
    AisTooSmall,
    #[msg("Always errors")]  // NEW ERROR, what do you think the error code will be?
    AlwaysErrors,
}

