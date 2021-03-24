use crate::rust::string_pros::StringRustFmt;
use idl::analyzer::Analyzer;
use proc_macro2::{self, TokenStream};
use quote::{ToTokens, TokenStreamExt};
use std::fmt;

pub struct FFIMod {
    module: TokenStream,
}

impl ToTokens for FFIMod {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(self.module.to_token_stream());
    }
}

impl fmt::Display for FFIMod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.module.to_string().rust_fmt())
    }
}

impl FFIMod {
    pub fn generate(_analyzer: &Analyzer) -> Result<Self, ()> {
        let module = quote! {
            mod ffi; // interface and static functions
            mod ffi_types; // ffi types
        };

        Ok(FFIMod { module })
    }
}