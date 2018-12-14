use crate::account_info::CAccountInfo;
use hedera::{query::QueryCryptoGetInfo, AccountId};

def_query!(QueryCryptoGetInfo: crypto_get_info(AccountId) -> CAccountInfo);
