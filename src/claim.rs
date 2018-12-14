use crate::array::CArray;
use hedera::{AccountId, Claim, PublicKey};
use std::{slice, convert::TryFrom};

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


impl From<CClaim> for Claim {
    fn from(c_claim: CClaim) -> Claim {

        let hash = unsafe {
            Vec::from(Box::from(slice::from_raw_parts(c_claim.hash.ptr, c_claim.hash.len)))
                .into_iter()
                .map(Into::into)
                .collect::<Vec<u8>>()
        };

        let keys: Vec<PublicKey> = unsafe {
            Vec::from(slice::from_raw_parts(c_claim.keys.ptr, c_claim.keys.len))
        };

        Claim{
            account: c_claim.account,
            hash,
            keys,
        }

    }

}

vec_to_carray!(Claim, CClaim);
