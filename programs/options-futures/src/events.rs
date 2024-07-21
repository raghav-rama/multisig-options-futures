use anchor_lang::prelude::*;

#[event]
pub struct Initialized {
    pub authority: Pubkey,
    pub data: u64,
}

#[event]
pub struct Updated {
    pub data: u64,
}
