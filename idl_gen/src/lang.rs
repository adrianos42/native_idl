use serde::{Deserialize, Serialize};
use idl::idl_types::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct LanguageRequest {
    pub args: std::collections::HashMap<String, String>,
    pub nodes: Vec<TypeNode>, 
}

#[derive(Debug, Deserialize, Serialize)]
pub enum ResponseType {
    Generated(Folder),
    Undefined(String),
} 

#[derive(Debug, Deserialize, Serialize)]
pub enum StorageItem {
    Source(Source),
    Folder(Folder),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Source {
    pub name: String,
    pub txt: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Folder {
    pub name: String,
    pub items: Vec<StorageItem>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LanguageResponse {
    pub gen_response: ResponseType,
}