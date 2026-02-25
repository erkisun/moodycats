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
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use crate::states::config::Config;
use crate::errors::DexErrors;

// Monatliche Verteilung von 100 Mio Token mit 9 Decimals: 100_000_000 * 10^9
// 1 Token = 1_000_000_000 (9 Decimals)
pub const TRANCHE_AMOUNT: u64 = 100_000_000 * 1_000_000_000;    // 100 Mio (9 Decimals)
pub const MIN_DAYS_BETWEEN_RELEASES: i64 = 30 * 86400;          // 30 Tage in Sekunden

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

// Die Handler-Funktion der ReleaseDex-Instruction
// 
// WAS PASSIERT HIER?
// Der Admin kann bis zu 4 Mal jeweils 100 Mio Token aus dem DEX-Vault
// an sein eigenes Wallet freigeben, um später damit das Raydium-LP zu erhöhen.
// Jede Freigabe unterliegt strengen Prüfungen:
// - Nur der Admin darf aufrufen
// - Maximal 4 Tranchen insgesamt
// - Mindestens 30 Tage Abstand zwischen Freigaben
// - Genügend Tokens im Vault
//
// Diese Funktion enthält die gesamte Logik der DEX-Freigabe:
// 1. Prüfung aller Bedingungen (Admin, Tranchen-Anzahl, Zeitabstand, Guthaben)
// 2. Transfer der Tokens vom DEX-Vault zum Admin-Token-Konto
// 3. Aktualisierung der Config (released_tranches +1, last_release = jetzt)
// 4. Ausführliches Logging für Transparenz
//
// ### Warum ist der Admin der einzige, der aufrufen darf?
// Die DEX-Tranchen sind strategische Liquidität, die nur der Dev zum optimalen
// Zeitpunkt ins LP einbringen soll. Eine Automatisierung oder ein öffentlicher
// Aufruf würde diese Kontrolle wegnehmen.
//
// ### Warum 30 Tage Abstand?
// - Verhindert Dumping: Zu viele Tokens auf einmal könnten den Kurs drücken
// - Zeigt Vertrauen: Der Dev verpflichtet sich zu einem langsamen, planbaren Release
// - Gibt dem Markt Zeit, jede Tranche zu absorbieren
// 
// ### Warum 4 Tranchen à 100 Mio?
// 400 Mio DEX-Token insgesamt, aufgeteilt in 4 gleich große Portionen.
// So kann der Dev viermal strategisch Liquidität nachfüllen, ohne den Markt zu überwältigen.
//
// ### Was passiert nach der 4. Freigabe?
// released_tranches = 4 → weitere Aufrufe sind unmöglich (require!-Prüfung)
// Der DEX-Vault sollte dann leer sein (400 Mio wurden transferiert)
//
// ### Warum Bumps aus der Config?
// In initialize.rs haben wir die Bumps gespeichert. Hier verwenden wir sie,
// um die PDAs zu validieren und als Signer aufzutreten. Das spart Rechenzeit
// und macht den Code sicherer (keine Neuberechnung mit möglichen Fehlern).
pub fn handler(ctx: Context<ReleaseDex>) -> Result<()> {
    let config = &mut ctx.accounts.config;
    let dex_vault = &ctx.accounts.dex_vault;
    let admin_token_account = &ctx.accounts.admin_token_account;
    let clock = Clock::get()?;

    // ======================================================
    // 1. PRÜFUNGEN (alle Bedingungen müssen erfüllt sein)
    // ======================================================
    
    // 1.1 Maximale Tranchenanzahl (0-3, denn 4 ist das Maximum)
    //     released_tranches zählt von 0 aufwärts. Bei 3 ist die nächste
    //     Freigabe die 4. und letzte. Bei 4 ist Schluss.
    require!(config.released_tranches < 4, DexErrors::MaxTranchesReached);

    // 1.2 30-Tage-Abstand seit der letzten Freigabe
    //     Wir berechnen den frühestmöglichen Zeitpunkt für die nächste Freigabe
    //     und vergleichen mit der aktuellen Blockchain-Zeit.
    let min_next_release = config.last_release
        .checked_add(MIN_DAYS_BETWEEN_RELEASES)
        .ok_or(DexErrors::NumericalOverflow)?;
    require!(
        clock.unix_timestamp >= min_next_release,
        DexErrors::ReleaseTooSoon
    );

    // 1.3 Genügend Tokens im DEX-Vault
    //     Sicherheitscheck: Der Vault muss mindestens 100 Mio Token enthalten.
    //     Sollte eigentlich immer wahr sein, aber sicher ist sicher.
    require!(
        dex_vault.amount >= TRANCHE_AMOUNT,
        DexErrors::InsufficientVaultBalance
    );

    // ======================================================
    // 2. TRANSFER: 100 MIO TOKEN VOM DEX-VAULT ZUM ADMIN
    // ======================================================
    // 
    // Der DEX-Vault ist ein PDA mit der Config als Authority.
    // Um Tokens daraus zu bewegen, müssen wir als Config signieren.
    // Dazu verwenden wir die Seeds der Config + den gespeicherten Bump.
    // 
    // Wichtig: Die Seeds MÜSSEN exakt denen aus initialize.rs entsprechen:
    // seeds = [b"config"] + Bump
    let seeds = &[&b"config"[..], &[config.bump]];
    let signer_seeds = &[&seeds[..]];

    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: dex_vault.to_account_info(),
                to: admin_token_account.to_account_info(),
                authority: config.to_account_info(), // Config unterschreibt
            },
            signer_seeds, // PDA-Signatur
        ),
        TRANCHE_AMOUNT,
    )?;

    // ======================================================
    // 3. CONFIG AKTUALISIEREN
    // ======================================================
    
    // 3.1 Tranchen-Zähler erhöhen
    //     checked_add verhindert Overflow (theoretisch nicht nötig, da wir
    //     vorher auf <4 prüfen, aber Sicherheit geht vor)
    config.released_tranches = config
        .released_tranches
        .checked_add(1)
        .ok_or(DexErrors::NumericalOverflow)?;

    // 3.2 Zeitstempel der letzten Freigabe aktualisieren
    //     Wichtig für die 30-Tage-Wartezeit bei der nächsten Freigabe
    config.last_release = clock.unix_timestamp;

    // ======================================================
    // 4. LOGGING (für Transparenz und Nachvollziehbarkeit)
    // ======================================================
    msg!("=== DEX-TRANCHE FREIGEGEBEN ===");
    msg!("Admin: {}", config.admin);
    msg!("Freigegebene Tranche: {} von 4", config.released_tranches);
    msg!("Betrag: {} Tokens (100 Mio)", TRANCHE_AMOUNT);
    msg!("Von DEX-Vault: {}", config.dex_vault);
    msg!("An Admin-Token-Konto: {}", admin_token_account.key());
    msg!("Zeitpunkt: {}", clock.unix_timestamp);
    msg!("Nächste Freigabe möglich ab: {}", min_next_release + MIN_DAYS_BETWEEN_RELEASES);
    msg!("Verbleibende Tranchen: {}", 4 - config.released_tranches);
    msg!("=== TRANSAKTION ERFOLGREICH ===");

    Ok(())
}