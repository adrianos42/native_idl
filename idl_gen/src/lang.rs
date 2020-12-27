use serde::{Deserialize, Serialize};
use idl::idl::idl_nodes;
use idl::ids::ids_nodes;

#[derive(Debug, Deserialize, Serialize)]
pub struct ClientType {
    pub client_name: String,
    pub server_name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ServerType {
    pub server_name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum RequestType {
    Client(String),
    Server(String),
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