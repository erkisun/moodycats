// -----------------------------------------------
// moodycats.com / moodycats.io
// programs/investor-dex-vesting/instructions/dev_allocation.rs
//
// DEV-ALLOCATION: Gestaffelte Auszahlung der Dev-Anteile (1% vom Supply)
// -----------------------------------------------
// 
// WAS PASSIERT HIER?
// Der Admin (Dev) erhält seinen Anteil von insgesamt 50 Mio Tokens (1% vom Total Supply),
// aufgeteilt in drei Phasen, um langfristige Motivation und Vertrauen zu zeigen.
//
// PHASEN:
// - Phase 1: 20 Mio Tokens – nach Abschluss von Contract 1 (vor revoke_admin)
// - Phase 2: 15 Mio Tokens – nachdem Contract 2 live ist
// - Phase 3: 15 Mio Tokens – beim Start von Contract 3
//
// WICHTIG:
// - Jede Phase kann NUR EINMAL ausgezahlt werden (Flags in Config)
// - Der Zeitpunkt der Auszahlung entscheidet der Admin MANUELL
// - Die Tokens kommen aus dem GIFT-VAULT (500 Mio Topf)
//
// PRÜFUNGEN:
// 1. admin == config.admin
// 2. phase muss 1, 2 oder 3 sein
// 3. Die entsprechende Phase darf noch nicht ausgezahlt sein
// 4. gift_vault.amount >= phasen_betrag
// 5. admin_token_account.owner == admin
// 6. admin_token_account.mint == config.mint
//
// -----------------------------------------------
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use crate::states::config::Config;
use crate::errors::{BaseErrors, DevErrors};

// Phasen-Definition: (Phasennummer, Betrag in Tokens mit 9 Decimals)
pub const DEV_PHASE_1: u64 = 20_000_000 * 1_000_000_000;  // Phase 1: 20 Mio Tokens
pub const DEV_PHASE_2: u64 = 15_000_000 * 1_000_000_000;  // Phase 2: 15 Mio Tokens
pub const DEV_PHASE_3: u64 = 15_000_000 * 1_000_000_000;  // Phase 3: 15 Mio Tokens

#[derive(Accounts)]
pub struct DevAllocation<'info> {
    /// Der Admin (Dev), der die Auszahlung durchführt (muss signieren)
    #[account(mut)]
    pub admin: Signer<'info>,

    /// Globale Config (enthält Admin, Flags und Bumps)
    #[account(
        mut,
        seeds = [b"config"],
        bump = config.bump,
        constraint = config.admin == admin.key() @ BaseErrors::Unauthorized,
    )]
    pub config: Account<'info, Config>,

    /// Gift-Vault: Quelle für Dev-Anteile (500 Mio Topf)
    #[account(
        mut,
        seeds = [b"gift_vault"],
        bump = config.gift_vault_bump,
    )]
    pub gift_vault: Account<'info, TokenAccount>,

    /// Admin-Token-Konto (wo die Tokens hin sollen)
    #[account(
        mut,
        constraint = admin_token_account.owner == admin.key() @ DevErrors::InvalidTokenAccountOwner,
        constraint = admin_token_account.mint == config.mint @ DevErrors::InvalidTokenAccountMint,
    )]
    pub admin_token_account: Account<'info, TokenAccount>,

    /// Token-Programm für den Transfer
    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<DevAllocation>, phase: u8) -> Result<()> {
    // ======================================================
    // 1. BETRAG BASIEREND AUF PHASE BESTIMMEN
    // ======================================================
    
    let amount = match phase {
        1 => DEV_PHASE_1,
        2 => DEV_PHASE_2,
        3 => DEV_PHASE_3,
        _ => return Err(DevErrors::InvalidPhase.into()),
    };

    // ======================================================
    // 2. PRÜFEN OB DIESE PHASE BEREITS AUSGEZAHLT WURDE
    // ======================================================
    
    match phase {
        1 => {
            if ctx.accounts.config.dev_phase1_paid {
                return Err(DevErrors::PhaseAlreadyPaid.into());
            }
        }
        2 => {
            if ctx.accounts.config.dev_phase2_paid {
                return Err(DevErrors::PhaseAlreadyPaid.into());
            }
        }
        3 => {
            if ctx.accounts.config.dev_phase3_paid {
                return Err(DevErrors::PhaseAlreadyPaid.into());
            }
        }
        _ => unreachable!(),
    }

    // ======================================================
    // 3. PRÜFEN OB GENUG TOKENS IM GIFT-VAULT SIND
    // ======================================================
    
    if ctx.accounts.gift_vault.amount < amount {
        return Err(DevErrors::InsufficientGiftVaultBalance.into());
    }

    // ======================================================
    // 4. PDA-SIGNER VORBEREITEN
    // ======================================================
    // Der Gift-Vault hat die Config als Authority.
    // Wir müssen als Config signieren, um Tokens zu transferieren.
    
    let seeds = &[&b"config"[..], &[ctx.accounts.config.bump]];
    let signer_seeds = &[&seeds[..]];

    // ======================================================
    // 5. TRANSFER: Phasen-Betrag an Admin
    // ======================================================
    
    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.gift_vault.to_account_info(),
                to: ctx.accounts.admin_token_account.to_account_info(),
                authority: ctx.accounts.config.to_account_info(),
            },
            signer_seeds,
        ),
        amount,
    )?;

    // ======================================================
    // 6. CONFIG UPDATEN: FLAG FÜR DIESE PHASE SETZEN
    // ======================================================
    
    let config = &mut ctx.accounts.config;
    
    match phase {
        1 => config.dev_phase1_paid = true,
        2 => config.dev_phase2_paid = true,
        3 => config.dev_phase3_paid = true,
        _ => unreachable!(),
    }

    // ======================================================
    // 7. LOGGING
    // ======================================================
    
    msg!("=== DEV-ALLOCATION PHASE {} AUSGEZAHLT ===", phase);
    msg!("Admin: {}", ctx.accounts.admin.key());
    msg!("Phase: {} ({} Tokens)", phase, amount);
    msg!("Aus Gift-Vault: {}", ctx.accounts.gift_vault.key());
    msg!("An Admin-Token-Konto: {}", ctx.accounts.admin_token_account.key());
    
    // Status anzeigen
    msg!("Flags nach Auszahlung:");
    msg!("  Phase1 paid: {}", config.dev_phase1_paid);
    msg!("  Phase2 paid: {}", config.dev_phase2_paid);
    msg!("  Phase3 paid: {}", config.dev_phase3_paid);
    
    msg!("Verbleibend im Gift-Vault: {}", 
         ctx.accounts.gift_vault.amount - amount);
    msg!("=== TRANSAKTION ERFOLGREICH ===");

    Ok(())
}