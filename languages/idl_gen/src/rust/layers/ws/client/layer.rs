use std::path::Path;

use super::ws_mod::{WSCargo, WSMod};
use super::{WSClient, WSInterface};
use crate::rust::string_pros::StringRustFmt;
use crate::rust::layers::bytes::types::BytesTypes;
use crate::rust::{
    rust_types::RustTypes,
};
use crate::{lang::StorageItem, rust::layers::Layer};
use idl::ids;
use quote::{ToTokens, TokenStreamExt};

pub fn ws_client_files(
    analyzers: &[idl::analyzer::Analyzer],
    ids_analyzer: &ids::analyzer::Analyzer,
    client_name: &str,
) -> Vec<StorageItem> {
    let mut lib_body = quote! {};
    let mut lib_items = vec![];

    let mut libs = quote! {
        #[macro_use]
        extern crate lazy_static;

        mod ws;
    };

    let package = ids_analyzer.get_package();
    let package_name = package.name();

    let ws_client = WSClient::generate(&package, &analyzers).unwrap();

    for analyzer in analyzers.iter().filter(|v| v.has_interface()) {
        let library_name = analyzer.library_name();
        
        let ws_lib = WSMod::generate(&analyzer).unwrap();
        let rus_t = RustTypes::generate(analyzer).unwrap();
        let bytes_rus_t = BytesTypes::generate(false, analyzer).unwrap();
        let ws_interface = WSInterface::generate(&package, &analyzer).unwrap();

        // This happens at most once, therefore the libraries' `use` declarations may be inserted first.
        if package_name == library_name {
            lib_body.append_all(ws_lib.to_token_stream());
            lib_body.append_all(rus_t.to_token_stream());
            lib_body.append_all(bytes_rus_t.to_token_stream());
            lib_body.append_all(ws_interface.to_token_stream());
        } else {
            let lib_name = format_ident!("{}", library_name);
            libs.append_all(quote! { pub mod #lib_name; });

            lib_items.push(StorageItem::Folder {
                name: library_name.to_owned(),
                items: vec![StorageItem::Source {
                    name: "mod.rs".to_owned(),
                    txt: quote! {
                        #ws_lib
                        #rus_t
                        #bytes_rus_t
                        #ws_interface
                    }
                    .to_string()
                    .rust_fmt(),
                }],
            })
        }
    }

    let ws_cargo = WSCargo::generate(&ids_analyzer, client_name).unwrap();

    lib_items.push(StorageItem::Source {
        name: "lib.rs".to_owned(),
        txt: quote! {
           #libs
           #lib_body
        }
        .to_string()
        .rust_fmt(),
    });

    lib_items.push(StorageItem::Source {
        name: "ws.rs".to_owned(),
        txt: ws_client.to_string(),
    });

    return vec![
        StorageItem::Folder {
            name: "src".to_owned(),
            items: lib_items,
        },
        StorageItem::Source {
            name: "Cargo.toml".to_owned(),
            txt: ws_cargo.to_string(),
        },
    ];
}

pub(crate) struct WSLayer {
    client_name: String,
}

impl WSLayer {
    pub(crate) fn new(client_name: String) -> Self {
        Self { client_name }
    }
}

impl Layer for WSLayer {
    fn build(
        &self,
        analyzers: &[idl::analyzer::Analyzer],
        ids_analyzer: &ids::analyzer::Analyzer,
    ) -> anyhow::Result<Vec<StorageItem>> {
        if analyzers.iter().any(|v| v.has_interface()) {
            return Ok(ws_client_files(analyzers, ids_analyzer, &self.client_name));
        }

        Ok(vec![])
    }
}
