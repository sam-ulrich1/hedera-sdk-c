use hedera::{query::QueryTransactionGetReceipt, TransactionReceipt, TransactionId};

def_query!(QueryTransactionGetReceipt: transaction_get_receipt(TransactionId) -> TransactionReceipt);
