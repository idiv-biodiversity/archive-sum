#![deny(clippy::all)]
#![warn(clippy::pedantic, clippy::nursery, clippy::cargo)]

mod cli;

use anyhow::Result;
use md5::Md5;

use cli::Arguments;

fn main() -> Result<()> {
    let args = cli::args()?;

    let result = if args.verify() {
        match run_verify(&args) {
            Ok(true) => Ok(()),
            Ok(false) => std::process::exit(1),
            Err(e) => Err(e),
        }
    } else {
        run_print(&args)
    };

    if let Err(error) = result {
        eprintln!("archive-sum: error: {}", error);
        std::process::exit(1)
    }

    Ok(())
}

fn run_print(args: &Arguments) -> Result<()> {
    let archive = args.archive()?;
    let append = args.append_or_stdout()?;

    archive_sum::print::<Md5>(archive, append)
}

fn run_verify(args: &Arguments) -> Result<bool> {
    let archive = args.archive()?;
    let source = args.verify_dir();
    let append = args.append_or_sink()?;
    let out = args.verify_out();
    let err = args.verify_err();

    archive_sum::verify::<Md5>(archive, source, append, out, err)
}
