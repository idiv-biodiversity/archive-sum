use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

use anyhow::Result;
use atty::Stream;
use clap::{crate_description, crate_version};
use clap::{Arg, ArgMatches, Command};
use tar::Archive;

/// CLI arguments.
pub struct Arguments {
    archive: Option<String>,
    append: Option<String>,
    check: Option<PathBuf>,
    last_quiet: Option<usize>,
    last_status: Option<usize>,
}

impl TryFrom<ArgMatches> for Arguments {
    type Error = anyhow::Error;

    fn try_from(args: ArgMatches) -> Result<Self> {
        let archive = args.value_of("archive").map(ToOwned::to_owned);
        let append = args.value_of("append").map(ToOwned::to_owned);
        let check = args.value_of("check").map(PathBuf::from);

        let last_quiet = args.indices_of("quiet").and_then(Iterator::last);
        let last_status = args.indices_of("status").and_then(Iterator::last);

        Ok(Self {
            archive,
            append,
            check,
            last_quiet,
            last_status,
        })
    }
}

impl Arguments {
    pub const fn verify(&self) -> bool {
        self.check.is_some()
    }

    pub fn verify_dir(&self) -> Option<&Path> {
        self.check.as_deref()
    }

    pub fn archive(&self) -> Result<Archive<Box<dyn Read>>> {
        let source: Box<dyn Read> = match self.archive {
            Some(ref archive) => {
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

/// Returns parsed arguments.
pub fn args() -> Result<Arguments> {
    let cli = build();
    let args = cli.get_matches();
    let arguments = Arguments::try_from(args)?;

    Ok(arguments)
}

/// Returns command-line parser.
pub fn build() -> Command<'static> {
    let append = Arg::with_name("append")
        .short('a')
        .long("append")
        .help("append hashes to file")
        .takes_value(true)
        .value_name("file")
        .validator(is_file);

    let archive = Arg::with_name("archive")
        .help("archive file")
        .required(atty::is(Stream::Stdin))
        .validator(is_file);

    let check = Arg::with_name("check")
        .short('c')
        .long("check")
        .value_name("dir")
        .min_values(0)
        .max_values(1)
        .help("verify archive file against source directory")
        .validator(is_dir);

    let quiet = Arg::with_name("quiet")
        .long("quiet")
        .help("don't print OK for each successfully verified file")
        .display_order(1);

    let status = Arg::with_name("status")
        .long("status")
        .help("don't output anything, status code shows success")
        .display_order(1);

    Command::new("archive-sum")
        .version(crate_version!())
        .about(crate_description!())
        .arg(append)
        .arg(check)
        .arg(archive)
        .arg(quiet)
        .arg(status)
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
