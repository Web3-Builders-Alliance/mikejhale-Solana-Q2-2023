use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token, TokenAccount};
use std::collections::BTreeMap;

use crate::errors::AmmError;
use crate::state::config::Config;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        payer = payer,
        associated_token::mint = mint_x,
        associated_token::authority = pda_auth,
    )]
    pub vault_x_token: Box<Account<'info, TokenAccount>>,

    #[account(
        init,
        payer = payer,
        associated_token::mint = mint_y,
        associated_token::authority = pda_auth,
    )]
    pub vault_y_token: Box<Account<'info, TokenAccount>>,

    #[account(
        init,
        seeds = [b"lp", config.key.as_ref()],
        payer = payer,
        bump,
        mint::decimals = 6,
        mint::authority = pda_auth,
    )]
    pub lp_mint: Box<Account<'info, Mint>>,

    #[account(
        seeds = [b"pda_auth"],
        bump,
    )]
    /// CHECK: this is just used for signing
    pub pda_auth: UncheckedAccount<'info>,

    #[account(init,
        seeds = [b"config"],
        payer = payer,
        bump,
        space = 8 + Config::INIT_SPACE,
    )]
    pub config: Box<Account<'info, Config>>,

    pub mint_x: Account<'info, Mint>,
    pub mint_y: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn init(
        &mut self,
        bumps: &BTreeMap<String, u8>,
        seed: u64,
        admin: Option<Pubkey>,
    ) -> Result<()> {
        let (auth_bump, config_bump, lp_bump) = (
            *bumps.get("pda_auth").ok_or(AmmError::BumpError)?,
            *bumps.get("config").ok_or(AmmError::BumpError)?,
            *bumps.get("token_lp").ok_or(AmmError::BumpError)?,
        );

        self.config.init(
            admin,
            self.mint_x.key(),
            self.mint_y.key(),
            false,
            seed,
            config_bump,
            auth_bump,
            lp_bump,
        )
    }
}
