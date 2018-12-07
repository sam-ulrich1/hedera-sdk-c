use super::errors::HederaResult;
use hedera::Client;
use libc::c_char;
use std::ffi::CStr;

#[no_mangle]
pub extern "C" fn hedera_client_new(address: *const c_char, out: *mut *mut Client) -> HederaResult {
    let address = unsafe { CStr::from_ptr(address) }.to_string_lossy();
    let client = try_ffi!(Client::new(address));

    unsafe {
        *out = Box::into_raw(Box::new(client));
    }

    HederaResult::Success
}

#[no_mangle]
pub unsafe extern "C" fn hedera_client_close(p: *mut Client) {
    let _ = Box::from_raw(p);
}
