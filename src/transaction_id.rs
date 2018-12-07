use super::timestamp::CTimestamp;
use failure::Error;
use hedera::{AccountId, TransactionId};
use std::fmt;
use std::str::FromStr;

#[repr(C)]
#[derive(Debug)]
pub struct CTransactionId {
    account_id: AccountId,
    transaction_valid_start: CTimestamp,
}

impl From<TransactionId> for CTransactionId {
    #[inline]
    fn from(id: TransactionId) -> Self {
        Self {
            account_id: id.account_id,
            transaction_valid_start: id.transaction_valid_start.into(),
        }
    }
}

impl From<CTransactionId> for TransactionId {
    #[inline]
    fn from(id: CTransactionId) -> Self {
        Self {
            account_id: id.account_id,
            transaction_valid_start: id.transaction_valid_start.into(),
        }
    }
}

impl fmt::Display for CTransactionId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}@{}.{}",
            self.account_id, self.transaction_valid_start.0, self.transaction_valid_start.1,
        )
    }
}

impl FromStr for CTransactionId {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.parse::<TransactionId>()?.into())
    }
}

#[doc(hidden)]
#[no_mangle]
pub extern "C" fn hedera_transaction_id_new(account_id: AccountId) -> CTransactionId {
    TransactionId::new(account_id).into()
}

def_to_str!(hedera_transaction_id_to_str: CTransactionId);
def_from_str!(hedera_transaction_id_from_str: CTransactionId);
