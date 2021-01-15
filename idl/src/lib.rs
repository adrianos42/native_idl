#[macro_use]
extern crate lazy_static;

mod scanner;
mod reserved;

pub mod formatting;
pub mod module;
pub mod ids;
pub mod range;

pub(crate) mod parser_ref;
pub mod analyzer;
pub mod parser;
pub mod idl_nodes;