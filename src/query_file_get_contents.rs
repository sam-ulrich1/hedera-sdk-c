use hedera::{query::QueryFileGetContents, FileId};
use crate::array::CArray;

def_query!(QueryFileGetContents: file_get_contents(FileId) -> CArray<u8>);
