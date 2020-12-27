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
    NatInt(String),
    NatFloat(String),
    NatString(String),
    NatBool(String),
    LayerTypeName(String),
    ClientTypeName(String),
    ServerTypeName(String),
    Values(Vec<ItemType>),
}

impl ItemType {
    pub fn is_int(&self) -> bool {
        match self {
            ItemType::NatInt(_) => true,
            _ => false,
        }
    }
    pub fn is_float(&self) -> bool {
        match self {
            ItemType::NatFloat(_) => true,
            _ => false,
        }
    }
    pub fn is_string(&self) -> bool {
        match self {
            ItemType::NatString(_) => true,
            _ => false,
        }
    }
    pub fn is_boolean(&self) -> bool {
        match self {
            ItemType::NatBool(_) => true,
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
    Layer(Box<super::layer::Layer>),
    Server(Box<super::server::Server>),
    Client(Box<super::client::Client>),
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


