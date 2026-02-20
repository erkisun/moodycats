// -----------------------------------------------
// moodycats.com / moodycats.io
// programs/investor-dex-vesting/instructions/release_dex.rs
//
// DEX-VESTING: Freigabe einer Tranche für den Admin
// -----------------------------------------------
// 
// WAS PASSIERT HIER?
// Der Admin kann bis zu 4 Mal jeweils 100 Mio Token aus dem DEX-Vault
// an sein eigenes Wallet freigeben, um später damit das Raydium-LP zu erhöhen.
// 
// ABLAUF:
// 1. Admin ruft Funktion auf (muss signieren)
// 2. Prüfungen: 
//    - Admin ist berechtigt
//    - Maximal 4 Tranchen (nicht überschritten)
//    - Mindestens 30 Tage seit letzter Freigabe
//    - Genug Tokens im Vault
// 3. Transfer: 100 Mio Token von DEX-Vault → Admin-Token-Konto
// 4. Config updaten: released_tranches +1, last_release = jetzt
// 5. Fertig! Admin hat jetzt Tokens für manuelles LP-Adding
//
// SECURITY:
// - Nur Admin darf aufrufen
// - Config ist Authority der Vaults (via PDA-Signer)
// - Time-Lock verhindert zu schnelle Freigaben
// - Maximale Anzahl begrenzt
//
// -----------------------------------------------
// WER: Admin
// WANN: max 4x, 30d Abstand
// 
// PRÜFUNGEN (6):
// 1. admin == config.admin
// 2. released_tranches < 4
// 3. last_release + 30d < now
// 4. dex_vault.amount >= 100 Mio
// 5. admin_token_account.owner == admin
// 6. admin_token_account.mint == config.mint
//
// AKTIONEN (3):
// 1. Transfer 100 Mio DEX-Vault → Admin-Token
// 2. released_tranches++
// 3. last_release = now
//
// SECURITY:
// - Ohne Prüfung 5: Jeder könnte sein Konto angeben!
// - Ohne Prüfung 6: Falscher Token könnte empfangen werden
//
// -----------------------------------------------
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};
use crate::states::config::Config;
//use crate::errors::*;

// Monatliche Verteilung von 100 Mio Token mit 9 Decimals: 100_000_000 * 10^9
pub const TRANCHE_AMOUNT: u64 = 100_000_000 * 1_000_000_000; // 100 Mio (9 Decimals)
pub const MIN_DAYS_BETWEEN_RELEASES: i64 = 30 * 86400; // 30 Tage in Sekunden

#[derive(Accounts)]
pub struct ReleaseDex<'info> {
    /// Der Admin, der die Freigabe durchführt (muss signieren)
    #[account(mut)]
    pub admin: Signer<'info>,

    // Globale Config (muss existieren und den Admin prüfen)
    #[account(
        mut,                                            // Hier wird das NUR das Konto verwendet, welches in initialize.rs erstellt wurde
        seeds = [b"config"],                            // PDA-Seeds: "config"
        bump = config.bump,                             // Bump automatisch berechnen
        constraint = config.admin == admin.key(),       // Ist der Admin in der Config genau der gleiche wie der, der gerade signiert ?
    )]
    pub config: Account<'info, Config>,
    
    // DEX-Vault (PDA), aus dem die Tokens kommen
    #[account(
        mut,                                                // Hier wird das NUR das Konto verwendet, welches in initialize.rs erstellt wurde
        seeds = [b"dex_vault"],                             // PDA-Seeds: "dex_vault" = eindeutige PDA-Adresse
        bump = config.dex_vault_bump,                       // Bump automatisch berechnen
        constraint = dex_vault.key() == config.dex_vault,   // Ist der Vault in der Config genau der gleiche wie der, der gerade signiert ?
    )]
    pub dex_vault: Account<'info, TokenAccount>,

    // Admin Token-Account (hält die Moodycats-Tokens)
    #[account(
        mut,
        constraint = admin_token_account.owner == admin.key(),
        constraint = admin_token_account.mint == config.mint,
    )]
    pub admin_token_account: Account<'info, TokenAccount>,
        
    // Das Standard-SPL-Token-Program von Solana.
    // Wird für die Initialisierung der Token-Vaults benötigt.
    pub token_program: Program<'info, Token>,
}

// Der Handler
pub fn handler(_ctx: Context<ReleaseDex>) -> Result<()> {
    // 1. Prüfungen (obwohl constraints schon viel prüfen)
    //    - released_tranches < 4
    //    - Zeit seit last_release > 30 Tage
    //    - Genug Tokens im Vault
    
    // 2. Transfer mit PDA-Signer
    
    // 3. Config updaten (released_tranches +1, last_release)
    
    // 4. Logging
    Ok(())
}