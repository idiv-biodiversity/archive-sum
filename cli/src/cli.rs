use std::fs::{self, File};
use std::path::PathBuf;

use clap::{crate_description, crate_version};
use clap::{Arg, ArgAction, Command};

/// Returns command-line parser.
pub fn build() -> Command {
    let append = Arg::new("append")
        .short('a')
        .long("append")
        .action(ArgAction::Set)
        .help("append hashes to `<file>`")
        .long_help(
            "Append hashes to `<file>`. For normal printing `archive-sum -a \
             sums archive` is equivalent to `archive-sum archive >> sums`, \
             for verification with `-c` hashes are written to `<file>` \
             additionally to the verification process.",
        )
        .value_name("file")
        .value_parser(is_file);

    let archive = Arg::new("archive")
        .action(ArgAction::Set)
        .help("archive file")
        .long_help("Input archive file.")
        .required_unless_present("list-digests")
        .value_parser(is_file);

    let check = Arg::new("check")
        .short('c')
        .long("check")
        .action(ArgAction::Set)
        .value_name("dir")
        .num_args(0..=1)
        .help("verify against source directory")
        .long_help(
            "Verify the input archive file against given source or current \
             working directory. You may need to use a `--` separator if you \
             do not specify a `<dir>` and this is the last argument, as in \
             `archive-sum -c -- archive.tar`, but not if this is not the last \
             argument, as in `archive-sum -c -a archive.tar.md5 archive.tar`.",
        )
        .value_parser(is_dir);

    let digest = clap_digest::arg::digest().default_value("MD5");

    let quiet = Arg::new("quiet")
        .long("quiet")
        .action(ArgAction::SetTrue)
        .help("don't print 'OK' for each successfully verified file")
        .long_help(
            "Do not print 'OK' for each successfully verified file. Only \
             'MISSING' and 'FAILED' are shown.",
        )
        .display_order(1);

    let status = Arg::new("status")
        .long("status")
        .action(ArgAction::SetTrue)
        .help("don't output anything, status code shows success")
        .long_help("Do not output anything, the status code shows success.")
        .display_order(1);

    let help = Arg::new("help")
        .short('?')
        .long("help")
        .help("print help (use --help to see all options)")
        .long_help("Print help.")
        .action(ArgAction::Help);

    let version = Arg::new("version")
        .long("version")
        .long_help("Print version.")
        .hide_short_help(true)
        .action(ArgAction::Version);

    Command::new("archive-sum")
        .version(crate_version!())
        .about(crate_description!())
        .arg(append)
        .arg(archive)
        .arg(check)
        .arg(digest)
        .arg(clap_digest::arg::list_digests())
        .arg(quiet)
        .arg(status)
        .disable_help_flag(true)
        .disable_version_flag(true)
        .arg(help)
        .arg(version)
}

// ----------------------------------------------------------------------------
// argument validator
// ----------------------------------------------------------------------------

fn is_dir(s: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(s);

    if !path.exists() {
        Err(format!("does not exist: {path:?}"))
    } else if !path.is_dir() {
        Err(format!("not a directory: {path:?}"))
    } else if let Err(error) = fs::read_dir(&path) {
        Err(format!("{error}"))
    } else {
        Ok(path)
    }
}

fn is_file(s: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(s);

    if !path.exists() {
        Err(format!("does not exist: {path:?}"))
    } else if !path.is_file() {
        Err(format!("not a file: {path:?}"))
    } else if let Err(error) = File::open(s) {
        Err(format!("{error}"))
    } else {
        Ok(path)
    }
}
