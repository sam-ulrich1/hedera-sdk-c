use hedera::{Claim, AccountId, PublicKey };
use std::convert::{TryInto, TryFrom};
use failure::Error;
use hedera::TransactionReceipt;

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct CTransactionRecord {
    pub(crate) receipt: TransactionReceipt,
    pub ( crate ) transaction_hash: * const u8, // pointer to hash
    pub ( crate ) transaction_hash_length: usize, // hash len
    pub ( crate ) consensus_timestamp: Option<CTimeStamp>,
    pub ( crate ) memo: String,
    pub ( crate ) transaction_fee: u64,
    // record body?
}

// todo: impl drop
