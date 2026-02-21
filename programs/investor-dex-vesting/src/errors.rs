use anchor_lang::prelude::*;

#[error_code]
pub enum VestingError {
    // ========== INVESTOR ERRORS ==========
    #[msg("Investor has already claimed all tokens")]
    AlreadyFullyClaimed,
    
    #[msg("Cannot claim tokens before cliff period ends")]
    CliffPeriodActive,
    
    #[msg("Calculated claim amount is zero")]
    NothingToClaim,
    
    // ========== ADMIN ERRORS ==========
    #[msg("Only admin can call this function")]
    Unauthorized,
    
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

#[error_code]
pub enum ErrorCode {
    #[msg("Nur der Admin darf diese Aktion ausführen.")]
    Unauthorized,
    #[msg("Maximale Anzahl an Tranchen bereits erreicht (4/4).")]
    MaxTranchesReached,
    #[msg("Es müssen mindestens 30 Tage seit der letzten Freigabe vergangen sein.")]
    ReleaseTooSoon,
    #[msg("Nicht genügend Tokens im DEX-Vault.")]
    InsufficientVaultBalance,
    #[msg("Der angegebene Vault stimmt nicht mit dem in der Config überein.")]
    InvalidVault,
    #[msg("Das Admin-Token-Konto gehört nicht dem Admin.")]
    InvalidTokenAccountOwner,
    #[msg("Das Admin-Token-Konto hat den falschen Mint.")]
    InvalidTokenAccountMint,
    #[msg("Numerischer Überlauf.")]
    NumericalOverflow,
}