use hedera::Claim;
use hedera::{AccountId, PublicKey};

#[no_mangle]
pub unsafe extern "C" fn hedera_claim_new(
    account: AccountId,
    hash: Vec<u8>
) -> Claim {
    Claim::new(account, hash)
}

#[no_mangle]
pub unsafe extern "C" fn hedera_claim_add_key(
    claim: *mut Claim,
    key: PublicKey
) {
    (&mut *claim).key(key);

}
