use hedera::{Claim, AccountId, PublicKey };
use std::convert::{TryInto, TryFrom};
use failure::Error;
use hedera::TransactionReceipt;
use crate::timestamp::CTimestamp;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct CTransactionRecord {
    pub(crate) receipt: TransactionReceipt,
    pub ( crate ) transaction_hash: * const u8, // pointer to hash
    pub ( crate ) transaction_hash_length: usize, // hash len
    pub ( crate ) consensus_timestamp: Option<CTimestamp>,
    pub ( crate ) memo: String,
    pub ( crate ) transaction_fee: u64,
    // record body?
}

// todo: impl drop
