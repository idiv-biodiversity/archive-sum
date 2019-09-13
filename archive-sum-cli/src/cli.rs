use atty::Stream;
use clap::{arg_enum, crate_description, crate_version};
use clap::{App, AppSettings, Arg, SubCommand};
use std::error::Error;
use std::fs::{self, File};
use std::path::Path;

arg_enum! {
    #[derive(PartialEq, Debug)]
    pub enum Digest {
        MD5,
        SHA1,
        SHA224,
        SHA256,
        SHA384,
        SHA512,
    }
}

pub fn build() -> App<'static, 'static> {
    let color = if atty::is(Stream::Stdout) {
        AppSettings::ColoredHelp
    } else {
        AppSettings::ColorNever
    };

    // ------------------------------------------------------------------------
    // arguments
    // ------------------------------------------------------------------------

    let append = Arg::with_name("append")
        .short("a")
        .long("append")
        .help("append digests to file")
        .takes_value(true)
        .value_name("file")
        .validator(is_file);

    let archive = Arg::with_name("archive")
        .help("archive file")
        .required(atty::is(Stream::Stdin))
        .validator(is_file);

    let digest = Arg::with_name("digest")
        .short("d")
        .long("digest")
        .help("digest algorithm to use")
        .takes_value(true)
        .case_insensitive(true)
        .possible_values(&Digest::variants())
        .default_value("MD5");

    let quiet = Arg::with_name("quiet")
        .long("quiet")
        .help("don't print OK for each successfully verified file")
        .display_order(1);

    let status = Arg::with_name("status")
        .long("status")
        .help("don't output anything, status code shows success")
        .display_order(1);

    let source = Arg::with_name("source")
        .long("source")
        .help("source of the archive")
        .takes_value(true)
        .value_name("dir")
        .validator(is_dir);

    // ------------------------------------------------------------------------
    // commands
    // ------------------------------------------------------------------------

    let print = SubCommand::with_name("print")
        .about("print archive content checksums")
        .help_short("?")
        .help_message("show this help output")
        .arg(&append)
        .arg(&archive)
        .arg(&digest);

    let verify = SubCommand::with_name("verify")
        .about("verify archive contents")
        .help_short("?")
        .help_message("show this help output")
        .arg(&append)
        .arg(&archive)
        .arg(&digest)
        .arg(&quiet)
        .arg(&source)
        .arg(&status);

    // ------------------------------------------------------------------------
    // put it all together
    // ------------------------------------------------------------------------

    App::new("archive-sum")
        .version(crate_version!())
        .about(crate_description!())
        .global_setting(color)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::VersionlessSubcommands)
        .help_short("?")
        .help_message("show this help output")
        .version_message("show version")
        .subcommand(print)
        .subcommand(verify)
}

// ----------------------------------------------------------------------------
// argument validator
// ----------------------------------------------------------------------------

fn is_dir(s: String) -> Result<(), String> {
    let path = Path::new(&s);

    if !path.exists() {
        Err(format!("does not exist: {:?}", path))
    } else if !path.is_dir() {
        Err(format!("not a directory: {:?}", path))
    } else if let Err(error) = fs::read_dir(path) {
        Err(error.description().to_string())
    } else {
        Ok(())
    }
}

fn is_file(s: String) -> Result<(), String> {
    let path = Path::new(&s);

    if !path.exists() {
        Err(format!("does not exist: {:?}", path))
    } else if !path.is_file() {
        Err(format!("not a file: {:?}", path))
    } else if let Err(error) = File::open(s) {
        Err(error.description().to_string())
    } else {
        Ok(())
    }
}
