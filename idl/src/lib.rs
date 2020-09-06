#[macro_use]
extern crate lazy_static;

mod scanner;
pub mod analyzer;
pub mod parser;
pub mod formatting;
pub mod idl_types;
pub mod module;
pub mod spec;
pub mod completion;
mod parser_ref;