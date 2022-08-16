use std::error::Error;
use std::process::Command;

use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use predicates::prelude::*;

mod util;

#[test]
fn stdout() -> Result<(), Box<dyn Error>> {
    let (temp, tarball) = util::setup()?;

    let mut cmd = Command::cargo_bin("archive-sum")?;
    cmd.arg("print").arg(&tarball);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains(
            "258622b1688250cb619f3c9ccaefb7eb  src/baz",
        ))
        .stdout(predicate::str::contains(
            "c157a79031e1c40f85931829bc5fc552  src/bar",
        ))
        .stdout(predicate::str::contains(
            "d3b07384d113edec49eaa6238ad5ff00  src/foo",
        ))
        .stderr(predicate::str::is_empty());

    temp.close()?;

    Ok(())
}

#[test]
fn append() -> Result<(), Box<dyn Error>> {
    let (temp, tarball) = util::setup()?;

    let digest_file = temp.child("src.tar.gz.md5");

    let mut cmd = Command::cargo_bin("archive-sum")?;
    cmd.arg("print");
    cmd.arg("--append").arg(digest_file.path());
    cmd.arg(&tarball);

    cmd.assert()
        .success()
        .stdout(predicate::str::is_empty())
        .stderr(predicate::str::is_empty());

    digest_file
        .assert(predicate::str::contains("src.tar.gz"))
        .assert(predicate::str::contains(
            "258622b1688250cb619f3c9ccaefb7eb  src/baz",
        ))
        .assert(predicate::str::contains(
            "c157a79031e1c40f85931829bc5fc552  src/bar",
        ))
        .assert(predicate::str::contains(
            "d3b07384d113edec49eaa6238ad5ff00  src/foo",
        ));

    temp.close()?;

    Ok(())
}
