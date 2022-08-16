#![deny(clippy::all)]
#![warn(clippy::pedantic, clippy::nursery, clippy::cargo)]

mod cli;

use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

use anyhow::Result;
use clap::value_t;
use clap::ArgMatches;
use libarchive::Archive;
use openssl::hash::MessageDigest;

fn main() {
    let args = cli::args();

    let result = match args.subcommand() {
        Some(("print", args)) => run_print(args),
        Some(("verify", args)) => match run_verify(args) {
            Ok(true) => Ok(()),
            Ok(false) => std::process::exit(1),
            Err(e) => Err(e),
        },
        Some((cmd, _)) => unimplemented!("sub-command {}", cmd),
        None => unreachable!("sub-command is required"),
    };

    if let Err(error) = result {
        eprintln!("archive-sum: error: {}", error);
    }
}

fn run_print(args: &ArgMatches) -> Result<()> {
    let archive = match args.value_of("archive") {
        Some(archive) => Archive::open(archive)?,
        None => Archive::stdin()?,
    };

    let append = args
        .value_of("append")
        .map(|file| Path::new(file).to_path_buf());

    let digest = digest(args);

    if let Some(file) = append {
        let file = OpenOptions::new().append(true).open(file)?;
        archive_sum::print(archive, digest, file)
    } else {
        archive_sum::print(archive, digest, &mut std::io::stdout())
    }
}

fn run_verify(args: &ArgMatches) -> Result<bool> {
    let archive = match args.value_of("archive") {
        Some(archive) => Archive::open(archive)?,
        None => Archive::stdin()?,
    };

    let digest = digest(args);

    let source = args
        .value_of("source")
        .map(|dir| Path::new(dir).to_path_buf());

    let append: Box<dyn Write> = if let Some(file) = args.value_of("append") {
        Box::new(OpenOptions::new().append(true).open(file)?)
    } else {
        Box::new(std::io::sink())
    };

    let last_quiet = args.indices_of("quiet").map(Iterator::last);
    let last_status = args.indices_of("status").map(Iterator::last);

    let out: Box<dyn Write> =
        if args.is_present("quiet") || args.is_present("status") {
            Box::new(std::io::sink())
        } else {
            Box::new(std::io::stdout())
        };

    let err: Box<dyn Write> = match (last_quiet, last_status) {
        (Some(quiet), Some(status)) if quiet > status => {
            Box::new(std::io::stderr())
        }
        (_, Some(_)) => Box::new(std::io::sink()),
        (_, None) => Box::new(std::io::stderr()),
    };

    archive_sum::verify(archive, digest, &source, append, out, err)
}

fn digest(args: &ArgMatches) -> MessageDigest {
    let digest = value_t!(args.value_of("digest"), cli::Digest).unwrap();

    match digest {
        cli::Digest::MD5 => MessageDigest::md5(),
        cli::Digest::SHA1 => MessageDigest::sha1(),
        cli::Digest::SHA224 => MessageDigest::sha224(),
        cli::Digest::SHA256 => MessageDigest::sha256(),
        cli::Digest::SHA384 => MessageDigest::sha384(),
        cli::Digest::SHA512 => MessageDigest::sha512(),
    }
}
