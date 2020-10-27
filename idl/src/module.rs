use super::analyzer;
use super::completion;
use super::idl_types;
use super::parser;
use super::spec;
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Arc;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ModuleError {
    #[error("Underfined error `{0}`")]
    Abort(String),
    #[error("Document already exists")]
    DuplicateDocument,
    #[error("Could not remove document")]
    RemoveDocument,
    #[error("Duplicate idlspec file")]
    DuplicateSpec,
    #[error("Invalid idlspec name")]
    MissingSpec,
    #[error("Missing idlspec file")]
    InvalidSpecName,
}

#[derive(Debug)]
struct Document {
    text: Option<String>,
    parser: Arc<Result<parser::Parser, (parser::Parser, parser::ParserError)>>,
    analyzer: Arc<Result<analyzer::Analyzer, analyzer::AnalyzerError>>,
    last_analyzer: Arc<Result<analyzer::Analyzer, analyzer::AnalyzerError>>,
}

#[derive(Debug)]
pub struct Module {
    documents: RefCell<HashMap<String, Document>>,
    spec: RefCell<Arc<Result<spec::Spec, spec::SpecError>>>,
}

impl Module {
    pub fn new() -> Module {
        Module {
            documents: RefCell::new(HashMap::new()),
            spec: RefCell::new(Arc::new(Err(spec::SpecError::Closed))),
        }
    }

    pub fn update_spec(&self, name: &str, text: &str) -> Result<(), ModuleError> {
        if name != "idlspec" {
            return Err(ModuleError::InvalidSpecName);
        }

        let mut sc = self
            .spec
            .try_borrow_mut()
            .map_err(|_| ModuleError::Abort("".to_owned()))?;
        *sc = Arc::new(spec::Spec::new(text));

        Ok(())
    }

    pub fn update_library(&self) -> Result<(), ModuleError> {
        if let Err(spec::SpecError::Closed) = self.spec.borrow().as_ref() {
            return Err(ModuleError::MissingSpec);
        }

        let mut documents = self.documents.borrow_mut();

        for doc in documents.values_mut() {
            if let Ok(parser) = doc.parser.clone().as_ref() {
                doc.analyzer = Arc::new(analyzer::Analyzer::resolve(parser, self))
            }
        }

        Ok(())
    }

    // Returns each document with an analyzer error
    pub fn get_analyze_errors(&self) -> HashMap<String, analyzer::AnalyzerError> {
        let documents = self.documents.borrow();
        let mut result = HashMap::new();
        for (name, doc) in documents.iter() {
            if let Ok(_) = &*doc.parser {
                if let Err(err) = &*doc.analyzer {
                    result.insert(name.to_owned(), err.clone());
                }
            }
        }
        result
    }

    // Returns each document parser error
    pub fn get_parser_errors(&self) -> HashMap<String, parser::ParserError> {
        let documents = self.documents.borrow();
        let mut result = HashMap::new();
        for (name, doc) in documents.iter() {
            if let Err(err) = &*doc.parser {
                result.insert(name.to_owned(), err.1.clone());
            }
        }
        result
    }

    pub fn get_spec_error(&self) -> Option<spec::SpecError> {
        if let Err(err) = self.spec.borrow().as_ref() {
            return Some(err.clone())
        }
        None
    }

    pub fn get_spec(&self) -> Arc<Result<spec::Spec, spec::SpecError>> {
        self.spec.borrow().clone()
    }

    pub fn get_document_names(&self) -> Vec<String> {
        self.documents
            .borrow()
            .keys()
            .map(|v| v.to_owned())
            .collect()
    }

    // Find any match from the spec library name, returns the name of the file.
    // To get the library name, the spec should be used instead.
    pub fn document_library_name(&self) -> Option<String> {
        let spec = &*self.get_spec();
        let library_name = spec.as_ref().ok()?.library.name.as_str();

        self.get_document_names().into_iter().find_map(|name| {
            let parser_r = self.get_parser(&name)?;
            let parser = parser_r.as_ref().as_ref().ok()?;
            let r_name = parser.get_library_name()?;
            if library_name == r_name {
                Some(name)
            } else {
                None
            }
        })
    }

