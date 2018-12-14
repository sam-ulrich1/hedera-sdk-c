use crate::{array::CArray, transaction_record::CTransactionRecord};
use hedera::{query::QueryContractGetRecords, ContractId, TransactionRecord};

def_query!(QueryContractGetRecords: contract_get_records(ContractId) -> CArray<CTransactionRecord>);
