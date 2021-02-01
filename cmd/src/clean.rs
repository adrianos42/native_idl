use super::diagnostics;
use anyhow::{anyhow, Result};
use clap::{App, Arg, ArgMatches};
use idl_gen::lang::*;
use std::collections::HashMap;
use std::fs;
use std::{io::Write, path::Path};

enum GenArgs {
    TargetLanguage(String),
}

pub fn create_command<'a>() -> App<'a> {
    App::new("clean").about("Clean generated files").args(&[
        Arg::new("output")
            .about("Output path")
            .short('o')
            .default_value(".")
            .long("output")
            .takes_value(true),
        Arg::new("input")
            .about("Package path")
            .default_value("idl/")
            .short('i')
            .long("input")
            .takes_value(true),
    ])
}

pub fn parse(matches: &ArgMatches) -> Result<()> {
    let input = matches.value_of("input").unwrap();
    let output = matches.value_of("output").unwrap();

    let mut module = crate::open_directory(std::path::Path::new(input))?;
    module.update_ids_analyzer()?;
    let package_name = module.package_name()?;
    let _ = fs::remove_dir_all(Path::new(output).join(&package_name));

    Ok(())
}
