use super::diagnostics;
use anyhow::{anyhow, Result};
use clap::{App, Arg, ArgMatches};
use idl::idl_types::*;
use idl_gen::lang::*;
use std::fs;
use std::{io::Write, path::Path};
use std::collections::HashMap;

enum GenArgs {
    TargetLanguage(String),
}

pub fn create_command<'a>() -> App<'a> {
    App::new("gen").about("Generate language files").args(&[
        Arg::new("output")
            .about("The output path")
            .short('o')
            .default_value(".")
            .long("output")
            .takes_value(true),
        Arg::new("input")
            .about("The library path")
            .default_value(".")
            .short('i')
            .long("input")
            .takes_value(true),
        Arg::new("target")
            .about("The target language")
            .short('t')
            .long("target")
            .takes_value(true)
            .required(true),
        Arg::new("arg")
            .about("Module argument")
            .takes_value(true)
            .short('a')
            .long("arg")
            .multiple(true)
            .required(true),
    ])
}

pub fn parse(matches: &ArgMatches) -> Result<()> {
    let input = matches.value_of("input").unwrap();
    let output = matches.value_of("output").unwrap();
    let target = matches.value_of("target").unwrap_or("rust");
    let mut layers = vec![];

    if let Some(arg_v) = matches.values_of("arg") {
        for arg in arg_v {
            match arg {
                "server" => {}
                "client" => {}
                n @ "ffi" => layers.push(n),
                _ => return Err(anyhow!("Unknown argument `{}`", arg)),
            }
        }
    }

    let module = crate::open_directory(std::path::Path::new(input))?;
    module.update_all_analyze()?;

    if diagnostics::diagnostic(&module)? {
        return Ok(());
    }

    // file used to be generated
    let library_doc = module.document_library_name().ok_or(anyhow!(""))?;
    let analyzer_r = &*module.get_analyzer(&library_doc).ok_or(anyhow!(""))?;
    let analyzer = analyzer_r.as_ref().map_err(|_| anyhow!(""))?;

    let request = LanguageRequest {
        args: HashMap::new(),
        nodes: analyzer.nodes.clone(),
    };

    let gen = idl_gen::for_language(target)?;
    let response = gen.send_request(request)?;

    match response.gen_response {
        ResponseType::Generated(value) => {
            let src = Path::new(output).join(target);
            write_items(&StorageItem::Folder(value), &src)?;
            println!("Generated files at {:#?}", src);
        }
        ResponseType::Undefined(err) => return Err(anyhow!("Response error `{}`", err)),
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
