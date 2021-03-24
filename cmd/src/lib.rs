pub mod analyze;
pub mod clean;
pub mod client;
mod diagnostics;
pub mod formatter;
pub mod message;
pub mod server;

use anyhow::{anyhow, Result};
use idl_gen::lang::LibraryItem;
use std::fs;
use std::path;
use std::sync::Arc;

#[derive(Debug)]
struct Document {
    text: String,
    file_name: String,
    file_type: String,
}

pub(crate) fn open_directory(path: &path::Path) -> Result<idl::module::Module> {
    let mut module = idl::module::Module::new();

    if !path.is_dir() {
        return Err(anyhow::format_err!(
            "Path nof found `{}`",
            path.to_str().unwrap()
        ));
    }

    for entry in fs::read_dir(path)? {
        let item = entry?.path();

        if item.is_file() {
            if let Some(doc) = add_file(&item) {
                match doc.file_type.as_str() {
                    "idl" => {
                        module.replace_idl_document(&doc.file_name, &doc.text);
                    }
                    "ids" => {
                        if !module.has_ids_document() {
                            let _ = module.replace_ids_document(&doc.file_name, &doc.text);
                        }
                    }
                    _ => panic!("Invalid file ext"),
                }
            }
        }
    }

    if module.has_ids_document() {
        Ok(module)
    } else {
        Err(anyhow!("Missing package"))
    }
}

fn add_file(path: &path::Path) -> Option<Document> {
    let file_name = path.file_name()?.to_str()?.to_owned();
    let file_type = path.extension()?.to_str()?.to_owned();

    match file_type.as_str() {
        "idl" | "ids" => {
            if let Ok(text) = fs::read_to_string(path) {
                return Some(Document {
                    file_name,
                    file_type,
                    text,
                });
            }
        }
        _ => {}
    }

    None
}

pub(crate) fn get_all_idl_nodes(
    analyzers: &[Arc<Result<idl::analyzer::Analyzer, idl::analyzer::AnalyzerError>>],
) -> Vec<LibraryItem> {
    analyzers
        .iter()
        .map(|v| match &**v {
            Ok(value) => LibraryItem {
                nodes: value.nodes.clone(),
            },
            Err(_) => panic!("Could not collect all module"),
        })
        .collect()
}
