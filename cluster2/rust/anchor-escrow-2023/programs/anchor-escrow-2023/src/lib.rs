use anchor_lang::prelude::*;

declare_id!("5rJDGSxbwrbi6CB2MgZKV3Q2gLwr9Bx9vYuiC9wv6h7X");

mod errors;
use errors::EscrowError;
mod contexts;
mod structs;
use contexts::*;

#[program]
pub mod anchor_escrow_2023 {
    use super::*;

    pub fn make(
        ctx: Context<Make>,
        seed: u64,
        deposit_amount: u64,
        offer_amount: u64,
        expires: u64,
    ) -> Result<()> {
        require!(expires.lt(&100000), EscrowError::EscrowMaxExpiryExceeded);

        // Set up our escrow variables
        let escrow = &mut ctx.accounts.escrow;
        escrow.maker = *ctx.accounts.maker.key;
        escrow.maker_token = *ctx.accounts.maker_token.to_account_info().key;
        escrow.taker_token = *ctx.accounts.taker_token.to_account_info().key;
        escrow.seed = seed;
        escrow.offer_amount = offer_amount;
        escrow.auth_bump = *ctx.bumps.get("auth").ok_or(EscrowError::AuthBumpError)?;
        escrow.vault_bump = *ctx.bumps.get("vault").ok_or(EscrowError::VaultBumpError)?;
        escrow.escrow_bump = *ctx
            .bumps
            .get("escrow")
            .ok_or(EscrowError::EscrowBumpError)?;

        if expires > 0 {
            escrow.expires_at = Clock::get()?
                .slot
                .checked_add(expires)
                .ok_or(EscrowError::EscrowExpiryError)?;
        } else {
            escrow.expires_at = 0;
        }
        // Transfer maker tokens to the vault
        ctx.accounts.transfer_to_vault(deposit_amount)
    }

    // Cancel and refund escrow to the maker
    pub fn refund(ctx: Context<Refund>) -> Result<()> {
        ctx.accounts.empty_vault()?;
        ctx.accounts.close_vault()
    }

    // Allow maker to change the token and offer amount of the escrow
    pub fn update(ctx: Context<Update>, offer_amount: u64, expires: u64) -> Result<()> {
        require!(expires.lt(&100000), EscrowError::EscrowMaxExpiryExceeded);

        let escrow = &mut ctx.accounts.escrow;
        escrow.taker_token = *ctx.accounts.new_taker_token.to_account_info().key;
        escrow.offer_amount = offer_amount;

        if expires > 0 {
            escrow.expires_at = Clock::get()?
                .slot
                .checked_add(expires)
                .ok_or(EscrowError::EscrowExpiryError)?;
        } else {
            escrow.expires_at = 0;
        }
        Ok(())
    }

    // Allow taker to accept the escrow
    pub fn take(ctx: Context<Take>) -> Result<()> {
        let escrow = &mut ctx.accounts.escrow;

        if escrow.expires_at != 0 {
            let slot = Clock::get()?.slot;
            require!(
                escrow.expires_at != 0 && escrow.expires_at > slot,
                EscrowError::EscrowExpired
            );
        }

        ctx.accounts.deposit_to_maker()?;
        ctx.accounts.empty_vault_to_taker()?;

        if ctx.accounts.escrow.expires_at != 0
            && Clock::get()?.slot > ctx.accounts.escrow.expires_at
        {
            return Err(EscrowError::EscrowExpired.into());
        }

        ctx.accounts.close_vault()
    }
}
