use crate::errors::HederaResult;
use crate::transaction_id::CTransactionId;
use hedera::transaction::Transaction;
use hedera::{AccountId, SecretKey};
use libc::c_char;
use std::ffi::CStr;

#[no_mangle]
pub unsafe extern "C" fn hedera_transaction_set_operator(
    tx: *mut Transaction<()>,
    operator: AccountId,
) {
    (&mut *tx).operator(operator);
}

#[no_mangle]
pub unsafe extern "C" fn hedera_transaction_set_node(tx: *mut Transaction<()>, node: AccountId) {
    (&mut *tx).node(node);
}

#[no_mangle]
pub unsafe extern "C" fn hedera_transaction_set_memo(
    tx: *mut Transaction<()>,
    memo: *const c_char,
) {
    (&mut *tx).memo(CStr::from_ptr(memo).to_string_lossy());
}

#[no_mangle]
pub unsafe extern "C" fn hedera_transaction_sign(
    tx: *mut Transaction<()>,
    secret: *const SecretKey,
) {
    (&mut *tx).sign(&*secret);
}

#[no_mangle]
pub unsafe extern "C" fn hedera_transaction_execute(
    tx: *mut Transaction<()>,
    out: *mut CTransactionId,
) -> HederaResult {
    *out = try_ffi!(Box::from_raw(tx).execute()).into();

    HederaResult::Success
}
