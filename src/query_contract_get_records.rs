use hedera::{ query::QueryContractGetRecords, ContractId, TransactionRecord };
use crate::{ transaction_record::CTransactionRecord, array::CArray };

def_query!(QueryContractGetRecords: contract_get_records(ContractId) -> CArray<CTransactionRecord>);
