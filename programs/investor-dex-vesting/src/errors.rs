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