use hedera::{query::Query, query::QueryContractGetBytecode, ContractId};

def_query_new!(
    QueryContractGetBytecode:
        hedera_query__contract_get_bytecode__new(ContractId) -> Vec<u8>
);

def_query_get!(hedera_query__contract_get_bytecode__get -> Vec<u8>);
