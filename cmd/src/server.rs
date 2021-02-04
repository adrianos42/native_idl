use std::{fs, path::Path};

use crate::{get_all_idl_nodes, message::Message};

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
    let server = matches.value_of("server").unwrap();

    //let mut layers = vec![];

    let mut module = crate::open_directory(std::path::Path::new(input))?;
    module.update()?;

    if diagnostics::diagnostic(&module)? {
        return Ok(());
    }

    let package_name = module.package_name()?;
    let analyzer_i = &*module.ids_analyzer()?;
    let analyzer_ids = analyzer_i.as_ref().map_err(|err| anyhow!("{}", err))?;

    let names = module.idl_documents_all_valid_names()?;
    let ref_names: Vec<&str> = names.iter().map(|v| v.as_str()).collect();
    let analyzers = module
        .idl_all_analyzers(&ref_names)
        .ok_or(anyhow::format_err!(""))?;

    let request = LanguageRequest {
        libraries: get_all_idl_nodes(&analyzers),
        ids_nodes: analyzer_ids.nodes.clone(),
        request_type: RequestType::Server(ServerType {
            server_name: server.to_owned(),
            args: ServerArg::Generate,
        }),
    };

    let gen = idl_gen::for_language("rust")?;
    let response = gen.send_request(request)?;

    match response.gen_response {
        ResponseType::Generated(value) => {
            let src = Path::new(output).join("rust").join(&package_name);
            fs::create_dir_all(&src)?; // The language folder is never cleaned.

            for item in value {
                item.write_items(&src, false)?;
            }

            Message::info(format!("Generated files at {:#?}", src))?;
        }
        ResponseType::Undefined(err) => return Err(anyhow!("Response error `{}`", err)),
    }

    Ok(())
}
