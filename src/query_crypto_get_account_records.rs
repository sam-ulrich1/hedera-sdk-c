use hedera::{query::QueryCryptoGetAccountRecords, AccountId};
use crate::{ transaction_record::CTransactionRecord, array::CArray };

def_query!(QueryCryptoGetAccountRecords: crypto_get_account(AccountId) -> CArray<CTransactionRecord>);
