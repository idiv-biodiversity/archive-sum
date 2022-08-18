use std::error::Error;
use std::process::Command;

use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use predicates::prelude::*;

mod util;

#[test]
fn success() -> Result<(), Box<dyn Error>> {
    let (temp, tarball) = util::setup()?;

    let mut cmd = Command::cargo_bin("archive-sum")?;
    cmd.arg("verify");
    cmd.arg("--source").arg(temp.path());
    cmd.arg(&tarball);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("src/foo: OK"))
        .stdout(predicate::str::contains("src/bar: OK"))
        .stdout(predicate::str::contains("src/baz: OK"))
        .stderr(predicate::str::is_empty());

    temp.close()?;

    Ok(())
}

#[test]
fn failed() -> Result<(), Box<dyn Error>> {
    let (temp, tarball) = util::setup()?;

    temp.child("src").child("bar").write_str("bar\nbar\n")?;

    let mut cmd = Command::cargo_bin("archive-sum")?;
    cmd.arg("verify");
    cmd.arg("--source").arg(temp.path());
    cmd.arg(&tarball);

    cmd.assert()
        .failure()
        .stdout(predicate::str::contains("src/foo: OK"))
        .stderr(predicate::str::contains("src/bar: FAILED"))
        .stdout(predicate::str::contains("src/baz: OK"))
        .stderr(predicate::str::contains("FATAL: 1 FAILED checksum(s)"));

    temp.close()?;

    Ok(())
}

#[test]
fn missing() -> Result<(), Box<dyn Error>> {
    let (temp, tarball) = util::setup()?;

    std::fs::remove_file(temp.child("src").child("foo").path())?;

    let mut cmd = Command::cargo_bin("archive-sum")?;
    cmd.arg("verify");
    cmd.arg("--source").arg(temp.path());
    cmd.arg(&tarball);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("src/foo: MISSING"))
        .stdout(predicate::str::contains("src/bar: OK"))
        .stdout(predicate::str::contains("src/baz: OK"))
        .stderr(predicate::str::contains("WARNING: 1 MISSING file(s)"));

    temp.close()?;

    Ok(())
}

#[test]
fn quiet() -> Result<(), Box<dyn Error>> {
    let (temp, tarball) = util::setup()?;

    std::fs::remove_file(temp.child("src").child("foo").path())?;
    temp.child("src").child("bar").write_str("bar\nbar\n")?;

    let mut cmd = Command::cargo_bin("archive-sum")?;
    cmd.arg("verify");
    cmd.arg("--quiet");
    cmd.arg("--source").arg(temp.path());
    cmd.arg(&tarball);

    cmd.assert()
        .failure()
        .stdout(predicate::str::is_empty())
        .stderr(predicate::str::contains("src/foo: MISSING"))
        .stderr(predicate::str::contains("src/bar: FAILED"))
        .stderr(predicate::str::contains("WARNING: 1 MISSING file(s)"))
        .stderr(predicate::str::contains("FATAL: 1 FAILED checksum(s)"));

    temp.close()?;

    Ok(())
}

#[test]
fn status() -> Result<(), Box<dyn Error>> {
    let (temp, tarball) = util::setup()?;

    std::fs::remove_file(temp.child("src").child("foo").path())?;
    temp.child("src").child("bar").write_str("bar\nbar\n")?;

    let mut cmd = Command::cargo_bin("archive-sum")?;
    cmd.arg("verify");
    cmd.arg("--status");
    cmd.arg("--source").arg(temp.path());
    cmd.arg(&tarball);

    cmd.assert()
        .failure()
        .stdout(predicate::str::is_empty())
        .stderr(predicate::str::is_empty());

    temp.close()?;

    Ok(())
}
