use crate::array::CArray;
use hedera::{query::QueryContractGetBytecode, ContractId};

def_query!(QueryContractGetBytecode: contract_get_bytecode(ContractId) -> CArray<u8>);
