use hedera::{query::QueryCryptoGetAccountBalance, AccountId};

def_query_new!(
    QueryCryptoGetAccountBalance: hedera_query__get_account_balance__new(AccountId) -> u64
);

def_query_get!(hedera_query__get_account_balance__get -> u64);
