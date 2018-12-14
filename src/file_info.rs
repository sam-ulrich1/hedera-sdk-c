use crate::{array::CArray, timestamp::CTimestamp};
use hedera::{FileId, FileInfo, PublicKey};
use std::convert::TryFrom;

#[repr(C)]
#[derive(Debug)]
pub struct CFileInfo {
    pub file_id: FileId,
    pub size: i64,
    pub expiration_time: CTimestamp,
    pub deleted: bool,
    pub keys: CArray<PublicKey>,
}

// everything will drop on its own

impl From<FileInfo> for CFileInfo {
    fn from(file_info: FileInfo) -> Self {
        CFileInfo {
            file_id: file_info.file_id,
            size: file_info.size,
            expiration_time: file_info.expiration_time.into(),
            deleted: file_info.deleted,
            keys: file_info.keys.into(),
        }
    }
}

impl TryFrom<CFileInfo> for FileInfo {
    type Error = failure::Error;

    fn try_from(c_file_info: CFileInfo) -> Result<Self, Self::Error> {
        let keys = unsafe {
            Vec::from_raw_parts(
                c_file_info.keys.ptr,
                c_file_info.keys.len,
                c_file_info.keys.len,
            )
            .into_iter()
            .map(Into::into)
            .collect::<Vec<PublicKey>>()
            .into()
        };

        Ok(FileInfo {
            file_id: c_file_info.file_id,
            size: c_file_info.size,
            expiration_time: c_file_info.expiration_time.into(),
            deleted: c_file_info.deleted,
            keys,
        })
    }
}
