use hedera::{query::QueryTransactionGetRecord, TransactionRecord};
use super::transaction_id::CTransactionId;

def_query_new!(
    QueryTransactionGetRecord:
        hedera_query__transaction_get_record__new(CTransactionId) -> TransactionRecord
);

def_query_get!(hedera_query__transaction_get_record__get -> TransactionRecord);
