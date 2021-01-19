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
    NatInt(i64),
    NatFloat(f64),
    NatString(String),
    NatBool(bool),
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

    pub fn as_string(&self) -> Option<String> {
        match self {
            ItemType::NatString(value) => Some(value.clone()),
            _ => None,
        }
    }

    pub fn as_int(&self) -> Option<i64> {
        match self {
            ItemType::NatInt(value) => Some(*value),
            _ => None,
        }
    }

    pub fn as_float(&self) -> Option<f64> {
        match self {
            ItemType::NatFloat(value) => Some(*value),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            ItemType::NatBool(value) => Some(*value),
            _ => None,
        }
    }

    pub fn as_layer_name(&self) -> Option<String> {
        match self {
            ItemType::LayerTypeName(value) => Some(value.clone()),
            _ => None,
        }
    }

    pub fn as_client_name(&self) -> Option<String> {
        match self {
            ItemType::ClientTypeName(value) => Some(value.clone()),
            _ => None,
        }
    }

    pub fn as_server_name(&self) -> Option<String> {
        match self {
            ItemType::ServerTypeName(value) => Some(value.clone()),
            _ => None,
        }
    }

    pub fn as_values(&self) -> Option<Vec<ItemType>> {
        match self {
            ItemType::Values(value) => Some(value.clone()),
            _ => None,
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
