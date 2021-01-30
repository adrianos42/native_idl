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
    App::new("analyze").about("Analyze files").args(&[
        Arg::new("library")
            .about("Target library name")
            .short('l')
            .long("library")
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
    let library = matches.value_of("library");
    
    println!("Analyzing files...");

    let module = crate::open_directory(std::path::Path::new(input))?;
    module.update_module()?;

    if !diagnostics::diagnostic(&module)? {
        println!("ok");
    } 
    
    Ok(())
}