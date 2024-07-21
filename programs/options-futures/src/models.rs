use crate::contants::{MAX_FUNDS, MAX_SIGNERS};
use anchor_lang::prelude::*;

#[account]
pub struct MyAccount {
    pub authority: Pubkey,
    pub data: u64,
}

#[account(zero_copy(unsafe))]
#[repr(packed)]
pub struct MultisigAccount {
    pub authority: Pubkey,
    pub multisig_pda: Pubkey,
    pub threshold: u64,
    pub signers: [Pubkey; MAX_SIGNERS],
    pub bump_seed: u8,
    pub reserved: [u8; 7],
    pub available_funds: [AvailableFunds; MAX_FUNDS],
}

#[repr(packed)]
#[zero_copy(unsafe)]
pub struct AvailableFunds {
    pub symbol: [u8; 32],
    pub amount: u64,
}

#[account(zero_copy(unsafe))]
#[repr(packed)]
pub struct TradeProposal {
    pub bump: u8,                         // Canonical bump for PDA
    pub multisig_account: Pubkey,         // PDA of the associated multisig account
    pub proposer: Pubkey,                 // Pubkey of the signer proposing the trade
    pub underlying_asset: Pubkey,         // Pubkey of the underlying asset (e.g., SOL, BTC)
    pub trade_type: TradeType,            // Enum for Call, Put, or Future
    pub strike_price: u64, // Strike price (for options) or agreed price (for futures)
    pub expiration_unix_timestamp: i64, // Unix timestamp for expiration (options only)
    pub quantity: u64,     // Quantity of the underlying asset
    pub premium_or_price: u64, // Premium (for options) or price per unit (for futures)
    pub status: ProposalStatus, // Enum for Proposed, Approved, Rejected, Executed, etc.
    pub approvals: [Pubkey; MAX_SIGNERS], // Signers who have approved (fixed-size array)
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum TradeType {
    Call = 0,
    Put = 1,
    Future = 2,
}

impl From<TradeType> for u8 {
    fn from(value: TradeType) -> Self {
        match value {
            TradeType::Call => 0,
            TradeType::Put => 1,
            TradeType::Future => 2,
        }
    }
}
impl From<u8> for TradeType {
    fn from(value: u8) -> Self {
        match value {
            0 => TradeType::Call,
            1 => TradeType::Put,
            2 => TradeType::Future,
            _ => panic!("Invalid TradeType"),
        }
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ProposalStatus {
    Proposed = 0,
    Approved = 1,
    Rejected = 2,
    Executed = 3,
    Settled = 4,
}

impl From<ProposalStatus> for u8 {
    fn from(value: ProposalStatus) -> Self {
        match value {
            ProposalStatus::Proposed => 0,
            ProposalStatus::Approved => 1,
            ProposalStatus::Rejected => 2,
            ProposalStatus::Executed => 3,
            ProposalStatus::Settled => 4,
        }
    }
}

impl From<u8> for ProposalStatus {
    fn from(value: u8) -> Self {
        match value {
            0 => ProposalStatus::Proposed,
            1 => ProposalStatus::Approved,
            2 => ProposalStatus::Rejected,
            3 => ProposalStatus::Executed,
            4 => ProposalStatus::Settled,
            _ => panic!("Invalid ProposalStatus"),
        }
    }
}
