#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate quote;

pub mod layers;

pub mod rust_types;
pub mod rust_impl;
mod string_pros;

pub(crate) mod con_idl;