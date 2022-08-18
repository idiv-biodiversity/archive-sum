use std::fs::{self, File};
use std::path::Path;

use atty::Stream;
use clap::{crate_description, crate_version};
use clap::{AppSettings, Arg, ArgMatches, Command, SubCommand};

pub fn args() -> ArgMatches {
    build().get_matches()
}

pub fn build() -> Command<'static> {
    // ------------------------------------------------------------------------
    // arguments
    // ------------------------------------------------------------------------

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
        .help_message("show this help output")
        .arg(&append)
        .arg(&archive);

    let verify = SubCommand::with_name("verify")
        .about("verify archive contents")
        .help_message("show this help output")
        .arg(&append)
        .arg(&archive)
        .arg(&quiet)
        .arg(&source)
        .arg(&status);

    // ------------------------------------------------------------------------
    // put it all together
    // ------------------------------------------------------------------------

    Command::new("archive-sum")
        .version(crate_version!())
        .about(crate_description!())
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(print)
        .subcommand(verify)
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
