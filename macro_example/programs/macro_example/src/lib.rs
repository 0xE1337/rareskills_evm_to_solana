use anchor_lang::prelude::*;

declare_id!("9STXv3go9YCBjNqMRr7cE9G8oBqQvsnrtFEkYaWViVRx");

#[program]
pub mod macro_example {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
