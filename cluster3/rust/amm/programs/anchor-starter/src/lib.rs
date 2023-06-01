use anchor_lang::prelude::*;

mod contexts;
mod errors;
mod state;

use contexts::*;
use errors::AmmError;
use state::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod amm {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, seed: u64, admin: Option<Pubkey>) -> Result<()> {
        ctx.accounts.init(&ctx.bumps, seed, admin)
    }

    pub fn add_liquidity(
        ctx: Context<AddLiquidity>,
        token_x_amount: u64,
        min_liquidity: u64,
        token_y_max: u64,
        expiration: i64,
    ) -> Result<()> {
        ctx.accounts
            .add_liquidity(token_x_amount, min_liquidity, token_y_max, expiration)
    }

    pub fn remove_liquidity(ctx: Context<RemoveLiquidity>) -> Result<()> {
        unimplemented!()
    }
}
