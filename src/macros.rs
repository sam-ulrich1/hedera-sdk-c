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
            unsafe{
                Box::into_raw((*p).to_string().into_boxed_str()) as _

            }
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
            let s = try_ffi!(unsafe { std::ffi::CStr::from_ptr(s) }.to_str());

            unsafe {
                *out = try_ffi!(s.parse());
            }

            crate::errors::HederaResult::Success
        }
    };
}

#[macro_export]
macro_rules! def_query_new {
    ($constructor:ident: $name:ident($ty:ty) -> $rty:ident) => {
        #[no_mangle]
        pub unsafe extern "C" fn $name(
            client: *mut hedera::Client,
            _1: $ty,
        ) -> *mut hedera::query::Query<$rty> {
            Box::into_raw(Box::new($constructor::new(&*client, _1.into())))
        }
    };
}

#[macro_export]
macro_rules! def_query_get {
    ($name:ident -> $ty:ident) => {
        #[no_mangle]
        pub unsafe extern "C" fn $name(
            query: *mut hedera::query::Query<$ty>,
            out: *mut $ty,
        ) -> crate::errors::HederaResult {
            *out = try_ffi!(Box::from_raw(query).get());

            crate::errors::HederaResult::Success
        }
    };
}

#[macro_export]
macro_rules! def_tx_new {
    ($constructor:ident: $name:ident) => {
        #[no_mangle]
        pub unsafe extern "C" fn $name(
            client: *mut hedera::Client,
        ) -> *mut hedera::transaction::Transaction<$constructor> {
            Box::into_raw(Box::new($constructor::new(&*client)))
        }
    };

    ($constructor:ident: $name:ident($ty:ty)) => {
        #[no_mangle]
        pub unsafe extern "C" fn $name(
            client: *mut hedera::Client,
            _1: $ty,
        ) -> *mut hedera::transaction::Transaction<$constructor> {
            Box::into_raw(Box::new($constructor::new(&*client, _1)))
        }
    };

    ($constructor:ident: $name:ident($p1:ty, $p2:ty)) => {
        #[no_mangle]
        pub unsafe extern "C" fn $name(
            client: *mut hedera::Client,
            _1: $p1,
            _2: $p2,
        ) -> *mut hedera::transaction::Transaction<$constructor> {
            Box::into_raw(Box::new($constructor::new(&*client, _1, _2)))
        }
    };
}

#[macro_export]
macro_rules! def_tx_method {
    ($constructor:ident: $name:ident($ty:ty): $member:ident) => {
        #[no_mangle]
        pub unsafe extern "C" fn $name(
            tx: *mut hedera::transaction::Transaction<$constructor>,
            _1: $ty,
        ) {
            (&mut *tx).$member(_1);
        }
    };

    ($constructor:ident: $name:ident($p1:ty, $p2:ty): $member:ident) => {
        #[no_mangle]
        pub unsafe extern "C" fn $name(
            tx: *mut hedera::transaction::Transaction<$constructor>,
            _1: $p1,
            _2: $p2,
        ) {
            (&mut *tx).$member(_1, _2);
        }
    };
}
