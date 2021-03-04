use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::{get_all_idl_nodes, message::Message};

use super::diagnostics;
use anyhow::{anyhow, Result};
use clap::{App, Arg, ArgMatches};
use idl_gen::lang::*;

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
            Arg::new("name")
                .about("Server name")
                .long("name")
                .takes_value(true),
        ])
        .subcommand(
            App::new("run").about("Compiles and run the server").args(&[
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
                Arg::new("name")
                    .about("Server name")
                    .long("name")
                    .takes_value(true),
                Arg::new("debug")
                    .about("Debug mode")
                    .long("debug")
                    .takes_value(false)
                    .conflicts_with_all(&["clean, no_build"]),
                // Arg::new("no_build")
                //     .about("Skip building server files before running")
                //     .long("no-build")
                //     .takes_value(false)
                //     .conflicts_with_all(&["clean", "debug"]),
                Arg::new("clean")
                    .about("Remove all generated files")
                    .long("clean")
                    .conflicts_with("output")
                    .takes_value(false)
                    .conflicts_with("no_build"),
            ]),
        )
}

pub fn parse(matches: &ArgMatches) -> Result<()> {
    match matches.subcommand() {
        Some(("run", value)) => run_server(value),
        _ => args_server(matches),
    }
}

fn get_paths(matches: &ArgMatches) -> (PathBuf, PathBuf) {
    let input: PathBuf;
    let output: PathBuf;

    let working_dir = std::env::current_dir().expect("working directory error");

    if let Some(path) = matches.value_of("path") {
        let path = Path::new(path);
        let path = if path.is_relative() {
            working_dir.join(path)
        } else {
            path.to_path_buf()
        };

        input = path.clone();
        output = path;
    } else {
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
        output = match matches.value_of("output") {
            Some(value) => {
                let path = Path::new(value);
                if path.is_relative() {
                    working_dir.join(path)
                } else {
                    path.to_path_buf()
                }
            }
            None => working_dir,
        };
    }

    (input, output)
}

fn args_server(matches: &ArgMatches) -> Result<()> {
    let server = matches.value_of("name");

    let (input, mut output) = get_paths(matches);

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
                return Err(anyhow!("More than one server were specified in package"));
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
            input_path: input.to_str().expect("path error").to_owned(),
            args: ServerArg::Generate,
            build_type: BuildType::Release,
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

fn run_server(matches: &ArgMatches) -> Result<()> {
    let server = matches.value_of("name");
    //let no_build = matches.is_present("no_build");
    let debug_mode = matches.is_present("debug");

    let (input, mut output) = get_paths(matches);

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
                return Err(anyhow!("More than one server were specified in package"));
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
            build_type: if debug_mode {
                BuildType::Debug
            } else {
                BuildType::Release
            },
            input_path: input.to_str().expect("path error").to_owned(),
            args: ServerArg::Run,
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
