use crate::message;

use super::diagnostics;
use anyhow::{anyhow, Result};
use clap::{App, Arg, ArgMatches};
use idl_gen::lang::*;
use message::Message;
use std::fs;
use std::{collections::HashMap, path::PathBuf};
use std::{io::Write, path::Path};

enum GenArgs {
    TargetLanguage(String),
}

pub fn create_command<'a>() -> App<'a> {
    App::new("analyze")
        .about("Analyze files")
        .args(&[Arg::new("input")
            .about("Package path")
            .short('i')
            .long("input")
            .takes_value(true)])
}

pub fn parse(matches: &ArgMatches) -> Result<()> {
    let input: PathBuf;

    let working_dir = std::env::current_dir().expect("working directory error");

    input = match matches.value_of("input") {
        Some(value) => {
            let path = Path::new(value);
            if path.is_relative() {
                working_dir.join(path)
            } else {
                path.to_path_buf()
            }
        }
        None => working_dir.clone(),
    };

    Message::info("Analyzing files...".to_owned())?;

    let mut module = crate::open_directory(&input)?;
    module.update()?;
    diagnostics::diagnostic(&module)?;

    Ok(())
}
