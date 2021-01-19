mod diagnostics;
pub mod formatter;
pub mod client;
pub mod server;
pub mod analyze;
pub mod clean;

use anyhow::{anyhow, Result};
use idl_gen::lang::StorageItem;
use std::fs;
use std::{io::Write, path::Path};
use std::path;

#[derive(Debug)]
struct Document {
    text: String,
    file_stem: String,
    file_type: String,
}

pub(crate) fn open_directory(path: &path::Path) -> Result<idl::module::Module> {
    let module = idl::module::Module::new();

    if !path.is_dir() {
        return Err(anyhow!("Path is not a directory"));
    }

    for entry in fs::read_dir(path)? {
        let item = entry?.path();

        if item.is_file() {
            if let Some(doc) = add_file(&item) {
                match doc.file_type.as_str() {
                    "idl" => {
                        module.add_idl_document_and_update(&doc.file_stem, &doc.text)?;
                    }
                    "ids" => {
                        module.add_ids_document_and_update(&doc.file_stem, &doc.text)?;
                    }
                    _ => panic!(),
                }
            }
        }
    }

    Ok(module)
}

fn add_file(path: &path::Path) -> Option<Document> {
    let file_stem_s = path.file_stem()?;
    let file_type_s = path.extension()?;

    let file_stem = file_stem_s.to_str()?.to_owned();
    let file_type = file_type_s.to_str()?.to_owned();

    match file_type.as_str() {
        "idl" | "ids" => {
            if let Ok(text) = fs::read_to_string(path) {
                return Some(Document {
                    file_stem,
                    file_type,
                    text,
                });
            }
        }
        _ => {}
    }

    None
}



pub(crate) fn write_items(storage: &StorageItem, path: &Path) -> Result<()> {
    match storage {
        idl_gen::lang::StorageItem::Source { name, txt } => {
            let mut file = fs::OpenOptions::new()
                .write(true)
                .truncate(true)
                .create(true)
                .open(path.join(name.as_str()))?;
            file.write_all(txt.as_bytes())?;
        }
        idl_gen::lang::StorageItem::Folder { name, items } => {
            let new_path = path.join(&name);
            let _ = fs::remove_dir_all(&new_path);
            fs::create_dir_all(&new_path)?;

            for item in items {
                write_items(item, &new_path)?;
            }
        }
        idl_gen::lang::StorageItem::BinarySource { name, data } => {
            let mut file = fs::OpenOptions::new()
                .write(true)
                .truncate(true)
                .create(true)
                .open(path.join(name.as_str()))?;
            file.write_all(data.as_bytes())?;
        }
    }

    Ok(())
}
