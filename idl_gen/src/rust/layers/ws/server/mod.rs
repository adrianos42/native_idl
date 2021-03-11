use idl::analyzer::Analyzer;
use idl::idl_nodes::*;

use crate::rust::con_idl::{get_rust_ty_name, get_rust_ty_ref};

use crate::rust::string_pros::{StringPros, StringRustFmt};
use proc_macro2::{self, Literal, TokenStream};
use quote::format_ident;
use quote::{ToTokens, TokenStreamExt};
use std::{collections::HashMap, fmt};

pub(crate) mod layer;
pub(super) mod ws_mod;
pub(super) mod ws_types;

#[derive(Debug)]
pub enum WSServerError {
    UnexpectedType,
    Undefined,
}

pub struct WSServer {
    module: Vec<TokenStream>,
}

impl ToTokens for WSServer {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(&self.module);
    }
}

impl fmt::Display for WSServer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result_code = String::new();

        self.module.iter().for_each(|value| {
            result_code += &value.to_string();
        });

        write!(f, "{}", result_code.rust_fmt())
    }
}

impl WSServer {
    pub fn generate(package_name: &str) -> Result<Self, WSServerError> {
        let mut context = WSServer::new();

        let package_ident = format_ident!("{}", package_name.to_pascal_case());

        context.module.push(quote! {
            use idl_ws::#package_ident;
        });

        context.add_main()?;

        Ok(context)
    }

    fn add_main(&mut self) -> Result<(), WSServerError> {
        self.module.push(quote! {
            fn main() {
                
            }
        });

        Ok(())
    }

    fn new() -> Self {
        Self { module: vec![] }
    }
}
