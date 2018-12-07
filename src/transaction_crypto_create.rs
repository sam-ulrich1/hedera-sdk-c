use hedera::crypto::PublicKey;
use hedera::transaction::{Transaction, TransactionCryptoCreate};
use hedera::Client;

#[no_mangle]
pub unsafe extern "C" fn hedera_transaction__crypto_create__new(
    client: *mut Client,
) -> *mut Transaction<TransactionCryptoCreate> {
    Box::into_raw(Box::new(TransactionCryptoCreate::new(&*client)))
}

#[no_mangle]
pub unsafe extern "C" fn hedera_transaction__crypto_create__set_key(
    tx: *mut Transaction<TransactionCryptoCreate>,
    public: PublicKey,
) {
    (&mut *tx).key(public);
}

#[no_mangle]
pub unsafe extern "C" fn hedera_transaction__crypto_create__set_initial_balance(
    tx: *mut Transaction<TransactionCryptoCreate>,
    balance: u64,
) {
    (&mut *tx).initial_balance(balance);
}
