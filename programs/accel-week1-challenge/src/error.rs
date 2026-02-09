use anchor_lang::prelude::*;

#[error_code]
pub enum VaultError {
    #[msg("Withdraw amount exceeds total vault lamports")]
    InsufficientVaultBalance,
    #[msg("Withdraw would violate rent-exemption requirements")]
    BelowRentExemption,
}
