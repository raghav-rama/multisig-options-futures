use anchor_lang::prelude::*;

use crate::{contants::*, events::Initialized, models::*};

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

impl Initialize<'_> {
    pub fn validate(&self, _ctx: &Context<Self>, _params: &InitializeParams) -> Result<()> {
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

#[derive(Accounts)]
#[instruction(params: InitializeMultisigParams)] // Pass bump and threshold as parameters
pub struct InitializeMultisig<'info> {
    #[account(
        init,
        payer = payer,
        space = 8 + std::mem::size_of::<MultisigAccount>(),
        seeds = [MULTISIG_SEED, payer.key().as_ref()],
        bump,
    )]
    pub multisig_account: AccountLoader<'info, MultisigAccount>,

    #[account(mut)]
    pub payer: Signer<'info>,
    pub signer1: Signer<'info>,
    pub signer2: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct InitializeMultisigParams {
    pub threshold: u64,
    pub signers: [Pubkey; MAX_SIGNERS],
}

impl InitializeMultisig<'_> {
    pub fn validate(&self, _ctx: &Context<Self>, _params: &InitializeMultisigParams) -> Result<()> {
        Ok(())
    }
    pub fn actuate(ctx: Context<Self>, params: InitializeMultisigParams) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        let mut multisig_account = ctx.accounts.multisig_account.load_init()?;
        multisig_account.authority = *ctx.accounts.payer.key;
        msg!(
            "Multisig account authority: {:?}",
            multisig_account.authority
        );
        multisig_account.threshold = params.threshold;
        msg!("Multisig account threshold: {:?}", params.threshold);
        multisig_account.bump_seed = ctx.bumps.multisig_account;
        msg!(
            "Multisig account bump seed: {:?}",
            multisig_account.bump_seed
        );
        multisig_account.signers = params.signers;
        msg!("Multisig account signers: {:?}", multisig_account.signers);
        multisig_account.available_funds = [AvailableFunds {
            symbol: [0; 32],
            amount: 0,
        }; MAX_FUNDS];
        multisig_account.multisig_pda = *ctx.accounts.multisig_account.to_account_info().key;
        msg!("Multisig account PDA: {:?}", multisig_account.multisig_pda);
        Ok(())
    }
}
