use crate::array::CArray;
use hedera::{AccountId, Claim, PublicKey};

#[repr(C)]
#[derive(Debug)]
pub struct CClaim {
    pub account: AccountId,
    pub hash: CArray<u8>,
    pub keys: CArray<PublicKey>,
}

impl From<Claim> for CClaim {
    fn from(claim: Claim) -> Self {
        CClaim {
            account: claim.account,
            hash: claim.hash.into(),
            keys: claim.keys.into(),
        }
    }
}
