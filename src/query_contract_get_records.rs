use crate::{array::CArray, transaction_record::CTransactionRecord};
use hedera::{query::QueryContractGetRecords, ContractId};

def_query!(QueryContractGetRecords: contract_get_records(ContractId) -> CArray<CTransactionRecord>);
