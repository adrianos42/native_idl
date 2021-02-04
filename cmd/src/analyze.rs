use crate::message;

use super::diagnostics;
use anyhow::{anyhow, Result};
use clap::{App, Arg, ArgMatches};
use idl_gen::lang::*;
use message::Message;
use std::collections::HashMap;
use std::fs;
use std::{io::Write, path::Path};

enum GenArgs {
    TargetLanguage(String),
}

pub fn create_command<'a>() -> App<'a> {
    App::new("analyze")
        .about("Analyze files")
        .args(&[Arg::new("input")
            .about("Package path")
            .default_value("idl/")
            .short('i')
            .long("input")
            .takes_value(true)])
}

pub fn parse(matches: &ArgMatches) -> Result<()> {
    let input = matches.value_of("input").unwrap();

    Message::normal("", vec!["Analyzing files...".to_owned()])?;

    let mut module = crate::open_directory(std::path::Path::new(input))?;
    module.update()?;
    diagnostics::diagnostic(&module)?;

    Ok(())
}
