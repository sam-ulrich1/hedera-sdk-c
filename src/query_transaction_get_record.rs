use super::transaction_id::CTransactionId;
use crate::transaction_record::CTransactionRecord;
use hedera::query::QueryTransactionGetRecord;

def_query_new!(
    QueryTransactionGetRecord:
        hedera_query__transaction_get_record__new(CTransactionId) -> TransactionRecord
);

def_query_get!(QueryTransactionGetRecord: hedera_query__transaction_get_record__get -> Box<CTransactionRecord>);
