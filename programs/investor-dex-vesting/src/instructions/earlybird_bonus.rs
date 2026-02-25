// -----------------------------------------------
// moodycats.com / moodycats.io
// programs/investor-dex-vesting/instructions/earlybird_bonus.rs
//
// EARLYBIRD-BONUS: Extra-Tokens für erste Supporter
// -----------------------------------------------
// 
// WAS PASSIERT HIER?
// Die ersten User und Investoren erhalten einen zusätzlichen Bonus
// als Dankeschön für ihr frühes Vertrauen. Diese Funktion ist FLEXIBEL:
// - Unterschiedliche Beträge pro User möglich
// - Zeitlich begrenzt (nur in der Earlybird-Phase)
// - Manuelle Steuerung durch Admin
//
// ABLAUF:
// 1. Admin entscheidet: "User X bekommt Y Tokens extra"
// 2. Admin ruft earlybird_bonus(user, amount) auf
// 3. Prüfungen:
//    - Admin ist berechtigt
//    - Genug Tokens im Gift-Vault
//    - User-Token-Konto ist korrekt
// 4. Transfer: amount aus gift_vault → user_token_account
// 5. (Optional) EarlybirdClaim-PDA für Doppel-Schutz
//
// SECURITY:
// - Nur Admin darf aufrufen (volle Kontrolle)
// - Config ist Authority des Gift-Vaults
// - Keine Automatisierung – jeder Bonus manuell
//
// -----------------------------------------------
// WER: Admin (manuell)
// WANN: Nur in der Earlybird-Phase (vor Markteintritt)
// 
// PRÜFUNGEN (4):
// 1. admin == config.admin
// 2. gift_vault.amount >= amount
// 3. user_token_account.owner == user
// 4. user_token_account.mint == config.mint
//
// AKTIONEN (1):
// 1. Transfer amount aus gift_vault → user_token_account
//
// -----------------------------------------------
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use crate::states::config::Config;
use crate::errors::EarlybirdErrors;

#[derive(Accounts)]
pub struct EarlybirdBonus<'info> {
    /// Der Admin, der den Bonus vergibt (muss signieren)
    #[account(mut)]
    pub admin: Signer<'info>,

    /// Globale Config (Admin-Prüfung)
    #[account(
        seeds = [b"config"],
        bump = config.bump,
        constraint = config.admin == admin.key() @ crate::errors::BaseErrors::Unauthorized,
    )]
    pub config: Account<'info, Config>,

    /// Gift-Vault: Quelle für Earlybird-Boni
    #[account(
        mut,
        seeds = [b"gift_vault"],
        bump = config.gift_vault_bump,
        constraint = gift_vault.key() == config.gift_vault,
    )]
    pub gift_vault: Account<'info, TokenAccount>,

    /// Der User, der den Bonus bekommt
    /// CHECK: Nur Empfänger
    pub user: SystemAccount<'info>,

    /// Token-Konto des Users
    #[account(
        mut,
        constraint = user_token_account.owner == user.key() @ EarlybirdErrors::InvalidUserTokenOwner,
        constraint = user_token_account.mint == config.mint @ EarlybirdErrors::InvalidUserTokenMint,
    )]
    pub user_token_account: Account<'info, TokenAccount>,

    /// Token-Programm
    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<EarlybirdBonus>, amount: u64) -> Result<()> {
    // ======================================================
    // 1. PRÜFUNGEN
    // ======================================================
    
    // 1.1 Genug Tokens im Gift-Vault?
    require!(
        ctx.accounts.gift_vault.amount >= amount,
        EarlybirdErrors::InsufficientGiftVaultBalance
    );

    // 1.2 Sinnvoller Betrag? (optional)
    require!(amount > 0, crate::errors::BaseErrors::InvalidAmount);

    // ======================================================
    // 2. PDA-SIGNER
    // ======================================================
    let seeds = &[&b"config"[..], &[ctx.accounts.config.bump]];
    let signer_seeds = &[&seeds[..]];

    // ======================================================
    // 3. TRANSFER: Bonus an User
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
        amount,
    )?;

    // ======================================================
    // 4. LOGGING
    // ======================================================
    msg!("=== EARLYBIRD-BONUS VERGEBEN ===");
    msg!("Admin: {}", ctx.accounts.admin.key());
    msg!("User: {}", ctx.accounts.user.key());
    msg!("Bonus: {} Tokens", amount);
    msg!("Aus Gift-Vault: {}", ctx.accounts.gift_vault.key());
    msg!("Verbleibend: {}", ctx.accounts.gift_vault.amount - amount);

    Ok(())
}