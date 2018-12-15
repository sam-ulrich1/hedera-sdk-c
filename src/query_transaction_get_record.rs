use crate::transaction_record::CTransactionRecord;
use hedera::{query::QueryTransactionGetRecord, TransactionId};

def_query!(QueryTransactionGetRecord: transaction_get_record(TransactionId) -> CTransactionRecord);
