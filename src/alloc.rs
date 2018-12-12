use std::ffi::c_void;

#[no_mangle]
pub extern "C" fn hedera_free(p: *mut c_void) {
    unsafe {
        let _ = Box::from_raw(p);
    }
}
