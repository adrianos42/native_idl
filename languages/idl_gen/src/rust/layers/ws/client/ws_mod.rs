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
use serde::Serialize;
use std::{collections::HashMap, path::Path};

use super::WSClientError;

pub struct WSCargo {
    cargo_toml: Option<String>,
}

impl fmt::Display for WSCargo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.cargo_toml.as_ref().unwrap())
    }
}

#[derive(Serialize)]
struct CargoWSDeps {
    idl_internal: CargoPath,
    lazy_static: CargoDep,
    byteorder: CargoDep,
    tungstenite: CargoDep,
    url: CargoDep,
    #[serde(rename(serialize = "tokio-tungstenite"))]
    tokio_tungstenite: CargoDep,
    futures: CargoDep,
    #[serde(rename(serialize = "async-trait"))]
    async_trait: CargoDep,
    #[serde(rename(serialize = "async-stream"))]
    async_stream: CargoDep,
    tokio: CargoDep,
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

        // git.insert(
        //     "git".to_owned(),
        //     "https://github.com/adrianos42/native_idl".to_owned(),
        // );
        
        let fields = CargoFields {
            package: CargoPackage {
                name: package_name.clone(),
                authors: None,
                edition: Some("2018".to_owned()),
                version: "0.1.0".to_owned(),
            },
            bin: None,
            lib: None,
            dependencies: Some(CargoWSDeps {
                async_stream: CargoDep {
                    version: "0.3.0".to_owned(),
                    features: None,
                },
                async_trait: CargoDep {
                    version: "0.1.48".to_owned(),
                    features: None,
                },
                byteorder: CargoDep {
                    version: "1.4.2".to_owned(),
                    features: None,
                },
                futures: CargoDep {
                    version: "0.3.13".to_owned(),
                    features: None,
                },
                idl_internal: CargoPath {
                    path: "/home/adriano/repos/native_idl/languages/rust/idl_internal/".to_owned(),
                },
                lazy_static: CargoDep {
                    version: "1.4.0".to_owned(),
                    features: None,
                },
                url: CargoDep {
                    version: "2.2.1".to_owned(),
                    features: None,
                },
                tokio: CargoDep {
                    version: "1.4.0".to_owned(),
                    features: Some(vec![
                        "rt".to_owned(),
                        "rt-multi-thread".to_owned(),
                        "net".to_owned(),
                        "macros".to_owned(),
                        "sync".to_owned(),
                    ]),
                },
                tungstenite: CargoDep {
                    version: "10.13.0".to_owned(),
                    features: None,
                },
                tokio_tungstenite: CargoDep {
                    version: "0.14.0".to_owned(),
                    features: None,
                },
            }),
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
            use ::async_stream::stream;
            use ::byteorder::{ReadBytesExt, WriteBytesExt};
            use ::futures::{
                stream::{Stream, StreamExt},
                SinkExt,
            };
            use ::idl_internal::{self, Uuid};
            use ::std::collections::HashMap;
            use ::std::io::Read;
            use ::std::io::Write;
        };

        Ok(WSMod { module })
    }
}
