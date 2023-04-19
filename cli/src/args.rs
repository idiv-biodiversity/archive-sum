use std::fs::OpenOptions;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use anyhow::Result;
use archive_rs::Archive;
use archive_sum::DynDigest;
use clap::parser::ValueSource;
use clap::ArgMatches;
use clap_digest::Digest;

/// Returns parsed arguments.
pub fn get() -> Arguments {
    let cli = crate::cli::build();
    let args = cli.get_matches();
    Arguments::from(args)
}

/// CLI arguments.
#[derive(Debug)]
pub struct Arguments {
    archive: Option<PathBuf>,
    append: Option<PathBuf>,
    check: bool,
    check_source: Option<PathBuf>,
    digest: Option<Digest>,
    list_digests: bool,
    last_quiet: Option<usize>,
    last_status: Option<usize>,
}

impl From<ArgMatches> for Arguments {
    fn from(args: ArgMatches) -> Self {
        let archive =
            args.get_one::<PathBuf>("archive").map(ToOwned::to_owned);
        let append = args.get_one::<PathBuf>("append").map(ToOwned::to_owned);
        let check = args.contains_id("check");
        let check_source =
            args.get_one::<PathBuf>("check").map(ToOwned::to_owned);
        let digest = args.get_one::<Digest>("digest").copied();
        let list_digests = args.get_flag("list-digests");

        let last_quiet = args.value_source("quiet").and_then(|source| {
            if source == ValueSource::CommandLine {
                args.indices_of("quiet").and_then(Iterator::last)
            } else {
                None
            }
        });

        let last_status = args.value_source("status").and_then(|source| {
            if source == ValueSource::CommandLine {
                args.indices_of("status").and_then(Iterator::last)
            } else {
                None
            }
        });

        Self {
            archive,
            append,
            check,
            check_source,
            digest,
            list_digests,
            last_quiet,
            last_status,
        }
    }
}

impl Arguments {
    pub const fn list_digests(&self) -> bool {
        self.list_digests
    }

    pub const fn verify(&self) -> bool {
        self.check
    }

    pub fn digest(&self) -> Box<dyn DynDigest> {
        self.digest
            .expect("required unless list-digests present")
            .into()
    }

    pub fn verify_dir(&self) -> Option<&Path> {
        self.check_source.as_deref()
    }

    pub fn archive(&self) -> Result<Archive> {
        let archive = self.archive.as_ref().expect("required argument");
        let archive = Archive::open(archive)?;
        Ok(archive)
    }

    pub fn append(&self) -> Result<Option<Box<dyn Write>>> {
        let append: Option<Box<dyn Write>> = if let Some(file) = &self.append {
            let file = OpenOptions::new().append(true).open(file)?;
            Some(Box::new(file))
        } else {
            None
        };

        Ok(append)
    }

    pub fn append_or_sink(&self) -> Result<Box<dyn Write>> {
        self.append()
            .map(|o| o.unwrap_or_else(|| Box::new(io::sink())))
    }

    pub fn append_or_stdout(&self) -> Result<Box<dyn Write>> {
        self.append()
            .map(|o| o.unwrap_or_else(|| Box::new(io::stdout())))
    }

    pub fn verify_out(&self) -> Box<dyn Write> {
        if self.last_quiet.is_some() || self.last_status.is_some() {
            // /dev/null if quiet or status
            Box::new(std::io::sink())
        } else {
            // STDOUT otherwise
            Box::new(std::io::stdout())
        }
    }

    pub fn verify_err(&self) -> Box<dyn Write> {
        match (self.last_quiet, self.last_status) {
            (Some(quiet), Some(status)) if quiet > status => {
                // STDERR if quiet beats status
                Box::new(std::io::stderr())
            }

            // /dev/null if status
            (_, Some(_)) => Box::new(std::io::sink()),

            // STDERR otherwise
            (_, None) => Box::new(std::io::stderr()),
        }
    }
}
