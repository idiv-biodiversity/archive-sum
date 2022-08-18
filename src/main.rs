#![deny(clippy::all)]
#![warn(clippy::pedantic, clippy::nursery, clippy::cargo)]

mod cli;

use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::Path;

use anyhow::Result;
use clap::ArgMatches;
use md5::Md5;
use tar::Archive;

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

fn archive_from(args: &ArgMatches) -> Result<Archive<Box<dyn Read>>> {
    let source: Box<dyn Read> = match args.value_of("archive") {
        Some(archive) => {
            let file = File::open(archive)?;

            if archive.ends_with(".tar.gz") || archive.ends_with(".tgz") {
                // we have gzipped tarball
                Box::new(flate2::read::GzDecoder::new(file))
            } else {
                // we have plain tarball
                Box::new(file)
            }
        }

        // no argument -> use STDIN
        None => Box::new(io::stdin()),
    };

    Ok(Archive::new(source))
}

fn run_print(args: &ArgMatches) -> Result<()> {
    let archive = archive_from(args)?;

    let append = args
        .value_of("append")
        .map(|file| Path::new(file).to_path_buf());

    let append: Box<dyn Write> = if let Some(file) = append {
        Box::new(OpenOptions::new().append(true).open(file)?)
    } else {
        Box::new(std::io::stdout())
    };

    archive_sum::print::<Md5>(archive, append)
}

fn run_verify(args: &ArgMatches) -> Result<bool> {
    let archive = archive_from(args)?;

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

    archive_sum::verify::<Md5>(archive, &source, append, out, err)
}
