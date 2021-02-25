use super::{client_mod::FFIMod, client_types::FFIClientTypes};
use crate::rust::rust_types::RustTypes;
use crate::rust::string_pros::StringRustFmt;
use crate::{lang::StorageItem, rust::layers::LayerBuilder};
use idl::ids;
use quote::{ToTokens, TokenStreamExt};

pub(crate) struct FFILayer {
    client_name: String,
}

impl FFILayer {
    pub(crate) fn new(client_name: String) -> Self {
        Self { client_name }
    }
}

impl LayerBuilder for FFILayer {
    fn build(
        &self,
        analyzers: &[idl::analyzer::Analyzer],
        ids_analyzer: &ids::analyzer::Analyzer,
    ) -> anyhow::Result<Vec<StorageItem>> {
        panic!("Client side is not fully implemented yet!");

        let ffi_cargo =
            super::client_cargo::FFIClientCargo::generate(&ids_analyzer, &self.client_name)?;

        let package = ids_analyzer.get_package();
        let package_name = package.name();

        let mut libs = quote! {};
        let mut lib_body = quote! {};
        let mut lib_items = vec![];

        for analyzer in analyzers.iter().filter(|v| v.has_interface()) {
            let rus_t = RustTypes::generate(analyzer).unwrap();
            let ffi_client_types = FFIClientTypes::generate(&package_name, &analyzer).unwrap();
            let ffi_client = super::FFIClient::generate(&package_name, &analyzer).unwrap();

            let ffi_lib = FFIMod::generate(&analyzer).unwrap();

            let library_name = analyzer.library_name();

            if package_name == library_name {
                lib_body.append_all(rus_t.to_token_stream());

                libs.append_all(ffi_lib.to_token_stream());

                lib_items.push(StorageItem::Source {
                    name: "ffi.rs".to_owned(),
                    txt: ffi_client.to_string(),
                });

                lib_items.push(StorageItem::Source {
                    name: "ffi_types.rs".to_owned(),
                    txt: ffi_client_types.to_string(),
                });
            } else {
                let lib_name = format_ident!("{}", library_name);
                libs.append_all(quote! { pub mod #lib_name; });

                lib_items.push(StorageItem::Folder {
                    name: format!("{}", library_name),
                    items: vec![
                        StorageItem::Source {
                            name: "mod.rs".to_owned(),
                            txt: quote! {
                               #ffi_lib
                               #rus_t
                            }
                            .to_string()
                            .rust_fmt(),
                        },
                        StorageItem::Source {
                            name: "ffi.rs".to_owned(),
                            txt: ffi_client.to_string(),
                        },
                        StorageItem::Source {
                            name: "ffi_types.rs".to_owned(),
                            txt: ffi_client_types.to_string(),
                        },
                    ],
                })
            }
        }

        lib_items.push(StorageItem::Source {
            name: "lib.rs".to_owned(),
            txt: quote! {
               #libs
               #lib_body
            }
            .to_string()
            .rust_fmt(),
        });

        Ok(vec![
            StorageItem::Folder {
                name: "src".to_owned(),
                items: lib_items,
            },
            StorageItem::Source {
                name: "Cargo.toml".to_owned(),
                txt: ffi_cargo.to_string(),
            },
        ])
    }
}
