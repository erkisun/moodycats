// -----------------------------------------------
// moodycats.com / moodycats.io
// programs/app-pay-per-use/lib.rs
// Contract 2
//
// -----------------------------------------------
// Whitepaper : https://moodycats.io/faq/Whitepaper
//
// Contract 1 : anchor new investor-dex-vesting    ─┐
// Contract 2 : anchor new app-per-pay-use         ─┼► Alle nutzen GLEICHEN Token Mint!
// Contract 3 : anchor new treasury-vesting        ─┘
//
// -----------------------------------------------
// MOODYCATS TOKENOMICS (FINAL)
use anchor_lang::prelude::*;

// Module deklarieren (werden später mit Inhalt gefüllt)
pub mod instructions;
pub mod states;
pub mod errors;

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
