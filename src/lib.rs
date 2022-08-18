#![deny(clippy::all)]
#![warn(clippy::pedantic, clippy::nursery, clippy::cargo)]

mod print;
mod verify;

pub const DEFAULT_BLOCK_SIZE: usize = 65536;

pub use print::run as print;
pub use verify::run as verify;

#[cfg(test)]
mod test {
    use std::error::Error;
    use std::path::PathBuf;
    use std::process::Command;

    use assert_cmd::prelude::*;
    use assert_fs::prelude::*;
    use assert_fs::TempDir;

    pub fn setup() -> Result<(TempDir, PathBuf), Box<dyn Error>> {
        let temp = assert_fs::TempDir::new()?;

        let source = temp.child("src");
        source.create_dir_all().unwrap();

        source.child("foo").write_str("foo\n")?;
        source.child("bar").write_str("bar\n")?;
        source.child("baz").write_str("baz\n")?;

        let tarball = temp.path().join("src.tar");

        let mut cmd = Command::new("bsdtar");
        cmd.arg("-C").arg(temp.path());
        cmd.arg("-cf").arg(&tarball);
        cmd.arg("src");
        cmd.assert().success();

        let mut cmd = Command::new("md5sum");
        cmd.arg(&tarball);
        let cmd = cmd.assert().success();
        let output = cmd.get_output();
        temp.child("src.tar.md5").write_binary(&output.stdout)?;

        Ok((temp, tarball))
    }
}
