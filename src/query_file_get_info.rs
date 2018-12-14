use hedera::{query::QueryFileGetInfo, FileId};
use crate::file_info::CFileInfo;

def_query!(QueryFileGetInfo: file_get_info(FileId) -> CFileInfo);
