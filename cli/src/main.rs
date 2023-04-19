#![deny(clippy::all)]
#![warn(clippy::pedantic, clippy::nursery, clippy::cargo)]

mod args;
mod cli;

use anyhow::Result;

use args::Arguments;
use clap::ValueEnum;

#[cfg(not(any(feature = "md5", feature = "sha1", feature = "sha2")))]
compile_error!("there must be at least one digest feature");

fn main() {
    let args = args::get();

    let result = if args.list_digests() {
        for digest in clap_digest::Digest::value_variants() {
            println!("{digest}");
        }

        Ok(())
    } else if args.verify() {
        match run_verify(&args) {
            Ok(true) => Ok(()),
            Ok(false) => std::process::exit(1),
            Err(e) => Err(e),
        }
    } else {
        run_print(&args)
    };

    if let Err(error) = result {
        eprintln!("archive-sum: error: {error}");
        std::process::exit(1)
    }
}

fn run_print(args: &Arguments) -> Result<()> {
    let archive = args.archive()?;
    let append = args.append_or_stdout()?;
    let mut digest = args.digest();

    archive_sum::print(archive, &mut (*digest), append)
}

fn run_verify(args: &Arguments) -> Result<bool> {
    let archive = args.archive()?;
    let source = args.verify_dir();
    let mut digest = args.digest();

    let append = args.append_or_sink()?;
    let out = args.verify_out();
    let err = args.verify_err();

    archive_sum::verify(archive, source, &mut (*digest), append, out, err)
}
