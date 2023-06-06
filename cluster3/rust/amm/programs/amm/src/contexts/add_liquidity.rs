use crate::errors::AmmError;
use crate::state::config::Config;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::{self, AssociatedToken};
use anchor_spl::token::{self, mint_to, Mint, MintTo, Token, TokenAccount, Transfer};

#[derive(Accounts)]
pub struct AddLiquidity<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut, associated_token::mint = config.mint_x, associated_token::authority = pda_auth)]
    pub vault_x_token: Box<Account<'info, TokenAccount>>,

    #[account(mut, associated_token::mint = config.mint_y, associated_token::authority = pda_auth)]
    pub vault_y_token: Box<Account<'info, TokenAccount>>,

    #[account(mut, associated_token::mint = lp_mint, associated_token::authority = pda_auth)]
    pub lp_token: Box<Account<'info, TokenAccount>>,

    #[account(init_if_needed, payer = user, associated_token::mint = lp_mint, associated_token::authority = user)]
    pub user_lp_ata: Box<Account<'info, TokenAccount>>,

    #[account(mut,
        associated_token::mint = config.mint_x,associated_token::authority = user)]
    pub user_token_x: Box<Account<'info, TokenAccount>>,

    #[account(mut,
            associated_token::mint = config.mint_y,associated_token::authority = user)]
    pub user_token_y: Box<Account<'info, TokenAccount>>,

    #[account(mut, seeds = [b"pda_auth"], bump = config.auth_bump)]
    /// CHECK: this is just used for signing
    pub pda_auth: UncheckedAccount<'info>,

    #[account(mut, seeds = [b"lp", config.key().as_ref()], bump = config.lp_bump)]
    pub lp_mint: Box<Account<'info, Mint>>,

    #[account(mut, seeds = [b"config", config.seed.to_le_bytes().as_ref()], bump = config.config_bump)]
    pub config: Box<Account<'info, Config>>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> AddLiquidity<'info> {
    pub fn add_liquidity(
        &self,
        token_x_amount: u64,
        min_liquidity: u64,
        token_y_max: u64,
        expiration: i64,
    ) -> Result<()> {
        require!(self.config.frozen == false, AmmError::PoolFrozen);

        let current_time = Clock::get()?.unix_timestamp;
        require!(current_time > expiration, AmmError::AddLiquidityExpired);

        self.transfer_token(true, token_x_amount)?;
        let token_y_amount = self.get_token_y(token_y_max, token_x_amount)?;
        self.transfer_token(false, token_y_amount)?;
        self.mint_lp_tokens(token_x_amount, min_liquidity)
    }

    pub fn mint_lp_tokens(&self, token_x_amount: u64, min_liquidity: u64) -> Result<()> {
        let liquidity_supply = self.lp_mint.supply;
        let mut lp_to_mint = token_x_amount;
        if liquidity_supply != 0u64 {
            lp_to_mint = token_x_amount
                .checked_mul(liquidity_supply)
                .unwrap()
                .checked_div(self.vault_x_token.amount)
                .unwrap();
        }

        if lp_to_mint < min_liquidity {
            return Err(AmmError::LiquidityLessThanMinimum.into());
        }

        let seeds = &[&b"pda_auth"[..], &[self.config.auth_bump]];
        let signer = &[&seeds[..]];

        let cpi_mint = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            MintTo {
                mint: self.lp_mint.to_account_info(),
                to: self.user_lp_ata.to_account_info(),
                authority: self.pda_auth.to_account_info(),
            },
            signer,
        );

        mint_to(cpi_mint, lp_to_mint)
    }

    pub fn get_token_y(&self, token_y_max: u64, token_x_amount: u64) -> Result<u64> {
        if self.lp_mint.supply == 0u64 {
            return Ok(token_y_max);
        } else {
            Ok(token_x_amount
                .checked_mul(self.vault_x_token.amount)
                .unwrap()
                .checked_div(self.vault_y_token.amount)
                .unwrap()
                .checked_add(1u64)
                .unwrap())
        }
    }

    pub fn transfer_token(&self, token_x: bool, amount: u64) -> Result<()> {
        let cpi_accounts = match token_x {
            true => Transfer {
                from: self.user_token_x.to_account_info(),
                to: self.vault_x_token.to_account_info(),
                authority: self.user.to_account_info(),
            },
            false => Transfer {
                from: self.user_token_y.to_account_info(),
                to: self.vault_y_token.to_account_info(),
                authority: self.user.to_account_info(),
            },
        };

        let ctx = CpiContext::new(self.token_program.to_account_info(), cpi_accounts);
        token::transfer(ctx, amount)
    }
}
