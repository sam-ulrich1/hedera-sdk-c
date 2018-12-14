use hedera::{query::QueryContractGetInfo, ContractId};
use crate::contract_info::CContractInfo;

def_query!(QueryContractGetInfo: contract_get_info(ContractId) -> CContractInfo);
