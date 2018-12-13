use hedera::{query::QueryContractGetInfo, ContractId};
use crate::contract_info::CContractInfo;

def_query_new!(
    QueryContractGetInfo: hedera_query__contract_get_info__new(ContractId) -> CContractInfo
);

def_query_get!(QueryContractGetInfo: hedera_query__contract_get_info__get -> CContractInfo);
