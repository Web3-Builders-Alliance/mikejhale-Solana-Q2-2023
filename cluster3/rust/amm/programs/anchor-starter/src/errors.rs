use anchor_lang::error_code;

#[error_code]
pub enum AmmError {
    #[msg("Program Error Occurred")]
    AmmErrorOccurred,
    #[msg("Bump Error Occurred")]
    BumpError,
    #[msg("Liquidity less than minimum")]
    LiquidityLessThanMinimum,
    #[msg("Pool is frozen")]
    PoolFrozen,
    #[msg("Add Liquidity Expired")]
    AddLiquidityExpired,
    #[msg("Insufficient Token Amount")]
    InsufficientTokenAmount,
}
