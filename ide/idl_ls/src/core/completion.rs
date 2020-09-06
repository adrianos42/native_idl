use super::session::Session;

use native_idl::formatting::INDENT;
use std::sync::Arc;
use tower_lsp::lsp_types::*;

use super::{create_parser_position, create_parser_range, file_name_from_uri};
use native_idl::{completion, parser};

pub(crate) async fn completion(
    session: Arc<Session>,
    params: CompletionParams,
) -> Option<CompletionResponse> {
    let CompletionParams {
        text_document_position:
            TextDocumentPositionParams {
                text_document,
                position,
            },
        ..
    } = params;

    let uri = text_document.uri;
    let pos = create_parser_position(position);

    if let Some(file_name) = file_name_from_uri(&uri) {
        let state = session.session_state.lock().await;

        if let Some(doc) = state.documents.get(&file_name) {
            let module_state = session.module_state.lock().await;
            if let Some(completions) = module_state
                .try_completion(&file_name, doc.text.as_str(), pos)
                .await
            {
                let mut items = vec![];

                for node in &completions.nodes {
                    match node {
                        completion::CompletionNode::Type(_) => {}
                        completion::CompletionNode::InterfaceName(name) => {
                            items.push(CompletionItem {
                                kind: Some(CompletionItemKind::Interface),
                                label: name.to_owned(),
                                ..CompletionItem::default()
                            })
                        }
                        completion::CompletionNode::StructName(name) => {
                            items.push(CompletionItem {
                                kind: Some(CompletionItemKind::Struct),
                                label: name.to_owned(),
                                ..CompletionItem::default()
                            })
                        }
                        completion::CompletionNode::NativeName(name) => {
                            items.push(CompletionItem {
                                kind: Some(CompletionItemKind::Keyword),
                                label: name.to_owned(),
                                ..CompletionItem::default()
                            })
                        }
                        completion::CompletionNode::ListName(name)
                        | completion::CompletionNode::EnumName(name) => {
                            items.push(CompletionItem {
                                kind: Some(CompletionItemKind::Enum),
                                label: name.to_owned(),
                                ..CompletionItem::default()
                            })
                        }
                        completion::CompletionNode::ConstName(name) => items.push(CompletionItem {
                            kind: Some(CompletionItemKind::Constant),
                            label: name.to_owned(),
                            ..CompletionItem::default()
                        }),
                        completion::CompletionNode::FactoryName(name) => {
                            items.push(CompletionItem {
                                kind: Some(CompletionItemKind::Interface),
                                label: name.to_owned(),
                                ..CompletionItem::default()
                            })
                        }
                        completion::CompletionNode::StreamName(name) => {
                            items.push(CompletionItem {
                                kind: Some(CompletionItemKind::Event),
                                label: name.to_owned(),
                                ..CompletionItem::default()
                            })
                        }
                        completion::CompletionNode::Keyword => {
                            items.append(&mut get_keyword_completion());
                        }
                        completion::CompletionNode::MapType => {
                            items.append(&mut get_map_type_completion());
                        }
                    }
                }

                return Some(CompletionResponse::Array(items));
            } else {
                return Some(CompletionResponse::Array(get_keyword_completion()));
            }
        }
    }

    None
}

// TODO get type from library.
fn get_keyword_completion() -> Vec<CompletionItem> {
    let create_keyword_snippet = |name: &str| CompletionItem {
        kind: Some(CompletionItemKind::Keyword),
        label: name.to_owned(),
        insert_text: Some(format!("{} $1 {{\n{}$0\n}}", name, INDENT)),
        insert_text_format: Some(InsertTextFormat::Snippet),
        ..CompletionItem::default()
    };

    vec![
        CompletionItem {
            kind: Some(CompletionItemKind::Keyword),
            label: "library".to_owned(),
            insert_text: Some("library $0".to_owned()),
            insert_text_format: Some(InsertTextFormat::Snippet),
            ..CompletionItem::default()
        },
        CompletionItem {
            kind: Some(CompletionItemKind::Keyword),
            label: "import".to_owned(),
            insert_text: Some("import { $0 }".to_owned()),
            insert_text_format: Some(InsertTextFormat::Snippet),
            ..CompletionItem::default()
        },
        create_keyword_snippet("type"),
        create_keyword_snippet("const"),
        create_keyword_snippet("interface"),
        create_keyword_snippet("struct"),
        create_keyword_snippet("enum"),
        create_keyword_snippet("stream"),
        create_keyword_snippet("factory"),
    ]
}

// TODO get map type from library.
fn get_map_type_completion() -> Vec<CompletionItem> {
    vec![
        create_native_type_item("int"),
        create_native_type_item("string"),
    ]
}

fn create_native_type_item(name: &str) -> CompletionItem {
    CompletionItem {
        kind: Some(CompletionItemKind::Keyword),
        label: name.to_owned(),
        ..CompletionItem::default()
    }
}
