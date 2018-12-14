use hedera::{query::QueryCryptoGetAccountBalance, AccountId};

def_query!(QueryCryptoGetAccountBalance: contract_get_records(AccountId) -> u64);
