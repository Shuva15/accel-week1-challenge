pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("4Pv6KXpLeKR3A1ZTX8PkhYH4SYMLaEZyuiPqqtU3Xvjz");

#[program]
pub mod accel_week1_challenge {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }
}
