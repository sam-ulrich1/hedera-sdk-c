use hedera::crypto::{PublicKey, SecretKey};
use libc::c_char;
use mbox::MString;

#[no_mangle]
pub extern "C" fn hedera_secret_key_generate() -> SecretKey {
    SecretKey::generate()
}

#[no_mangle]
pub extern "C" fn hedera_secret_key_to_str(p: *const SecretKey) -> *mut c_char {
    MString::from_str(&unsafe { &(*p) }.to_string())
        .into_mbox_with_sentinel()
        .into_raw() as _
}

#[no_mangle]
pub extern "C" fn hedera_public_key_from_secret_key(p: *const SecretKey) -> PublicKey {
    (unsafe { &*p }).public()
}

#[no_mangle]
pub extern "C" fn hedera_public_key_to_str(p: *const PublicKey) -> *mut c_char {
    MString::from_str(&unsafe { &(*p) }.to_string())
        .into_mbox_with_sentinel()
        .into_raw() as _
}
