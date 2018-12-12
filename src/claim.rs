use hedera::{Claim, AccountId, PublicKey };
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

impl Drop for CClaim {
    fn drop(&mut self) {
        unsafe {
            Box::from_raw(&mut self.1);
            Box::from_raw(&mut self.3);
        };
    }
}

impl TryFrom<Claim> for CClaim {
    type Error = failure::Error;

    fn try_from(claim: Claim) -> Result<Self, Error> {

        let hash_length = claim.hash.len();
        let keys_length = claim.keys.len();

        let hash = Box::into_raw(claim.hash.into_boxed_slice()) as _;
        let keys = Box::into_raw(claim.keys.into_boxed_slice()) as _;

        Ok(CClaim(
            claim.account.try_into()?,
            hash,
            hash_length,
            keys,
            keys_length,
        ))
    }
}