    pub fn get_document_text(&self, name: &str) -> Option<String> {
        let documents = self.documents.borrow();
        let doc = documents.get(name)?;
        Some(doc.text.as_ref()?.to_string())
    }

    pub fn add_document(&self, name: &str) -> Result<(), ModuleError> {
        let mut documents = self.documents.borrow_mut();

        if documents.contains_key(name) {
            return Err(ModuleError::DuplicateDocument);
        }

        documents.insert(
            name.to_owned(),
            Document {
                text: None,
                parser: Arc::new(parser::Parser::closed()),
                analyzer: Arc::new(analyzer::Analyzer::closed()),
                last_analyzer: Arc::new(analyzer::Analyzer::closed()),
            },
        );

        Ok(())
    }

    // Adds a document but also updates it with the source text
    pub fn add_document_and_update(&self, name: &str, text: &str) -> Result<(), ModuleError> {
        let mut documents = self.documents.borrow_mut();

        if documents.contains_key(name) {
            return Err(ModuleError::DuplicateDocument);
        }

        documents.insert(
            name.to_owned(),
            Document {
                text: Some(text.to_owned()),
                parser: Arc::new(parser::Parser::parse(text)),
                analyzer: Arc::new(analyzer::Analyzer::closed()),
                last_analyzer: Arc::new(analyzer::Analyzer::closed()),
            },
        );

        Ok(())
    }

    pub fn remove_document(&self, name: &str) -> Result<(), ModuleError> {
        let mut documents = self.documents.borrow_mut();
        match documents.remove(name) {
            Some(_) => Ok(()),
            None => Err(ModuleError::RemoveDocument),
        }
    }

    pub fn update_parse(&self, name: &str, text: &str) {
        let mut documents = self.documents.borrow_mut();

        if let Some(mut doc) = documents.get_mut(name) {
            doc.text = Some(text.to_owned());
            doc.parser = Arc::new(parser::Parser::parse(text));
            if doc.analyzer.is_ok() {
                // Saves the last analyzer.
                doc.last_analyzer = doc.analyzer.clone();
            }
            doc.analyzer = Arc::new(analyzer::Analyzer::closed());
        }
    }

    pub fn update_analyze(&self, name: &str) {
        let mut documents = self.documents.borrow_mut();

        if let Some(mut doc) = documents.get_mut(name) {
            if let Ok(parser) = doc.parser.clone().as_ref() {
                doc.analyzer = Arc::new(analyzer::Analyzer::resolve(parser, self))
            }
        }
    }

    // Updates the entire module. The spec is required  for this.
    pub fn update_all_analyze(&self) -> Result<(), ModuleError> {
        if let Err(spec::SpecError::Closed) = self.spec.borrow().as_ref() {
            return Err(ModuleError::MissingSpec);
        }

        let mut documents = self.documents.borrow_mut();

        for doc in documents.values_mut() {
            if let Ok(parser) = &*doc.parser {
                doc.analyzer = Arc::new(analyzer::Analyzer::resolve(parser, self))
            }
        }

        Ok(())
    }

    pub fn get_parser(
        &self,
        name: &str,
    ) -> Option<Arc<Result<parser::Parser, (parser::Parser, parser::ParserError)>>> {
        let documents = self.documents.borrow();
        documents.get(name).map(|doc| doc.parser.clone())
    }

    pub fn get_analyzer(
        &self,
        name: &str,
    ) -> Option<Arc<Result<analyzer::Analyzer, analyzer::AnalyzerError>>> {
        let documents = self.documents.borrow();
        documents.get(name).map(|doc| doc.analyzer.clone())
    }

    pub fn try_completion(
        &self,
        name: &str,
        text: &str,
        pos: parser::Position,
    ) -> Option<completion::Completion> {
        return completion::Completion::try_complete(text, pos, self);
    }

