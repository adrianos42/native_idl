use super::diagnostics;
use anyhow::{anyhow, Result};
use clap::{App, Arg, ArgMatches};
use diagnostics::diagnostic_generic;
use idl_gen::lang::*;
use std::collections::HashMap;
use std::fs;
use std::{io::Write, path::Path};

enum GenArgs {
    TargetLanguage(String),
}

pub fn create_command<'a>() -> App<'a> {
    App::new("client").about("Generate client files").args(&[
        Arg::new("output")
            .about("Output path")
            .short('o')
            .default_value(".")
            .long("output")
            .takes_value(true),
        Arg::new("library")
            .about("Target library name")
            .short('l')
            .long("library")
            .takes_value(true),
        Arg::new("input")
            .about("Idl path")
            .default_value("idl/")
            .short('i')
            .long("input")
            .takes_value(true),
        Arg::new("server")
            .about("Server")
            .short('s')
            .long("server")
            .takes_value(true),
        Arg::new("client")
            .about("Client")
            .short('c')
            .long("client")
            .default_value("Main")
            .takes_value(true),
    ])
}

pub fn parse(matches: &ArgMatches) -> Result<()> {
    let input = matches.value_of("input").unwrap();
    let output = matches.value_of("output").unwrap();
    let client = matches.value_of("client").unwrap();

    let library = matches.value_of("library");
    let server = matches.value_of("server");

    //let mut layers = vec![];

    let module = crate::open_directory(std::path::Path::new(input))?;
    module.update_module()?;

    if diagnostics::diagnostic(&module)? {
        return Ok(());
    }

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

    let analyzer_r = &*module.get_idl_analyzer(&library_name).ok_or(anyhow!(""))?;
    let analyzer_idl = analyzer_r.as_ref().map_err(|err| anyhow!("{}", err))?;
    let analyzer_i = &*module.get_ids_analyzer(&library_name).ok_or(anyhow!(""))?;
    let analyzer_ids = analyzer_i.as_ref().map_err(|err| anyhow!("{}", err))?;

    if !analyzer_ids.has_client(client) {
        return Err(anyhow!("Client `{}` not defined", client));
    }

    let target_lang = analyzer_ids.find_client(client).unwrap().language().unwrap();

    let request = LanguageRequest {
        idl_nodes: analyzer_idl.nodes.clone(),
        ids_nodes: analyzer_ids.nodes.clone(),
        request_type: RequestType::Client(client.to_owned()),
    };

    println!("Sending language `{}` request", target_lang);
    let gen = idl_gen::for_language(&target_lang)?;
    let response = gen.send_request(request)?;

    match response.gen_response {
        ResponseType::Generated(value) => {
            let src = Path::new(output).join(&library_name);
            fs::create_dir_all(&src)?;

            for item in value {
                write_items(&item, &src)?;
            }

            println!("Generated files at {:#?}", src);
        }
        ResponseType::Undefined(err) => {
            diagnostic_generic("Request error", err.as_str())?;
            return Err(anyhow!(""));
        } 
    }

    Ok(())
}

fn write_items(storage: &StorageItem, path: &Path) -> Result<()> {
    match storage {
        idl_gen::lang::StorageItem::Source(source) => {
            let mut file = fs::OpenOptions::new()
                .write(true)
                .truncate(true)
                .create(true)
                .open(path.join(source.name.as_str()))?;
            file.write_all(source.txt.as_bytes())?;
        }
        idl_gen::lang::StorageItem::Folder(dir) => {
            let new_path = path.join(&dir.name);
            fs::create_dir_all(&new_path)?;

            for item in &dir.items {
                write_items(item, &new_path)?;
            }
        }
    }

    Ok(())
}
