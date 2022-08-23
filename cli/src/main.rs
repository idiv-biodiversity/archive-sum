#![deny(clippy::all)]
#![warn(clippy::pedantic, clippy::nursery, clippy::cargo)]

mod args;
mod cli;

use anyhow::Result;

use args::Arguments;
use cli::Digest;

#[cfg(not(any(feature = "md5", feature = "sha1", feature = "sha2")))]
compile_error!("there must be at least one digest feature");

fn main() {
    let args = args::get();

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
}

fn run_print(args: &Arguments) -> Result<()> {
    let archive = args.archive()?;
    let append = args.append_or_stdout()?;

    // TODO this is stupid
    match args.digest() {
        #[cfg(feature = "md5")]
        Digest::MD5 => archive_sum::print::<md5::Md5>(archive, append),

        #[cfg(feature = "sha1")]
        Digest::SHA1 => archive_sum::print::<sha1::Sha1>(archive, append),

        #[cfg(feature = "sha2")]
        Digest::SHA224 => archive_sum::print::<sha2::Sha224>(archive, append),

        #[cfg(feature = "sha2")]
        Digest::SHA256 => archive_sum::print::<sha2::Sha256>(archive, append),

        #[cfg(feature = "sha2")]
        Digest::SHA384 => archive_sum::print::<sha2::Sha384>(archive, append),

        #[cfg(feature = "sha2")]
        Digest::SHA512 => archive_sum::print::<sha2::Sha512>(archive, append),
    }
}

fn run_verify(args: &Arguments) -> Result<bool> {
    let archive = args.archive()?;
    let source = args.verify_dir();
    let append = args.append_or_sink()?;
    let out = args.verify_out();
    let err = args.verify_err();

    // TODO this is stupid
    match args.digest() {
        #[cfg(feature = "md5")]
        Digest::MD5 => {
            archive_sum::verify::<md5::Md5>(archive, source, append, out, err)
        }

        #[cfg(feature = "sha1")]
        Digest::SHA1 => archive_sum::verify::<sha1::Sha1>(
            archive, source, append, out, err,
        ),

        #[cfg(feature = "sha2")]
        Digest::SHA224 => archive_sum::verify::<sha2::Sha224>(
            archive, source, append, out, err,
        ),

        #[cfg(feature = "sha2")]
        Digest::SHA256 => archive_sum::verify::<sha2::Sha256>(
            archive, source, append, out, err,
        ),

        #[cfg(feature = "sha2")]
        Digest::SHA384 => archive_sum::verify::<sha2::Sha384>(
            archive, source, append, out, err,
        ),

        #[cfg(feature = "sha2")]
        Digest::SHA512 => archive_sum::verify::<sha2::Sha512>(
            archive, source, append, out, err,
        ),
    }
}
