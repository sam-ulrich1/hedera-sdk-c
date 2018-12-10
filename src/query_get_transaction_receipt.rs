use hedera::{query::QueryGetTransactionReceipt, TransactionReceipt};
use super::transaction_id::CTransactionId;

def_query_new!(
    QueryGetTransactionReceipt:
        hedera_query__get_transaction_receipt__new(CTransactionId) -> TransactionReceipt
);

def_query_get!(hedera_query__get_transaction_receipt__get -> TransactionReceipt);
