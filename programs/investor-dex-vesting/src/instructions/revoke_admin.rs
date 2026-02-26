// -----------------------------------------------
// moodycats.com / moodycats.io
// programs/investor-dex-vesting/instructions/revoke_admin.rs
//
// REVOKE ADMIN: Admin setzt sich selbst auf Null-Adresse
// -----------------------------------------------
// 
// WAS PASSIERT HIER?
// Sobald Contract 1 seinen Zweck erfüllt hat (DEX-Tranchen alle draussen,
// Investoren-Verkauf abgeschlossen, Gift-Vault verteilt), kann der Admin
// seine eigenen Rechte entziehen. Danach kann NIEMAND mehr Funktionen
// aufrufen – Contract 1 ist "herrenlos" und bereit für Phase 2.
//
// ABLAUF:
// 1. Admin ruft revoke_admin() auf
// 2. Prüfung: Aufrufer ist aktueller Admin
// 3. Config.admin wird auf Null-Adresse (Pubkey::default()) gesetzt
// 4. Danach sind alle Admin-Funktionen blockiert
//
// WICHTIG:
// - Diese Aktion ist ENDGÜLTIG und kann nicht rückgängig gemacht werden!
// - Nur der aktuelle Admin kann sie ausführen
// - Nach dem Aufruf kann niemand mehr release_dex, register_investor, etc. aufrufen
//
// -----------------------------------------------
use anchor_lang::prelude::*;
use crate::states::config::Config;

#[derive(Accounts)]
pub struct RevokeAdmin<'info> {
    /// Der Admin, der seine Rechte entziehen will
    #[account(mut)]
    pub admin: Signer<'info>,

    /// Config (Admin wird auf Null gesetzt)
    #[account(
        mut,
        seeds = [b"config"],
        bump = config.bump,
        constraint = config.admin == admin.key() @ crate::errors::BaseErrors::Unauthorized,
    )]
    pub config: Account<'info, Config>,
}

pub fn handler(ctx: Context<RevokeAdmin>) -> Result<()> {
    let config = &mut ctx.accounts.config;
    let old_admin = config.admin;
    
    // Admin auf Null-Adresse setzen
    config.admin = Pubkey::default();
    
    msg!("=== ADMIN-REVOKE AUSGEFÜHRT ===");
    msg!("Alter Admin: {}", old_admin);
    msg!("Neuer Admin: {} (NULL)", config.admin);
    msg!("Contract 1 ist jetzt herrenlos! 🏁");
    
    Ok(())
}