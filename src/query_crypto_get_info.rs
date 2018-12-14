use hedera::{query::QueryCryptoGetInfo, AccountId};
use contract_info::CContractInfo;

def_query!(QueryContractGetInfo: crypto_get_info(AccountId) -> CContractInfo);


