use super::session::Session;

use std::sync::Arc;
use tower_lsp::lsp_types::*;

use super::{file_name_from_uri, get_range_from_parser};
use idl::idl::parser;

fn create_document_symbol(
    kind: SymbolKind,
    name: String,
    range: Range,
    children: Option<Vec<DocumentSymbol>>,
    detail: Option<String>,
    selection_range: Range,
) -> DocumentSymbol {
    DocumentSymbol {
        kind,
        name,
        deprecated: None,
        range,
        children,
        detail,
        selection_range,
    }
}

fn get_symbol_from_parser_node(nodes: &[parser::ParserNode]) -> Vec<DocumentSymbol> {
    nodes
        .iter()
        .filter_map(|node| {
            match node {
                parser::ParserNode::Interface(value) => {
                    let name = value.ident.to_string();
                    let children = get_interface_fields(&value.fields);
                    let detail = String::new();
                    let range = get_range_from_parser(value.range);
                    let selection_range = range; // TODO

                    Some(create_document_symbol(
                        SymbolKind::Interface,
                        name,
                        range,
                        Some(children),
                        Some(detail),
                        selection_range,
                    ))
                }
                parser::ParserNode::Struct(value) => {
                    let name = value.ident.to_string();
                    let children = get_struct_fields(&value.fields);
                    let detail = String::new();
                    let range = get_range_from_parser(value.range);
                    let selection_range = range; // TODO

                    Some(create_document_symbol(
                        SymbolKind::Struct,
                        name,
                        range,
                        Some(children),
                        Some(detail),
                        selection_range,
                    ))
                }
                parser::ParserNode::Enum(value) => {
                    let name = value.ident.to_string();
                    let children = get_enum_fields(&value.fields);
                    let detail = String::new();
                    let range = get_range_from_parser(value.range);
                    let selection_range = range; // TODO

                    Some(create_document_symbol(
                        SymbolKind::Enum,
                        name,
                        range,
                        Some(children),
                        Some(detail),
                        selection_range,
                    ))
                }
                parser::ParserNode::TypeList(value) => {
                    let name = value.ident.to_string();
                    let children = get_type_fields(&value.fields);
                    let detail = String::new();
                    let range = get_range_from_parser(value.range);
                    let selection_range = range; // TODO

                    Some(create_document_symbol(
                        SymbolKind::Enum,
                        name,
                        range,
                        Some(children),
                        Some(detail),
                        selection_range,
                    ))
                }
                parser::ParserNode::Const(value) => {
                    let name = value.ident.to_string();
                    let children = get_const_fields(&value.fields, value.const_type);
                    let detail = String::new();
                    let range = get_range_from_parser(value.range);
                    let selection_range = range; // TODO

                    Some(create_document_symbol(
                        SymbolKind::Constant,
                        name,
                        range,
                        Some(children),
                        Some(detail),
                        selection_range,
                    ))
                }
                _ => None,
            }
        })
        .collect()
}

fn get_enum_fields(value: &[parser::EnumNode]) -> Vec<DocumentSymbol> {
    value
        .iter()
        .filter_map(|node| match node {
            parser::EnumNode::EnumField(value) => {
                let range = get_range_from_parser(value.range);
                let name = value.ident.to_owned();
                let detail = None;
                let selection_range = range;
                Some(create_document_symbol(
                    SymbolKind::EnumMember,
                    name,
                    range,
                    None,
                    detail,
                    selection_range,
                ))
            }
            _ => None,
        })
        .collect()
}

fn get_struct_fields(value: &[parser::StructNode]) -> Vec<DocumentSymbol> {
    value
        .iter()
        .filter_map(|node| match node {
            parser::StructNode::StructField(value) => {
                let range = get_range_from_parser(value.range);
                let name = value.ident.to_owned();
                let detail = Some(value.ty.to_string());
                let selection_range = range;
                Some(create_document_symbol(
                    SymbolKind::Field,
                    name,
                    range,
                    None,
                    detail,
                    selection_range,
                ))
            }
            _ => None,
        })
        .collect()
}

fn get_interface_fields(value: &[parser::InterfaceNode]) -> Vec<DocumentSymbol> {
    value
        .iter()
        .filter_map(|node| match node {
            parser::InterfaceNode::InterfaceField(value) => {
                let range = get_range_from_parser(value.range);
                let name = value.ident.to_owned();
                let detail = Some(value.ty.to_string());
                let selection_range = range;

                Some(create_document_symbol(
                    SymbolKind::Function,
                    name,
                    range,
                    None,
                    detail,
                    selection_range,
                ))
            }
            _ => None,
        })
        .collect()
}

fn get_type_fields(value: &[parser::TypeListNode]) -> Vec<DocumentSymbol> {
    value
        .iter()
        .filter_map(|node| match node {
            parser::TypeListNode::TypeListField(value) => {
                let range = get_range_from_parser(value.range);
                let name = value.ident.to_owned();
                let detail = Some(value.ty.to_string());
                let selection_range = range;
                Some(create_document_symbol(
                    SymbolKind::EnumMember,
                    name,
                    range,
                    None,
                    detail,
                    selection_range,
                ))
            }
            _ => None,
        })
        .collect()
}

fn get_static_fields(value: &[parser::InterfaceNode]) -> Vec<DocumentSymbol> {
    value
        .iter()
        .filter_map(|node| match node {
            parser::InterfaceNode::InterfaceField(value) => {
                let range = get_range_from_parser(value.range);
                let name = value.ty.to_string();
                let detail = None;
                let selection_range = range;
                Some(create_document_symbol(
                    SymbolKind::Constructor,
                    name,
                    range,
                    None,
                    detail,
                    selection_range,
                ))
            }
            _ => None,
        })
        .collect()
}

fn get_const_fields(
    value: &[parser::ConstNode],
    const_ty: parser::ConstType,
) -> Vec<DocumentSymbol> {
    value
        .iter()
        .filter_map(|node| match node {
            parser::ConstNode::ConstField(value) => {
                let range = get_range_from_parser(value.range);
                let name = value.ident.to_owned();
                let detail = Some(const_ty.to_string());
                let children = None;
                let selection_range = range;
                Some(create_document_symbol(
                    SymbolKind::Field,
                    name,
                    range,
                    children,
                    detail,
                    selection_range,
                ))
            }
            _ => None,
        })
        .collect()
}

pub(crate) async fn get_document_symbol(
    session: Arc<Session>,
    params: DocumentSymbolParams,
) -> Option<DocumentSymbolResponse> {
    let DocumentSymbolParams {
        text_document: TextDocumentIdentifier { uri },
        ..
    } = params;

    let file_name = file_name_from_uri(&uri)?;
    let module_state = session.module_state.lock().await;
    let parser_s = module_state.get_idl_parser(&file_name)?;

    let parser = match &*parser_s {
        Ok(parser) => parser,
        Err((parser, _)) => parser,
    };

    return Some(DocumentSymbolResponse::Nested(get_symbol_from_parser_node(
        &parser.nodes,
    )));
}
