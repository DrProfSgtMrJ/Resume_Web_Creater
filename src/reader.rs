use std::fs::File;
use toml;
use std::io::Read;
use crate::APP_ARGS;

use super::resume::*;

pub struct ResumeReader;


impl ResumeReader {
    pub fn make() -> Resume {
        let mut file = File::open(&APP_ARGS.resume).expect("cannot fine resume file");
        let mut res = String::new();

        file.read_to_string(&mut res).expect("cannot read resume");
        let resume = toml::from_str(&res).expect("syntax error in resume file");

        //Self::validate(&resume);
        resume
    }

    // fn validate(resume: &Resume) {
    //     Self::validate_identifiers(resume);
    // }

    // fn validate_identifiers(resume: &Resume) {

    // }
}