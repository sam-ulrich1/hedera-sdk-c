use hedera::{query::Query, query::QueryContractGetBytecode, ContractId};

def_query_new!(
    QueryContractGetInfo:
        hedera_query__contract_get_info__new(ContractId) -> Vec<u8>
);

def_query_get!(hedera_query__contract_get_info__get -> Vec<u8>);
