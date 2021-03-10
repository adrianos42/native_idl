use super::analyzer;
use super::ids_nodes::*;
use super::parser;
use serde::{Deserialize, Serialize};

static DEFAULT_VERSION: &str = "1.0";
static DEFAULT_IDL_VERSION: &str = "0.1";

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Package {
    pub ident: String,
    pub nodes: Vec<PackageNode>,
    pub hash: Box<[u8]>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum PackageNode {
    PackageField(Box<PackageField>),
    Comment(Vec<String>),
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PackageField {
    pub ident: String,
    pub value: Box<ItemType>,
}

impl Package {
    pub fn name(&self) -> String {
        self.ident.to_owned()
    }
    
    pub fn lib_names(&self) -> Option<Vec<String>> {
        let field = self.get_field("libs")?.as_values()?;
        let mut result = vec![];

        for value in field {
            result.push(value.as_identifier()?)
        }

        Some(result)
    }

    pub fn get_field(&self, name: &str) -> Option<&ItemType> {
        for node in &self.nodes {
            match node {
                PackageNode::PackageField(field) => {
                    if field.ident == name {
                        return Some(&field.value);
                    }
                }
                _ => {}
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
    let mut has_version = false;
    let mut has_idl_version = false;
    let mut has_authors = false;
    let mut has_client = false;
    let mut has_server = false;

    let create_field_node = |ident: &str, value| {
        PackageNode::PackageField(Box::new(PackageField {
            value: Box::new(value),
            ident: ident.to_owned(),
        }))
    };

    for node in item.nodes.iter() {
        match node {
            parser::ItemNode::ItemField(field) => {
                let value = analyzer_items.get_ids_node(&field.value)?;
                match field.ident.as_str() {
                    "libs" => match &value {
                        ItemType::Values(values) => {
                            for value in values {
                                if !value.is_identifier() {
                                    return Err(analyzer::ReferenceError(
                                        analyzer::ReferenceErrorKind::NotIdentifier,
                                        field.range,
                                        value.to_string(),
                                    )
                                    .into());
                                }
                            }
                        }
                        _ => {
                            return Err(analyzer::ReferenceError(
                                analyzer::ReferenceErrorKind::NotValues,
                                field.range,
                                field.ident.to_owned(),
                            )
                            .into());
                        }
                    },
                    "version" => {
                        if !value.is_float() {
                            return Err(analyzer::ReferenceError(
                                analyzer::ReferenceErrorKind::NotFloat,
                                field.range,
                                field.ident.to_owned(),
                            )
                            .into());
                        }

                        has_version = true;
                    }
                    "idl_version" => {
                        if !value.is_float() {
                            return Err(analyzer::ReferenceError(
                                analyzer::ReferenceErrorKind::NotFloat,
                                field.range,
                                field.ident.to_owned(),
                            )
                            .into());
                        }

                        has_idl_version = true;
                    }
                    "authors" => {
                        match &value {
                            ItemType::Values(values) => {
                                for value in values {
                                    if !value.is_string() {
                                        return Err(analyzer::ReferenceError(
                                            analyzer::ReferenceErrorKind::NotString,
                                            field.range,
                                            field.ident.to_owned(),
                                        )
                                        .into());
                                    }
                                }
                            }
                            _ => {
                                return Err(analyzer::ReferenceError(
                                    analyzer::ReferenceErrorKind::NotValues,
                                    field.range,
                                    field.ident.to_owned(),
                                )
                                .into());
                            }
                        }

                        has_authors = true;
                    }
                    _ => {
                        return Err(analyzer::ReferenceError(
                            analyzer::ReferenceErrorKind::UnknownField,
                            field.range,
                            field.ident.to_owned(),
                        )
                        .into())
                    }
                }
                nodes.push(create_field_node(&field.ident, value));
            }
            parser::ItemNode::Comment(_) => {}
        }
    }

    if !has_version {
        nodes.push(create_field_node(
            "version",
            ItemType::NatString(DEFAULT_VERSION.to_owned()),
        ));
    }

    if !has_idl_version {
        nodes.push(create_field_node(
            "idl_version",
            ItemType::NatString(DEFAULT_IDL_VERSION.to_owned()),
        ));
    }

    if !has_authors {
        nodes.push(create_field_node(
            "author",
            ItemType::Values(vec![])
        ))
    }

    let ident = match &item.ident {
        parser::ItemIdent::Identifier(value) => value.ident.to_owned(),
        parser::ItemIdent::TypeName(_) => return Err(analyzer::AnalyzerError::PackageDefinition),
    };

    let hash = vec![0x0u8; 0x10].into_boxed_slice();  // TODO Proper hash values

    Ok(IdsNode::Package(Box::new(Package { ident, nodes, hash })))
}
