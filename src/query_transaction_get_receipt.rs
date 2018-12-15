use hedera::{query::QueryTransactionGetReceipt, TransactionReceipt};
use crate::transaction_id::CTransactionId;

def_query!(
    QueryTransactionGetReceipt: transaction_get_receipt(CTransactionId) -> TransactionReceipt
);
