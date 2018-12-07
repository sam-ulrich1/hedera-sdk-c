use crate::errors::HederaResult;
use hedera::query::Query;

#[no_mangle]
pub unsafe extern "C" fn hedera_query_cost(query: *mut Query<()>, out: *mut u64) -> HederaResult {
    *out = try_ffi!(Box::from_raw(query).cost());
    HederaResult::Success
}
