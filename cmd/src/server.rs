use std::{fs, path::Path};

use super::diagnostics;
use anyhow::{anyhow, Result};
use clap::{App, Arg, ArgMatches};
use idl_gen::lang::*;

enum GenArgs {
    TargetLanguage(String),
}

pub fn create_command<'a>() -> App<'a> {
    App::new("server")
        .about("Generate server files for implementation")
        .args(&[
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
                .default_value("Main")
                .takes_value(true),
        ])
}

pub fn parse(matches: &ArgMatches) -> Result<()> {
    let input = matches.value_of("input").unwrap();
    let output = matches.value_of("output").unwrap();
    let library = matches.value_of("library");
    let server = matches.value_of("server").unwrap();

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
    let analyzer = analyzer_r.as_ref().map_err(|_| anyhow!(""))?;

    let request = LanguageRequest {
        idl_nodes: analyzer.nodes.clone(),
        ids_nodes: vec![],
        request_type: RequestType::Server(ServerType {
            server_name: server.to_owned(),
            args: ServerArg::Generate,
        }),
    };

    let gen = idl_gen::for_language("rust")?;
    let response = gen.send_request(request)?;

    match response.gen_response {
        ResponseType::Generated(value) => {
            let src = Path::new(output).join("rust").join(&library_name);
            fs::create_dir_all(&src)?; // The language folder is never cleaned.

            for item in value {
                item.write_items(&src, false)?;
            }

            println!("Generated files at {:#?}", src);
        }
        ResponseType::Undefined(err) => return Err(anyhow!("Response error `{}`", err)),
    }

    Ok(())
}
