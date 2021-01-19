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
        data: String,
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LanguageResponse {
    pub gen_response: ResponseType,
}
