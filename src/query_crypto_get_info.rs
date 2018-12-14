use hedera::{query::QueryCryptoGetInfo, AccountId};
use crate::account_info::CAccountInfo;

def_query!(QueryCryptoGetInfo: crypto_get_info(AccountId) -> CAccountInfo);
