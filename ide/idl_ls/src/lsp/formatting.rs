use super::session::Session;
use std::sync::Arc;
use tower_lsp::lsp_types::*;

use super::{file_name_from_uri, get_range_from_text_inclusive};
use idl::formatting;

pub(crate) async fn format_document(
    session: Arc<Session>,
    params: DocumentFormattingParams,
) -> Option<Vec<TextEdit>> {
    let DocumentFormattingParams {
        text_document: TextDocumentIdentifier { uri, .. },
        ..
    } = params;

    // let file_name = file_name_from_uri(&uri)?;
    // let module_state = session.module_state.lock().await;
    // let parser_s = module_state.get_idl_parser(&file_name)?;

    // if let Ok(parser) = &*parser_s {
    //     let new_text = formatting::idl::format_document(parser)?;
    //     let state = session.session_state.lock().await;
    //     let doc = state.documents.get(&file_name)?;
    //     let range = get_range_from_text_inclusive(doc.text.as_str());
    //     return Some(vec![TextEdit { range, new_text }]);
    // }

    None
}