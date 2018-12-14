use hedera::{query::QueryCryptoGetAccountBalance, AccountId};

def_query!(QueryCryptoGetAccountBalance: crypto_get_account_balance(AccountId) -> u64);
