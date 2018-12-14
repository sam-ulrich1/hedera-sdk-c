use hedera::{query::QueryGetByKey, PublicKey};
use crate::{array::CArray, entity::CEntity};

def_query!(QueryGetByKey: get_by_key(PublicKey) -> CArray<CEntity>);
