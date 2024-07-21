use anchor_lang::prelude::*;

declare_id!("HjjypxqS89XAKLgXyBhxvVPkiXoxheEoGqtCVYQHMtKd");

#[program]
pub mod options_futures {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        ctx.accounts.my_account.authority = *ctx.accounts.signer2.key;
        ctx.accounts.my_account.data = 0;
        emit!(Initialized {
            authority: ctx.accounts.my_account.authority,
            data: ctx.accounts.my_account.data,
        });
        Ok(())
    }

    pub fn update(ctx: Context<Update>, data: u64) -> Result<()> {
        msg!("Greetings from Update");
        assert_eq!(
            ctx.accounts.my_account.authority,
            *ctx.accounts.authority.key
        );
        ctx.accounts.my_account.data = data;
        emit!(Updated { data });
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = signer2, space = 8 + 8 + 32)]
    pub my_account: Account<'info, MyAccount>,
    #[account(mut)]
    pub signer1: Signer<'info>,
    #[account(mut)]
    pub signer2: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    pub my_account: Account<'info, MyAccount>,
    #[account(mut)]
    pub authority: Signer<'info>,
}

#[account]
pub struct MyAccount {
    pub authority: Pubkey,
    pub data: u64,
}

#[event]
pub struct Initialized {
    pub authority: Pubkey,
    pub data: u64,
}

#[event]
pub struct Updated {
    pub data: u64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized access")]
    Unauthorized,
}
