use anchor_lang::error_code;

#[error_code]
pub enum EscrowError {
    #[msg("Unable to get auth bump")]
    AuthBumpError,
    #[msg("Unable to get vault bump")]
    VaultBumpError,
    #[msg("Unable to get escrow bump")]
    EscrowBumpError,
    #[msg("Escrow has expired")]
    EscrowExpired,
    #[msg("Max expiry exceeded")]
    EscrowMaxExpiryExceeded,
    #[msg("Escrow expiry error")]
    EscrowExpiryError,
}
