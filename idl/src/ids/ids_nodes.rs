use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub enum Keywords {
    Library,
    Layer,
    Client,
    Server,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum ItemType {
    Int(String),
    Float(String),
    String(String),
    Boolean(String),
    LayerTypeName(String),
    ClientTypeName(String),
    ServerTypeName(String),
    Values(Vec<ItemType>),
}

impl ItemType {
    pub fn is_int(&self) -> bool {
        match self {
            ItemType::Int(_) => true,
            _ => false,
        }
    }
    pub fn is_float(&self) -> bool {
        match self {
            ItemType::Float(_) => true,
            _ => false,
        }
    }
    pub fn is_string(&self) -> bool {
        match self {
            ItemType::String(_) => true,
            _ => false,
        }
    }
    pub fn is_boolean(&self) -> bool {
        match self {
            ItemType::Boolean(_) => true,
            _ => false,
        }
    }
    pub fn is_layer(&self) -> bool {
        match self {
            ItemType::LayerTypeName(_) => true,
            _ => false,
        }
    }
    pub fn is_client(&self) -> bool {
        match self {
            ItemType::ClientTypeName(_) => true,
            _ => false,
        }
    }
    pub fn is_server(&self) -> bool {
        match self {
            ItemType::ServerTypeName(_) => true,
            _ => false,
        }
    }
    pub fn is_values(&self) -> bool {
        match self {
            ItemType::Values(_) => true,
            _ => false,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum IdsNode {
    Library(Box<Library>),
    Layer(Box<Layer>),
    Server(Box<Server>),
    Client(Box<Client>),
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Library {
    pub ident: String,
    pub nodes: Vec<LibraryNode>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum LibraryNode {
    LibraryField(Box<LibraryField>),
    Comment(Vec<String>),
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LibraryField {
    pub ident: String,
    pub value: Box<ItemType>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Layer {
    pub ident: String,
    pub nodes: Vec<LayerNode>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum LayerNode {
    LayerField(Box<LayerField>),
    Comment(Vec<String>),
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LayerField {
    pub ident: String,
    pub value: Box<ItemType>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Client {
    pub ident: String,
    pub nodes: Vec<ClientNode>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum ClientNode {
    ClientField(Box<ClientField>),
    Comment(Vec<String>),
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ClientField {
    pub ident: String,
    pub value: Box<ItemType>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Server {
    pub ident: String,
    pub nodes: Vec<ServerNode>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum ServerNode {
    ServerField(Box<ServerField>),
    Comment(Vec<String>),
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ServerField {
    pub ident: String,
    pub value: Box<ItemType>,
}
