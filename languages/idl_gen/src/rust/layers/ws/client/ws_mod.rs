use crate::cargo_m::*;
use core::fmt;
use idl::ids;
use std::{collections::HashMap, path::Path};
use idl::{analyzer::Analyzer};
use idl::idl_nodes::*;
use crate::{rust::con_idl::{get_rust_ty_name, get_rust_ty_ref}};
use crate::rust::string_pros::{StringPros, StringRustFmt};
use proc_macro2::{self, Literal, TokenStream};
use quote::{TokenStreamExt, ToTokens};
use quote::format_ident;

use super::WSClientError;

pub struct WSCargo {
    cargo_toml: Option<String>,
}

impl fmt::Display for WSCargo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.cargo_toml.as_ref().unwrap())
    }
}

impl WSCargo {
    pub fn generate(
        analyzer: &ids::analyzer::Analyzer,
        client_name: &str,
    ) -> Result<Self, WSClientError> {
        let package_name = analyzer.get_package().name();
        let target_client = analyzer
            .find_client(client_name)
            .ok_or(WSClientError::Undefined)?;

        let mut context = Self::new();

        let mut dependencies = HashMap::<String, HashMap<String, String>>::new();

        let mut git = HashMap::<String, String>::new();

        // git.insert(
        //     "git".to_owned(),
        //     "https://github.com/adrianos42/native_idl".to_owned(),
        // );

        let mut v_version = HashMap::new();
        v_version.insert("version".to_owned(), "1.4.0".to_owned());
        dependencies.insert("lazy_static".to_owned(), v_version);

        let mut n_path = HashMap::new();
        n_path.insert("path".to_owned(), "/home/adriano/repos/native_idl/languages/rust/idl_internal/".to_owned());

        dependencies.insert("idl_internal".to_owned(), n_path);

        let fields = CargoFields {
            package: CargoPackage {
                name: package_name.clone(),
                authors: None,
                edition: Some("2018".to_owned()),
                version: "0.1.0".to_owned(),
            },
            bin: None,
            lib: None,
            dependencies: Some(dependencies),
        };

        context.cargo_toml = Some(create_cargo(fields));

        Ok(context)
    }

    fn new() -> Self {
        Self { cargo_toml: None }
    }
}

pub struct WSMod {
    module: TokenStream,
}

impl ToTokens for WSMod {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(self.module.to_token_stream());
    }
}

impl fmt::Display for WSMod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.module.to_string().rust_fmt())
    }
}

impl WSMod {
    pub fn generate(_analyzer: &Analyzer) -> Result<Self, ()> {
        let module = quote! {
            use ::std::io::Write;
            use ::std::io::Read;
            use ::std::collections::HashMap;
            use ::idl_internal::{
                futures::SinkExt,
                byteorder::{ReadBytesExt, WriteBytesExt},
                Uuid,
            };
        };

        Ok(WSMod { module })
    }
}