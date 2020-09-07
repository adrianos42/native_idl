use super::analyzer;
use super::completion;
use super::idl_types;
use super::parser;

use dashmap::{
    DashMap,
};
use std::sync::Arc;

#[derive(Debug)]
pub enum ModuleError {
    Abort,
}

#[derive(Debug)]
struct Document {
    parser: Arc<Result<parser::Parser, (parser::Parser, parser::ParserError)>>,
    analyzer: Arc<Result<analyzer::Analyzer, analyzer::AnalyzerError>>,
    last_analyzer: Arc<Result<analyzer::Analyzer, analyzer::AnalyzerError>>,
}

impl Document {}

#[derive(Debug)]
pub struct Module {
    documents: DashMap<String, Document>,
}

impl Module {
    pub fn new() -> Module {
        Module {
            documents: DashMap::new(),
        }
    }

    pub async fn add_document(&self, name: &str) -> Result<(), ModuleError> {
        if !self.documents.contains_key(name) {
            self.documents.insert(
                name.to_owned(),
                Document {
                    parser: Arc::new(parser::Parser::closed()),
                    analyzer: Arc::new(analyzer::Analyzer::closed()),
                    last_analyzer: Arc::new(analyzer::Analyzer::closed()),
                },
            );

            return Ok(());
        }

        Err(ModuleError::Abort)
    }

    pub async fn remove_document(&self, name: &str) -> Result<(), ModuleError> {
        match self.documents.remove(name) {
            Some(_) => Ok(()),
            None => Err(ModuleError::Abort),
        }
    }

    pub async fn update_parse(&self, name: &str, text: &str) {
        if let Some(mut doc) = self.documents.get_mut(name) {
            doc.parser = Arc::new(parser::Parser::parse(text));
            if doc.analyzer.is_ok() {
                // Saves the last analyzer.
                doc.last_analyzer = doc.analyzer.clone();
            }
            doc.analyzer = Arc::new(analyzer::Analyzer::closed());
        }
    }

    pub async fn update_analyze(&self, name: &str) {
        if let Some(mut doc) = self.documents.get_mut(name) {
            if let Ok(parser) = doc.parser.clone().as_ref() {
                doc.analyzer = Arc::new(analyzer::Analyzer::resolve(parser, self))
            }
        }
    }

    pub async fn get_parser(
        &self,
        name: &str,
    ) -> Option<Arc<Result<parser::Parser, (parser::Parser, parser::ParserError)>>> {
        self.documents.get(name).map(|doc| doc.parser.clone())
    }

    pub async fn get_analyzer(
        &self,
        name: &str,
    ) -> Option<Arc<Result<analyzer::Analyzer, analyzer::AnalyzerError>>> {
        self.documents.get(name).map(|doc| doc.analyzer.clone())
    }

    pub async fn try_completion(
        &self,
        name: &str,
        text: &str,
        pos: parser::Position,
    ) -> Option<completion::Completion> {
        return completion::Completion::try_complete(text, pos, self);
    }

    pub(super) fn get_all_struct_refs(&self) -> Vec<String> {
        let mut names = vec![];

        for doc in self.documents.iter() {
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
        let mut names = vec![];

        for doc in self.documents.iter() {
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
        let mut names = vec![];

        for doc in self.documents.iter() {
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
        let mut names = vec![];

        for doc in self.documents.iter() {
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

    pub(super) fn get_all_stream_refs(&self) -> Vec<String> {
        let mut names = vec![];

        for doc in self.documents.iter() {
            match &*doc.analyzer {
                Ok(analyzer) => {
                    for node in &analyzer.nodes {
                        match node {
                            idl_types::TypeNode::TypeStream(value) => {
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
                                idl_types::TypeNode::TypeStream(value) => {
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

    pub(super) fn get_all_factory_refs(&self) -> Vec<String> {
        let mut names = vec![];

        for doc in self.documents.iter() {
            match &*doc.analyzer {
                Ok(analyzer) => {
                    for node in &analyzer.nodes {
                        match node {
                            idl_types::TypeNode::TypeFactory(value) => {
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
                                idl_types::TypeNode::TypeFactory(value) => {
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
        let mut names = vec![];

        for doc in self.documents.iter() {
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
