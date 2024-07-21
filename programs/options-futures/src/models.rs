use anchor_lang::prelude::*;

#[account]
pub struct MyAccount {
    pub authority: Pubkey,
    pub data: u64,
}
