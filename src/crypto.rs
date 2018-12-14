use super::errors::HederaResult;
use hedera::{PublicKey, SecretKey, Signature};
use std::slice;

#[no_mangle]
pub extern "C" fn hedera_secret_key_generate(
    password: *const libc::c_char,
    mnemonic_: *mut *mut libc::c_char,
) -> SecretKey {
    let password = unsafe { std::ffi::CStr::from_ptr(password) }.to_string_lossy();
    let (key, mnemonic) = SecretKey::generate(&password);

    unsafe {
        *mnemonic_ = mbox::MString::from_str(&mnemonic)
            .into_mbox_with_sentinel()
            .into_raw() as _;
    }

    key
}

def_to_str!(hedera_secret_key_to_str: SecretKey);
def_from_str!(hedera_secret_key_from_str: SecretKey);

//vec_to_carray!(SecretKey, SecretKey);

#[no_mangle]
pub extern "C" fn hedera_public_key_from_secret_key(p: *const SecretKey) -> PublicKey {
    (unsafe { &*p }).public()
}

def_to_str!(hedera_public_key_to_str: PublicKey);
def_from_str!(hedera_public_key_from_str: PublicKey);
//vec_to_carray!(PublicKey, PublicKey);

#[no_mangle]
pub extern "C" fn hedera_crypto_sign(
    p: *const SecretKey,
    message: *const u8,
    message_len: usize,
    out: *mut Signature,
) -> HederaResult {
    let message = unsafe { slice::from_raw_parts(message, message_len) };

    unsafe {
        *out = (*p).sign(message);
    }

    HederaResult::Success
}

#[no_mangle]
pub extern "C" fn hedera_crypto_verify(
    p: *const PublicKey,
    s: *mut Signature,
    message: *const u8,
    message_len: usize,
    out: *mut u8,
) -> HederaResult {
    let message = unsafe { slice::from_raw_parts(message, message_len) };

    unsafe {
        *out = try_ffi!((*p).verify(message, &(*s))) as u8;
    }

    HederaResult::Success
}

def_to_str!(hedera_signature_to_str: Signature);
def_from_str!(hedera_signature_from_str: Signature);
//vec_to_carray!(Signature, Signature);
