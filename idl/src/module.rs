use crate::mod_package;

use super::ids;
use std::{collections::HashSet, sync::Arc};
use std::{borrow::BorrowMut, collections::HashMap};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ModuleError {
    #[error("Underfined error `{0}`")]
    Abort(String),
    #[error("Document already exists")]
    DuplicateDocument,
    #[error("Could not remove document")]
    RemoveDocument,
    #[error("Invalid package name")]
    PackageInvalid,
    #[error("Package not defined")]
    PackageMissing,
    #[error("Invalid document name")]
    Document,
    #[error("Could not find package name")]
    PackageName,
    #[error("Invalid package name")]
    InvalidPackageName,
}

#[derive(Debug)]
struct IdsDocument {
    text: Option<String>,
    parser: Arc<Result<ids::parser::Parser, (ids::parser::Parser, ids::parser::ParserError)>>,
    analyzer: Arc<Result<ids::analyzer::Analyzer, ids::analyzer::AnalyzerError>>,
    last_analyzer: Arc<Result<ids::analyzer::Analyzer, ids::analyzer::AnalyzerError>>,
}

#[derive(Debug)]
struct IdlDocument {
    text: Option<String>,
    parser: Arc<Result<super::parser::Parser, (super::parser::Parser, super::parser::ParserError)>>,
    analyzer: Arc<Result<super::analyzer::Analyzer, super::analyzer::AnalyzerError>>,
    last_analyzer: Arc<Result<super::analyzer::Analyzer, super::analyzer::AnalyzerError>>,
}

#[derive(Debug, Default)]
pub struct Module {
    idl_documents: HashMap<String, IdlDocument>,
    ids_document: Option<(String, IdsDocument)>,
    modules: Vec<Module>,
}

impl Module {
    pub fn new() -> Module {
        Module::default()
    }

    // Returns each document with an analyzer error
    // TODO Ignore closed error
    pub fn idl_analyze_errors(&self) -> HashMap<String, super::analyzer::AnalyzerError> {
        let mut result = HashMap::new();
        for (name, doc) in self.idl_documents.iter() {
            if let Ok(_) = &*doc.parser {
                if let Err(err) = &*doc.analyzer {
                    result.insert(name.to_owned(), err.clone());
                }
            }
        }
        result
    }

    // Returns each document parser error
    pub fn idl_parser_errors(&self) -> HashMap<String, super::parser::ParserError> {
        let mut result = HashMap::new();
        for (name, doc) in self.idl_documents.iter() {
            if let Err(err) = &*doc.parser {
                result.insert(name.to_owned(), err.1.clone());
            }
        }
        result
    }

    pub fn idl_document_text(&self, name: &str) -> Option<String> {
        let doc = self.idl_documents.get(name)?;
        Some(doc.text.as_ref()?.to_string())
    }

    // pub fn add_idl_document(&mut self, name: &str) -> Result<(), ModuleError> {
    //     if self.idl_documents.contains_key(name) {
    //         return Err(ModuleError::DuplicateDocument);
    //     }

    //     self.idl_documents.insert(
    //         name.to_owned(),
    //         IdlDocument {
    //             text: None,
    //             parser: Arc::new(super::parser::Parser::closed()),
    //             analyzer: Arc::new(super::analyzer::Analyzer::closed()),
    //             last_analyzer: Arc::new(super::analyzer::Analyzer::closed()),
    //         },
    //     );

    //     Ok(())
    // }

    // Adds a document but also updates it with the source text
    // pub fn add_idl_document_and_update(
    //     &mut self,
    //     name: &str,
    //     text: &str,
    // ) -> Result<(), ModuleError> {
    //     if self.idl_documents.contains_key(name) {
    //         return Err(ModuleError::DuplicateDocument);
    //     }

    //     Ok(())
    // }

    pub fn remove_idl_document(&mut self, name: &str) -> Result<(), ModuleError> {
        match self.idl_documents.remove(name) {
            Some(_) => Ok(()),
            None => Err(ModuleError::RemoveDocument),
        }
    }

