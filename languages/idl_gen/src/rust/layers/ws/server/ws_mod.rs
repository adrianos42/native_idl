use crate::cargo_m::*;
use crate::rust::con_idl::{get_rust_ty_name, get_rust_ty_ref};
use crate::rust::string_pros::{StringPros, StringRustFmt};
use core::fmt;
use idl::analyzer::Analyzer;
use idl::idl_nodes::*;
use idl::ids;
use proc_macro2::{self, Literal, TokenStream};
use quote::format_ident;
use quote::{ToTokens, TokenStreamExt};
use std::{collections::HashMap, path::Path};

use super::WSServerError;

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
        server_name: &str,
        input_path: &str,
    ) -> Result<Self, WSServerError> {
        let package_name = analyzer.get_package().name();
        let target_server = analyzer
            .find_server(server_name)
            .ok_or(WSServerError::Undefined)?;

        let mut context = Self::new();

        let mut dependencies = HashMap::<String, HashMap<String, String>>::new();

        let lib_path = target_server
            .get_field("path")
            .and_then(|v| {
                let mut lib_path = HashMap::<String, String>::new();
                lib_path.insert("path".to_owned(), v.as_string_value()?);
                Some(lib_path)
            })
            .or_else(|| {
                target_server.get_field("git").and_then(|v| {
                    let mut git = HashMap::<String, String>::new();
                    git.insert("git".to_owned(), v.as_string_value()?);
                    Some(git)
                })
            })
            .unwrap_or_else(|| {
                let path = Path::new(input_path)
                    .join("rust")
                    .join(&package_name)
                    .to_str()
                    .expect("path error")
                    .to_owned();
                let mut lib_path = HashMap::<String, String>::new();
                lib_path.insert("path".to_owned(), path);
                lib_path
            });

        dependencies.insert(package_name.to_owned(), lib_path);

        let mut git = HashMap::<String, String>::new();

        // git.insert(
        //     "git".to_owned(),
        //     "https://github.com/adrianos42/native_idl".to_owned(),
        // );

        let mut v_version = HashMap::new();
        v_version.insert("version".to_owned(), "1.4.0".to_owned());
        dependencies.insert("lazy_static".to_owned(), v_version);

        let mut n_path = HashMap::<String, String>::new();
        n_path.insert(
            "path".to_owned(),
            "/home/adriano/repos/native_idl/languages/rust/idl_internal/".to_owned(),
        );

        dependencies.insert("idl_internal".to_owned(), n_path);

        let fields = CargoFields {
            package: CargoPackage {
                name: "idl_ws".to_owned(),
                authors: None,
                edition: Some("2018".to_owned()),
                version: "0.1.0".to_owned(),
            },
            bin: Some(vec![CargoBin {
                name: package_name.clone(),
                path: "src/bin/main.rs".to_owned(),
            }]),
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
    pub fn generate(package_name: &str, analyzer: &Analyzer) -> Result<Self, ()> {
        let library_name = analyzer.library_name();
        let library_ident = format_ident!("{}", library_name);

        let lib_ident = if package_name == analyzer.library_name() {
            quote! { #library_ident }
        } else {
            let package_ident = format_ident!("{}", package_name);
            quote! { #package_ident::#library_ident }
        };

        let module = quote! {
            pub use #lib_ident::idl_types::*;
            mod ws;
            mod ws_impl;
        };

        Ok(WSMod { module })
    }
}
