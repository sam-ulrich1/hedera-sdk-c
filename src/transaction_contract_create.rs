use hedera::PublicKey;
use hedera::transaction::TransactionContractCreate;
use hedera::AccountId;
use hedera::FileId;
use std::time::Duration;

def_tx_new!(TransactionContractCreate: hedera_transaction__contract_create__new);

def_tx_method!(TransactionContractCreate: hedera_transaction__contract_create__set_file(FileId): file);

def_tx_method!(
    TransactionContractCreate: hedera_transaction__contract_create__set_gas(i64): gas
);

def_tx_method!(
    TransactionContractCreate: hedera_transaction__contract_create__set_admin_key(PublicKey): admin_key
);

def_tx_method!(
    TransactionContractCreate: hedera_transaction__contract_create__set_initial_balance(i64): initial_balance
);

def_tx_method!(
    TransactionContractCreate: hedera_transaction__contract_create__set_proxy_account(AccountId): proxy_account
);

def_tx_method!(
    TransactionContractCreate: hedera_transaction__contract_create__set_proxy_fraction(i32): proxy_fraction
);

def_tx_method!(
    TransactionContractCreate: hedera_transaction__contract_create__set_auto_renew_period(Duration): auto_renew_period
);

def_tx_method!(
    TransactionContractCreate: hedera_transaction__contract_create__set_constructor_parameters(Vec<u8>): constructor_parameters
);

// todo: to proto
