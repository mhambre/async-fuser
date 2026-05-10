//! Response data implementation of [`crate::AsyncFilesystem::ioctl`] operation.

use std::io::IoSlice;

use zerocopy::IntoBytes;

use crate::ll::fuse_abi;
use crate::ll::reply::Response;

/// Response data from [`crate::AsyncFilesystem::ioctl`].
#[derive(Debug)]
pub struct IoctlResponse {
    out: fuse_abi::fuse_ioctl_out,
    data: Vec<u8>,
}

impl IoctlResponse {
    /// Creates a new [`IoctlResponse`].
    pub fn new(result: i32, data: Vec<u8>) -> Self {
        let out_iovs = if data.is_empty() { 0 } else { 1 };
        Self {
            out: fuse_abi::fuse_ioctl_out {
                result,
                flags: 0,
                in_iovs: 1,
                out_iovs,
            },
            data,
        }
    }
}

impl Response for IoctlResponse {
    fn payload(&self) -> impl crate::ll::ioslice_concat::IosliceConcat + Send {
        (
            [IoSlice::new(self.out.as_bytes())],
            Some([IoSlice::new(&self.data)]).filter(|_| !self.data.is_empty()),
        )
    }
}
