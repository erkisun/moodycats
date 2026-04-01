// -----------------------------------------------
// moodycats.com / moodycats.io
// Contract 1
//
// programs/investor_dex_vesting/events.rs
//
// Contract 1 : anchor new investor_dex_vesting    ─┐
// Contract 2 : anchor new app_per_pay_use         ─┼► Alle nutzen GLEICHEN Token Mint!
// Contract 3 : anchor new treasury_vesting        ─┘
//
// -----------------------------------------------
// CONTRACT 1: EVENTS
// -----------------------------------------------
//
// 1. DevAllocation (1% Team-Anteil)
// 2. AdminRevoke
// -----------------------------------------------
use anchor_lang::prelude::*;

// Event: Dev-Auszahlung wurde durchgeführt
// Emitted von: dev_allocation.rs
#[event]
pub struct DevAllocationExecuted {
    pub phase: u8,              // Phase (1, 2 oder 3)
    pub amount: u64,            // Anzahl Tokens
    pub admin: Pubkey,          // Admin der ausgezahlt hat
    pub remaining_vault: u64,   // Verbleibend im Gift-Vault
    pub phase1_paid: bool,      // Flag nach Auszahlung
    pub phase2_paid: bool,      // Flag nach Auszahlung
    pub phase3_paid: bool,      // Flag nach Auszahlung
    pub timestamp: i64,         // Zeitstempel
}

/// Event: Admin wurde entzogen
/// Emitted von: admin_revoke.rs
#[event]
pub struct AdminRevoked {
    pub old_admin: Pubkey,      // Alter Admin
    pub revoked_by: Pubkey,     // Wer hat entzogen
    pub timestamp: i64,         // Zeitstempel
}