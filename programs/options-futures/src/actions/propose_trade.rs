use crate::contants::*;
use crate::models::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(params: ProposeTradeParams)]
pub struct ProposeTrade<'info> {
    #[account(
        init,
        payer = proposer,
        space = 8 + std::mem::size_of::<TradeProposal>(),
        seeds = [
            TRADE_PROPOSAL_SEED,
            multisig_account.to_account_info().key.as_ref(),
        ],
        bump,
    )]
    pub trade_proposal: AccountLoader<'info, TradeProposal>,

    #[account(mut, seeds = [MULTISIG_SEED, proposer.key().as_ref()], bump = multisig_account.load()?.bump_seed)]
    pub multisig_account: AccountLoader<'info, MultisigAccount>,

    #[account(mut)]
    pub proposer: Signer<'info>,
    pub signer1: Signer<'info>,
    pub signer2: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ProposeTradeParams {
    pub underlying_asset: Pubkey,
    pub trade_type: TradeType,
    pub strike_price: u64,
    pub expiration_unix_timestamp: i64,
    pub quantity: u64,
    pub premium_or_price: u64,
}

impl ProposeTrade<'_> {
    pub fn validate(&self, _ctx: &Context<Self>, _params: &ProposeTradeParams) -> Result<()> {
        Ok(())
    }
    pub fn actuate(ctx: Context<Self>, params: &ProposeTradeParams) -> Result<()> {
        msg!("Greetings from ProposeTrade");
        let trade_proposal = &mut ctx.accounts.trade_proposal.load_init()?;
        trade_proposal.bump = ctx.bumps.trade_proposal;
        msg!("Bump seed: {}", trade_proposal.bump);
        trade_proposal.multisig_account = *ctx.accounts.multisig_account.to_account_info().key;
        msg!("Multisig account: {:?}", trade_proposal.multisig_account);
        trade_proposal.proposer = *ctx.accounts.proposer.key;
        msg!("Proposer pubkey: {:?}", trade_proposal.proposer);
        trade_proposal.underlying_asset = params.underlying_asset;
        msg!("Underlying asset: {:?}", trade_proposal.underlying_asset);
        trade_proposal.trade_type = params.trade_type;
        msg!("Trade type: {:?}", trade_proposal.trade_type);
        trade_proposal.strike_price = params.strike_price;
        msg!("Strike price: {:?}", params.strike_price);
        trade_proposal.expiration_unix_timestamp = params.expiration_unix_timestamp;
        msg!(
            "Expiration timestamp: {:?}",
            params.expiration_unix_timestamp
        );
        trade_proposal.quantity = params.quantity;
        msg!("Quantity: {:?}", params.quantity);
        trade_proposal.premium_or_price = params.premium_or_price;
        msg!("Premium/price: {:?}", params.premium_or_price);
        trade_proposal.status = ProposalStatus::Proposed;
        msg!("Status: {:?}", trade_proposal.status);
        trade_proposal.approvals = [Pubkey::default(); MAX_SIGNERS];
        trade_proposal.approvals[0] = *ctx.accounts.proposer.key;
        Ok(())
    }
}
