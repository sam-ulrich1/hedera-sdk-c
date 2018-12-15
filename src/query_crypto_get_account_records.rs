use crate::{array::CArray, transaction_record::CTransactionRecord};
use hedera::{query::QueryCryptoGetAccountRecords, AccountId};

def_query!(
    QueryCryptoGetAccountRecords: crypto_get_account(AccountId) -> CArray<CTransactionRecord>
);
