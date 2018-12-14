use hedera::{ AccountId, ContractId, FileId, Entity};
use crate::claim::CClaim;
use std::mem::ManuallyDrop;

#[repr(C)]
pub union CEntity {
    pub account: AccountId,
    pub claim: ManuallyDrop<CClaim>,
    pub file: FileId,
    pub contract: ContractId,
}

impl From<Entity> for CEntity {
    fn from(entity: Entity) -> Self {
        match entity {
            Entity::Account(account_id) => CEntity{account: account_id},
            Entity::Claim(claim) => CEntity{claim: ManuallyDrop::new(claim.into())},
            Entity::File(file_id) => CEntity{file: file_id},
            Entity::Contract(contract_id) => CEntity{contract: contract_id},
        }
    }
}

vec_to_carray!(Entity, CEntity);
