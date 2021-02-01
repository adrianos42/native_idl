use core::fmt;

use serde::{de::value, Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub enum Keywords {
    Package,
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
    Identifier(String),
    LayerTypeName(String),
    ClientTypeName(String),
    ServerTypeName(String),
    Values(Vec<ItemType>),
}

impl fmt::Display for ItemType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let errstr = match self {
            ItemType::NatInt(value) => value.to_string(),
            ItemType::NatFloat(value) => value.to_string(),
            ItemType::NatString(value) => format!("\"{}\"", value),
            ItemType::NatBool(value) => value.to_string(),
            ItemType::Identifier(value) => value.to_owned(),
            ItemType::ServerTypeName(value)
            | ItemType::ClientTypeName(value)
            | ItemType::LayerTypeName(value) => value.to_owned(),
            ItemType::Values(value) => {
                format!(
                    "{}]",
                    value
                        .iter()
                        .fold("[".to_owned(), |p, val| format!("{}{},", p, val))
                )
            }
        };

        write!(f, "{}", errstr)
    }
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

    pub fn is_identifier(&self) -> bool {
        match self {
            ItemType::Identifier(_) => true,
            _ => false,
        }
    }

    pub fn is_values(&self) -> bool {
        match self {
            ItemType::Values(_) => true,
            _ => false,
        }
    }

    pub fn as_identifier(&self) -> Option<String> {
        match self {
            ItemType::Identifier(value) => Some(value.clone()),
            _ => None,
        }
    }

    pub fn as_string_value(&self) -> Option<String> {
        match self {
            ItemType::NatString(value) => Some(value.clone()),
            _ => None,
        }
    }

    pub fn as_int_value(&self) -> Option<i64> {
        match self {
            ItemType::NatInt(value) => Some(*value),
            _ => None,
        }
    }

    pub fn as_float_value(&self) -> Option<f64> {
        match self {
            ItemType::NatFloat(value) => Some(*value),
            _ => None,
        }
    }

    pub fn as_bool_value(&self) -> Option<bool> {
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
    Package(Box<super::package::Package>),
    Layer(Box<super::layer::Layer>),
    Server(Box<super::server::Server>),
    Client(Box<super::client::Client>),
}