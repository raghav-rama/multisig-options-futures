use crate::contants::*;
use crate::models::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(params: ApproveTradeParams)]
pub struct ApproveTrade<'info> {
    #[account(
        mut,
        seeds = [
            TRADE_PROPOSAL_SEED,
            multisig_account.to_account_info().key.as_ref(),
        ],
        bump,
    )]
    pub trade_proposal: AccountLoader<'info, TradeProposal>,

    #[account(mut, seeds = [MULTISIG_SEED, approver.key().as_ref()], bump = multisig_account.load()?.bump_seed)]
    pub multisig_account: AccountLoader<'info, MultisigAccount>,

    #[account(mut)]
    pub approver: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct ApproveTradeParams {}

impl<'info> ApproveTrade<'info> {
    pub fn process(ctx: Context<Self>, _params: ApproveTradeParams) -> Result<()> {
        let mut trade_proposal = ctx.accounts.trade_proposal.load_mut()?;
        trade_proposal.approvals[0] = ctx.accounts.approver.key();
        Ok(())
    }
}
