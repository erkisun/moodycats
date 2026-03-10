use anchor_lang::prelude::*;

declare_id!("3BBzYvsM27uF5VoBwKQtF1RgcnkmvFxMA1otD25niMHW");

#[program]
pub mod treasury_vesting {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
