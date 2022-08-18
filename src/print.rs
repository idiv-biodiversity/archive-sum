use std::io::{Read, Write};

use anyhow::Result;
use tar::Archive;

/// Runs print.
///
/// # Errors
///
/// Errors when I/O errors happen.
pub fn run<Digest>(
    mut archive: Archive<impl Read>,
    mut out: impl Write,
) -> Result<()>
where
    Digest: digest::Digest + Write,
{
    for entry in archive.entries()? {
        let mut entry = entry?;

        if !entry.header().entry_type().is_file() {
            continue;
        }

        let mut hasher = Digest::new();

        std::io::copy(&mut entry, &mut hasher)?;

        let hash = hasher.finalize();
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

    use md5::Md5;
    use predicates::prelude::*;

    use super::*;

    #[test]
    fn print() {
        let (temp, tarball) = crate::test::setup().unwrap();

        let archive = File::open(tarball).unwrap();
        let archive = Archive::new(archive);
        let mut result = Vec::new();

        run::<Md5>(archive, &mut result).unwrap();

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
