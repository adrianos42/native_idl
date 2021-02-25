use std::{
    fs,
    path::{Path, PathBuf},
};

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
                .long("output")
                .takes_value(true),
            Arg::new("input")
                .about("Idl path")
                .short('i')
                .long("input")
                .conflicts_with("path")
                .takes_value(true),
            Arg::new("path")
                .about("The target directory")
                .long("path")
                .takes_value(true)
                .conflicts_with_all(&["input", "output"]),
            Arg::new("server")
                .about("Server")
                .short('s')
                .long("server")
                .takes_value(true),
        ])
}

pub fn parse(matches: &ArgMatches) -> Result<()> {
    let server = matches.value_of("server");

    let input: PathBuf;
    let mut output: PathBuf;

    if let Some(path) = matches.value_of("path") {
        let path = std::path::Path::new(path);
        input = path.to_path_buf();
        output = path.to_path_buf();
    } else {
        input = match matches.value_of("input") {
            Some(value) => Path::new(value).to_path_buf(),
            None => Path::new(".").to_path_buf(),
        };
        output = match matches.value_of("output") {
            Some(value) => Path::new(value).to_path_buf(),
            None => Path::new(".").to_path_buf(),
        };
    }

    //let mut layers = vec![];

    let mut module = crate::open_directory(&input)?;
    module.update()?;

    if diagnostics::diagnostic(&module)? {
        return Ok(());
    }

    let package_name = module.package_name()?;
    let analyzer_i = &*module.ids_analyzer()?;
    let analyzer_ids = analyzer_i.as_ref().map_err(|err| anyhow!("{}", err))?;

    let server = match server {
        Some(server) => analyzer_ids
            .find_server(server)
            .ok_or_else(|| anyhow!("Undefined server `{}`", server))?,
        None => {
            let n_servers = analyzer_ids
                .nodes
                .iter()
                .filter_map(|v| match v {
                    idl::ids::ids_nodes::IdsNode::Server(value) => Some(value),
                    _ => None,
                })
                .count();

            if n_servers > 1 {
                return Err(anyhow!("More than one server or specified in package"));
            } else if n_servers == 0 {
                return Err(anyhow!("No server defined in package"));
            } else {
                analyzer_ids
                    .nodes
                    .iter()
                    .find_map(|v| match v {
                        idl::ids::ids_nodes::IdsNode::Server(value) => Some(value),
                        _ => None,
                    })
                    .unwrap()
            }
        }
    };

    let language = server
        .get_field("language")
        .ok_or_else(|| anyhow!("Missing languange field"))?
        .as_string_value()
        .ok_or_else(|| anyhow!(""))?;

    if !matches.is_present("output") {
        output = output.join(&language).join(&package_name);
    }

    let names = module.idl_documents_all_valid_names()?;
    let ref_names: Vec<&str> = names.iter().map(|v| v.as_str()).collect();
    let analyzers = module
        .idl_all_analyzers(&ref_names)
        .ok_or(anyhow::format_err!(""))?;

    let request = LanguageRequest {
        libraries: get_all_idl_nodes(&analyzers),
        ids_nodes: analyzer_ids.nodes.clone(),
        request_type: RequestType::Server(ServerType {
            server_name: server.ident.to_owned(),
            args: ServerArg::Generate,
        }),
    };

    let gen = idl_gen::for_language(&language)?;
    let response = gen.send_request(request)?;

    match response.gen_response {
        ResponseType::Generated(value) => {
            fs::create_dir_all(&output)?; // The language folder is never cleaned.

            for item in value {
                item.write_items(&output, false)?;
            }

            Message::info(format!("Generated files at {:#?}", output))?;
        }
        ResponseType::Undefined(err) => return Err(anyhow!("Response error `{}`", err)),
    }

    Ok(())
}
