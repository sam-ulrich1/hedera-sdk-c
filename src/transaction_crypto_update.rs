use hedera::transaction::TransactionCryptoUpdate;
use hedera::{PublicKey, AccountId};

def_tx_new!(TransactionCryptoUpdate: hedera_transaction__crypto_update__new(AccountId));

def_tx_method!(TransactionCryptoUpdate: hedera_transaction__crypto_update__set_key(PublicKey): key);

def_tx_method!(
    TransactionCryptoUpdate: hedera_transaction__crypto_update__set_proxy_account(AccountId):
        proxy_account
);

def_tx_method!(
    TransactionCryptoUpdate: hedera_transaction__crypto_update__set_proxy_fraction(i32):
        proxy_fraction
);

def_tx_method!(
    TransactionCryptoUpdate: hedera_transaction__crypto_update__set_send_record_threshold(u64):
        send_record_threshold
);

def_tx_method!(
    TransactionCryptoUpdate: hedera_transaction__crypto_update__set_receive_record_threshold(u64):
        receive_record_threshold
);

// todo: auto_renew_period
// todo: expiration_time
