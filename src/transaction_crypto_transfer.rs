use hedera::transaction::TransactionCryptoTransfer;
use hedera::AccountId;

def_tx_new!(TransactionCryptoTransfer: hedera_transaction__crypto_transfer__new);

def_tx_method!(
    TransactionCryptoTransfer: hedera_transaction__crypto_transfer__add_transfer(AccountId, i64):
        transfer
);
