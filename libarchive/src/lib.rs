#![deny(clippy::all)]
#![warn(clippy::pedantic, clippy::nursery, clippy::cargo)]

mod archive;
mod entry;

pub use archive::Archive;
pub use archive::Entries;
pub use entry::Blocks;
pub use entry::Entry;
pub use entry::FileType;
