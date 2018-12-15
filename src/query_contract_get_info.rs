use crate::contract_info::CContractInfo;
use hedera::{query::QueryContractGetInfo, ContractId};

def_query!(QueryContractGetInfo: contract_get_info(ContractId) -> CContractInfo);
