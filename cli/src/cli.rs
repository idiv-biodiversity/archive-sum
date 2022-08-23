use std::fs::{self, File};
use std::path::Path;

use atty::Stream;
use clap::{crate_description, crate_version};
use clap::{Arg, Command, PossibleValue, ValueEnum};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Digest {
    #[cfg(feature = "md5")]
    MD5,

    #[cfg(feature = "sha1")]
    SHA1,

    #[cfg(feature = "sha2")]
    SHA224,

    #[cfg(feature = "sha2")]
    SHA256,

    #[cfg(feature = "sha2")]
    SHA384,

    #[cfg(feature = "sha2")]
    SHA512,
}

impl ValueEnum for Digest {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            #[cfg(feature = "md5")]
            Self::MD5,
            #[cfg(feature = "sha1")]
            Self::SHA1,
            #[cfg(feature = "sha2")]
            Self::SHA224,
            #[cfg(feature = "sha2")]
            Self::SHA256,
            #[cfg(feature = "sha2")]
            Self::SHA384,
            #[cfg(feature = "sha2")]
            Self::SHA512,
        ]
    }

    fn to_possible_value<'a>(&self) -> Option<PossibleValue<'a>> {
        match self {
            #[cfg(feature = "md5")]
            Self::MD5 => Some(PossibleValue::new("md5")),
            #[cfg(feature = "sha1")]
            Self::SHA1 => Some(PossibleValue::new("sha1")),
            #[cfg(feature = "sha2")]
            Self::SHA224 => Some(PossibleValue::new("sha224")),
            #[cfg(feature = "sha2")]
            Self::SHA256 => Some(PossibleValue::new("sha256")),
            #[cfg(feature = "sha2")]
            Self::SHA384 => Some(PossibleValue::new("sha384")),
            #[cfg(feature = "sha2")]
            Self::SHA512 => Some(PossibleValue::new("sha512")),
        }
    }
}

/// Returns command-line parser.
pub fn build() -> Command<'static> {
    let append = Arg::with_name("append")
        .short('a')
        .long("append")
        .help("append hashes to `<file>`")
        .long_help("Append hashes to `<file>`.")
        .value_name("file")
        .validator(is_file);

    let archive = Arg::with_name("archive")
        .help("archive file")
        .long_help(
            "Input archive file. This argument is required only if STDIN is a \
             TTY."
        )
        .required(atty::is(Stream::Stdin))
        .validator(is_file);

    let check = Arg::with_name("check")
        .short('c')
        .long("check")
        .value_name("dir")
        .min_values(0)
        .max_values(1)
        .help("verify against source directory")
        .long_help(
            "Verify the input archive file against given source or current \
             working directory. You may need to use a `--` separator if you \
             do not specify a `<dir>` and this is the last argument, as in \
             `archive-sum -c -- archive.tar`, but not if this is not the last \
             argument, as in `archive-sum -c -a archive.tar.md5 archive.tar`.",
        )
        .validator(is_dir);

    let first_digest_variant = Digest::value_variants()
        .iter()
        .next()
        .expect("at least one digest feature should be required")
        .to_possible_value()
        .expect("there should be no skipped digest variants")
        .get_name();

    let digest = Arg::with_name("digest")
        .short('d')
        .long("digest")
        .help("digest algorithm")
        .long_help(
            "Use this digest algorithm. These algorithms are optional \
             dependencies/features that may be chosen during compilation.",
        )
        .takes_value(true)
        .default_value(first_digest_variant)
        .value_parser(clap::builder::EnumValueParser::<Digest>::new());

    let quiet = Arg::with_name("quiet")
        .long("quiet")
        .help("don't print 'OK' for each successfully verified file")
        .long_help(
            "Do not print 'OK' for each successfully verified file. Only \
             'MISSING' and 'FAILED' are shown.",
        )
        .display_order(1);

    let status = Arg::with_name("status")
        .long("status")
        .help("don't output anything, status code shows success")
        .long_help("Do not output anything, the status code shows success.")
        .display_order(1);

    Command::new("archive-sum")
        .version(crate_version!())
        .about(crate_description!())
        .arg(append)
        .arg(archive)
        .arg(check)
        .arg(digest)
        .arg(quiet)
        .arg(status)
        .mut_arg("help", |a| {
            a.short('?').help("print help").long_help("Print help.")
        })
        .mut_arg("version", |a| {
            a.help("print version").long_help("Print version.")
        })
}

// ----------------------------------------------------------------------------
// argument validator
// ----------------------------------------------------------------------------

fn is_dir(s: &str) -> Result<(), String> {
    let path = Path::new(&s);

    if !path.exists() {
        Err(format!("does not exist: {:?}", path))
    } else if !path.is_dir() {
        Err(format!("not a directory: {:?}", path))
    } else if let Err(error) = fs::read_dir(path) {
        Err(format!("{}", error))
    } else {
        Ok(())
    }
}

fn is_file(s: &str) -> Result<(), String> {
    let path = Path::new(&s);

    if !path.exists() {
        Err(format!("does not exist: {:?}", path))
    } else if !path.is_file() {
        Err(format!("not a file: {:?}", path))
    } else if let Err(error) = File::open(s) {
        Err(format!("{}", error))
    } else {
        Ok(())
    }
}
