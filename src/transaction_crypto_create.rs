use hedera::crypto::PublicKey;
use hedera::transaction::TransactionCryptoCreate;

def_tx_new!(TransactionCryptoCreate: hedera_transaction__crypto_create__new);

def_tx_method!(TransactionCryptoCreate: hedera_transaction__crypto_create__set_key(PublicKey): key);

def_tx_method!(
    TransactionCryptoCreate: hedera_transaction__crypto_create__set_initial_balance(u64):
        initial_balance
);

// todo: the rest
