use hedera::Claim;
use hedera::{AccountId, PublicKey};

use std::convert::TryInto;

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct CClaim(pub(crate) AccountId, pub(crate) Vec<u8>, pub(crate) Vec<PublicKey>);

impl From<CClaim> for Claim {
    fn from(cclaim: CClaim) -> Self {
        Claim{
            account: cclaim.0.try_into().unwrap(),
            hash: (cclaim.1).try_into().unwrap(),
            keys: (cclaim.2).try_into().unwrap(),
        }
    }
}

impl From<Claim> for CClaim {
    fn from(claim: Claim) -> Self {
        CClaim(
            claim.account,
            claim.hash,
            claim.keys
        )
    }
}
