use anchor_lang::prelude::*;

declare_id!("HjjypxqS89XAKLgXyBhxvVPkiXoxheEoGqtCVYQHMtKd");

#[program]
pub mod options_futures {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
