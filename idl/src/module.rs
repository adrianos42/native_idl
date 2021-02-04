use crate::mod_package;

use super::ids;
use std::iter::FromIterator;
use std::{borrow::BorrowMut, collections::HashMap};
use std::{collections::HashSet, sync::Arc};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PackageModuleError {
    #[error("Multiple library definition")]
    Definition,
    #[error("Missing libraries '{0}`")]
    MissingLibraries(String),
    #[error("Update Analyzer")]
    UpdateAnalyzer,
    #[error("Update Parser")]
    UpdateParser,
    #[error("Invalid document name")]
    Document,
    #[error("Package not defined")]
    PackageMissing,
}

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

    pub fn idl_all_parsers(
        &self,
        names: &[&str],
    ) -> Option<
        Vec<
            Arc<Result<super::parser::Parser, (super::parser::Parser, super::parser::ParserError)>>,
        >,
    > {
        let mut result = vec![];
        for name in names {
            let doc = self.idl_documents.get(*name)?.parser.clone();
            result.push(doc);
        }
        Some(result)
    }

    pub fn idl_all_analyzers(
        &self,
        names: &[&str],
    ) -> Option<Vec<Arc<Result<super::analyzer::Analyzer, super::analyzer::AnalyzerError>>>> {
        let mut result = vec![];
        for name in names {
            let doc = self.idl_documents.get(*name)?.analyzer.clone();
            result.push(doc);
        }
        Some(result)
    }

    // Returns the value of files that are defined
    // All documents must have valid parser
    pub fn idl_valid_names(&self) -> Result<Vec<String>, PackageModuleError> {
        let mut result = vec![];

        match &self.ids_document {
            Some(ids_doc) => match &*ids_doc.1.analyzer {
                Ok(ids_analyzer) => {
                    let package = ids_analyzer.get_package();
                    let package_name = package.name();

                    let lib_names = package.lib_names().unwrap_or_else(|| vec![]);

                    for (_, doc) in self.idl_documents.iter() {
                        let idl_parser = match &*doc.parser {
                            // TODO
                            Ok(value) => value,
                            Err(_) => return Err(PackageModuleError::UpdateParser),
                        };
                        let library_name = idl_parser.library_name().ok_or(PackageModuleError::UpdateAnalyzer)?;
                        if library_name == package_name || lib_names.contains(&library_name) {
                            result.push(library_name.clone());
                        }
                    }

                    Ok(result)
                }
                Err(_) => Err(PackageModuleError::UpdateAnalyzer),
            },
            None => Err(PackageModuleError::PackageMissing),
        }
    }

    // Returns the idl document names, only if all documents have a valid syntax.
    // This means that the entire package is correct.
    // - All library defined in ids are valid.
    // - There's only one definition of the same library.
    // - There's at most one library with the same name as the package.
    pub fn idl_documents_all_valid_names(&self) -> Result<Vec<String>, PackageModuleError> {
        let mut result_names = vec![];

        match &self.ids_document {
            Some(ids_doc) => match &*ids_doc.1.analyzer {
                Ok(ids_analyzer) => {
                    let package = ids_analyzer.get_package();
                    let package_name = package.name();

                    let mut analayzer_lib_names = vec![];

                    for (name, doc) in self.idl_documents.iter() {
                        match &*doc.parser {
                            Ok(idl_parser) => {
                                let p_library_name =
                                    idl_parser.library_name().ok_or(PackageModuleError::Document)?;
                                match &*doc.analyzer {
                                    Ok(idl_analyzer) => {
                                        let library_name = idl_analyzer.library_name();
                                        if library_name != p_library_name {
                                            return Err(PackageModuleError::Document);
                                        }
                                        if library_name != package_name {
                                            analayzer_lib_names
                                                .push((name.to_owned(), library_name));
                                        } else {
                                            result_names.push(name.to_owned());
                                        }
                                    }
                                    Err(_) => {
                                        if p_library_name == package_name {
                                            return Err(PackageModuleError::UpdateAnalyzer);
                                        }
                                    }
                                }
                            }
                            Err(_) => return Err(PackageModuleError::UpdateAnalyzer),
                        }
                    }

                    // Check all the library names, but also the package name when it's the
                    // as in a library definition.
                    let mut checked_names = HashSet::new();
                    let lib_names = HashSet::from_iter(
                        package.lib_names().unwrap_or_else(|| vec![]).into_iter(),
                    );

                    let mut result_name_pakage_name = None; // When the library name is the same as the package name.

                    for (name, library_name) in analayzer_lib_names {
                        if lib_names.contains(&library_name) {
                            if checked_names.contains(&library_name) {
                                return Err(PackageModuleError::Definition.into());
                            }

                            checked_names.insert(library_name);
                            result_names.push(name);
                        } else if library_name == package_name {
                            if result_name_pakage_name.is_some() {
                                return Err(
                                    PackageModuleError::MissingLibraries(library_name).into()
                                );
                            }
                            result_name_pakage_name = Some(name);
                        }
                    }

                    if !checked_names.is_superset(&lib_names) {
                        let diff =
                            lib_names
                                .difference(&checked_names)
                                .fold("".to_owned(), |p, v| {
                                    if !p.is_empty() {
                                        format!("{}, {}", p, v.to_owned())
                                    } else {
                                        format!("{}", v.to_owned())
                                    }
                                });
                        return Err(PackageModuleError::MissingLibraries(diff).into());
                    }

                    if let Some(name) = result_name_pakage_name {
                        result_names.push(name);
                    }

                    Ok(result_names)
                }
                Err(_) => Err(PackageModuleError::UpdateAnalyzer),
            },
            None => Err(PackageModuleError::PackageMissing),
        }
    }

    pub fn idl_documents_are_all_valid(&self) -> Result<(), PackageModuleError> {
        match &self.ids_document {
            Some(ids_doc) => match &*ids_doc.1.analyzer {
                Ok(ids_analyzer) => {
                    let package = ids_analyzer.get_package();
                    let package_name = package.name();

                    let mut checked_names = HashSet::new();
                    let lib_names = HashSet::from_iter(
                        package.lib_names().unwrap_or_else(|| vec![]).into_iter(),
                    );

                    for (_, doc) in self.idl_documents.iter() {
                        match &*doc.parser {
                            Ok(idl_parser) => {
                                let p_library_name =
                                    idl_parser.library_name().ok_or(PackageModuleError::Document)?;
                                match &*doc.analyzer {
                                    Ok(idl_analyzer) => {
                                        let library_name = idl_analyzer.library_name();
                                        if library_name != p_library_name {
                                            return Err(PackageModuleError::Document);
                                        }
                                        if library_name != package_name {
                                            checked_names.insert(library_name);
                                        }
                                    }
                                    Err(_) => {
                                        if p_library_name == package_name {
                                            return Err(PackageModuleError::UpdateAnalyzer);
                                        }
                                    }
                                }
                            }
                            Err(_) => return Err(PackageModuleError::UpdateAnalyzer),
                        }
                    }

                    if !checked_names.is_superset(&lib_names) {
                        let diff =
                            lib_names
                                .difference(&checked_names)
                                .fold("".to_owned(), |p, v| {
                                    if !p.is_empty() {
                                        format!("{}, {}", p, v.to_owned())
                                    } else {
                                        format!("{}", v.to_owned())
                                    }
                                });
                        return Err(
                            PackageModuleError::MissingLibraries(format!("{}", diff)).into()
                        );
                    }

                    Ok(())
                }
                Err(_) => Err(PackageModuleError::UpdateAnalyzer),
            },
            None => Err(PackageModuleError::PackageMissing),
        }
    }

    // Returns the names not defined as libraries in package.
    // This considers not only the files with valid syntax, but any file with a library name.
    pub fn idl_documents_names_not_in_package(&self) -> Result<Option<Vec<String>>, ModuleError> {
        match &self.ids_document {
            Some(ids_doc) => match &*ids_doc.1.analyzer {
                Ok(ids_analyzer) => {
                    let package = ids_analyzer.get_package();
                    let package_name = package.name();
                    let lib_names = package.lib_names().unwrap_or_else(|| vec![]);
                    let mut analayzer_lib_names = vec![];
                    let mut result_names = vec![];

                    for (name, doc) in self.idl_documents.iter() {
                        match &*doc.parser {
                            Ok(idl_parser) => {
                                let library_name =
                                    idl_parser.library_name().ok_or(ModuleError::Document)?;

                                if library_name != package_name {
                                    analayzer_lib_names.push((name.to_owned(), library_name))
                                }
                            }
                            Err(_) => {},
                        }
                    }

                    for (name, library_name) in analayzer_lib_names {
                        if !lib_names.contains(&library_name) {
                            result_names.push(name)
                        }
                    }

                    Ok(Some(result_names))
                }
                Err(_) => Ok(None),
            },
            None => Err(ModuleError::PackageMissing),
        }
    }

    pub fn all_document_names(&self) -> Result<Vec<String>, ModuleError> {
        let mut result: Vec<String> = self.idl_documents.keys().map(|v| v.to_owned()).collect();
        if let Some(doc) = &self.ids_document {
            if result.contains(&doc.0) {
                return Err(ModuleError::DuplicateDocument);
            }
            result.push(doc.0.to_owned());
        }
        Ok(result)
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
                        let package = analyzer.get_package();
                        let package_name = package.name();

                        if package_name != parse_package_name {
                            return Err(ModuleError::InvalidPackageName);
                        }

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
