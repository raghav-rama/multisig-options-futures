use anchor_lang::prelude::*;

use crate::{events::Initialized, models::MyAccount};

#[derive(Accounts)]
#[instruction(params: InitializeParams)]
pub struct Initialize<'info> {
    #[account(init, payer = signer2, space = 8 + 8 + 32)]
    pub my_account: Account<'info, MyAccount>,
    #[account(mut)]
    pub signer1: Signer<'info>,
    #[account(mut)]
    pub signer2: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct InitializeParams {}

impl<'info> Initialize<'_> {
    pub fn validate<'a>(&self, _ctx: &Context<Self>, _params: &InitializeParams) -> Result<()> {
        Ok(())
    }
    pub fn actuate(ctx: Context<Self>, _params: &InitializeParams) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        ctx.accounts.my_account.authority = *ctx.accounts.signer2.key;
        ctx.accounts.my_account.data = 0;
        emit!(Initialized {
            authority: ctx.accounts.my_account.authority,
            data: ctx.accounts.my_account.data,
        });
        Ok(())
    }
}
