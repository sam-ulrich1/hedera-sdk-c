use crate::{array::CArray, entity::CEntity};
use hedera::{query::QueryGetByKey, PublicKey};

def_query!(QueryGetByKey: get_by_key(PublicKey) -> CArray<CEntity>);
