use anchor_lang::prelude::*;

declare_id!("14NoHFVhTmTFrrhzgALaK6yQR6zP6hf5mnAoUKyvCfH6");

#[program]
pub mod solana_twitter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
