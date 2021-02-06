use tower_lsp::{lsp_types::*, Client};

use super::{create_error_diagnostic, file_name_from_uri, get_range_from_parser};

use idl::{idl::analyzer, module, idl::parser};

use std::path::Path;
use std::sync::Arc;
use tokio::fs;
use tokio::sync::Mutex;

use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq)]
enum DocumentType {
    Idl,
    IdlSpec,
}

#[derive(Debug)]
pub(crate) struct Document {
    doc_type: DocumentType,
    pub(crate) text: String,
    pub(crate) uri: Option<Url>,
}

#[derive(Debug, Default)]
pub(crate) struct SessionState {
    // Key is the file name, for example, `interface.idl`.
    pub(crate) documents: HashMap<String, Document>,
    root_uri: Option<Url>,
}

impl SessionState {
    fn set_root_uri(&mut self, root_uri: Option<Url>) {
        self.root_uri = root_uri;
    }

    fn get_root_uri(&self) -> &Option<Url> {
        &self.root_uri
    }
}

pub(crate) struct Session {
    pub(crate) client: Client,
    pub(crate) session_state: Arc<Mutex<SessionState>>,
    pub(crate) module_state: Arc<Mutex<module::Module>>,
}

impl Session {
    pub(crate) fn new(client: Client) -> Result<Self, i64> {
        let session_state = Arc::new(Mutex::new(SessionState::default()));
        let module_state = module::Module::new();

        Ok(Session {
            client,
            session_state,
            module_state: Arc::new(Mutex::new(module_state)),
        })
    }

    pub(crate) async fn open_analyzer(&self, root_uri: Option<Url>) {
        let mut inst = self.session_state.lock().await;
        inst.set_root_uri(root_uri);
    }

    pub(crate) async fn close_analyzer(&self) {}

    pub(crate) async fn start_analyer(&self) {
        let mut state = self.session_state.lock().await;

        if let Some(root_uri) = state.get_root_uri() {
            if let Ok(file_path) = root_uri.to_file_path() {
                if let Ok(mut dir) = fs::read_dir(file_path.as_path()).await {
                    while let Ok(Some(entry)) = dir.next_entry().await {
                        let path = entry.path().to_owned();
                        if path.is_file() {
                            let path = path.as_path();
                            if let Some((file_name, doc)) = Self::add_file(path).await {
                                state.documents.insert(file_name, doc);
                            }
                        }
                    }

                    let module_state = self.module_state.lock().await;
                    for (name, doc) in &state.documents {
                        if module_state.add_idl_document(name).is_ok() {
                            module_state.update_idl_parser(name, &doc.text);
                            module_state.update_idl_analyzer(name);
                        }
                    }
                }
            }
        }
    }

    async fn add_file(path: &Path) -> Option<(String, Document)> {
        if let Some(file_name) = path.file_name() {
            let file_name = file_name.to_str().expect("Invalid str.");

            if let Some(ext_s) = path.extension() {
                let ext = ext_s.to_str().expect("Invalid str.");

                match ext {
                    "idl" => {
                        if let Ok(text) = fs::read_to_string(path).await {
                            return Some((
                                file_name.to_owned(),
                                Document {
                                    doc_type: DocumentType::Idl,
                                    text,
                                    uri: None,
                                },
                            ));
                        }
                    }
                    "ids" => {
                        if let Ok(text) = fs::read_to_string(path).await {
                            return Some((
                                file_name.to_owned(),
                                Document {
                                    doc_type: DocumentType::IdlSpec,
                                    text,
                                    uri: None,
                                },
                            ));
                        }
                    }
                    _ => return None,
                }
            }
        }

        None
    }

