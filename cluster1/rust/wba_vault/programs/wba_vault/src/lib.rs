use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod wba_vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.vault_state.owner = *ctx.accounts.owner.key;
        ctx.accounts.vault_state.auth_bump = *ctx.bumps.get("vault_auth").unwrap();
        ctx.accounts.vault_state.auth_bump = *ctx.bumps.get("vault").unwrap();
        ctx.accounts.vault_state.score = 0;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(init, payer = owner, space = 8 + 32 + 1 + 1 + 1)]
    pub vault_state: Account<'info, VaultState>,
    #[account(mut, seeds = [b"auth", vault_state.key().as_ref()], bump)]
    /// CHECK: OK
    pub vault_auth: UncheckedAccount<'info>,
    #[account(mut, seeds = [b"vault", vault_auth.key().as_ref()], bump)]
    pub vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct VaultState {
    owner: Pubkey,
    auth_bump: u8,
    vault_bump: u8,
    score: u8,
}

/*
owner: keypair.publicKey,
vaultState: vaultState.publicKey,
vaultAuth: vaultAuth,
vault: vaultPda,
systemProgram: SystemProgram.programId,
 */
