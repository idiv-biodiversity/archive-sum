use libarchive_sys as ffi;
use std::convert::TryInto;
use std::ffi::{CStr, CString};
use std::fs;
use std::os::raw::c_char;
use std::path::Path;

use crate::Entry;
use crate::Error;
use crate::Result;

pub struct Archive {
    underlying: *mut ffi::archive,
    block_size: usize,
}

impl Archive {
    pub const DEFAULT_BLOCK_SIZE: usize = 65536;

    pub fn open<P: AsRef<Path>>(path: P) -> Result<Archive> {
        let path = path.as_ref();

        let file = path.to_string_lossy();
        let file = CString::new(file.as_bytes()).unwrap();

        let block_size: usize = if cfg!(unix) {
            use std::os::unix::fs::MetadataExt;
            let meta = fs::metadata(path)?;
            let block_size = meta.blksize();
            block_size.try_into().unwrap_or(Archive::DEFAULT_BLOCK_SIZE)
        } else {
            Archive::DEFAULT_BLOCK_SIZE
        };

        Archive::open_filename(file.as_ptr(), block_size)
    }

    pub fn stdin() -> Result<Archive> {
        Archive::open_filename(std::ptr::null(), Archive::DEFAULT_BLOCK_SIZE)
    }

    fn open_filename(
        path: *const c_char,
        block_size: usize,
    ) -> Result<Archive> {
        unsafe {
            let archive = ffi::archive_read_new();

            if archive.is_null() {
                return Err(Error::new("archive allocation error"));
            }

            ffi::archive_read_support_filter_all(archive);
            ffi::archive_read_support_format_all(archive);

            match ffi::archive_read_open_filename(archive, path, block_size) {
                ffi::fix::ARCHIVE_OK => {
                    let archive = Archive {
                        underlying: archive,
                        block_size,
                    };

                    Ok(archive)
                }

                _ => {
                    let msg = ffi::archive_error_string(archive);
                    let msg = CStr::from_ptr(msg);
                    let msg = msg.to_string_lossy();
                    Err(Error::new(&msg))
                }
            }
        }
    }

    pub fn block_size(&self) -> usize {
        self.block_size
    }

    pub fn entries(self) -> Entries {
        Entries::new(self)
    }
}

impl Drop for Archive {
    fn drop(&mut self) {
        unsafe {
            ffi::archive_read_free(self.underlying);
        }
    }
}

impl IntoIterator for Archive {
    type Item = Entry;
    type IntoIter = Entries;

    fn into_iter(self) -> Entries {
        self.entries()
    }
}

pub struct Entries {
    archive: Archive,
    current: *mut ffi::archive_entry,
}

impl Entries {
    fn new(archive: Archive) -> Entries {
        Entries {
            archive,
            current: std::ptr::null_mut(),
        }
    }
}

impl Iterator for Entries {
    type Item = Entry;

    fn next(&mut self) -> Option<Entry> {
        unsafe {
            let result = ffi::archive_read_next_header(
                self.archive.underlying,
                &mut self.current,
            );

            match result {
                0 => {
                    let entry = Entry {
                        archive: self.archive.underlying,
                        underlying: self.current,
                    };
                    Some(entry)
                }

                _ => None,
            }
        }
    }
}

// ----------------------------------------------------------------------------
// tests
// ----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use assert_cmd::prelude::*;
    use assert_fs::prelude::*;
    use std::process::Command;

    #[test]
    fn archive_read_entries() {
        let temp = assert_fs::TempDir::new().unwrap();

        let source = temp.child("src");
        source.create_dir_all().unwrap();

        source.child("foo").write_str("foo\n").unwrap();
        source.child("bar").write_str("bar\n").unwrap();
        source.child("baz").write_str("baz\n").unwrap();

        let tarball = temp.path().join("src.tar.gz");

        let mut cmd = Command::new("bsdtar");
        cmd.arg("-C").arg(temp.path());
        cmd.arg("-czf").arg(&tarball);
        cmd.arg("src");
        cmd.assert().success();

        let archive = Archive::open(&tarball).unwrap();
        let entries: Vec<String> =
            archive.entries().map(|entry| entry.path()).collect();

        assert_eq!(4, entries.len());
        assert!(entries.iter().any(|path| path == "src/"));
        assert!(entries.iter().any(|path| path == "src/foo"));
        assert!(entries.iter().any(|path| path == "src/bar"));
        assert!(entries.iter().any(|path| path == "src/baz"));

        temp.close().unwrap();
    }
}
