use super::analyzer;
use super::ids_nodes::*;
use super::parser;
use serde::{Deserialize, Serialize};

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
pub struct Client {
    pub ident: String,
    pub nodes: Vec<ClientNode>,
}

impl Client {
    pub fn language(&self) -> Option<String> {
        for node in &self.nodes {
            match node {
                ClientNode::ClientField(field) => {
                    if field.ident == "language" {
                        match field.value.as_ref() {
                            ItemType::NatString(value) => return Some(value.clone()),
                            _ => {}
                        }
                    }
                }
                ClientNode::Comment(_) => {}
            }
        }

        None
    }
}

pub(super) fn get_node(
    item: &parser::Item,
    analyzer_items: &analyzer::AnalyzerItems,
) -> Result<IdsNode, analyzer::AnalyzerError> {
    let mut nodes = vec![];

    let create_field_node = |ident: &str, value| {
        ClientNode::ClientField(Box::new(ClientField {
            value: Box::new(value),
            ident: ident.to_owned(),
        }))
    };

    for node in item.nodes.iter() {
        match node {
            parser::ItemNode::ItemField(field) => {
                let value = analyzer_items.get_ids_node(&field.value)?;
                match field.ident.as_str() {
                    "layers" => {
                        if match &value {
                            ItemType::Values(values) => values.iter().any(|v| !v.is_layer()),
                            _ => true,
                        } {
                            return Err(analyzer::ReferenceError(
                                analyzer::ReferenceErrorKind::NotLayerTypeName,
                                field.range,
                                field.ident.to_owned(),
                            )
                            .into());
                        }
                    }
                    "description" => {
                        if !value.is_string() {
                            return Err(analyzer::ReferenceError(
                                analyzer::ReferenceErrorKind::NotString,
                                field.range,
                                field.ident.to_owned(),
                            )
                            .into());
                        }
                    }
                    "language" => {
                        if !value.is_string() {
                            return Err(analyzer::ReferenceError(
                                analyzer::ReferenceErrorKind::NotString,
                                field.range,
                                field.ident.to_owned(),
                            )
                            .into());
                        }
                    }
                    _ => {}
                }
                nodes.push(create_field_node(&field.ident, value));
            }
            parser::ItemNode::Comment(_) => {}
        }
    }

    let ident = match &item.ident {
        parser::ItemIdent::Identifier(_) => return Err(analyzer::AnalyzerError::Undefined),
        parser::ItemIdent::TypeName(value) => value.ident.to_owned(),
    };

    Ok(IdsNode::Client(Box::new(Client { ident, nodes })))
}
