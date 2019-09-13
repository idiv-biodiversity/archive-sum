#![allow(clippy::all)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::os::raw::c_uint;

/// Fixes for bindgen issues.
pub mod fix {
    /// Macro define, bindgen uses u32.
    pub const ARCHIVE_OK: i32 = 0;
    /// Macro define, bindgen uses u32.
    pub const ARCHIVE_EOF: i32 = 1;
}

pub const AE_IFMT: c_uint = 0o170000;
pub const AE_IFREG: c_uint = 0o100000;
pub const AE_IFLNK: c_uint = 0o120000;
pub const AE_IFSOCK: c_uint = 0o140000;
pub const AE_IFCHR: c_uint = 0o020000;
pub const AE_IFBLK: c_uint = 0o060000;
pub const AE_IFDIR: c_uint = 0o040000;
pub const AE_IFIFO: c_uint = 0o010000;
