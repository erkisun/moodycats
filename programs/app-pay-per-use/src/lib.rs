use anchor_lang::prelude::*;

declare_id!("GS3fEj1FYDCMohPb8EGkYxith7eRM3aRpbXK4ZCSToe6");

#[program]
pub mod app_pay_per_use {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
