use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

use anyhow::Result;
use archive_sum::DynDigest;
use clap::ArgMatches;
use clap_digest::Digest;
use tar::Archive;

/// Returns parsed arguments.
pub fn get() -> Arguments {
    let cli = crate::cli::build();
    let args = cli.get_matches();
    Arguments::from(args)
}

/// CLI arguments.
pub struct Arguments {
    archive: Option<String>,
    append: Option<String>,
    check: bool,
    check_source: Option<PathBuf>,
    digest: Digest,
    last_quiet: Option<usize>,
    last_status: Option<usize>,
}

impl From<ArgMatches> for Arguments {
    fn from(args: ArgMatches) -> Self {
        let archive = args.value_of("archive").map(ToOwned::to_owned);
        let append = args.value_of("append").map(ToOwned::to_owned);
        let check = args.is_present("check");
        let check_source = args.value_of("check").map(PathBuf::from);
        let digest = *args
            .get_one::<Digest>("digest")
            .expect("digest should have default value");

        let last_quiet = args.indices_of("quiet").and_then(Iterator::last);
        let last_status = args.indices_of("status").and_then(Iterator::last);

        Self {
            archive,
            append,
            check,
            check_source,
            digest,
            last_quiet,
            last_status,
        }
    }
}

impl Arguments {
    pub const fn verify(&self) -> bool {
        self.check
    }

    pub fn digest(&self) -> Box<dyn DynDigest> {
        self.digest.into()
    }

    pub fn verify_dir(&self) -> Option<&Path> {
        self.check_source.as_deref()
    }

    pub fn archive(&self) -> Result<Archive<Box<dyn Read>>> {
        let source: Box<dyn Read> = match &self.archive {
            Some(archive) => {
                let archive = Path::new(archive);
                let file = File::open(archive)?;

                if archive.extension().map_or(false, |ext| {
                    ext.eq_ignore_ascii_case("gz")
                        || ext.eq_ignore_ascii_case("tgz")
                }) {
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