    pub(crate) async fn open_document(&self, params: DidOpenTextDocumentParams) {
        let DidOpenTextDocumentParams {
            text_document:
                TextDocumentItem {
                    language_id,
                    text,
                    uri,
                    ..
                },
        } = params;

        if let Some(file_name) = file_name_from_uri(&uri) {
            let mut state = self.session_state.lock().await;

            if !state.documents.contains_key(&file_name) {
                match Self::add_file(&uri.to_file_path().unwrap()).await {
                    Some((file_name, doc)) => {
                        let module_state = self.module_state.lock().await;
                        if module_state.add_idl_document(&file_name).is_ok() {
                            module_state.update_ids_parser(&file_name, &doc.text);
                            module_state.update_idl_analyzer(&file_name);
                        } else {
                            self.client
                                .log_message(MessageType::Log, "error adding document")
                                .await;
                        }

                        state.documents.insert(file_name, doc);
                    }
                    None => {
                        self.client
                            .log_message(MessageType::Log, "error adding file")
                            .await
                    }
                }
            }

            if let Some(doc) = state.documents.get_mut(&file_name) {
                assert!(
                    language_id == "idl" && doc.doc_type == DocumentType::Idl
                        || language_id == "ids" && doc.doc_type == DocumentType::IdlSpec
                );

                let module_state = self.module_state.lock().await;
                let _ = module_state.add_idl_document(&file_name);

                module_state.update_idl_parser(&file_name, &text);
                if let Some(parser) = module_state.get_idl_parser(&file_name) {
                    self.update_diagnostics_parser(uri.clone(), parser).await;

                    module_state.update_idl_analyzer(&file_name);
                    if let Some(analyzer) = module_state.get_idl_analyzer(&file_name) {
                        self.update_diagnostics_analyzer(uri.clone(), analyzer)
                            .await;
                    }
                }
                doc.text = text;
            } else {
                self.client
                    .log_message(MessageType::Log, "could not find document")
                    .await;
            }
        }
    }

    pub(crate) async fn change_document(&self, params: DidChangeTextDocumentParams) {
        let DidChangeTextDocumentParams {
            text_document: VersionedTextDocumentIdentifier { uri, .. },
            mut content_changes,
        } = params;

        let TextDocumentContentChangeEvent { text, .. } = content_changes.remove(0);

        if let Some(file_name) = file_name_from_uri(&uri) {
            let mut state = self.session_state.lock().await;

            if let Some(doc) = state.documents.get_mut(&file_name) {
                let module_state = self.module_state.lock().await;
                module_state.update_idl_parser(&file_name, &text);
                if let Some(parser) = module_state.get_idl_parser(&file_name) {
                    match &*parser {
                        Err(_) => self.update_diagnostics_parser(uri.clone(), parser).await,
                        Ok(_) => {
                            module_state.update_idl_analyzer(&file_name);
                            if let Some(analyzer) = module_state.get_idl_analyzer(&file_name) {
                                self.update_diagnostics_analyzer(uri.clone(), analyzer)
                                    .await;
                            }
                        }
                    }
                }
                doc.text = text;
            }
        }
    }

    pub(crate) async fn close_document(&self, params: DidCloseTextDocumentParams) {
        let DidCloseTextDocumentParams {
            text_document: TextDocumentIdentifier { uri, .. },
        } = params;

        if let Some(file_name) = file_name_from_uri(&uri) {
            let mut state = self.session_state.lock().await;
            if let Some(doc) = &mut state.documents.get_mut(&file_name) {
                doc.uri = None;
            }
        }
    }

    async fn update_diagnostics_parser(
        &self,
        uri: Url,
        parse: Arc<Result<parser::Parser, (parser::Parser, parser::ParserError)>>,
    ) {
        let mut mes = vec![];

        if let Err((_, err)) = &*parse {
            let (message, range) = err.get_message_with_range();
            mes.push(create_error_diagnostic(
                message,
                get_range_from_parser(range),
            ));
        }

        self.client.publish_diagnostics(uri, mes, None).await;
    }

    async fn update_diagnostics_analyzer(
        &self,
        uri: Url,
        analyze: Arc<Result<analyzer::Analyzer, analyzer::AnalyzerError>>,
    ) {
        let mut mes = vec![];
        if let Err(err) = &*analyze {
            let (message, range) = err.get_message_with_range();
            mes.push(create_error_diagnostic(
                message,
                get_range_from_parser(range),
            ));
        };

        self.client.publish_diagnostics(uri, mes, None).await;
    }
}
