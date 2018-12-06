#[macro_export]
macro_rules! try_ffi {
    ($expr:expr) => {
        match $expr {
            Ok(expr) => expr,
            Err(err) => {
                crate::errors::ERROR
                    .lock()
                    .replace(failure::Error::from(err));

                return crate::errors::HederaResult::Failure;
            }
        }
    };
}

#[macro_export]
macro_rules! def_to_str {
    ($name:ident: $ty:ty) => {
        #[no_mangle]
        pub extern "C" fn $name(p: *mut $ty) -> *mut libc::c_char {
            mbox::MString::from_str(&unsafe { &(*p) }.to_string())
                .into_mbox_with_sentinel()
                .into_raw() as _
        }
    };
}

#[macro_export]
macro_rules! def_from_str {
    ($name:ident: $ty:ty) => {
        #[no_mangle]
        pub extern "C" fn $name(
            s: *const libc::c_char,
            out: *mut $ty,
        ) -> crate::errors::HederaResult {
            let s = try_ffi!(unsafe { std::ffi::CStr::from_ptr(s as _) }.to_str());

            unsafe {
                *out = try_ffi!(s.parse());
            }

            crate::errors::HederaResult::Success
        }
    };
}
