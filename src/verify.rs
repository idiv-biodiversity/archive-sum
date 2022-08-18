use std::convert::TryInto;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::PathBuf;

use anyhow::Result;
use tar::Archive;

/// Runs verification.
///
/// # Errors
///
/// Errors when I/O errors happen.
pub fn run<Digest>(
    mut archive: Archive<impl Read>,
    source: &Option<PathBuf>,
    mut append: impl Write,
    mut out: impl Write,
    mut err: impl Write,
) -> Result<bool>
where
    Digest: digest::Digest + Write,
{
    let mut missing = 0;
    let mut failures = 0;

    for entry in archive.entries()? {
        let mut entry = entry?;

        if !entry.header().entry_type().is_file() {
            continue;
        }

        let mut hasher_archive = Digest::new();

        std::io::copy(&mut entry, &mut hasher_archive)?;

        let hash_archive = hasher_archive.finalize();
        let hash_archive: String = hash_archive
            .iter()
            .map(|byte| format!("{:02x}", byte))
            .collect();

        let path = entry.path()?;

        writeln!(append, "{}  {}", hash_archive, path.display())?;

        let source_file = if let Some(ref source) = source {
            source.join(path)
        } else {
            PathBuf::from(path)
        };

        if !source_file.exists() {
            writeln!(err, "{}: MISSING", source_file.display())?;
            missing += 1;
            continue;
        }

        let block_size: usize = if cfg!(unix) {
            use std::os::unix::fs::MetadataExt;
            let meta = fs::metadata(&source_file)?;
            let block_size = meta.blksize();
            block_size.try_into().unwrap_or(crate::DEFAULT_BLOCK_SIZE)
        } else {
            crate::DEFAULT_BLOCK_SIZE
        };

        let mut hasher_source = Digest::new();

        let mut source_file_f = File::open(&source_file)?;

        let mut buf = vec![0; block_size];

        loop {
            let nbytes = source_file_f.read(&mut buf)?;

            if nbytes > 0 {
                hasher_source.update(&buf[..nbytes]);
            } else {
                break;
            }
        }

        let hash_source = hasher_source.finalize();
        let hash_source: String = hash_source
            .iter()
            .map(|byte| format!("{:02x}", byte))
            .collect();

        if hash_archive == hash_source {
            writeln!(out, "{}: OK", source_file.display())?;
        } else {
            writeln!(err, "{}: FAILED", source_file.display())?;
            failures += 1;
        }
    }

    if missing > 0 {
        writeln!(err, "archive-sum: WARNING: {} MISSING file(s)", missing)?;
    }

    if failures > 0 {
        writeln!(err, "archive-sum: FATAL: {} FAILED checksum(s)", failures)?;
    }

    if failures == 0 && missing == 0 {
        Ok(true)
    } else {
        Ok(false)
    }
}

// ----------------------------------------------------------------------------
// tests
// ----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use std::fs::File;

    use assert_fs::prelude::*;
    use md5::Md5;
    use predicates::prelude::*;

    use super::*;

    #[test]
    fn ok() {
        let (temp, tarball) = crate::test::setup().unwrap();

        let archive = File::open(tarball).unwrap();
        let archive = Archive::new(archive);
        let source = Some(temp.path().to_path_buf());
        let mut append = Vec::new();
        let mut out = Vec::new();
        let mut err = Vec::new();

        let result =
            run::<Md5>(archive, &source, &mut append, &mut out, &mut err);

        assert!(result.unwrap());

        let append = std::str::from_utf8(&append).unwrap();
        let out = std::str::from_utf8(&out).unwrap();
        let err = std::str::from_utf8(&err).unwrap();

        assert_eq!(append.lines().count(), 3);
        assert_eq!(out.lines().count(), 3);
        assert_eq!(err.lines().count(), 0);

        assert!(predicate::str::contains("src/foo: OK").eval(out));
        assert!(predicate::str::contains("src/bar: OK").eval(out));
        assert!(predicate::str::contains("src/baz: OK").eval(out));

        assert!(predicate::str::contains(
            "d3b07384d113edec49eaa6238ad5ff00  src/foo"
        )
        .eval(append));

        assert!(predicate::str::contains(
            "c157a79031e1c40f85931829bc5fc552  src/bar"
        )
        .eval(append));

        assert!(predicate::str::contains(
            "258622b1688250cb619f3c9ccaefb7eb  src/baz"
        )
        .eval(append));

        temp.close().unwrap();
    }

    #[test]
    fn missing() {
        let (temp, tarball) = crate::test::setup().unwrap();

        std::fs::remove_file(temp.child("src").child("foo").path()).unwrap();

        let archive = File::open(tarball).unwrap();
        let archive = Archive::new(archive);
        let source = Some(temp.path().to_path_buf());
        let mut append = std::io::sink();
        let mut out = Vec::new();
        let mut err = Vec::new();

        let result =
            run::<Md5>(archive, &source, &mut append, &mut out, &mut err);

        assert!(!result.unwrap());

        let out = std::str::from_utf8(&out).unwrap();

        assert_eq!(out.lines().count(), 2);

        assert!(predicate::str::contains("src/bar: OK").eval(out));
        assert!(predicate::str::contains("src/baz: OK").eval(out));

        let err = std::str::from_utf8(&err).unwrap();

        assert_eq!(err.lines().count(), 2);

        assert!(predicates::str::contains("src/foo: MISSING").eval(err));
        assert!(predicates::str::contains("1 MISSING file(s)").eval(err));

        temp.close().unwrap();
    }

    #[test]
    fn failed() {
        let (temp, tarball) = crate::test::setup().unwrap();

        temp.child("src")
            .child("bar")
            .write_str("bar\nbar\n")
            .unwrap();

        let archive = File::open(tarball).unwrap();
        let archive = Archive::new(archive);
        let source = Some(temp.path().to_path_buf());
        let mut append = std::io::sink();
        let mut out = Vec::new();
        let mut err = Vec::new();

        let result =
            run::<Md5>(archive, &source, &mut append, &mut out, &mut err);

        assert!(!result.unwrap());

        let out = std::str::from_utf8(&out).unwrap();

        assert_eq!(out.lines().count(), 2);

        assert!(predicate::str::contains("src/foo: OK").eval(out));
        assert!(predicate::str::contains("src/baz: OK").eval(out));

        let err = std::str::from_utf8(&err).unwrap();

        assert_eq!(err.lines().count(), 2);
        assert!(predicate::str::contains("src/bar: FAILED").eval(err));
        assert!(predicate::str::contains("1 FAILED checksum(s)").eval(err));

        temp.close().unwrap();
    }
}