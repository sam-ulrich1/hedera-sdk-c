use hedera::transaction::TransactionCryptoDelete;
use hedera::AccountId;

def_tx_new!(TransactionCryptoDelete: hedera_transaction__crypto_delete__new(AccountId));

def_tx_method!(
    TransactionCryptoDelete: hedera_transaction__crypto_delete__set_transfer_to(AccountId):
        transfer_to
);

// todo: to proto
