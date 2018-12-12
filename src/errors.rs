use failure::Error;
use libc::c_char;
use once_cell::{sync::Lazy, sync_lazy};
use parking_lot::Mutex;
use std::ptr::null_mut;

#[repr(u8)]
pub enum HederaResult {
    Success = 0,
    Failure = 1,
}

// The last error that occurred in this SDK.
pub(crate) static ERROR: Lazy<Mutex<Option<Error>>> = sync_lazy! {
    Mutex::new(None)
};

#[no_mangle]
pub extern "C" fn hedera_last_error() -> *mut c_char {
    match ERROR.lock().take() {
        Some(err) => Box::into_raw(err.to_string().into_boxed_str()) as _,
        None => null_mut(),
    }
}
