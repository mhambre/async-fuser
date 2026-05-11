//! Response data implementation of [`crate::AsyncFilesystem::readdirplus`] operation to
//! send to the kernel

use std::ffi::OsStr;
use std::io::IoSlice;
use std::time::Duration;

use crate::FileAttr;
use crate::Generation;
use crate::INodeNo;
use crate::ll::ioslice_concat::IosliceConcat;
use crate::ll::reply::DirEntOffset;
use crate::ll::reply::DirEntPlusList;
use crate::ll::reply::DirEntryPlus;
use crate::ll::reply::Response;

/// Response data from [`crate::AsyncFilesystem::readdirplus`] operation
#[derive(Debug)]
pub struct DirectoryPlusResponse {
    data: DirEntPlusList,
}

impl DirectoryPlusResponse {
    /// Creates a new [`DirectoryPlusResponse`] with a specified buffer size.
    pub fn new(size: usize) -> DirectoryPlusResponse {
        DirectoryPlusResponse {
            data: DirEntPlusList::new(size),
        }
    }

    /// Add an entry to the directory reply buffer. Returns true if the buffer is full.
    #[must_use]
    pub fn add<T: AsRef<OsStr>>(
        &mut self,
        ino: INodeNo,
        offset: u64,
        name: T,
        ttl: &Duration,
        attr: &FileAttr,
        generation: Generation,
    ) -> bool {
        let name = name.as_ref();
        self.data.push(&DirEntryPlus::new(
            ino,
            generation,
            DirEntOffset(offset),
            name,
            *ttl,
            attr.into(),
            *ttl,
        ))
    }
}

impl Response for DirectoryPlusResponse {
    fn payload(&self) -> impl IosliceConcat {
        [IoSlice::new(self.data.as_bytes())]
    }
}
