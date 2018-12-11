use hedera::Claim;
use hedera::{AccountId, PublicKey};

use std::convert::{TryInto, TryFrom};
use failure::Error;

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct CClaim(
    pub(crate) AccountId,
    pub(crate) *const u8, // pointer to hash
    pub(crate) usize, // hash len
    pub(crate) *const PublicKey, // pointer to key list
    pub(crate) usize, // key list len
);

impl TryFrom<CClaim> for Claim {
    type Error = failure::Error;

    fn try_from(cclaim: CClaim) -> Result<Self, Error> {
        let hash = unsafe{
            Vec::from_raw_parts((cclaim.1).try_into()?, (cclaim.2).try_into()?, (cclaim.3).try_into())
        };

        let keys = unsafe{
            Vec::from_raw_parts((cclaim.4).try_into()?, (cclaim.5).try_into()?, (cclaim.6).try_into()?)
        };

        Ok(Claim{
            account: cclaim.0.try_into()?,
            hash,
            keys
        })
    }
}

impl TryFrom<Claim> for CClaim {
    type Error = failure::Error;

    fn try_from(claim: Claim) -> Result<Self, Error> {
        Ok(CClaim(
            claim.account,
            claim.hash.as_ptr(),
            claim.hash.capacity(),
            claim.hash.len(),
            claim.keys.as_ptr(),
            claim.keys.capacity(),
            claim.keys.len(),
        ))
    }
}
