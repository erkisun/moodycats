// -----------------------------------------------
// moodycats.com / moodycats.io
// programs/investor-dex-vesting/instructions/initialize.rs
//
// -----------------------------------------------
// 
// WAS PASSIERT HIER?
// Diese Instruktion wird in lib.rs GENAU EINMAL nach dem Deploy des Contracts aufgerufen.
// Sie legt die globale Config an und erstellt die beiden Token-Vaults (PDA-Konten für DEX und Investoren).
// Die initialize Funktion macht genau das, was sie soll: Die Infrastruktur bereitstellen (Config + leere Vaults). 
// Das Befüllen kommt danach als separater, manueller Schritt.
//
// ABLAUF:
// 1. Admin (Dev) ruft die Funktion auf (muss signieren)
// 2. Config-PDA wird erstellt (enthält Admin, Mint, Vault-Adressen, Bumps)
// 3. DEX-Vault-PDA wird erstellt (später: 400 Mio Token für DEX-Vesting)
// 4. Investor-Vault-PDA wird erstellt (später: 500 Mio Token für OTC-Verkauf)
// 5. Alle relevanten Daten werden in der Config gespeichert (On-Chain Storage)
// 6. Fertig! Contract ist bereit für die nächsten Schritte
//
// WICHTIG:
// - Nur der Admin kann diese Funktion aufrufen
// - Die Vaults sind PDAs, deren Authority die Config ist (=> Programm kontrolliert sie)
// - Die Bumps werden gespeichert, um später beim Zugriff auf PDAs nicht neu berechnen zu müssen
// - Nach der Initialisierung müssen die Vaults MANUELL mit Tokens befüllt werden
//   (500 Mio in investor_vault, 400 Mio in dex_vault)
//
// SECURITY:
// - Da die Config die Authority der Vaults ist, kann NIEMAND außer dem Programm
//   (durch autorisierte Instruktionen) Tokens aus den Vaults bewegen
// - Der Admin kann nur über die programm-definierten Funktionen darauf zugreifen
//
// -----------------------------------------------
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use crate::states::config::Config;

// Instruction Validation (On-Chain - Context)
// Definiert alle Konten, die bei der Initialisierung benötigt werden:
// - Wer muss signieren ?
// - Welche Konten werden erstellt ?
// - Welche Beziehungen haben die Konten zueinander ?
#[derive(Accounts)]
pub struct Initialize<'info> {
    // Der Admin, der die Initialisierung durchführt (muss signieren)
    #[account(mut)]
    pub admin: Signer<'info>,

    // Der Token-Mint des Moodycats-Tokens (muss dem Mint der Vaults entsprechen)
    // - Referenz für die Vaults (sie müssen denselben Mint haben)
    // - Wird in der Config gespeichert
    // - Authority wurde bereits revoked (siehe Whitepaper)
    pub mint: Account<'info, Mint>,

    // Globale Config (PDA)
    // Das Herzstück des Contracts.
    // - Wird hier neu als PDA angelegt (`init`)
    // - Admin bezahlt die Gebühren (`payer = admin`)
    // - Seeds: "config" - dadurch ist die Adresse deterministisch
    // - Bump wird automatisch berechnet und gespeichert
    // - Enthält alle wichtigen Parameter des Contracts
    #[account(
        init,                                           // Neues Konto anlegen
        payer = admin,                                  // Admin bezahlt
        space = 8 + std::mem::size_of::<Config>(),      // Größe: Discriminator(8) + Config-Daten
        seeds = [b"config"],                            // PDA-Seeds: "config"
        bump,                                           // Bump automatisch berechnen
    )]
    pub config: Account<'info, Config>,

    // DEX-Vault (PDA-Token-Account)
    // Wird NACH der Initialisierung manuell mit 400 Mio Tokens befüllt
    #[account(
        init,                           // Neues Token-Konto anlegen
        payer = admin,                  // Admin bezahlt
        token::mint = mint,             // Gehört zum Token-Mint des Moodycats-Tokens
        token::authority = config,      // Config ist Authority (=> Programm kontrolliert den Zugriff)
        seeds = [b"dex_vault"],         // PDA-Seeds: "dex_vault" = eindeutige PDA-Adresse
        bump,                           // Bump automatisch berechnen
    )]
    pub dex_vault: Account<'info, TokenAccount>,

    // Investor-Vault (PDA-Token-Account)
    // Wird NACH der Initialisierung manuell mit 500 Mio Tokens befüllt
    #[account(
        init,                           // Neues Token-Konto anlegen
        payer = admin,                  // Admin bezahlt
        token::mint = mint,             // Gehört zum Token-Mint des Moodycats-Tokens
        token::authority = config,      // Config ist Authority (=> Programm kontrolliert den Zugriff)
        seeds = [b"investor_vault"],    // PDA-Seeds: "investor_vault" = eindeutige PDA-Adresse
        bump,                           // Bump automatisch berechnen
    )]
    pub investor_vault: Account<'info, TokenAccount>,

    // Gift-Vault (PDA-Token-Account)
    // Wird mit .csv aus DB manuell verteilt : Airdrops, Investoren Bonus, Starter-Tokens, Earlybirds, Dev
    #[account(
        init,
        payer = admin,
        token::mint = mint,
        token::authority = config,
        seeds = [b"gift_vault"],
        bump,
    )]
    pub gift_vault: Account<'info, TokenAccount>,

    // Das Standard-SPL-Token-Program von Solana.
    // Wird für die Initialisierung der Token-Vaults benötigt.
    pub token_program: Program<'info, Token>,
    // Das Standard-System-Program von Solana.
    // Wird zum Erstellen neuer Konten benötigt.
    pub system_program: Program<'info, System>,
    // Enthält Informationen über das Rent-Modell von Solana.
    // Wird benötigt, um die Konten rent-exempt zu machen.
    pub rent: Sysvar<'info, Rent>,
}