    pub fn replace_idl_document(&mut self, name: &str, text: &str) {
        self.idl_documents.insert(
            name.to_owned(),
            IdlDocument {
                text: Some(text.to_owned()),
                parser: Arc::new(super::parser::Parser::parse(text)),
                analyzer: Arc::new(super::analyzer::Analyzer::closed()),
                last_analyzer: Arc::new(super::analyzer::Analyzer::closed()),
            },
        );
    }

    pub fn idl_parser(
        &self,
        name: &str,
    ) -> Option<
        Arc<Result<super::parser::Parser, (super::parser::Parser, super::parser::ParserError)>>,
    > {
        self.idl_documents.get(name).map(|doc| doc.parser.clone())
    }

    pub fn idl_analyzer(
        &self,
        name: &str,
    ) -> Option<Arc<Result<super::analyzer::Analyzer, super::analyzer::AnalyzerError>>> {
        self.idl_documents.get(name).map(|doc| doc.analyzer.clone())
    }

    pub fn all_document_names(&self) -> Vec<String> {
        let mut result = HashSet::<String>::from(self.idl_documents
            .keys()
            .map(|v| v.to_owned())
            .collect());
        if let Some(doc) = &self.ids_document {
            result.insert(doc.0.to_owned());
        }
        result.into_iter().collect()
    }

    // Updates the entire module. The ids document is required for this.
    pub fn update(&mut self) -> Result<(), ModuleError> {
        let parse_package_name = self.package_name()?;

        match &mut self.ids_document {
            Some(ids_doc) => match &*ids_doc.1.parser {
                Ok(parser) => {
                    if ids_doc.1.analyzer.is_ok() {
                        // TODO
                        ids_doc.1.last_analyzer = ids_doc.1.analyzer.clone()
                    }

                    let ids_analyzer = ids::analyzer::Analyzer::resolve(parser);

                    if let Ok(analyzer) = &ids_analyzer {
                        if analyzer.get_package().name() != parse_package_name {
                            return Err(ModuleError::InvalidPackageName);
                        }

                        let package = analyzer.get_package();
                        let package_name = package.name();
                        let lib_names = package.lib_names().unwrap_or_else(|| vec![]);

                        for (_, doc) in self.idl_documents.iter_mut() {
                            if let Ok(parser) = &*doc.parser {
                                if let Some(library_name) = parser.library_name() {
                                    if library_name == package_name
                                        || lib_names.contains(&library_name)
                                    {
                                        if doc.analyzer.is_ok() {
                                            doc.last_analyzer = doc.analyzer.clone();
                                        }

                                        doc.analyzer = Arc::new(
                                            super::analyzer::Analyzer::resolve(parser, None),
                                        );
                                    }
                                }
                            }
                        }
                    }

                    ids_doc.1.analyzer = Arc::new(ids_analyzer);

                    Ok(())
                }
                Err(_) => Ok(()),
            },
            None => Err(ModuleError::PackageMissing),
        }
    }

    pub fn ids_analyze_errors(
        &self,
    ) -> Result<Option<(String, ids::analyzer::AnalyzerError)>, ModuleError> {
        match &self.ids_document {
            Some(doc) => match &*doc.1.parser {
                Ok(_) => match &*doc.1.analyzer {
                    Err(err) => Ok(Some((doc.0.to_owned(), err.clone()))),
                    _ => Ok(None),
                },
                Err(_) => Ok(None),
            },
            None => Err(ModuleError::PackageMissing),
        }
    }

    pub fn ids_parser_errors(
        &self,
    ) -> Result<Option<(String, ids::parser::ParserError)>, ModuleError> {
        match &self.ids_document {
            Some(doc) => match &*doc.1.parser {
                Err(err) => Ok(Some((doc.0.to_owned(), err.1.clone()))),
                _ => Ok(None),
            },
            None => Err(ModuleError::PackageMissing),
        }
    }

