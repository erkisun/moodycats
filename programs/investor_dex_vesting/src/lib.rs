// -----------------------------------------------
// moodycats.com / moodycats.io
// Contract 1
//
// programs/investor-dex-vesting/lib.rs
//
// Contract 1 : anchor new investor-dex-vesting    ─┐
// Contract 2 : anchor new app-per-pay-use         ─┼► Alle nutzen GLEICHEN Token Mint!
// Contract 3 : anchor new treasury-vesting        ─┘
//
// -----------------------------------------------
// CONTRACT 1: FUNKTIONSÜBERSICHT
// -----------------------------------------------
//
// 1. initialize()  – Einmalige Initialisierung (Config, 3 Vaults)
// 2. release_dex() – DEX-Tranchen freigeben (4×100 Mio, 30d Abstand)
// 3. register_investor(amount) – Investoren-Kauf (Grundbetrag aus Investor-Vault, 20% Bonus aus Gift-Vault)
// 4. claim_starter(user) – 7 Starter-Tokens für Neuregistrierungen (aus Gift-Vault, mit Replay-Schutz)
// 5. earlybird_bonus(user, amount) – Extra-Boni für erste 100k User (aus Gift-Vault)
// 6. dev_allocation(phase) – Dev-Anteile gestaffelt (Phase 1,2,3; aus Gift-Vault)
// 7. revoke_admin() – Admin-Rechte entziehen (nach Abschluss von Contract 1)
//
// -----------------------------------------------
// WICHTIG: Was Contract 1 NICHT kann
// -----------------------------------------------
// ❌ Kein SOL empfangen (SOL geht off-chain an Admin-Wallet)
// ❌ Kein CPI an Raydium (keine automatische LP-Erhöhung)
// ❌ Kein Oracle
// ❌ Kein Vesting für Investoren
// ❌ Keine Mint-/Freeze-Funktionen
// ❌ Keine Upgradeability
//
// 🔥 Aus Sicherheitsgründen ist aktuell hier eine Automatisierung nicht sinnvoll.
// -----------------------------------------------

use anchor_lang::prelude::*;

// Module deklarieren (werden später mit Inhalt gefüllt)
pub mod instructions;
pub mod states;
pub mod errors;
pub mod events;

use instructions::initialize::*;
use instructions::release_dex::*;
use instructions::register_investor::*;
use instructions::claim_starter::*;
use instructions::earlybird_bonus::*;
use instructions::dev_allocation::*;

// Programm-ID aus declare_id! übernehmen
declare_id!("A35GmMxidLvM6LaL8n17PCFU9zoQeEp5Zm5TtmRRwddy");

#[program]
pub mod investor_dex_vesting {
    use super::*;

    // 1. INITIALISIERUNG (einmalig nach Deploy)
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        instructions::initialize::handler(ctx)
    }

    // 2. DEX-VESTING (für LP-Erhöhungen)
    //    - 400 Mio Tokens insgesamt
    //    - 4 Tranchen à 100 Mio
    //    - Nur für Admin (um LP zu erhöhen)
    //    - 30 Tage Mindestabstand
    pub fn release_dex(ctx: Context<ReleaseDex>) -> Result<()> {
        instructions::release_dex::handler(ctx)
    }

    // 3. INVESTOREN-VERKAUF (für Webseiten-Käufer mit 20% Bonus aus Gift-Vault)
    //    - 500 Mio Tokens insgesamt
    //    - Kleinste Tranche (ab 100.-$ = 100'000 Token)
    //    - Für Investoren (OTC-Verkauf)
    //    - Admin ruft auf mit: amount (gekaufte Menge)
    //    - Holt amount aus investor_vault
    //    - Holt bonus (20%) aus gift_vault
    pub fn register_investor(ctx: Context<RegisterInvestor>, amount: u64) -> Result<()> {
        instructions::register_investor::handler(ctx, amount)
    }

    // 4. STARTER-TOKENS (7 für jeden neuen User)
    //    - Admin ruft auf mit: user (Pubkey)
    //    - Holt 7 Tokens aus gift_vault
    //    - Pro User einmalig (PDA = Replay-Schutz)
    pub fn claim_starter(ctx: Context<ClaimStarter>) -> Result<()> {
        instructions::claim_starter::handler(ctx)
    }

    // 5. EARLYBIRD-BONUS (für erste User, z.B. 100 Tokens für die ersten 100k)
    //    - Admin ruft auf mit: user, amount
    //    - Holt amount aus gift_vault
    pub fn earlybird_bonus(ctx: Context<EarlyBirdBonus>, amount: u64) -> Result<()> {
        instructions::earlybird_bonus::handler(ctx, amount)
    }

    // 6. DEV-ALLOCATION (gestaffelte Auszahlung der Dev-Anteile)
    //    - Phase 1: 20 Mio (nach Abschluss Contract 1)
    //    - Phase 2: 15 Mio (nach Livegang Contract 2)
    //    - Phase 3: 15 Mio (beim Start Contract 3)
    //    - Admin ruft auf mit: phase (1,2,3)
    //    - Holt den Phasen-Betrag aus gift_vault
    pub fn dev_allocation(ctx: Context<DevAllocation>, phase: u8) -> Result<()> {
        instructions::dev_allocation::handler(ctx, phase)
    }

    // .. später .. //
    
    // 7. ADMIN REVOKE (nach Abschluss aller Aufgaben)
    //    - Admin setzt sich selbst auf Null-Adresse
    //    - Danach ist Contract 1 herrenlos
    //pub fn revoke_admin(ctx: Context<RevokeAdmin>) -> Result<()> {
    //    instructions::revoke_admin::handler(ctx)
    //}

}