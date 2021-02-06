use super::diagnostics;
use crate::{get_all_idl_nodes, message};
use anyhow::{anyhow, Result};
use clap::{App, Arg, ArgMatches};
use idl_gen::lang::*;
use message::Message;
use std::fs;
use std::path::Path;

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
        Arg::new("no_build")
            .about("Don't build server files")
            .long("no-build")
            .takes_value(false)
            .conflicts_with_all(&["clean"]),
        Arg::new("clean")
            .about("Remove all generated files")
            .long("clean")
            .takes_value(false)
            .conflicts_with("no_build"),
    ])
}

pub fn parse(matches: &ArgMatches) -> Result<()> {
    let input = matches.value_of("input").unwrap();
    let output = matches.value_of("output").unwrap();
    let client = matches.value_of("client").unwrap();

    let server = matches.value_of("server");

    let no_building = matches.is_present("no_build");

    //let mut layers = vec![];

    let mut module = crate::open_directory(std::path::Path::new(input))?;
    module.update()?;

    if diagnostics::diagnostic(&module)? {
        return Ok(());
    }

    let analyzer_i = &*module.ids_analyzer()?;
    let analyzer_ids = analyzer_i.as_ref().map_err(|err| anyhow!("{}", err))?;
    let package_name = analyzer_ids.get_package().name();

    if matches.is_present("clean") {
        let src = Path::new(output).join(&package_name);
        let _ = fs::remove_dir_all(&src);
        let build = Path::new(output).join("build").join(&package_name);
        let _ = fs::remove_dir_all(&build);
        return Ok(());
    }

    let names = module.idl_documents_all_valid_names()?;
    let ref_names: Vec<&str> = names.iter().map(|v| v.as_str()).collect(); // TODO Better way to do this?
    let analyzers = module
        .idl_all_analyzers(&ref_names)
        .ok_or(anyhow::format_err!(""))?;

    if !analyzer_ids.has_client(client) {
        return Err(anyhow!("Client `{}` not defined", client));
    }

    let target_client = analyzer_ids.find_client(client).unwrap();
    let target_lang = target_client.language().unwrap();

    let request = LanguageRequest {
        libraries: get_all_idl_nodes(&analyzers),
        ids_nodes: analyzer_ids.nodes.clone(),
        request_type: RequestType::Client(client.to_owned()),
    };

    // Message::info(format!("Sending language `{}` request", target_lang))?;
    // let gen = idl_gen::for_language(&target_lang)?;
    // let response = gen.send_request(request)?;

    // Message::normal("Response message", response.response_messages)?;

    // match response.gen_response {
    //     ResponseType::Generated(value) => {
    //         let src = Path::new(output).join(&package_name);
    //         let _ = fs::remove_dir_all(&src);
    //         fs::create_dir_all(&src)?;

    //         for item in value {
    //             item.write_items(&src, true)?;
    //         }

    //         Message::info(format!("Generated files at {:#?}", src))?;
    //     }
    //     ResponseType::Undefined(err) => {
    //         Message::error("Request error", err)?;
    //         return Err(anyhow!(""));
    //     }
    // }

    match target_client.servers(&analyzer_ids) {
        Some(servers) => {
            let target_server =
                match server.and_then(|name| servers.iter().find(|v| v.ident == name)) {
                    Some(&value) => value,
                    None => servers.first().unwrap(),
                };

            let target_lang = target_server.language().unwrap();

            let server_request = LanguageRequest {
                libraries: get_all_idl_nodes(&analyzers),
                ids_nodes: analyzer_ids.nodes.clone(),
                request_type: RequestType::Server(ServerType {
                    args: ServerArg::Build,
                    server_name: target_server.ident.clone(),
                }),
            };

            if !no_building {
                Message::info(format!(
                    "Sending language `{}` request for building",
                    target_lang
                ))?;
                let gen = idl_gen::for_language(&target_lang)?;
                let response = gen.send_request(server_request)?;

                match response.gen_response {
                    ResponseType::Generated(value) => {
                        let src = Path::new(output)
                            .join(&package_name)
                            .join("build")
                            .join("idl");
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
        }
        None => {}
    }

    Ok(())
}
