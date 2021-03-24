use super::diagnostics;
use crate::{get_all_idl_nodes, message};
use anyhow::{anyhow, Result};
use clap::{App, Arg, ArgMatches};
use idl_gen::lang::*;
use message::Message;
use std::path::Path;
use std::{fs, path::PathBuf};

enum GenArgs {
    TargetLanguage(String),
}

pub fn create_command<'a>() -> App<'a> {
    App::new("client").about("Generate client files").args(&[
        Arg::new("output")
            .about("Output path")
            .short('o')
            .long("output")
            .conflicts_with("path")
            .takes_value(true),
        Arg::new("input")
            .about("Idl input path")
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
            .long("server")
            .takes_value(true),
        Arg::new("client")
            .about("Client")
            .long("client")
            .default_value("Main")
            .takes_value(true),
        Arg::new("debug")
            .about("Debug mode")
            .long("debug")
            .takes_value(false)
            .conflicts_with_all(&["clean, no_build"]),
        Arg::new("no_build")
            .about("Skip building server files")
            .long("no-build")
            .takes_value(false)
            .conflicts_with_all(&["clean", "debug"]),
        Arg::new("only_build")
            .about("Only build the server without any client side code generation")
            .long("only-build")
            .takes_value(false)
            .conflicts_with_all(&["clean", "no_build"]),
        Arg::new("clean")
            .about("Remove all generated files")
            .long("clean")
            .conflicts_with("output")
            .takes_value(false)
            .conflicts_with("no_build"),
    ])
}

pub fn parse(matches: &ArgMatches) -> Result<()> {
    let input: PathBuf;
    let mut output: PathBuf;

    let working_dir = std::env::current_dir().expect("working directory error");

    if let Some(path) = matches.value_of("path") {
        let path = Path::new(path);
        let path = if path.is_relative() {
            working_dir.join(path)
        } else {
            path.to_path_buf()
        };

        output = path.join("build");
        input = path;
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
            None => working_dir.join("build"),
        };
    }

    if matches.is_present("clean") {
        let _ = fs::remove_dir_all(&output);
        return Ok(());
    }

    let client = matches.value_of("client").unwrap();
    let server = matches.value_of("server");
    let no_building = matches.is_present("no_build");
    let only_building = matches.is_present("only_build");
    let debug_mode = matches.is_present("debug");

    let mut module = crate::open_directory(&input)?;
    module.update()?;

    if diagnostics::diagnostic(&module)? {
        return Ok(());
    }

    let analyzer_i = &*module.ids_analyzer()?;
    let analyzer_ids = analyzer_i.as_ref().map_err(|err| anyhow!("{}", err))?;
    let package_name = analyzer_ids.get_package().name();

    if !matches.is_present("output") {
        output = output.join(&package_name);
    }

    let names = module.idl_documents_all_valid_names()?;
    let ref_names: Vec<&str> = names.iter().map(|v| v.as_str()).collect();
    let analyzers = module
        .idl_all_analyzers(&ref_names)
        .ok_or(anyhow::format_err!(""))?;

    if !analyzer_ids.has_client(client) {
        return Err(anyhow!("Client `{}` not defined", client));
    }

    let target_client = analyzer_ids.find_client(client).unwrap();

    let servers = target_client
        .servers(analyzer_ids)
        .ok_or(anyhow!("No server specified in client"))?;

    if servers.len() == 0 {
        return Err(anyhow!("At least one server must be specified"));
    }

    let target_server = server
        .and_then(|name| servers.iter().find(|v| v.ident == name))
        .unwrap_or_else(|| servers.first().unwrap());

    if !only_building {
        let target_lang = target_client.language().unwrap();

        let request = LanguageRequest {
            libraries: get_all_idl_nodes(&analyzers),
            ids_nodes: analyzer_ids.nodes.clone(),
            request_type: RequestType::Client(ClientType {
                client_name: target_client.ident.to_owned(),
                server_name: target_server.ident.to_owned(),
            }),
        };

        Message::info(format!("Sending language `{}` request", target_lang))?;
        let gen = idl_gen::for_language(&target_lang)?;
        let response = gen.send_request(request)?;

        Message::normal("Response message", response.response_messages)?;

        match response.gen_response {
            ResponseType::Generated(value) => {
                let _ = fs::remove_dir_all(&output);
                fs::create_dir_all(&output)?;

                for item in value {
                    item.write_items(&output, true)?;
                }

                Message::info(format!("Generated files at {:#?}", output))?;
            }
            ResponseType::Undefined(err) => {
                Message::error("Request error", err)?;
                return Err(anyhow!(""));
            }
        }
    } else {
        Message::warning("This only builds the server side code. This might occur in errors since client generated code was not updated.".to_owned())?;
    }

    if !no_building {
        let target_lang = target_server.language().unwrap();

        let server_request = LanguageRequest {
            libraries: get_all_idl_nodes(&analyzers),
            ids_nodes: analyzer_ids.nodes.clone(),
            request_type: RequestType::Server(ServerType {
                args: ServerArg::Build,
                build_type: if debug_mode {
                    BuildType::Debug
                } else {
                    BuildType::Release
                },
                server_name: target_server.ident.clone(),
                input_path: input.to_str().expect("path error").to_owned(),
            }),
        };

        Message::info(format!(
            "Sending language `{}` request for building",
            target_lang
        ))?;

        let gen = idl_gen::for_language(&target_lang)?;
        let response = gen.send_request(server_request)?;

        match response.gen_response {
            ResponseType::Generated(value) => {
                let src = output.join("build").join("idl");
                let _ = fs::remove_dir_all(&src); // Always remove the contents of `build` folder.
                fs::create_dir_all(&src)?;

                for item in value {
                    item.write_items(&src, true)?;
                }

                Message::info(format!("Generated build files at {:#?}", src))?;
            }
            ResponseType::Undefined(err) => {
                Message::error("Request error", err)?;
                return Err(anyhow!(""));
            }
        }
    }

    Ok(())
}
