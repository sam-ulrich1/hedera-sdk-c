use hedera::{Claim, AccountId, PublicKey };
use std::convert::{TryInto, TryFrom};
use failure::Error;

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct CClaim {
    pub(crate) account_id: AccountId,
    pub ( crate ) hash: * const u8, // pointer to hash
    pub ( crate ) hash_length: usize, // hash len
    pub ( crate ) keys: * const PublicKey, // pointer to key list
    pub ( crate ) keys_length: usize, // key list len
}

impl Drop for CClaim {
    fn drop(&mut self) {
        unsafe {
            Box::from_raw(&mut self.hash);
            Box::from_raw(&mut self.keys);
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

        Ok(CClaim {
            account_id: claim.account.try_into()?,
            hash,
            hash_length,
            keys,
            keys_length,
        })
    }
}
