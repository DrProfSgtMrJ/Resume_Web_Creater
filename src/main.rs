#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate log;

mod logger;
mod reader;
mod resume;

use crate::logger::ConfigLogger;
use crate::reader::ResumeReader;
use crate::resume::Resume;

use clap::Parser;
use log::LevelFilter;
use std::{ops::Deref, str::FromStr};

#[derive(Parser, Default, Debug)]
#[clap(
    author = "Jade Dever Matthews",
    version,
    about = "Generates a web page from a resume"
)]
struct AppArgs {
    #[clap(short, long)]
    #[clap(default_value = "./resume_example.toml")]
    /// Path to the resume config file
    resume_config: String,

    #[clap(short, long)]
    #[clap(default_value = "debug")]
    /// Level for log: off, error, warn, info, debug, trace
    log_level: String,
}

lazy_static! {
    static ref APP_ARGS: AppArgs = AppArgs::parse();
    static ref RESUME: Resume = ResumeReader::make();
}

fn ensure_states() {
    // Ensure all statics are valid (a `deref` is enough to lazily initialize them)
    let (_, _) = (APP_ARGS.deref(), RESUME.deref());
}

fn main() {
    // Initialize shared logger
    let lvl_filter = LevelFilter::from_str(&APP_ARGS.log_level).expect("invalid log level");
    let _logger = ConfigLogger::init(lvl_filter);

    info!("starting up");
    ensure_states();
    debug!("Read {:?}", APP_ARGS.resume_config);
    debug!("Resume Read for: {}", RESUME.name);
}
