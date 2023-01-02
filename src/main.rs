
#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate log;


use std::ops::Deref;
mod resume;
mod reader;
use clap::{Arg, Command};

use crate::resume::Resume;
use crate::reader::ResumeReader;


struct AppArgs {
    resume: String,
}

fn make_app_args() -> AppArgs {
    let matches = Command::new(clap::crate_name!())
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about(clap::crate_description!())
        .arg(
            Arg::new("resume")
                .short('r')
                .long("resume")
                .help("Path to resume file")
                .default_value("./resume.toml")
        )
        .get_matches();

    // Generate owned app arguments
    AppArgs {
        resume: String::from(matches.get_one::<String>("resume").expect("invalid resume value")),
    }
}

lazy_static! {
    static ref APP_ARGS: AppArgs = make_app_args();
    static ref RESUME: Resume = ResumeReader::make();
}

fn ensure_states() {
    // Ensure all statics are valid (a `deref` is enough to lazily initialize them)
    let (_, _) = (APP_ARGS.deref(), RESUME.deref());
}

fn main() {
    info!("starting up");
    ensure_states();
    println!("Hello, world!");
}