    pub(super) fn get_all_struct_refs(&self) -> Vec<String> {
        let documents = self.documents.borrow();
        let mut names = vec![];

        for doc in documents.values() {
            match &*doc.analyzer {
                Ok(analyzer) => {
                    for node in &analyzer.nodes {
                        match node {
                            idl_types::TypeNode::TypeStruct(value) => {
                                names.push(value.ident.to_owned());
                            }
                            _ => {}
                        }
                    }
                }
                Err(_) => {
                    if let Ok(analyzer) = &*doc.last_analyzer {
                        for node in &analyzer.nodes {
                            match node {
                                idl_types::TypeNode::TypeStruct(value) => {
                                    names.push(value.ident.to_owned());
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
        }

        names
    }

    pub(super) fn get_all_type_refs(&self) -> Vec<String> {
        let documents = self.documents.borrow();
        let mut names = vec![];

        for doc in documents.values() {
            match &*doc.analyzer {
                Ok(analyzer) => {
                    for node in &analyzer.nodes {
                        match node {
                            idl_types::TypeNode::TypeList(value) => {
                                names.push(value.ident.to_owned());
                            }
                            _ => {}
                        }
                    }
                }
                Err(_) => {
                    if let Ok(analyzer) = &*doc.last_analyzer {
                        for node in &analyzer.nodes {
                            match node {
                                idl_types::TypeNode::TypeList(value) => {
                                    names.push(value.ident.to_owned());
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
        }

        names
    }

    pub(super) fn get_all_enum_refs(&self) -> Vec<String> {
        let documents = self.documents.borrow();
        let mut names = vec![];

        for doc in documents.values() {
            match &*doc.analyzer {
                Ok(analyzer) => {
                    for node in &analyzer.nodes {
                        match node {
                            idl_types::TypeNode::TypeEnum(value) => {
                                names.push(value.ident.to_owned());
                            }
                            _ => {}
                        }
                    }
                }
                Err(_) => {
                    if let Ok(analyzer) = &*doc.last_analyzer {
                        for node in &analyzer.nodes {
                            match node {
                                idl_types::TypeNode::TypeEnum(value) => {
                                    names.push(value.ident.to_owned());
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
        }

        names
    }

    pub(super) fn get_all_const_refs(&self) -> Vec<String> {
        let documents = self.documents.borrow();
        let mut names = vec![];

        for doc in documents.values() {
            match &*doc.analyzer {
                Ok(analyzer) => {
                    for node in &analyzer.nodes {
                        match node {
                            idl_types::TypeNode::TypeConst(value) => {
                                names.push(value.ident.to_owned());
                            }
                            _ => {}
                        }
                    }
                }
                Err(_) => {
                    if let Ok(analyzer) = &*doc.last_analyzer {
                        for node in &analyzer.nodes {
                            match node {
                                idl_types::TypeNode::TypeConst(value) => {
                                    names.push(value.ident.to_owned());
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
        }

        names
    }

    pub(super) fn get_all_interface_refs(&self) -> Vec<String> {
        let documents = self.documents.borrow();
        let mut names = vec![];

        for doc in documents.values() {
            match &*doc.analyzer {
                Ok(analyzer) => {
                    for node in &analyzer.nodes {
                        match node {
                            idl_types::TypeNode::TypeInterface(value) => {
                                names.push(value.ident.to_owned());
                            }
                            _ => {}
                        }
                    }
                }
                Err(_) => {
                    if let Ok(analyzer) = &*doc.last_analyzer {
                        for node in &analyzer.nodes {
                            match node {
                                idl_types::TypeNode::TypeInterface(value) => {
                                    names.push(value.ident.to_owned());
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
        }

        names
    }

    pub(super) fn get_all_native_refs(&self) -> Vec<String> {
        vec![
            parser::NativeTypes::Int.to_string(),
            parser::NativeTypes::Float.to_string(),
            parser::NativeTypes::String.to_string(),
            parser::NativeTypes::Bytes.to_string(),
            parser::NativeTypes::Bool.to_string(),
            parser::NativeTypes::None.to_string(),
        ]
    }
}
