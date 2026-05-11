//! Response data implementation of [`crate::AsyncFilesystem::lseek`] operation.

use crate::ll::ResponseStruct;
use crate::ll::ioslice_concat::IosliceConcat;
use crate::ll::reply::Response;

/// Response data from [`crate::AsyncFilesystem::lseek`] operation
#[derive(Debug)]
pub struct LseekResponse {
    offset: i64,
}

impl LseekResponse {
    /// Creates a new [`LseekResponse`] with the resulting offset.
    pub fn new(offset: i64) -> Self {
        Self { offset }
    }
}

impl Response for LseekResponse {
    fn payload(&self) -> impl IosliceConcat {
        ResponseStruct::new_lseek(self.offset)
    }
}
