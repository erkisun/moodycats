// -----------------------------------------------
// programs/investor-dex-vesting/instructions/register_investor.rs
//
// INVESTOREN-VERKAUF (OTC)
// -----------------------------------------------
// 
// WAS PASSIERT HIER?
// Diese Funktion wird AUFGERUFEN, wenn ein Investor auf der Webseite SOL bezahlt hat.
// Der Off-Chain-Teil (Zahlung) passiert separat, diese Funktion transferiert NUR die Tokens.
//
// ABLAUF:
// 1. Admin/Webseite ruft Funktion auf (mit Investor-Infos)
// 2. Berechnet Token-Menge + 20% Bonus
// 3. Prüft: Genug Tokens im Investor-Vault?
// 4. Transferiert Tokens SOFORT an Investor
// 5. Optional: Erstellt InvestorReceipt (für Transparenz)
//
// WICHTIG:
// - KEINE SOL-Transaktion! SOL wurde bereits OFF-CHAIN gezahlt
// - Sofortige Übertragung, kein Vesting
// - Investor erhält Tokens direkt in seine Wallet
// -----------------------------------------------
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use crate::states::config::Config;
use crate::errors::BaseErrors;
use crate::errors::RegisterInvestorErrors;

pub const BONUS_PERCENT: u8 = 20;  // 20% Bonus

#[derive(Accounts)]
pub struct RegisterInvestor<'info> {
    /// Der Admin, der den Kauf ausführt (muss signieren)
    #[account(mut)]
    pub admin: Signer<'info>,

    /// Globale Config (muss existieren und Admin prüfen)
    #[account(
        mut,  // Config wird nicht geändert, aber Vaults werden gelesen
        seeds = [b"config"],
        bump = config.bump,
        constraint = config.admin == admin.key() @ BaseErrors::Unauthorized,
    )]
    pub config: Account<'info, Config>,

    /// Investor-Vault: 500 Mio Tokens (reiner Verkauf)
    #[account(
        mut,
        seeds = [b"investor_vault"],
        bump = config.investor_vault_bump,
        constraint = investor_vault.key() == config.investor_vault,
    )]
    pub investor_vault: Account<'info, TokenAccount>,

    /// Gift-Vault: 500 Mio Tokens (für Bonus, Starter, EarlyBird)
    #[account(
        mut,
        seeds = [b"gift_vault"],
        bump = config.gift_vault_bump,
        constraint = gift_vault.key() == config.gift_vault,
    )]
    pub gift_vault: Account<'info, TokenAccount>,

    /// Der Investor, der die Tokens bekommt (muss nicht signieren)
    /// CHECK: Wird nur als Empfänger verwendet
    pub investor: SystemAccount<'info>,

    /// Das Token-Konto des Investors (wo die Tokens hin sollen)
    #[account(
        mut,
        constraint = investor_token_account.owner == investor.key(),
        constraint = investor_token_account.mint == config.mint,
    )]
    pub investor_token_account: Account<'info, TokenAccount>,

    /// Token-Programm für Transfers
    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<RegisterInvestor>, amount: u64) -> Result<()> {
    // ======================================================
    // 1. PRÜFUNGEN
    // ======================================================
    
    // 1.1 Bonus berechnen (20%)
    let bonus = amount
        .checked_mul(BONUS_PERCENT as u64)
        .ok_or(BaseErrors::NumericalOverflow)?
        .checked_div(100)
        .ok_or(BaseErrors::NumericalOverflow)?;

    // 1.2 Genug Tokens im Investor-Vault?
    require!(
        ctx.accounts.investor_vault.amount >= amount,
        RegisterInvestorErrors::InsufficientInvestorVaultBalance
    );

    // 1.3 Genug Tokens im Gift-Vault für Bonus?
    require!(
        ctx.accounts.gift_vault.amount >= bonus,
        RegisterInvestorErrors::InsufficientGiftVaultBalance
    );

    // ======================================================
    // 2. PDA-SIGNER VORBEREITEN (für beide Transfers)
    // ======================================================
    let seeds = &[&b"config"[..], &[ctx.accounts.config.bump]];
    let signer_seeds = &[&seeds[..]];

    // ======================================================
    // 3. TRANSFER 1: Grundbetrag aus investor_vault
    // ======================================================
    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.investor_vault.to_account_info(),
                to: ctx.accounts.investor_token_account.to_account_info(),
                authority: ctx.accounts.config.to_account_info(),
            },
            signer_seeds,
        ),
        amount,
    )?;

    // ======================================================
    // 4. TRANSFER 2: Bonus aus gift_vault
    // ======================================================
    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.gift_vault.to_account_info(),
                to: ctx.accounts.investor_token_account.to_account_info(),
                authority: ctx.accounts.config.to_account_info(),
            },
            signer_seeds,
        ),
        bonus,
    )?;

    // ======================================================
    // 5. LOGGING
    // ======================================================
    msg!("=== INVESTOREN-KAUF ERFOLGREICH ===");
    msg!("Investor: {}", ctx.accounts.investor.key());
    msg!("Gekaufte Menge: {} Tokens", amount);
    msg!("Bonus ({}%): {} Tokens", BONUS_PERCENT, bonus);
    msg!("Total erhalten: {} Tokens", amount + bonus);
    msg!("Verbleibend im Investor-Vault: {}", ctx.accounts.investor_vault.amount - amount);
    msg!("Verbleibend im Gift-Vault: {}", ctx.accounts.gift_vault.amount - bonus);

    Ok(())
}