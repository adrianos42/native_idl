use std::{io::Write, path::Path};

use idl::idl_nodes;
use idl::ids::ids_nodes;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ClientType {
    pub client_name: String,
    pub server_name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum ServerArg {
    Build,
    Generate,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ServerType {
    pub server_name: String,
    pub args: ServerArg,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum RequestType {
    Client(String),
    Server(ServerType),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LanguageRequest {
    pub idl_nodes: Vec<idl_nodes::IdlNode>,
    pub ids_nodes: Vec<ids_nodes::IdsNode>,
    pub request_type: RequestType,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum ResponseType {
    Generated(Vec<StorageItem>),
    Undefined(String),
}

#[derive(Debug, Deserialize, Serialize)]
pub enum StorageItem {
    Source {
        name: String,
        txt: String,
    },
    Folder {
        name: String,
        items: Vec<StorageItem>,
    },
    BinarySource {
        name: String,
        data: Vec<u8>,
    },
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LanguageResponse {
    pub gen_response: ResponseType,
    pub response_messages: Vec<String>,
}

impl StorageItem {
    pub fn write_items(&self, path: &Path, is_merged: bool) -> anyhow::Result<()> {
        match self {
            Self::Source { name, txt } => {
                let mut file = std::fs::OpenOptions::new()
                    .write(true)
                    .truncate(true)
                    .create(true)
                    .open(path.join(name.as_str()))?;
                file.write_all(txt.as_bytes())?;
            }
            Self::Folder { name, items } => {
                let new_path = path.join(&name);
                if !is_merged {
                    let _ = std::fs::remove_dir_all(&new_path);
                }
                std::fs::create_dir_all(&new_path)?;

                for item in items {
                    item.write_items(&new_path, is_merged)?;
                }
            }
            Self::BinarySource { name, data } => {
                let mut file = std::fs::OpenOptions::new()
                    .write(true)
                    .truncate(true)
                    .create(true)
                    .open(path.join(name.as_str()))?;
                file.write_all(data)?;
            }
        }

        Ok(())
    }
}
