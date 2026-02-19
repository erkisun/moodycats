// -----------------------------------------------
// moodycats.com / moodycats.io
// programs/investor-dex-vesting/states/config.rs
//
// -----------------------------------------------
use anchor_lang::prelude::*;

// Warum Bumps speichern ?
// Wenn ich später in anderen Instruktionen auf die Vault-PDAs zugreife (z.B. in release_dex_tokens),
// brauche ich den Bump, um den seeds-Constraint zu erfüllen. 
// Ich könnte ihn auch jedes Mal neu berechnen, aber das speichern ist effizienter und sicherer.

// On-Chain Storage (Accounts)
#[account]
pub struct Config {
    pub admin: Pubkey,                // Der Admin (Dev)
    pub mint: Pubkey,                 // Token-Mint
    pub dex_vault: Pubkey,            // PDA-Token-Account (400 Mio Token)
    pub investor_vault: Pubkey,       // PDA-Token-Account (500 Mio Token)
    pub released_tranches: u8,        // 0..4
    pub last_release: i64,            // Unix-Timestamp der letzten Freigabe
    pub bump: u8,                     // Bump der Config selbst
    pub dex_vault_bump: u8,           // Bump des DEX-Vaults
    pub investor_vault_bump: u8,      // Bump des Investor-Vaults
}