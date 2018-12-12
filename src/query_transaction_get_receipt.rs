use super::transaction_id::CTransactionId;
use hedera::{query::QueryTransactionGetReceipt, TransactionReceipt};

def_query_new!(
    QueryTransactionGetReceipt:
        hedera_query__get_transaction_receipt__new(CTransactionId) -> TransactionReceipt
);

def_query_get!(QueryTransactionGetReceipt: hedera_query__get_transaction_receipt__get -> TransactionReceipt);
