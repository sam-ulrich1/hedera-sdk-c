use hedera::{query::QueryTransactionGetReceipt, TransactionId, TransactionReceipt};

def_query!(
    QueryTransactionGetReceipt: transaction_get_receipt(TransactionId) -> TransactionReceipt
);
