use std::path::Path;

use super::ws_mod::{WSImpl, WSMod, WSCargo};
use super::ws_types::WSTypes;
use crate::rust::string_pros::StringRustFmt;
use crate::{lang::StorageItem, rust::layers::Layer};
use crate::rust::layers::bytes::server::{BytesInterface, package::BytesPackage};
use cargo::core::compiler::CompileMode;
use cargo::core::Workspace;
use cargo::{
    ops::CompileOptions,
    util::{interning::InternedString, paths::read_bytes},
    Config,
};
use idl::ids;
use quote::{ToTokens, TokenStreamExt};
use tempfile::tempdir;

pub fn ws_server_files(
    analyzers: &[idl::analyzer::Analyzer],
    ids_analyzer: &ids::analyzer::Analyzer,
    server_name: &str,
    input_dir: &str,
) -> StorageItem {
    let ws_cargo =
        WSCargo::generate(&ids_analyzer, server_name, input_dir)
            .unwrap();

    let package = ids_analyzer.get_package();
    let package_name = package.name();

    let mut libs = quote! {};
    let mut lib_items = vec![];

    let mut lib_names = vec![];

    for analyzer in analyzers.iter().filter(|v| v.has_interface()) {
        let ws_server_impl = WSImpl::generate(&package_name, &analyzer).unwrap();
        let ws_server_types = WSTypes::generate(&package_name, &analyzer).unwrap();
        let ws_lib = WSMod::generate(&analyzer).unwrap();

        let ws_interface = BytesInterface::generate(&package_name, &analyzer).unwrap();

        let library_name = analyzer.library_name();
        lib_names.push(library_name.clone());

        if package_name == library_name {
            libs.append_all(ws_lib.to_token_stream());

            lib_items.push(StorageItem::Source {
                name: "ws.rs".to_owned(),
                txt: ws_interface.to_string(),
            });

            lib_items.push(StorageItem::Source {
                name: "ws_impl.rs".to_owned(),
                txt: ws_server_impl.to_string(),
            });

            lib_items.push(StorageItem::Source {
                name: "ws_types.rs".to_owned(),
                txt: ws_server_types.to_string(),
            });
        } else {let lib_name = format_ident!("ws_{}", library_name);
            libs.append_all(quote! { pub mod #lib_name; });

            lib_items.push(StorageItem::Folder {
                name: format!("ws_{}", library_name),
                items: vec![
                    StorageItem::Source {
                        name: "mod.rs".to_owned(),
                        txt: ws_lib.to_string(),
                    },
                    StorageItem::Source {
                        name: "ws.rs".to_owned(),
                        txt: ws_interface.to_string(),
                    },
                    StorageItem::Source {
                        name: "ws_impl.rs".to_owned(),
                        txt: ws_server_impl.to_string(),
                    },
                    StorageItem::Source {
                        name: "ws_types.rs".to_owned(),
                        txt: ws_server_types.to_string(),
                    },
                ],
            })
        }
    }

    // Since the library names are all valid here, there's no need to verify them.
    let ws_main = BytesPackage::generate(package, analyzers).unwrap();
    libs.append_all(ws_main.to_token_stream());    

    lib_items.push(StorageItem::Source {
        name: "lib.rs".to_owned(),
        txt: libs.to_string().rust_fmt(),
    });

    StorageItem::Folder {
        items: vec![
            StorageItem::Folder {
                name: "src".to_owned(),
                items: lib_items,
            },
            StorageItem::Source {
                name: "Cargo.toml".to_owned(),
                txt: ws_cargo.to_string(),
            },
        ],
        name: "idl_ws".to_owned(),
    }
}

pub(crate) struct WSLayer {
    server_name: String,
    input_dir: String,
    debug_mode: bool,
}

impl WSLayer {
    pub(crate) fn new(server_name: String, input_dir: String, debug_mode: bool) -> Self {
        Self {
            server_name,
            input_dir,
            debug_mode,
        }
    }
}

impl Layer for WSLayer {
    fn build(
        &self,
        analyzers: &[idl::analyzer::Analyzer],
        ids_analyzer: &ids::analyzer::Analyzer,
    ) -> anyhow::Result<Vec<StorageItem>> {
        //let dir = tempdir()?;
        let path = Path::new("/home/adriano/repos_tmp/basic_types");
        let mut files = vec![];

        if analyzers.iter().any(|v| v.has_interface()) {
            let item =
                ws_server_files(analyzers, ids_analyzer, &self.server_name, &self.input_dir);
            item.write_items(path, true)?;

            let package_path = path.join("idl_ws/Cargo.toml");

            let profile = if self.debug_mode { "debug" } else { "release" };

            let config = Config::default()?;
            let ws = Workspace::new(&package_path, &config)?;

            let mut compile_options = CompileOptions::new(&config, CompileMode::Build)?;
            compile_options.build_config.requested_profile = InternedString::new(profile);

            let comp = cargo::ops::compile(&ws, &compile_options)?;

            for (_, path) in comp.binaries {
                // lib file
                files.push(StorageItem::BinarySource {
                    name: path.file_name().unwrap().to_str().unwrap().to_owned(),
                    data: read_bytes(&path)?,
                });
            }

            //dir.close()?;
        }

        Ok(files)
    }
}
