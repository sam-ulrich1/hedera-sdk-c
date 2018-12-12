use hedera::{query::QueryContractGetInfo, ContractId, ContractInfo};

def_query_new!(
    QueryContractGetInfo: hedera_query__contract_get_info__new(ContractId) -> ContractInfo
);

def_query_get!(QueryContractGetInfo: hedera_query__contract_get_info__get -> ContractInfo);
