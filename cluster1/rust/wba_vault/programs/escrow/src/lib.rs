use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod escrow {
    use super::*;

    const AUTHORITY_SEED: &[u8] = b"authority";

    pub fn initialize(
        ctx: Context<Initialize>,
        escrow_seed: u64,
        initializer_amount: u64,
        taker_amount: u64,
    ) -> Result<()> {
        ctx.accounts.escrow_state.initializer = *ctx.accounts.initializer.key;
        ctx.accounts.escrow_state.initializer_deposit_token_account = *ctx
            .accounts
            .initializer_deposit_token_account
            .to_account_info()
            .key;
        ctx.accounts.escrow_state.initializer_receive_token_account = *ctx
            .accounts
            .initializer_receive_token_account
            .to_account_info()
            .key;
        ctx.accounts.escrow_state.taker_amount = taker_amount;
        ctx.accounts.escrow_state.escrow_seed = escrow_seed;

        let (_vault_athority, vault_authority_bump) =
            Pubkey::find_program_address(&[AUTHORITY_SEED], ctx.program_id);
        ctx.accounts.escrow_state.vault_authority_bump = vault_authority_bump;

        ctx.accounts.escrow_state.vault_authority_bump = *ctx.bumps.get("vault_auth").unwrap();
        Ok(())
    }

    #[derive(Accounts)]
    #[instruction(escrow_seed: u64, initializer_amount: u64)]
    pub struct Initialize<'info> {
        #[account(mut)]
        pub initializer: Signer<'info>,
        pub mint: Account<'info, Mint>,
        #[account(
            seeds = [b"authority".as_ref()],
            bump,
        )]
        /// CHECK: OK
        pub vault_authority: UncheckedAccount<'info>,
        #[account(
            init,
            payer = initializer,
            associated_token::mint = mint,
            associated_token::authority = vault_authority,
        )]
        pub vault: Account<'info, TokenAccount>,
        pub initializer_deposit_token_account: Account<'info, TokenAccount>,
        pub initializer_receive_token_account: Account<'info, TokenAccount>,
        #[account(
            init,
             seeds = [b"state".as_ref(), &escrow_seed.to_le_bytes()],
             bump,
             payer = initializer,
             space = 8 + EscrowState::INIT_SPACE,
        )]
        pub escrow_state: Account<'info, EscrowState>,
        pub system_program: Program<'info, System>,
        pub token_program: Program<'info, Token>,
        pub associated_token_program: Program<'info, AssociatedToken>,
    }

    #[derive(Accounts)]
    pub struct Cancel<'info> {
        pub initializer: Signer<'info>,
        pub mint: Account<'info, Mint>,
        #[account(mut)]
        pub vault: Account<'info, TokenAccount>,
        #[account(seeds = [b"authority".as_ref()], bump)]
        /// CHECK: OK
        pub vault_authority: UncheckedAccount<'info>,
        pub initializer_deposit_token_account: Account<'info, TokenAccount>,

        #[account(
        mut,
        constraint = escrow_state.initializer == *initializer.key,
        constraint = escrow_state.initializer_deposit_token_account == *initializer_deposit_token_account.to_account_info().key,
        //constraint = initializer_deposit_token_account == *initializer_deposit_token_account.to_account_info().key,
        close = initializer
    )]
        pub escrow_state: Account<'info, EscrowState>,
    }

    #[derive(Accounts)]
    pub struct Exchange<'info> {
        #[account(mut)]
        pub taker: Signer<'info>,

        pub initializer_deposit_token_mint: Account<'info, Mint>,
        pub taker_deposit_token_mint: Account<'info, Mint>,
        #[account(mut)]
        pub taker_deposit_token_account: Account<'info, TokenAccount>,
        #[account(mut)]
        pub taker_receive_token_account: Account<'info, TokenAccount>,
        #[account(mut)]
        pub initializer_receive_token_account: Account<'info, TokenAccount>,
        #[account(mut)]
        pub initializer_deposit_token_account: Account<'info, TokenAccount>,
        #[account(mut)]
        /// CHECK: OK
        pub initializer: UncheckedAccount<'info>,
        #[account(
            mut,
            constraint = escrow_state.taker_amount <= taker_deposit_token_account.amount,
            constraint = escrow_state.initializer_deposit_token_account == *initializer_deposit_token_account.to_account_info().key,
            constraint = escrow_state.initializer_receive_token_account == *initializer_receive_token_account.to_account_info().key,
            constraint = escrow_state.initializer == *initializer.key,
            close = initializer
        )]
        pub escrow_state: Account<'info, EscrowState>,
        #[account(mut)]
        pub vault: Account<'info, TokenAccount>,
        #[account(seeds = [b"authority".as_ref()], bump)]
        /// CHECK: OK
        pub vault_authority: UncheckedAccount<'info>,
        pub token_program: Program<'info, Token>,
    }

    #[account]
    #[derive(InitSpace)]
    pub struct EscrowState {
        pub escrow_seed: u64,
        pub initializer: Pubkey,
        pub initializer_deposit_token_account: Pubkey,
        pub initializer_receive_token_account: Pubkey,
        pub taker_amount: u64,
        pub vault_authority_bump: u8,
    }
}