// Die Handler-Funktion der Initialize-Instruction (als Public Function)
// Hier wurde der Anchor Standard (pub fn) gewählt und bewusst KEINE Methode zur besseren Kapselung !
//
// Diese Funktion enthält die eigentliche Logik der Initialisierung:
// 1. Alle wichtigen Adressen in der Config speichern (Admin, Mint, Vaults)
// 2. Die berechneten Bumps speichern (für spätere Zugriffe)
// 3. Startwerte für das Vesting setzen
// 
// ### Warum Bumps speichern?
// Wenn wir später auf die PDAs zugreifen (z.B. in release_dex_tokens), 
// müssen wir die Seeds + Bump angeben. Statt den Bump jedes Mal neu zu 
// berechnen (teuer), speichern wir ihn einfach in der Config.
// 
// ### Warum last_release = jetzt?
// Wir setzen last_release auf den aktuellen Timestamp, damit die 30-Tage-Wartezeit
// ab sofort gilt. Die erste Freigabe kann also frühestens 30 Tage nach 
// Initialisierung erfolgen.
pub fn handler(ctx: Context<Initialize>) -> Result<()> {
    let config = &mut ctx.accounts.config;
    
    // 1. BASISPRAMETER SPEICHERN
    // Admin-Adresse (wer den Contract initialisiert hat)
    config.admin = ctx.accounts.admin.key();
    // Token-Mint (damit wir später wissen, welcher Token zu uns gehört)
    config.mint = ctx.accounts.mint.key();
    // Die Adressen der Vaults (für schnellen Zugriff)
    config.dex_vault = ctx.accounts.dex_vault.key();
    config.investor_vault = ctx.accounts.investor_vault.key();
    config.gift_vault = ctx.accounts.gift_vault.key();
    
    // 2. BUMPS SPEICHERN (für spätere PDA-Zugriffe)
    // ctx.bumps enthält alle automatisch berechneten Bumps
    config.bump = ctx.bumps.config;                         // Bump der Config selbst
    config.dex_vault_bump = ctx.bumps.dex_vault;            // Bump des DEX-Vaults
    config.investor_vault_bump = ctx.bumps.investor_vault;  // Bump des Investor-Vaults
    config.gift_vault_bump = ctx.bumps.gift_vault;          // Bump des Geschenke-Vaults
    
    // 3. VESTING-PARAMETER INITIALISIEREN 
    // Noch keine Tranche freigegeben (0 von 4)
    config.released_tranches = 0;
    
    // Last Release = JETZT (damit die 30-Tage-Uhr ab heute tickt)
    // Die erste Freigabe ist frühestens in 30 Tagen möglich
    config.last_release = Clock::get()?.unix_timestamp;

    // ------------------------------------------------------
    // 4. LOGGING (für Transparenz)
    // ------------------------------------------------------
    msg!("=== INVESTOR-DEX-VESTING INITIALISIERT ===");
    msg!("Admin: {}", config.admin);
    msg!("Mint: {}", config.mint);
    msg!("DEX-Vault: {}", config.dex_vault);
    msg!("Investor-Vault: {}", config.investor_vault);
    msg!("Gift-Vault: {}", config.gift_vault);
    msg!("Bump Config: {}", config.bump);
    msg!("Bump DEX: {}", config.dex_vault_bump);
    msg!("Bump Investor: {}", config.investor_vault_bump);
    msg!("Bump Gift: {}", config.gift_vault_bump);
    msg!("Initialisierung abgeschlossen. Viel Erfolg! 🚀");

    Ok(())
}
