use crate::errors::AmmError;
use crate::state::config::Config;
use anchor_lang::prelude::*;
use anchor_spl::token::{self, burn, Burn, Mint, Token, TokenAccount, Transfer};

#[derive(Accounts)]
pub struct RemoveLiquidity<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut, associated_token::mint = config.mint_x, associated_token::authority = pda_auth)]
    pub vault_x_token: Box<Account<'info, TokenAccount>>,

    #[account(mut, associated_token::mint = config.mint_y, associated_token::authority = pda_auth)]
    pub vault_y_token: Box<Account<'info, TokenAccount>>,

    #[account(mut, associated_token::mint = lp_mint, associated_token::authority = pda_auth)]
    pub lp_token: Box<Account<'info, TokenAccount>>,

    #[account(associated_token::mint = lp_mint, associated_token::authority = user)]
    pub user_lp_ata: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = config.mint_x,
        associated_token::authority = user
    )]
    pub user_token_x: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = config.mint_y,
        associated_token::authority = user
    )]
    pub user_token_y: Box<Account<'info, TokenAccount>>,

    #[account(mut, seeds = [b"pda_auth"], bump = config.auth_bump)]
    /// CHECK: this is just used for signing
    pub pda_auth: UncheckedAccount<'info>,

    #[account(mut, seeds = [b"lp", config.key().as_ref()], bump = config.lp_bump)]
    pub lp_mint: Box<Account<'info, Mint>>,

    #[account(mut, seeds = [b"config", config.seed.to_le_bytes().as_ref()], bump = config.config_bump)]
    pub config: Box<Account<'info, Config>>,

    pub token_program: Program<'info, Token>,
}

impl<'info> RemoveLiquidity<'info> {
    pub fn remove_liquidity(&self) -> Result<()> {
        unimplemented!();
    }

    pub fn get_token_amount(&self, amount: u64, minimum: u64) -> Result<u64> {
        let token_amount = amount
            .checked_mul(self.vault_x_token.amount)
            .unwrap()
            .checked_div(self.lp_mint.supply)
            .unwrap();

        if token_amount < minimum {
            return Err(AmmError::InsufficientTokenAmount.into());
        }

        Ok(token_amount)
    }

    pub fn burn_lp(&self, amount: u64) -> Result<()> {
        let cpi_accounts = Burn {
            mint: self.lp_mint.to_account_info(),
            from: self.user_lp_ata.to_account_info(),
            authority: self.user.to_account_info(),
        };
        let ctx = CpiContext::new(self.token_program.to_account_info(), cpi_accounts);
        burn(ctx, amount)
    }

    pub fn transfer_tokens(&self, token_x: bool, amount: u64) -> Result<()> {
        let cpi_accounts = match token_x {
            true => Transfer {
                from: self.vault_x_token.to_account_info(),
                to: self.user_token_x.to_account_info(),
                authority: self.pda_auth.to_account_info(),
            },
            false => Transfer {
                from: self.vault_y_token.to_account_info(),
                to: self.user_token_y.to_account_info(),
                authority: self.pda_auth.to_account_info(),
            },
        };

        let seeds = &[&b"pda_auth"[..], &[self.config.auth_bump]];
        let signer = &[&seeds[..]];

        let ctx =
            CpiContext::new_with_signer(self.token_program.to_account_info(), cpi_accounts, signer);
        token::transfer(ctx, amount)
    }
}
