use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{
    self, close_account, transfer_checked, CloseAccount, Mint, Token, TokenAccount, Transfer,
    TransferChecked,
};
declare_id!("GYatJTRxmLz6K6xMuNY9dprfbtLj8jZJ46TpRAqYk8wL");

#[program]
pub mod escrow {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        escrow_seed: u64,
        initializer_amount: u64,
        taker_amount: u64,
    ) -> Result<()> {
        ctx.accounts.escrow_state.initializer_key = *ctx.accounts.initializer.key;
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
        ctx.accounts.escrow_state.initializer_amount = initializer_amount;
        ctx.accounts.escrow_state.taker_amount = taker_amount;
        ctx.accounts.escrow_state.escrow_seed = escrow_seed;
        ctx.accounts.escrow_state.vault_authority_bump = *ctx.bumps.get("vault_authority").unwrap();
        Ok(())
    }

    pub fn cancel(ctx: Context<Cancel>) -> Result<()> {
        let seeds = &[
            b"authority".as_ref(),
            &[ctx.accounts.escrow_state.vault_authority_bump],
        ];
        let signer_seeds = &[&seeds[..]];

        // transfer_checked
        transfer_checked(
            ctx.accounts
                .into_transfer_to_initializer_context()
                .with_signer(&(&signer_seeds[..])),
            ctx.accounts.escrow_state.initializer_amount,
            ctx.accounts.mint.decimals,
        )?;

        close_account(
            ctx.accounts
                .into_close_context()
                .with_signer(&(&signer_seeds[..])),
        )?;

        ctx.accounts.escrow_state.initializer_amount = 0;

        Ok(())
    }

    pub fn exchange(ctx: Context<Exchange>) -> Result<()> {
        let seeds = &[
            b"authority".as_ref(),
            &[ctx.accounts.escrow_state.vault_authority_bump],
        ];
        let signer_seeds = &[&seeds[..]];

        transfer_checked(
            ctx.accounts
                .into_transfer_to_initializer_context()
                .with_signer(&(&signer_seeds[..])),
            ctx.accounts.escrow_state.taker_amount,
            ctx.accounts.taker_deposit_token_mint.decimals,
        )?;

        transfer_checked(
            ctx.accounts
                .into_transfer_to_taker_context()
                .with_signer(&(&signer_seeds[..])),
            ctx.accounts.escrow_state.initializer_amount,
            ctx.accounts.initializer_deposit_token_mint.decimals,
        )?;

        close_account(
            ctx.accounts
                .into_close_context()
                .with_signer(&(&signer_seeds[..])),
        )?;

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(escrow_seed: u64, initializer_amount: u64)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub initializer: Signer<'info>,
    #[account(
        seeds = [b"authority".as_ref()],
        bump,
    )]
    /// CHECK: OK
    pub vault_authority: AccountInfo<'info>,
    #[account(
        init,
        payer = initializer,
        associated_token::mint = mint,
        associated_token::authority = vault_authority,
    )]
    pub vault: Account<'info, TokenAccount>,
    #[account(mut, constraint = initializer_deposit_token_account.amount >= initializer_amount)]
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
    pub mint: Account<'info, Mint>,
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
    pub vault_authority: AccountInfo<'info>,
    pub initializer_deposit_token_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        constraint = escrow_state.initializer_key == *initializer.key,
        constraint = escrow_state.initializer_deposit_token_account == *initializer_deposit_token_account.to_account_info().key,
        close = initializer
    )]
    pub escrow_state: Account<'info, EscrowState>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct Exchange<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,

    pub mint: Account<'info, Mint>,
    pub initializer_deposit_token_mint: Account<'info, Mint>,
    pub taker_deposit_token_mint: Account<'info, Mint>,
    #[account(mut)]
    pub taker_deposit_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub taker_receive_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub initializer_deposit_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub initializer_receive_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    /// CHECK: OK
    pub initializer: AccountInfo<'info>,
    #[account(
        mut,
        constraint = escrow_state.taker_amount <= taker_deposit_token_account.amount,
        constraint = escrow_state.initializer_deposit_token_account == *initializer_deposit_token_account.to_account_info().key,
        constraint = escrow_state.initializer_receive_token_account == *initializer_receive_token_account.to_account_info().key,
        constraint = escrow_state.initializer_key == *initializer.key,
        close = initializer
    )]
    pub escrow_state: Account<'info, EscrowState>,
    #[account(mut)]
    pub vault: Account<'info, TokenAccount>,
    #[account(seeds = [b"authority".as_ref()], bump)]
    /// CHECK: OK
    pub vault_authority: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}

#[account]
#[derive(InitSpace)]
pub struct EscrowState {
    pub escrow_seed: u64,
    pub initializer_key: Pubkey,
    pub initializer_deposit_token_account: Pubkey,
    pub initializer_receive_token_account: Pubkey,
    pub initializer_amount: u64,
    pub taker_amount: u64,
    pub vault_authority_bump: u8,
}

impl<'info> Initialize<'info> {
    fn into_transfer_to_pda_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, TransferChecked<'info>> {
        let cpi_accounts = TransferChecked {
            from: self.initializer_deposit_token_account.to_account_info(),
            mint: self.mint.to_account_info(),
            to: self.vault.to_account_info(),
            authority: self.initializer.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
    }
}

impl<'info> Exchange<'info> {
    fn into_transfer_to_initializer_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, TransferChecked<'info>> {
        let cpi_accounts = TransferChecked {
            from: self.vault.to_account_info(),
            mint: self.mint.to_account_info(),
            to: self.initializer_deposit_token_account.to_account_info(),
            authority: self.vault_authority.clone(),
        };
        CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
    }

    fn into_transfer_to_taker_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, TransferChecked<'info>> {
        let cpi_accounts = TransferChecked {
            from: self.initializer_deposit_token_account.to_account_info(),
            mint: self.mint.to_account_info(),
            to: self.vault.to_account_info(),
            authority: self.initializer.clone(),
        };
        CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
    }

    fn into_close_context(&self) -> CpiContext<'_, '_, '_, 'info, CloseAccount<'info>> {
        let cpi_accounts = CloseAccount {
            account: self.vault.to_account_info(),
            destination: self.initializer.to_account_info(),
            authority: self.vault_authority.clone(),
        };
        CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
    }
}

impl<'info> Cancel<'info> {
    fn into_transfer_to_initializer_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, TransferChecked<'info>> {
        let cpi_accounts = TransferChecked {
            from: self.vault.to_account_info(),
            mint: self.mint.to_account_info(),
            to: self.initializer_deposit_token_account.to_account_info(),
            authority: self.vault_authority.clone(),
        };
        CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
    }

    fn into_close_context(&self) -> CpiContext<'_, '_, '_, 'info, CloseAccount<'info>> {
        let cpi_accounts = CloseAccount {
            account: self.vault.to_account_info(),
            destination: self.initializer.to_account_info(),
            authority: self.vault_authority.clone(),
        };
        CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
    }
}
