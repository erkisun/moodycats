// -----------------------------------------------
// moodycats.com / moodycats.io
// programs/investor-dex-vesting/errors.rs
// Contract 1
//
// -----------------------------------------------
use anchor_lang::prelude::*;

// -----------------------------------------------
// INVESTOR DEX VESTING - EVENTS
// moodycats.com / moodycats.io
//
// Nur die Events, die wir aktuell brauchen:
// - DevAllocation (1% Team-Anteil)
// - AdminRevoke (für später)
// -----------------------------------------------

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

/// Event: Admin wurde entzogen (für später)
/// Emitted von: admin_revoke.rs
#[event]
pub struct AdminRevoked {
    /// Alter Admin
    pub old_admin: Pubkey,
    
    /// Wer hat entzogen
    pub revoked_by: Pubkey,
    
    /// Zeitstempel
    pub timestamp: i64,
}