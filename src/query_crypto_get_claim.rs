use hedera::{query::QueryCryptoGetClaim, AccountId, ContractInfo};
use crate::claim::CClaim;

def_query!(QueryCryptoGetClaim: crypto_get_claim(AccountId, Vec<u8>) -> CClaim);
