// -----------------------------------------------
// moodycats.com / moodycats.io
// programs/app-pay-per-use/states/config.rs
// Contract 2
//
// -----------------------------------------------
use anchor_lang::prelude::*;

// Idee .. Zahlenleiter (veränderlich) .. Metamorphose
// Statt π als unendliche Dezimalzahl zu speichern:
// π_dezimal: String = "3.141592653589793..." // ❌ Unmöglich!
// In Basis π:
// π_in_base_π: (u8, u8) = (1, 0) // "10" in Basis π ✅ 2 Bytes!

// On-Chain Storage (Accounts)
#[account]
pub struct Config {
    pub admin: Pubkey,
    pub mint: Pubkey,
    pub api_call_counter: u64,
    pub burn_rate: u64,                         // 1 Token = 1 Call
    
    // 🔮 ZUKUNFTS-RESERVIERUNG (ohne jetzige Logik!)
    pub future_math_reserved: [u8; 512],        // 512 Bytes für spätere Zahlenleitern
    pub metamorphic_program: Option<Pubkey>,    // Adresse von Contract 4 (später)
    
    // Optional: Für Metamorphose-Historie
    pub metamorphosis_count: u64,
    pub last_form: [u8; 32],                    // Hash der letzten Form
}