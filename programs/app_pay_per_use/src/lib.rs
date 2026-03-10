use anchor_lang::prelude::*;

declare_id!("54cC4hc6Wo9VHxQX3HMcjUh9vBu58EVSU3ao3gTZNVzc");

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
