// -----------------------------------------------
// moodycats.com / moodycats.io
// programs/investor-dex-vesting/instructions/claim_starter.rs
//
// STARTER-TOKENS: 7 Tokens für jede Neuregistrierung
// -----------------------------------------------
// 
// WAS PASSIERT HIER?
// Jeder neue User (egal ob Investor oder normaler User) erhält 7 Starter-Tokens
// als Willkommensgeschenk. Die Auszahlung erfolgt via Batch-Script durch den Admin,
// basierend auf einer CSV aus der Webseiten-Datenbank.
//
// ABLAUF:
// 1. Admin exportiert CSV mit allen neuen, verifizierten Wallets
// 2. Batch-Script ruft claim_starter() für jede Wallet auf
// 3. Prüfungen:
//    - Admin ist berechtigt
//    - User hat noch nie Starter-Tokens erhalten (PDA)
//    - Genug Tokens im Gift-Vault
//    - User-Token-Konto existiert und ist korrekt
// 4. Transfer: 7 Tokens aus gift_vault → user_token_account
// 5. StarterClaim-PDA wird erstellt (verhindert Doppel-Claims)
// 6. Fertig! User hat 7 Tokens erhalten
//
// SECURITY:
// - Nur Admin darf aufrufen (via Batch-Script)
// - PDA pro User = 100% Replay-Schutz
// - Config ist Authority des Gift-Vaults (via PDA-Signer)
//
// -----------------------------------------------
// WER: Admin (per Batch-Script)
// WANN: Beliebig oft, aber pro User nur einmal
// 
// PRÜFUNGEN (5):
// 1. admin == config.admin
// 2. gift_vault.amount >= STARTER_AMOUNT (7 Tokens)
// 3. starter_claim PDA existiert noch NICHT (wird durch init geprüft)
// 4. user_token_account.owner == user
// 5. user_token_account.mint == config.mint
//
// AKTIONEN (2):
// 1. Transfer 7 Tokens gift_vault → user_token_account
// 2. StarterClaim-PDA erstellen (user, claimed_at, bump)
//
// SPEICHER:
// StarterClaim-PDA unter seeds = [b"starter", user.key()]
// Enthält: user, claimed_at, bump
//
// -----------------------------------------------
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use crate::states::config::Config;
use crate::errors::StarterErrors;

pub const STARTER_AMOUNT: u64 = 7 * 1_000_000_000;  // 7 Tokens (9 Decimals)

// StarterClaim-PDA: Speichert, wer schon Starter-Tokens erhalten hat
#[account]
pub struct StarterClaim {
    /// Der User, der die Starter-Tokens erhalten hat
    pub user: Pubkey,
    
    /// Zeitpunkt der Auszahlung (Unix-Timestamp)
    pub claimed_at: i64,
    
    /// Bump für diesen PDA (für spätere Zugriffe, falls nötig)
    pub bump: u8,
}

impl StarterClaim {
    pub const LEN: usize = 8 + 32 + 8 + 1;  // Discriminator + user + claimed_at + bump
}

#[derive(Accounts)]
pub struct ClaimStarter<'info> {
    /// Der Admin, der das Batch-Script ausführt (muss signieren)
    #[account(mut)]
    pub admin: Signer<'info>,

    /// Globale Config (muss existieren und Admin prüfen)
    #[account(
        seeds = [b"config"],
        bump = config.bump,
        constraint = config.admin == admin.key() @ crate::errors::BaseErrors::Unauthorized,
    )]
    pub config: Account<'info, Config>,

    /// Gift-Vault: 500 Mio Tokens (für Starter, Bonus, etc.)
    #[account(
        mut,
        seeds = [b"gift_vault"],
        bump = config.gift_vault_bump,
        constraint = gift_vault.key() == config.gift_vault,
    )]
    pub gift_vault: Account<'info, TokenAccount>,

    /// Der User, der die Starter-Tokens bekommen soll
    /// (muss nicht signieren – Admin tut es für ihn)
    /// CHECK: Wird nur als Empfänger und für PDA-Seeds verwendet
    pub user: SystemAccount<'info>,

    /// Das Token-Konto des Users (wo die 7 Tokens hin sollen)
    /// Muss vorher existieren (der User muss es angelegt haben)
    #[account(
        mut,
        constraint = user_token_account.owner == user.key() @ StarterErrors::InvalidUserTokenOwner,
        constraint = user_token_account.mint == config.mint @ StarterErrors::InvalidUserTokenMint,
    )]
    pub user_token_account: Account<'info, TokenAccount>,

    /// StarterClaim-PDA: Beweis, dass dieser User schon erhalten hat
    /// Wird hier neu erstellt – falls es schon existiert, schlägt die TX fehl
    /// Das ist der Replay-Schutz!
    #[account(
        init,
        payer = admin,
        space = StarterClaim::LEN,
        seeds = [b"starter", user.key().as_ref()],
        bump,
    )]
    pub starter_claim: Account<'info, StarterClaim>,

    /// Token-Programm für Transfer
    pub token_program: Program<'info, Token>,

    /// System-Programm für Konto-Erstellung (starter_claim)
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<ClaimStarter>) -> Result<()> {
    // ======================================================
    // 1. PRÜFUNGEN (was nicht schon durch Constraints abgedeckt ist)
    // ======================================================
    
    // 1.1 Genug Tokens im Gift-Vault?
    //     (Constaints prüfen nur die Konten, nicht den Saldo)
    require!(
        ctx.accounts.gift_vault.amount >= STARTER_AMOUNT,
        StarterErrors::InsufficientGiftVaultBalance
    );

    // ======================================================
    // 2. PDA-SIGNER VORBEREITEN
    // ======================================================
    // Der Gift-Vault hat die Config als Authority.
    // Um Tokens zu transferieren, müssen wir als Config signieren.
    let seeds = &[&b"config"[..], &[ctx.accounts.config.bump]];
    let signer_seeds = &[&seeds[..]];

    // ======================================================
    // 3. TRANSFER: 7 TOKENS AUS GIFT-VAULT AN USER
    // ======================================================
    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.gift_vault.to_account_info(),
                to: ctx.accounts.user_token_account.to_account_info(),
                authority: ctx.accounts.config.to_account_info(),
            },
            signer_seeds,
        ),
        STARTER_AMOUNT,
    )?;

    // ======================================================
    // 4. STARTER-CLAIM SPEICHERN (Replay-Schutz)
    // ======================================================
    let claim = &mut ctx.accounts.starter_claim;
    claim.user = ctx.accounts.user.key();
    claim.claimed_at = Clock::get()?.unix_timestamp;
    claim.bump = ctx.bumps.starter_claim;

    // ======================================================
    // 5. LOGGING (für Transparenz und Batch-Protokoll)
    // ======================================================
    msg!("=== STARTER-TOKENS AUSGEZAHLT ===");
    msg!("Admin: {}", ctx.accounts.admin.key());
    msg!("User: {}", ctx.accounts.user.key());
    msg!("Betrag: 7 Tokens ({})", STARTER_AMOUNT);
    msg!("Aus Gift-Vault: {}", ctx.accounts.gift_vault.key());
    msg!("An Token-Konto: {}", ctx.accounts.user_token_account.key());
    msg!("Claim-PDA: {}", ctx.accounts.starter_claim.key());
    msg!("Zeitpunkt: {}", claim.claimed_at);
    msg!("Verbleibend im Gift-Vault: {}", 
         ctx.accounts.gift_vault.amount - STARTER_AMOUNT);
    msg!("=== TRANSAKTION ERFOLGREICH ===");

    Ok(())
}