    pub fn ids_document_text(&self, name: &str) -> Result<Option<String>, ModuleError> {
        match &self.ids_document {
            Some(doc) => {
                if doc.0 != name {
                    return Err(ModuleError::RemoveDocument); // TODO
                }

                if let Some(txt) = doc.1.text.as_ref() {
                    return Ok(Some(txt.to_owned()));
                }
            }
            None => return Err(ModuleError::PackageMissing),
        }

        Ok(None)
    }

    pub fn ids_document_name(&self) -> Result<Option<String>, ModuleError> {
        match &self.ids_document {
            Some(doc) => Ok(Some(doc.0.to_owned())),
            None => Err(ModuleError::PackageMissing),
        }
    }

    // pub fn replace_ids_document(&mut self, name: &str) -> Result<(), ModuleError> {
    //     self.ids_document.replace((
    //         name.to_owned(),
    //         IdsDocument {
    //             text: None,
    //             parser: Arc::new(ids::parser::Parser::closed()),
    //             analyzer: Arc::new(ids::analyzer::Analyzer::closed()),
    //             last_analyzer: Arc::new(ids::analyzer::Analyzer::closed()),
    //         },
    //     ));

    //     Ok(())
    // }

    // Returns the previous name, if it had one.
    pub fn replace_ids_document(&mut self, name: &str, text: &str) -> Option<String> {
        self.ids_document
            .replace((
                name.to_owned(),
                IdsDocument {
                    text: Some(text.to_owned()),
                    parser: Arc::new(ids::parser::Parser::parse(text)),
                    analyzer: Arc::new(ids::analyzer::Analyzer::closed()),
                    last_analyzer: Arc::new(ids::analyzer::Analyzer::closed()),
                },
            ))
            .map(|v| v.0)
    }

    pub fn has_ids_document(&self) -> bool {
        self.ids_document.is_some()
    }

    pub fn remove_ids_document(&mut self, name: &str) -> Result<(), ModuleError> {
        match self.ids_document.take() {
            Some(value) => {
                if value.0 != name {
                    return Err(ModuleError::Document);
                }
                Ok(())
            }
            None => Err(ModuleError::RemoveDocument),
        }
    }

    pub fn update_ids_analyzer(&mut self) -> Result<(), ModuleError> {
        match &mut self.ids_document {
            Some(document) => {
                let doc = &mut document.1;
                match &*doc.parser {
                    Ok(parser) => {
                        doc.analyzer = Arc::new(ids::analyzer::Analyzer::resolve(parser));
                        Ok(())
                    }
                    Err(_) => Ok(()),
                }
            }
            None => Err(ModuleError::PackageMissing),
        }
    }

    pub fn ids_parser(
        &self,
    ) -> Result<
        Arc<Result<ids::parser::Parser, (ids::parser::Parser, ids::parser::ParserError)>>,
        ModuleError,
    > {
        match &self.ids_document {
            Some(document) => Ok(document.1.parser.clone()),
            None => Err(ModuleError::PackageMissing),
        }
    }

    pub fn ids_analyzer(
        &self,
    ) -> Result<Arc<Result<ids::analyzer::Analyzer, ids::analyzer::AnalyzerError>>, ModuleError>
    {
        match &self.ids_document {
            Some(document) => Ok(document.1.analyzer.clone()),
            None => Err(ModuleError::PackageMissing),
        }
    }

    // Tries to get the package name, even if it's not everything correct.
    pub fn package_name(&self) -> Result<String, ModuleError> {
        match &self.ids_document {
            Some(document) => {
                let parse = match &*document.1.parser {
                    Ok(parse) => parse,
                    Err((parse, _)) => parse,
                };

                match parse.package_name() {
                    Some(value) => Ok(value),
                    None => Err(ModuleError::PackageName),
                }
            }
            None => Err(ModuleError::PackageMissing),
        }
    }
}
