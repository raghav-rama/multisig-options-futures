use anchor_lang::prelude::*;
pub mod actions;
pub mod contants;
pub mod errors;
pub mod events;
pub mod models;
pub use actions::*;

declare_id!("HjjypxqS89XAKLgXyBhxvVPkiXoxheEoGqtCVYQHMtKd");

#[program]
pub mod options_futures {

    use super::*;
    #[access_control(ctx.accounts.validate(&ctx, &params))]
    pub fn initialize(ctx: Context<Initialize>, params: InitializeParams) -> Result<()> {
        Initialize::actuate(ctx, &params)
    }

    #[access_control(ctx.accounts.validate(&ctx, &params))]
    pub fn update(ctx: Context<Update>, params: UpdateParams) -> Result<()> {
        Update::actuate(ctx, &params)
    }
}
