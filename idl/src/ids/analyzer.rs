use super::ids_nodes::*;
use super::parser;
use crate::module;
use crate::range::Range;
use crate::reserved::{
    field_name_is_valid, is_reserved_type, is_reserved_word, type_name_is_valid, NameError,
};

use super::client;
use super::layer;
use super::library;
use super::server;

use std::fmt;
use thiserror::Error;

#[derive(Debug, Clone, Copy)]
pub enum ReferenceErrorKind {
    Invalid,
    UndefinedType,
    UnknownField,
    NotInt,
    NotFloat,
    NotString,
    NotBoolean,
    NotLayerTypeName,
    NotClientTypeName,
    NotServerTypeName,
    NotValues,
}

#[derive(Error, Debug, Clone)]
pub struct ReferenceError(pub ReferenceErrorKind, pub Range, pub String);

impl fmt::Display for ReferenceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = self.2.as_str();

        let errstr = match &self.0 {
            ReferenceErrorKind::Invalid => format!("Invalid type name `{}`", name),
            ReferenceErrorKind::UndefinedType => format!("Undefined type `{}`", name),
            ReferenceErrorKind::UnknownField => format!("Unknown field `{}`", name),
            ReferenceErrorKind::NotInt => format!("Not a integer `{}`", name),
            ReferenceErrorKind::NotFloat => format!("Not a float `{}`", name),
            ReferenceErrorKind::NotString => format!("Not a string `{}`", name),
            ReferenceErrorKind::NotBoolean => format!("Not a boolean `{}`", name),
            ReferenceErrorKind::NotLayerTypeName => format!("Not a layer `{}`", name),
            ReferenceErrorKind::NotClientTypeName => format!("Not a client `{}`", name),
            ReferenceErrorKind::NotServerTypeName => format!("Not a server `{}`", name),
            ReferenceErrorKind::NotValues => format!("Not a value vector `{}`", name),
        };

        write!(f, "{}", errstr)
    }
}
#[derive(Error, Debug, Clone)]
pub struct DuplicateFieldNameError(String, Range);

impl fmt::Display for DuplicateFieldNameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Error, Debug, Clone)]
pub struct DuplicateNameError(String);

