use crate::array::CArray;
use hedera::{query::QueryFileGetContents, FileId};

def_query!(QueryFileGetContents: file_get_contents(FileId) -> CArray<u8>);
