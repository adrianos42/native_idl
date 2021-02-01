use super::ids;
use std::collections::HashMap;
use std::sync::{Arc, Weak};
use thiserror::Error;



pub struct Package {
    package_name: String,
//    libraries: HashMap<String, Idl>,
}

// impl Package {
//      // Returns each document with an analyzer error
//     pub fn get_idl_analyze_errors(&self) -> HashMap<String, super::analyzer::AnalyzerError> {
//         let mut result = HashMap::new();
//         for (name, doc) in self.idl_documents.iter() {
//             if let Ok(_) = &*doc.parser {
//                 if let Err(err) = &*doc.analyzer {
//                     result.insert(name.to_owned(), err.clone());
//                 }
//             }
//         }
//         result
//     }

//     // Returns each document parser error
//     pub fn get_idl_parser_errors(&self) -> HashMap<String, super::parser::ParserError> {
//         let mut result = HashMap::new();
//         for (name, doc) in self.idl_documents.iter() {
//             if let Err(err) = &*doc.parser {
//                 result.insert(name.to_owned(), err.1.clone());
//             }
//         }
//         result
//     }

//     pub fn all_type_refs(&self) -> Vec<String> {
//         let mut names = vec![];

//         for doc in self.libraries.values() {
//             match &*doc.analyzer {
//                 Ok(analyzer) => {
//                     for node in &analyzer.nodes {
//                         match node {
//                             super::idl_nodes::IdlNode::TypeList(value) => {
//                                 names.push(value.ident.to_owned());
//                             }
//                             _ => {}
//                         }
//                     }
//                 }
//                 Err(_) => {
//                     if let Ok(analyzer) = &*doc.last_analyzer {
//                         for node in &analyzer.nodes {
//                             match node {
//                                 super::idl_nodes::IdlNode::TypeList(value) => {
//                                     names.push(value.ident.to_owned());
//                                 }
//                                 _ => {}
//                             }
//                         }
//                     }
//                 }
//             }
//         }

//         names
//     }

//     pub fn all_enum_refs(&self) -> Vec<String> {
//         let mut names = vec![];

//         for doc in self.libraries.values() {
//             match &*doc.analyzer {
//                 Ok(analyzer) => {
//                     for node in &analyzer.nodes {
//                         match node {
//                             super::idl_nodes::IdlNode::TypeEnum(value) => {
//                                 names.push(value.ident.to_owned());
//                             }
//                             _ => {}
//                         }
//                     }
//                 }
//                 Err(_) => {
//                     if let Ok(analyzer) = &*doc.last_analyzer {
//                         for node in &analyzer.nodes {
//                             match node {
//                                 super::idl_nodes::IdlNode::TypeEnum(value) => {
//                                     names.push(value.ident.to_owned());
//                                 }
//                                 _ => {}
//                             }
//                         }
//                     }
//                 }
//             }
//         }

//         names
//     }

//     pub fn all_const_refs(&self) -> Vec<String> {
//         let mut names = vec![];

//         for doc in self.libraries.values() {
//             match &*doc.analyzer {
//                 Ok(analyzer) => {
//                     for node in &analyzer.nodes {
//                         match node {
//                             super::idl_nodes::IdlNode::TypeConst(value) => {
//                                 names.push(value.ident.to_owned());
//                             }
//                             _ => {}
//                         }
//                     }
//                 }
//                 Err(_) => {
//                     if let Ok(analyzer) = &*doc.last_analyzer {
//                         for node in &analyzer.nodes {
//                             match node {
//                                 super::idl_nodes::IdlNode::TypeConst(value) => {
//                                     names.push(value.ident.to_owned());
//                                 }
//                                 _ => {}
//                             }
//                         }
//                     }
//                 }
//             }
//         }

//         names
//     }

//     pub fn all_interface_refs(&self) -> Vec<String> {
//         let mut names = vec![];

//         for doc in self.libraries.values() {
//             match &*doc.analyzer {
//                 Ok(analyzer) => {
//                     for node in &analyzer.nodes {
//                         match node {
//                             super::idl_nodes::IdlNode::TypeInterface(value) => {
//                                 names.push(value.ident.to_owned());
//                             }
//                             _ => {}
//                         }
//                     }
//                 }
//                 Err(_) => {
//                     if let Ok(analyzer) = &*doc.last_analyzer {
//                         for node in &analyzer.nodes {
//                             match node {
//                                 super::idl_nodes::IdlNode::TypeInterface(value) => {
//                                     names.push(value.ident.to_owned());
//                                 }
//                                 _ => {}
//                             }
//                         }
//                     }
//                 }
//             }
//         }

//         names
//     }

// pub fn get_all_struct_refs(&self) -> Vec<String> {
//         let mut names = vec![];

//         for doc in self.idl_documents.values() {
//             match &*doc.analyzer {
//                 Ok(analyzer) => {
//                     for node in &analyzer.nodes {
//                         match node {
//                             super::idl_nodes::IdlNode::TypeStruct(value) => {
//                                 names.push(value.ident.to_owned());
//                             }
//                             _ => {}
//                         }
//                     }
//                 }
//                 Err(_) => {
//                     if let Ok(analyzer) = &*doc.last_analyzer {
//                         for node in &analyzer.nodes {
//                             match node {
//                                 super::idl_nodes::IdlNode::TypeStruct(value) => {
//                                     names.push(value.ident.to_owned());
//                                 }
//                                 _ => {}
//                             }
//                         }
//                     }
//                 }
//             }
//         }

//         names
//     }


//     pub fn get_all_native_refs() -> Vec<String> {
//         vec![
//             super::parser::NativeTypes::Int.to_string(),
//             super::parser::NativeTypes::Float.to_string(),
//             super::parser::NativeTypes::String.to_string(),
//             super::parser::NativeTypes::Bytes.to_string(),
//             super::parser::NativeTypes::Bool.to_string(),
//             super::parser::NativeTypes::None.to_string(),
//         ]
//     }
// }
