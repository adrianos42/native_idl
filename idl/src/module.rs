use super::idl;
use super::ids;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
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
struct IdlDocument {
    text: Option<String>,
    parser: Arc<Result<idl::parser::Parser, (idl::parser::Parser, idl::parser::ParserError)>>,
    analyzer: Arc<Result<idl::analyzer::Analyzer, idl::analyzer::AnalyzerError>>,
    last_analyzer: Arc<Result<idl::analyzer::Analyzer, idl::analyzer::AnalyzerError>>,
}

#[derive(Debug)]
struct IdsDocument {
    text: Option<String>,
    parser: Arc<Result<ids::parser::Parser, (ids::parser::Parser, ids::parser::ParserError)>>,
    analyzer: Arc<Result<ids::analyzer::Analyzer, ids::analyzer::AnalyzerError>>,
    last_analyzer: Arc<Result<ids::analyzer::Analyzer, ids::analyzer::AnalyzerError>>,
}

#[derive(Debug, Default)]
pub struct Module {
    idl_documents: RwLock<HashMap<String, IdlDocument>>,
    ids_documents: RwLock<HashMap<String, IdsDocument>>,
}

impl Module {
    pub fn new() -> Module {
        Module::default()
    }

    // Returns each document with an analyzer error
    pub fn get_idl_analyze_errors(&self) -> HashMap<String, idl::analyzer::AnalyzerError> {
        let documents = self.idl_documents.read().unwrap();
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
    pub fn get_idl_parser_errors(&self) -> HashMap<String, idl::parser::ParserError> {
        let documents = self.idl_documents.read().unwrap();
        let mut result = HashMap::new();
        for (name, doc) in documents.iter() {
            if let Err(err) = &*doc.parser {
                result.insert(name.to_owned(), err.1.clone());
            }
        }
        result
    }

    pub fn get_idl_document_text(&self, name: &str) -> Option<String> {
        let documents = self.idl_documents.read().unwrap();
        let doc = documents.get(name)?;
        Some(doc.text.as_ref()?.to_string())
    }

    pub fn add_idl_document(&self, name: &str) -> Result<(), ModuleError> {
        let mut documents = self.idl_documents.write().unwrap();

        if documents.contains_key(name) {
            return Err(ModuleError::DuplicateDocument);
        }

        documents.insert(
            name.to_owned(),
            IdlDocument {
                text: None,
                parser: Arc::new(idl::parser::Parser::closed()),
                analyzer: Arc::new(idl::analyzer::Analyzer::closed()),
                last_analyzer: Arc::new(idl::analyzer::Analyzer::closed()),
            },
        );

        Ok(())
    }

    // Adds a document but also updates it with the source text
    pub fn add_idl_document_and_update(&self, name: &str, text: &str) -> Result<(), ModuleError> {
        let mut documents = self.idl_documents.write().unwrap();

        if documents.contains_key(name) {
            return Err(ModuleError::DuplicateDocument);
        }

        documents.insert(
            name.to_owned(),
            IdlDocument {
                text: Some(text.to_owned()),
                parser: Arc::new(idl::parser::Parser::parse(text)),
                analyzer: Arc::new(idl::analyzer::Analyzer::closed()),
                last_analyzer: Arc::new(idl::analyzer::Analyzer::closed()),
            },
        );

        Ok(())
    }

    pub fn remove_idl_document(&self, name: &str) -> Result<(), ModuleError> {
        let mut documents = self.idl_documents.write().unwrap();
        match documents.remove(name) {
            Some(_) => Ok(()),
            None => Err(ModuleError::RemoveDocument),
        }
    }

    pub fn update_idl_parser(&self, name: &str, text: &str) {
        let mut documents = self.idl_documents.write().unwrap();

        if let Some(mut doc) = documents.get_mut(name) {
            doc.text = Some(text.to_owned());
            doc.parser = Arc::new(idl::parser::Parser::parse(text));
            if doc.analyzer.is_ok() {
                // Saves the last analyzer.
                doc.last_analyzer = doc.analyzer.clone();
            }
            doc.analyzer = Arc::new(idl::analyzer::Analyzer::closed());
        }
    }

    pub fn update_idl_analyzer(&self, name: &str) {
        let mut documents = self.idl_documents.write().unwrap();

        if let Some(mut doc) = documents.get_mut(name) {
            if let Ok(parser) = doc.parser.clone().as_ref() {
                doc.analyzer = Arc::new(idl::analyzer::Analyzer::resolve(parser, Some(self)))
            }
        }
    }

    // Updates the entire module. The spec is required for this.
    pub fn update_all_idl_analyzer(&self) -> Result<(), ModuleError> {
        let mut documents = self.idl_documents.write().unwrap();

        // TODO match the ids document

        for doc in documents.values_mut() {
            if let Ok(parser) = &*doc.parser {
                doc.analyzer = Arc::new(idl::analyzer::Analyzer::resolve(parser, Some(self)))
            }
        }

        Ok(())
    }

    pub fn get_idl_parser(
        &self,
        name: &str,
    ) -> Option<Arc<Result<idl::parser::Parser, (idl::parser::Parser, idl::parser::ParserError)>>> {
        let documents = self.idl_documents.read().unwrap();
        documents.get(name).map(|doc| doc.parser.clone())
    }

    pub fn get_idl_analyzer(
        &self,
        name: &str,
    ) -> Option<Arc<Result<idl::analyzer::Analyzer, idl::analyzer::AnalyzerError>>> {
        let documents = self.idl_documents.read().unwrap();
        documents.get(name).map(|doc| doc.analyzer.clone())
    }

    pub fn get_all_struct_refs(&self) -> Vec<String> {
        let documents = self.idl_documents.read().unwrap();
        let mut names = vec![];

        for doc in documents.values() {
            match &*doc.analyzer {
                Ok(analyzer) => {
                    for node in &analyzer.nodes {
                        match node {
                            idl::idl_types::TypeNode::TypeStruct(value) => {
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
                                idl::idl_types::TypeNode::TypeStruct(value) => {
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

    pub fn get_all_type_refs(&self) -> Vec<String> {
        let documents = self.idl_documents.read().unwrap();
        let mut names = vec![];

        for doc in documents.values() {
            match &*doc.analyzer {
                Ok(analyzer) => {
                    for node in &analyzer.nodes {
                        match node {
                            idl::idl_types::TypeNode::TypeList(value) => {
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
                                idl::idl_types::TypeNode::TypeList(value) => {
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

    pub fn get_all_enum_refs(&self) -> Vec<String> {
        let documents = self.idl_documents.read().unwrap();
        let mut names = vec![];

        for doc in documents.values() {
            match &*doc.analyzer {
                Ok(analyzer) => {
                    for node in &analyzer.nodes {
                        match node {
                            idl::idl_types::TypeNode::TypeEnum(value) => {
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
                                idl::idl_types::TypeNode::TypeEnum(value) => {
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

    pub fn get_all_const_refs(&self) -> Vec<String> {
        let documents = self.idl_documents.read().unwrap();
        let mut names = vec![];

        for doc in documents.values() {
            match &*doc.analyzer {
                Ok(analyzer) => {
                    for node in &analyzer.nodes {
                        match node {
                            idl::idl_types::TypeNode::TypeConst(value) => {
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
                                idl::idl_types::TypeNode::TypeConst(value) => {
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

    pub fn get_all_interface_refs(&self) -> Vec<String> {
        let documents = self.idl_documents.read().unwrap();
        let mut names = vec![];

        for doc in documents.values() {
            match &*doc.analyzer {
                Ok(analyzer) => {
                    for node in &analyzer.nodes {
                        match node {
                            idl::idl_types::TypeNode::TypeInterface(value) => {
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
                                idl::idl_types::TypeNode::TypeInterface(value) => {
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

    pub fn get_all_native_refs() -> Vec<String> {
        vec![
            idl::parser::NativeTypes::Int.to_string(),
            idl::parser::NativeTypes::Float.to_string(),
            idl::parser::NativeTypes::String.to_string(),
            idl::parser::NativeTypes::Bytes.to_string(),
            idl::parser::NativeTypes::Bool.to_string(),
            idl::parser::NativeTypes::None.to_string(),
        ]
    }

    pub fn get_document_names(&self) -> Vec<String> {
        self.ids_documents
            .read().unwrap()
            .keys()
            .map(|v| v.to_owned())
            .collect()
    }

    pub fn get_ids_analyze_errors(&self) -> HashMap<String, ids::analyzer::AnalyzerError> {
        let documents = self.ids_documents.read().unwrap();
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

    pub fn get_ids_parser_errors(&self) -> HashMap<String, ids::parser::ParserError> {
        let documents = self.ids_documents.read().unwrap();
        let mut result = HashMap::new();
        for (name, doc) in documents.iter() {
            if let Err(err) = &*doc.parser {
                result.insert(name.to_owned(), err.1.clone());
            }
        }
        result
    }

    pub fn get_ids_document_text(&self, name: &str) -> Option<String> {
        let documents = self.ids_documents.read().unwrap();
        let doc = documents.get(name)?;
        Some(doc.text.as_ref()?.to_string())
    }

    pub fn add_ids_document(&self, name: &str) -> Result<(), ModuleError> {
        let mut documents = self.ids_documents.write().unwrap();

        if documents.contains_key(name) {
            return Err(ModuleError::DuplicateDocument);
        }

        documents.insert(
            name.to_owned(),
            IdsDocument {
                text: None,
                parser: Arc::new(ids::parser::Parser::closed()),
                analyzer: Arc::new(ids::analyzer::Analyzer::closed()),
                last_analyzer: Arc::new(ids::analyzer::Analyzer::closed()),
            },
        );

        Ok(())
    }

    pub fn add_ids_document_and_update(&self, name: &str, text: &str) -> Result<(), ModuleError> {
        let mut documents = self.ids_documents.write().unwrap();

        if documents.contains_key(name) {
            return Err(ModuleError::DuplicateDocument);
        }

        documents.insert(
            name.to_owned(),
            IdsDocument {
                text: Some(text.to_owned()),
                parser: Arc::new(ids::parser::Parser::parse(text)),
                analyzer: Arc::new(ids::analyzer::Analyzer::closed()),
                last_analyzer: Arc::new(ids::analyzer::Analyzer::closed()),
            },
        );

        Ok(())
    }

    pub fn remove_ids_document(&self, name: &str) -> Result<(), ModuleError> {
        let mut documents = self.ids_documents.write().unwrap();
        match documents.remove(name) {
            Some(_) => Ok(()),
            None => Err(ModuleError::RemoveDocument),
        }
    }

    pub fn update_ids_parser(&self, name: &str, text: &str) {
        let mut documents = self.ids_documents.write().unwrap();

        if let Some(mut doc) = documents.get_mut(name) {
            doc.text = Some(text.to_owned());
            doc.parser = Arc::new(ids::parser::Parser::parse(text));
            if doc.analyzer.is_ok() {
                // Saves the last analyzer.
                doc.last_analyzer = doc.analyzer.clone();
            }
            doc.analyzer = Arc::new(ids::analyzer::Analyzer::closed());
        }
    }

    pub fn update_ids_analyzer(&self, name: &str) {
        let mut documents = self.ids_documents.write().unwrap();

        if let Some(mut doc) = documents.get_mut(name) {
            if let Ok(parser) = doc.parser.clone().as_ref() {
                doc.analyzer = Arc::new(ids::analyzer::Analyzer::resolve(parser, Some(self)))
            }
        }
    }

    pub fn get_ids_parser(
        &self,
        name: &str,
    ) -> Option<Arc<Result<ids::parser::Parser, (ids::parser::Parser, ids::parser::ParserError)>>> {
        let documents = self.ids_documents.read().unwrap();
        documents.get(name).map(|doc| doc.parser.clone())
    }

    pub fn get_ids_analyzer(
        &self,
        name: &str,
    ) -> Option<Arc<Result<ids::analyzer::Analyzer, ids::analyzer::AnalyzerError>>> {
        let documents = self.ids_documents.read().unwrap();
        documents.get(name).map(|doc| doc.analyzer.clone())
    }
}