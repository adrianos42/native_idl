use super::analyzer;
use super::ids_nodes::*;
use super::parser;

static DEFAULT_VERSION: &str = "1.0";
static DEFAULT_IDL_VERSION: &str = "0.1";

pub(super) fn get_node(
    item: &parser::Item,
    analyzer_items: &analyzer::AnalyzerItems,
) -> Result<IdsNode, analyzer::AnalyzerError> {
    let mut nodes = vec![];
    let mut has_version = false;
    let mut has_idl_version = false;
    let mut has_author = false;
    let mut has_client = false;
    let mut has_server = false;

    let create_field_node = |ident: &str, value| {
        LibraryNode::LibraryField(Box::new(LibraryField {
            value: Box::new(value),
            ident: ident.to_owned(),
        }))
    };

    for node in item.nodes.iter() {
        match node {
            parser::ItemNode::ItemField(field) => {
                let value = analyzer_items.get_ids_node(&field.value)?;
                match field.ident.as_str() {
                    "version" => {
                        if !value.is_string() {
                            return Err(analyzer::ReferenceError(
                                analyzer::ReferenceErrorKind::NotString,
                                field.range,
                                field.ident.to_owned(),
                            )
                            .into());
                        }

                        has_version = true;
                    }
                    "idl_version" => {
                        if !value.is_string() {
                            return Err(analyzer::ReferenceError(
                                analyzer::ReferenceErrorKind::NotString,
                                field.range,
                                field.ident.to_owned(),
                            )
                            .into());
                        }

                        has_idl_version = true;
                    }
                    "author" => {
                        if !value.is_string() {
                            return Err(analyzer::ReferenceError(
                                analyzer::ReferenceErrorKind::NotString,
                                field.range,
                                field.ident.to_owned(),
                            )
                            .into());
                        }

                        has_author = true;
                    }
                    "client" => {
                        if !value.is_client()
                            || value.is_values()
                                && match &value {
                                    ItemType::Values(values) => {
                                        values.iter().any(|v| !v.is_client())
                                    }
                                    _ => false,
                                }
                        {
                            return Err(analyzer::ReferenceError(
                                analyzer::ReferenceErrorKind::NotClientTypeName,
                                field.range,
                                field.ident.to_owned(),
                            )
                            .into());
                        }

                        has_client = true;
                    }
                    "server" => {
                        if !value.is_server()
                            || value.is_values()
                                && match &value {
                                    ItemType::Values(values) => {
                                        values.iter().any(|v| !v.is_server())
                                    }
                                    _ => false,
                                }
                        {
                            return Err(analyzer::ReferenceError(
                                analyzer::ReferenceErrorKind::NotServerTypeName,
                                field.range,
                                field.ident.to_owned(),
                            )
                            .into());
                        }

                        has_server = true;
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

    if !has_author {
        nodes.push(create_field_node("author", ItemType::NatString("".to_owned())))
    }

    if !has_client {
        nodes.push(create_field_node(
            "client",
            ItemType::ClientTypeName("Default".to_owned()),
        ));
    }

    if !has_server {
        nodes.push(create_field_node(
            "server",
            ItemType::ServerTypeName("Default".to_owned()),
        ));
    }

    let ident = match &item.ident {
        parser::ItemIdent::Identifier(value) => value.ident.to_owned(),
        parser::ItemIdent::TypeName(_) => return Err(analyzer::AnalyzerError::LibraryDefinition),
    };

    Ok(IdsNode::Library(Box::new(Library { ident, nodes })))
}
