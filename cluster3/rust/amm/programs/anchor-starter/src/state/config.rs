use crate::constants::*;
use anchor_lang::{prelude::*, solana_program::stake::config};

#[account]
#[derive(InitSpace)]
pub struct Config {
    pub admin: Option<Pubkey>,
    pub mint_x: Pubkey,
    pub mint_y: Pubkey,
    pub seed: u64,
    pub config_bump: u8,
    pub auth_bump: u8,
    pub lp_bump: u8,
    pub frozen: bool,
}

impl Config {
    pub fn init(
        &mut self,
        admin: Option<Pubkey>,
        mint_x: Pubkey,
        mint_y: Pubkey,
        frozen: bool,
        seed: u64,
        config_bump: u8,
        auth_bump: u8,
        lp_bump: u8,
    ) -> Result<()> {
        self.admin = admin;
        self.mint_x = mint_x;
        self.mint_y = mint_y;
        self.seed = seed;
        self.frozen = frozen;
        self.config_bump = config_bump;
        self.auth_bump = auth_bump;
        self.lp_bump = lp_bump;
        Ok(())
    }
}
