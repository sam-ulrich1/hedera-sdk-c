use hedera::{query::QueryContractGetBytecode, ContractId};

def_query!(QueryContractGetBytecode: contract_get_bytecode(ContractId) -> Vec<u8>);
