//! Buffer wrappers implementing default so we can allocate the buffers with `Box::default()`
//! to avoid stack copies. Box::new() doesn't at the moment, and using a vec means we would lose
//! static length info.

use crate::deflate::core::{LZ_DICT_SIZE, MAX_MATCH_LEN};

/// Size of the buffer of lz77 encoded data.
pub const LZ_CODE_BUF_SIZE: usize = 64 * 1024;
/// Size of the output buffer.
pub const OUT_BUF_SIZE: usize = (LZ_CODE_BUF_SIZE * 13) / 10;
pub const LZ_DICT_FULL_SIZE: usize = LZ_DICT_SIZE + MAX_MATCH_LEN - 1 + 1;

pub struct HashBuffers {
    pub dict: [u8; LZ_DICT_FULL_SIZE],
    pub next: [u16; LZ_DICT_SIZE],
    pub hash: [u16; LZ_DICT_SIZE],
}

impl HashBuffers {
    #[inline]
    pub fn reset(&mut self) {
        *self = HashBuffers::default();
    }
}

impl Default for HashBuffers {
    fn default() -> HashBuffers {
        HashBuffers {
            dict: [0; LZ_DICT_FULL_SIZE],
            next: [0; LZ_DICT_SIZE],
            hash: [0; LZ_DICT_SIZE],
        }
    }
}

pub struct LocalBuf {
    pub b: [u8; OUT_BUF_SIZE],
}

impl Default for LocalBuf {
    fn default() -> LocalBuf {
        LocalBuf {
            b: [0; OUT_BUF_SIZE]
        }
    }
}
