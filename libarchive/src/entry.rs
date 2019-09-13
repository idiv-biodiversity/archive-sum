use libarchive_sys as ffi;
use std::ffi::CStr;
use std::io::Read;

use crate::Error;
use crate::Result;

#[derive(PartialEq)]
pub enum FileType {
    BlockDevice,
    CharacterDevice,
    Directory,
    Mount,
    NamedPipe,
    RegularFile,
    Socket,
    SymbolicLink,
}

pub struct Entry {
    pub(crate) archive: *mut ffi::archive,
    pub(crate) underlying: *mut ffi::archive_entry,
}

impl Entry {
    pub fn path(&self) -> String {
        unsafe {
            let path = ffi::archive_entry_pathname(self.underlying);
            let path = CStr::from_ptr(path);
            path.to_string_lossy().into_owned()
        }
    }

    pub fn file_type(&self) -> FileType {
        unsafe {
            match ffi::archive_entry_filetype(self.underlying) {
                ffi::AE_IFMT => FileType::Mount,
                ffi::AE_IFREG => FileType::RegularFile,
                ffi::AE_IFLNK => FileType::SymbolicLink,
                ffi::AE_IFSOCK => FileType::Socket,
                ffi::AE_IFCHR => FileType::CharacterDevice,
                ffi::AE_IFBLK => FileType::BlockDevice,
                ffi::AE_IFDIR => FileType::Directory,
                ffi::AE_IFIFO => FileType::NamedPipe,
                filetype => unreachable!("unknown file type: {}", filetype),
            }
        }
    }

    pub fn is_file(&self) -> bool {
        self.file_type() == FileType::RegularFile
    }

    pub fn blocks(&self) -> Blocks {
        Blocks { entry: self }
    }
}

impl Read for Entry {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let nbytes_read = unsafe {
            ffi::archive_read_data(
                self.archive,
                buf.as_mut_ptr() as *mut std::ffi::c_void,
                buf.len(),
            )
        };

        Ok(nbytes_read as usize)
    }
}

pub struct Blocks<'a> {
    entry: &'a Entry,
}

impl<'a> Iterator for Blocks<'a> {
    type Item = Result<&'a [u8]>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buf = std::ptr::null();
        let mut size = 0;
        let mut offset = 0;

        unsafe {
            let result = ffi::archive_read_data_block(
                self.entry.archive,
                &mut buf,
                &mut size,
                &mut offset,
            );

            match result {
                ffi::fix::ARCHIVE_EOF => None,

                ffi::fix::ARCHIVE_OK => {
                    let buf = buf as *const u8;
                    let buf = std::slice::from_raw_parts(buf, size);
                    Some(Ok(buf))
                }

                _ => {
                    let msg = ffi::archive_error_string(self.entry.archive);
                    let msg = CStr::from_ptr(msg);
                    let msg = msg.to_string_lossy();
                    Some(Err(Error::new(&msg)))
                }
            }
        }
    }
}
