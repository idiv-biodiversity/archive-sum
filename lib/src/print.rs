use std::io::{Read, Write};

use anyhow::Result;
use tar::Archive;

use crate::DynDigest;
use crate::DEFAULT_BLOCK_SIZE;

/// Prints hashes of entries in `archive` to `out`.
///
/// For file-typed entries in `archive` a hash is computed with the `hasher`
/// digest algorithm and written to `out` mimicking tools like `md5sum`.
///
/// # Errors
///
/// Returns `Err` if reading `archive` fails or if writing to `out` fails.
pub fn run(
    mut archive: Archive<impl Read>,
    hasher: &mut dyn DynDigest,
    mut out: impl Write,
) -> Result<()> {
    for entry in archive.entries()? {
        let mut entry = entry?;

        if !entry.header().entry_type().is_file() {
            continue;
        }

        let mut buf = vec![0; DEFAULT_BLOCK_SIZE];

        loop {
            let nbytes = entry.read(&mut buf)?;

            if nbytes > 0 {
                hasher.update(&buf[..nbytes]);
            } else {
                break;
            }
        }

        let hash = hasher.finalize_reset();
        let hash: String =
            hash.iter().map(|byte| format!("{:02x}", byte)).collect();

        writeln!(out, "{}  {}", hash, entry.path()?.display())?;
    }

    Ok(())
}

// ----------------------------------------------------------------------------
// tests
// ----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use std::fs::File;

    use digest::Digest;
    use predicates::prelude::*;

    use super::*;

    #[test]
    fn print() {
        let (temp, tarball) = crate::test::setup().unwrap();

        let archive = File::open(tarball).unwrap();
        let archive = Archive::new(archive);
        let mut result = Vec::new();
        let mut hasher = md5::Md5::new();

        run(archive, &mut hasher, &mut result).unwrap();

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
