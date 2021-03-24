use anyhow::{anyhow, Result};
use clap::{App, Arg, ArgMatches};
use idl::module::Module;
use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
};

pub fn create_command<'a>() -> App<'a> {
    App::new("format").about("Format idl files").arg(
        Arg::new("file")
            .short('f')
            .takes_value(true)
            .about("Specifies a file instead of the entire project"),
    )
}

pub fn parse(matches: &ArgMatches) -> Result<()> {
    let mut module = Module::new();

    // if let Some(ma) = matches.value_of("file") {
    //     let name = PathBuf::from(ma);
    //     if let Some(file_name) = name.file_name() {
    //         let idl_name = file_name.to_str().expect("Invalid str");
    //         if let Err(_) = module.add_document(idl_name) {
    //             return Ok(());
    //         }

    //         match fs::read_to_string(name.as_path()) {
    //             Ok(text) => {
    //                 module.update_parse(idl_name, &text);

    //                 let parse = module.get_parser(idl_name).unwrap();

    //                 match &*parse {
    //                     Ok(parse) => match format_document(parse) {
    //                         Some(text) => println!("{}", text),
    //                         None => {}
    //                     },
    //                     Err(_) => {}
    //                 }
    //             }
    //             Err(_) => return Ok(()),
    //         }
    //     }
    // }

    Ok(())
}
