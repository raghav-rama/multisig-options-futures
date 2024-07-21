use crate::{events::Updated, models::MyAccount};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    pub my_account: Account<'info, MyAccount>,
    #[account(mut)]
    pub authority: Signer<'info>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct UpdateParams {
    data: u64,
}
impl Update<'_> {
    pub fn validate(&self, _ctx: &Context<Self>, _params: &UpdateParams) -> Result<()> {
        Ok(())
    }
    pub fn actuate(ctx: Context<Self>, params: &UpdateParams) -> Result<()> {
        msg!("Greetings from Update");
        assert_eq!(
            ctx.accounts.my_account.authority,
            *ctx.accounts.authority.key
        );
        ctx.accounts.my_account.data = params.data;
        emit!(Updated { data: params.data });
        Ok(())
    }
}
