use std::io::{Read, Write};

use anyhow::Result;
use tar::Archive;

pub fn run<Digest, R: Read>(
    mut archive: Archive<R>,
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
