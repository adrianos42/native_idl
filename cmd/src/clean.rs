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
        Arg::new("library")
            .about("Target library name")
            .short('l')
            .long("library")
            .takes_value(true),
        Arg::new("output")
            .about("Output path")
            .short('o')
            .default_value(".")
            .long("output")
            .takes_value(true),
        Arg::new("input")
            .about("Library path")
            .default_value("idl/")
            .short('i')
            .long("input")
            .takes_value(true),
    ])
}

pub fn parse(matches: &ArgMatches) -> Result<()> {
    let input = matches.value_of("input").unwrap();
    let output = matches.value_of("output").unwrap();
    let library = matches.value_of("library");

    let module = crate::open_directory(std::path::Path::new(input))?;

    let library_name = match library {
        Some(_) => {
            return Err(anyhow!("Cannot specify library name yet"));
        }
        None => {
            if module.library_size() != 1 {
                //return Err(anyhow!("Library must be specified"));
            }

            module.library_name().unwrap()
        }
    };

    let _ = fs::remove_dir_all(Path::new(output).join(&library_name));

    Ok(())
}
