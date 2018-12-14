use hedera::{query::QueryContractGetBytecode, ContractId};
use crate::array::CArray;

def_query!(QueryContractGetBytecode: contract_get_bytecode(ContractId) -> CArray<u8>);