impl fmt::Display for DuplicateNameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Error, Debug, Clone)]
pub enum AnalyzerError {
    #[error("Undefined error")]
    Undefined,
    #[error("Analyzer closed")]
    Closed,
    #[error("Library redefinition")]
    LibraryDefinition,
    #[error("Missing library definition")]
    MissingLibraryDefinition,
    #[error("Duplicate name `{0}`")]
    DuplicateName(#[from] DuplicateNameError),
    #[error("Duplicate name field `{0}`")]
    DuplicateFieldNameError(#[from] DuplicateFieldNameError),
    #[error("Name `{0}`")]
    NameError(#[from] NameError),
    #[error("Reference `{0}`")]
    ReferenceError(#[from] ReferenceError),
}

impl AnalyzerError {
    pub fn get_message_with_range(&self) -> (String, Range) {
        match self {
            AnalyzerError::Undefined
            | AnalyzerError::Closed
            | AnalyzerError::LibraryDefinition
            | AnalyzerError::MissingLibraryDefinition => (self.to_string(), Range::default()),
            AnalyzerError::DuplicateFieldNameError(value) => (self.to_string(), value.1),
            AnalyzerError::DuplicateName(_value) => (self.to_string(), Range::default()),
            AnalyzerError::ReferenceError(value) => (self.to_string(), value.1),
            AnalyzerError::NameError(value) => (self.to_string(), value.1),
        }
    }
}

#[derive(Debug, Default)]
pub struct Analyzer {
    pub nodes: Vec<IdsNode>,
}

#[derive(Debug, Default)]
pub(super) struct AnalyzerItems {
    library: Option<String>,
    layers: Vec<String>,
    clients: Vec<String>,
    servers: Vec<String>,
}

impl AnalyzerItems {
    pub(super) fn get_ids_node(&self, item: &parser::ItemType) -> Result<ItemType, ReferenceError> {
        match item {
            parser::ItemType::Int(value) => Ok(ItemType::NatInt(value.to_owned())),
            parser::ItemType::Float(value) => Ok(ItemType::NatFloat(value.to_owned())),
            parser::ItemType::String(value) => Ok(ItemType::NatString(value.to_owned())),
            parser::ItemType::Boolean(value) => Ok(ItemType::NatBool(value.to_owned())),
            parser::ItemType::TypeName(value) => {
                if self.clients.contains(&value.ident) {
                    Ok(ItemType::ClientTypeName(value.ident.to_owned()))
                } else if self.servers.contains(&value.ident) {
                    Ok(ItemType::ServerTypeName(value.ident.to_owned()))
                } else if self.layers.contains(&value.ident) {
                    Ok(ItemType::LayerTypeName(value.ident.to_owned()))
                } else {
                    Err(ReferenceError(
                        ReferenceErrorKind::UndefinedType,
                        value.range,
                        value.ident.to_owned(),
                    ))
                }
            }
            parser::ItemType::Values(value) => {
                let mut result = vec![];
                for item in value.iter() {
                    result.push(self.get_ids_node(item)?);
                }
                Ok(ItemType::Values(result))
            }
        }
    }
}

impl Analyzer {
    pub fn closed() -> Result<Self, AnalyzerError> {
        Err(AnalyzerError::Closed)
    }

    pub fn from_nodes(nodes: Vec<IdsNode>) -> Self {
        Self { nodes }
    }

    pub fn resolve(
        parsers: &parser::Parser,
        module: Option<&crate::module::Module>,
    ) -> Result<Self, AnalyzerError> {
        let mut items = AnalyzerItems::default();
        let mut nodes = vec![];

        for p_value in parsers.nodes.iter() {
            match p_value {
                parser::ParserNode::Library(value) => {
                    if items.library.is_some() {
                        return Err(AnalyzerError::LibraryDefinition);
                    }

                    let name = match &value.ident {
                        parser::ItemIdent::Identifier(value) => value,
                        parser::ItemIdent::TypeName(_) => {
                            return Err(AnalyzerError::LibraryDefinition)
                        }
                    };

                    is_reserved_type(name.ident.to_lowercase().as_str(), value.range)?;
                    is_reserved_word(name.ident.to_lowercase().as_str(), value.range)?;
                    Self::has_duplicate_fields(&value.nodes)?;
                    Self::has_valid_name_fields(&value.nodes)?;
                    items.library = Some(name.ident.to_owned());
                }
                parser::ParserNode::Layer(value) => {
                    let name = match &value.ident {
                        parser::ItemIdent::TypeName(value) => value,
                        parser::ItemIdent::Identifier(_) => return Err(AnalyzerError::Undefined),
                    };

                    type_name_is_valid(name.ident.as_str(), value.range)?;
                    is_reserved_type(name.ident.to_lowercase().as_str(), value.range)?;
                    is_reserved_word(name.ident.to_lowercase().as_str(), value.range)?;
                    Self::has_duplicate_fields(&value.nodes)?;
                    Self::has_valid_name_fields(&value.nodes)?;
                    items.layers.push(name.ident.to_owned());
                }
                parser::ParserNode::Server(value) => {
                    let name = match &value.ident {
                        parser::ItemIdent::TypeName(value) => value,
                        parser::ItemIdent::Identifier(_) => return Err(AnalyzerError::Undefined),
                    };

                    type_name_is_valid(name.ident.as_str(), value.range)?;
                    is_reserved_type(name.ident.to_lowercase().as_str(), value.range)?;
                    is_reserved_word(name.ident.to_lowercase().as_str(), value.range)?;
                    Self::has_duplicate_fields(&value.nodes)?;
                    Self::has_valid_name_fields(&value.nodes)?;
                    items.servers.push(name.ident.to_owned());
                }
                parser::ParserNode::Client(value) => {
                    let name = match &value.ident {
                        parser::ItemIdent::TypeName(value) => value,
                        parser::ItemIdent::Identifier(_) => return Err(AnalyzerError::Undefined),
                    };

                    type_name_is_valid(name.ident.as_str(), value.range)?;
                    is_reserved_type(name.ident.to_lowercase().as_str(), value.range)?;
                    is_reserved_word(name.ident.to_lowercase().as_str(), value.range)?;
                    Self::has_duplicate_fields(&value.nodes)?;
                    Self::has_valid_name_fields(&value.nodes)?;
                    items.clients.push(name.ident.to_owned());
                }
            }
        }

        Self::has_duplicate_names(&items)?;
        Self::has_one_main_definition(&items)?;

        for p_value in parsers.nodes.iter() {
            match p_value {
                parser::ParserNode::Library(value) => {
                    nodes.push(library::get_node(&value, &items)?)
                }
                parser::ParserNode::Layer(value) => nodes.push(layer::get_node(&value, &items)?),
                parser::ParserNode::Server(value) => nodes.push(server::get_node(&value, &items)?),
                parser::ParserNode::Client(value) => nodes.push(client::get_node(&value, &items)?),
            }
        }

        Ok(Self::from_nodes(nodes))
    }

    pub fn find_client(&self, name: &str) -> Option<&Box<client::Client>> {
        for node in &self.nodes {
            match node {
                IdsNode::Client(value) => {
                    if value.ident == name {
                        return Some(value);
                    }
                }
                _ => {}
            }
        }

        None
    }

    pub fn has_client(&self, name: &str) -> bool {
        for node in &self.nodes {
            match node {
                IdsNode::Client(value) => {
                    if value.ident == name {
                        return true;
                    }
                }
                _ => {}
            }
        }

        false
    }

    pub fn find_server(&self, name: &str) -> Option<&Box<server::Server>> {
        for node in &self.nodes {
            match node {
                IdsNode::Server(value) => {
                    if value.ident == name {
                        return Some(value);
                    }
                }
                _ => {}
            }
        }

        None
    }

     pub fn has_server(&self, name: &str) -> bool {
        for node in &self.nodes {
            match node {
                IdsNode::Server(value) => {
                    if value.ident == name {
                        return true;
                    }
                }
                _ => {}
            }
        }

        false
    }

    fn has_duplicate_fields(nodes: &[parser::ItemNode]) -> Result<(), DuplicateFieldNameError> {
        for (i, node) in nodes.iter().enumerate() {
            if let parser::ItemNode::ItemField(field) = node {
                for (j, c_node) in nodes.iter().enumerate() {
                    if j == i {
                        continue;
                    }
                    if let parser::ItemNode::ItemField(c_field) = c_node {
                        if &field.ident == &c_field.ident {
                            return Err(DuplicateFieldNameError(
                                field.ident.to_owned(),
                                field.range,
                            ));
                        }
                    }
                }
            }
        }

        Ok(())
    }

    fn has_valid_name_fields(nodes: &[parser::ItemNode]) -> Result<(), NameError> {
        for node in nodes {
            match node {
                parser::ItemNode::ItemField(field) => {
                    field_name_is_valid(&field.ident, field.range)?
                }
                _ => {}
            }
        }

        Ok(())
    }

    fn has_duplicate_names(items: &AnalyzerItems) -> Result<(), DuplicateNameError> {
        let names = items
            .clients
            .iter()
            .filter(|&x| x != "Main")
            .chain(items.servers.iter().filter(|&x| x != "Main"))
            .chain(items.layers.iter());
        if let Some(value) = names
            .clone()
            .find(|v| names.clone().filter(|f| f == v).count() != 1)
        {
            return Err(DuplicateNameError(value.to_owned()));
        }

        Ok(())
    }

    // TODO Create error for `Main` names duplicates in server and client definitions
    fn has_one_main_definition(items: &AnalyzerItems) -> Result<(), DuplicateNameError> {
        if items.clients.iter().filter(|&x| x == "Main").count() > 1
            || items.servers.iter().filter(|&x| x == "Main").count() > 1
        {
            return Err(DuplicateNameError("Main".to_owned()));
        }

        Ok(())
    }
}
