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

#[error_code]
pub enum VestingError {
    // ========== INVESTOR ERRORS ==========
    #[msg("Investor has already claimed all tokens")]
    AlreadyFullyClaimed,
    
    #[msg("Cannot claim tokens before cliff period ends")]
    CliffPeriodActive,
    
    #[msg("Calculated claim amount is zero")]
    NothingToClaim,
    
    #[msg("Admin privileges have already been revoked")]
    AdminAlreadyRevoked,
    
    // ========== DEX ERRORS ==========
    #[msg("No more DEX tokens available to release")]
    NoMoreDexTokens,
    
    #[msg("Cannot release DEX tokens more than once per month")]
    ReleaseTooEarly,
    
    #[msg("DEX vesting account not initialized")]
    DexVestingNotInitialized,
    
    // ========== MATH ERRORS ==========
    #[msg("Arithmetic overflow occurred")]
    ArithmeticOverflow,
    
    #[msg("Invalid amount provided")]
    InvalidAmount,
    
    // ========== ACCOUNT ERRORS ==========
    #[msg("Account is not initialized")]
    AccountNotInitialized,
    
    #[msg("Account already initialized")]
    AccountAlreadyInitialized,
    
    #[msg("PDA bump seed invalid")]
    InvalidBump,
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
    #[msg("Nicht genug Tokens im Gift-Vault für Starter.")]
    InsufficientGiftVaultBalance,
    #[msg("User hat bereits Starter-Tokens erhalten.")]
    AlreadyClaimed,
    #[msg("User-Token-Konto gehört nicht dem User.")]
    InvalidUserTokenOwner,
    #[msg("User-Token-Konto hat falschen Mint.")]
    InvalidUserTokenMint,
}