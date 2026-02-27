// -----------------------------------------------
// moodycats.com / moodycats.io
// programs/investor-dex-vesting/errors.rs
// Contract 1
//
// -----------------------------------------------
use anchor_lang::prelude::*;

// ========== BASIS-FEHLER (für alle) ==========
#[error_code]
pub enum BaseErrors {
    #[msg("Nur der Admin darf diese Aktion ausführen.")]
    Unauthorized,
    #[msg("Numerischer Überlauf.")]
    NumericalOverflow,
    #[msg("Ungültiger Betrag.")]
    InvalidAmount,
    #[msg("Konto nicht initialisiert.")]
    AccountNotInitialized,
    #[msg("Konto bereits initialisiert.")]
    AccountAlreadyInitialized,
    #[msg("PDA Bump ungültig.")]
    InvalidBump,
}

// ========== DEV-ALLOCATION (dev_allocation.rs) ==========
#[error_code]
pub enum DevErrors {
    #[msg("Ungültige Phase. Nur 1, 2 oder 3 erlaubt.")]
    InvalidPhase,
     #[msg("Diese Phase wurde bereits ausgezahlt.")]
    PhaseAlreadyPaid,
     #[msg("Nicht genug Tokens im Gift-Vault für Dev-Allocation.")]
    InsufficientGiftVaultBalance,
     #[msg("Admin-Token-Konto gehört nicht dem Admin.")]
    InvalidTokenAccountOwner,
     #[msg("Admin-Token-Konto hat falschen Mint.")]
    InvalidTokenAccountMint,
}

// ========== DEX VESTING (release_dex.rs) ==========
#[error_code]
pub enum DexErrors {
    #[msg("Maximale Anzahl Tranchen bereits erreicht (4/4).")]
    MaxTranchesReached,
    #[msg("Es müssen mindestens 30 Tage seit der letzten Freigabe vergangen sein.")]
    ReleaseTooSoon,
    #[msg("Nicht genügend Tokens im DEX-Vault.")]
    InsufficientVaultBalance,
    #[msg("Der DEX-Vault stimmt nicht mit der Config.")]
    InvalidVault,
    #[msg("Das Admin-Token-Konto gehört nicht dem Admin.")]
    InvalidTokenAccountOwner,
    #[msg("Das Admin-Token-Konto hat den falschen Mint.")]
    InvalidTokenAccountMint,
    #[msg("Nur der Admin darf diese Aktion ausführen.")]
    Unauthorized,
    #[msg("Numerischer Überlauf.")]
    NumericalOverflow,
}

// ========== INVESTOREN-VERKAUF (register_investor.rs) ==========
#[error_code]
pub enum RegisterInvestorErrors {
    #[msg("Nicht genug Tokens im Investor-Vault")]
    InsufficientInvestorVaultBalance,
    #[msg("Nicht genug Tokens im Gift-Vault für Bonus")]
    InsufficientGiftVaultBalance,
    #[msg("Investor-Token-Konto gehört nicht dem Investor.")]
    InvalidInvestorTokenOwner,
    #[msg("Investor-Token-Konto hat falschen Mint.")]
    InvalidInvestorTokenMint,
}

// ========== STARTER-TOKENS (claim_starter.rs) ==========
#[error_code]
pub enum StarterErrors {
    #[msg("Nicht genug Tokens im Gift-Vault für 7 Starter Tokens.")]
    InsufficientGiftVaultBalance,
    #[msg("User hat bereits 7 Starter-Tokens erhalten.")]
    AlreadyClaimed,
    #[msg("User-Token-Konto gehört nicht dem User.")]
    InvalidUserTokenOwner,
    #[msg("User-Token-Konto hat falschen Mint.")]
    InvalidUserTokenMint,
}

// ========== EARLYBIRD-BONUS (earlybird_bonus.rs) ==========
#[error_code]
pub enum EarlyBirdErrors {
    #[msg("Nicht genug Tokens im Gift-Vault für Starter.")]
    InsufficientGiftVaultBalance,
    #[msg("User hat bereits EarlyBird-Bonus erhalten.")]
    AlreadyEarlyBirdBonusClaimed,
    #[msg("User-Token-Konto gehört nicht dem User.")]
    InvalidUserTokenOwner,
    #[msg("User-Token-Konto hat falschen Mint.")]
    InvalidUserTokenMint,
}