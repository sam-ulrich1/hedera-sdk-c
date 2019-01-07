use super::errors::HederaResult;
use failure::{bail, Error};
use hedera::{AccountId, Client, SecretKey};
use libc::{c_char, c_int};
use std::ffi::CStr;
use std::mem::MaybeUninit;

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
pub unsafe extern "C" fn hedera_client_set_node(client: *mut Client, node: AccountId) {
    (&mut *client).set_node(node);
}

#[no_mangle]
pub unsafe extern "C" fn hedera_client_set_operator(
    client: *mut Client,
    operator: AccountId,
    secret: extern "C" fn(out: *mut SecretKey) -> c_int,
) {
    (&mut *client).set_operator(operator, move || -> Result<SecretKey, Error> {
        let mut key = MaybeUninit::<SecretKey>::uninitialized();

        let res = secret(key.as_mut_ptr());
        if res != 0 {
            bail!("failed to get the operator secret key (via callback given to [hedera_client_set_operator])");
        }

        Ok(key.into_inner())
    });
}

#[no_mangle]
pub unsafe extern "C" fn hedera_client_close(client: *mut Client) {
    let _ = Box::from_raw(client);
}
