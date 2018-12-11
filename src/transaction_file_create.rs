use hedera::transaction::TransactionFileCreate;
use hedera::PublicKey;
use chrono::{DateTime, Utc};

def_tx_new!(TransactionFileCreate: hedera_transaction__file_create__new);

def_tx_method!(TransactionFileCreate: hedera_transaction__file_create__set_expires_at(DateTime<Utc>): expires_at);

def_tx_method!(TransactionFileCreate: hedera_transaction__file_create__set_key(PublicKey): key);

def_tx_method!(TransactionFileCreate: hedera_transaction__file_create__set_contents(Vec<u8>): contents);
