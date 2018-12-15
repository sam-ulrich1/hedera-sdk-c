use crate::file_info::CFileInfo;
use hedera::{query::QueryFileGetInfo, FileId};

def_query!(QueryFileGetInfo: file_get_info(FileId) -> CFileInfo);
