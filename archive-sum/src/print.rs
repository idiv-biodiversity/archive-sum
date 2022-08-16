use std::io::Write;

use anyhow::Result;
use libarchive::Archive;
use openssl::hash::{Hasher, MessageDigest};

/// Perform verification.
///
/// # Errors
///
/// I/O error.
pub fn run(
    archive: Archive,
    digest: MessageDigest,
    mut out: impl Write,
) -> Result<()> {
    for entry in archive {
        if !entry.is_file() {
            continue;
        }

        let mut hasher = Hasher::new(digest)?;

        for block in entry.blocks() {
            hasher.update(block?)?;
        }

        let hash = hasher.finish()?;
        let hash: String =
            hash.iter().map(|byte| format!("{:02x}", byte)).collect();

        writeln!(out, "{}  {}", hash, entry.path())?;
    }

    Ok(())
}

// ----------------------------------------------------------------------------
// tests
// ----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use predicates::prelude::*;

    #[test]
    fn print() {
        let (temp, tarball) = crate::test::setup().unwrap();

        let archive = Archive::open(&tarball).unwrap();
        let mut result = Vec::new();

        run(archive, MessageDigest::md5(), &mut result).unwrap();

        let result = std::str::from_utf8(&result).unwrap();

        assert_eq!(result.lines().count(), 3);

        assert!(predicate::str::contains(
            "d3b07384d113edec49eaa6238ad5ff00  src/foo"
        )
        .eval(result));

        assert!(predicate::str::contains(
            "c157a79031e1c40f85931829bc5fc552  src/bar"
        )
        .eval(result));

        assert!(predicate::str::contains(
            "258622b1688250cb619f3c9ccaefb7eb  src/baz"
        )
        .eval(result));

        temp.close().unwrap();
    }
}
