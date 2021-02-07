use crate::rust::string_pros::StringRustFmt;
use crate::{lang::StorageItem, rust::layers::LayerBuilder};
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

pub fn ffi_server_files(
    analyzers: &[idl::analyzer::Analyzer],
    ids_analyzer: &ids::analyzer::Analyzer,
    server_name: &str,
) -> StorageItem {
    let ffi_cargo =
        super::server_cargo::FFIServerCargo::generate(&ids_analyzer, server_name).unwrap();

    let package = ids_analyzer.get_package();
    let package_name = package.name();

    let mut libs = quote! {};
    let mut lib_items = vec![];

    for analyzer in analyzers.iter().filter(|v| v.has_interface()) {
        let ffi_server_impl = super::FFIServerImpl::generate(&package_name, &analyzer).unwrap();
        let ffi_server_types = super::FFIServerTypes::generate(&package_name, &analyzer).unwrap();
        let ffi_server = super::FFIServer::generate(&package_name, &analyzer).unwrap();

        let ffi_lib = super::FFIMod::generate(&analyzer).unwrap();

        let library_name = analyzer.library_name();

        if package_name == library_name {
            libs.append_all(ffi_lib.to_token_stream());

            lib_items.push(StorageItem::Source {
                name: "ffi.rs".to_owned(),
                txt: ffi_server.to_string(),
            });

            lib_items.push(StorageItem::Source {
                name: "ffi_impl.rs".to_owned(),
                txt: ffi_server_impl.to_string(),
            });

            lib_items.push(StorageItem::Source {
                name: "ffi_types.rs".to_owned(),
                txt: ffi_server_types.to_string(),
            });
        } else {
            let lib_name = format_ident!("ffi_{}", library_name);
            libs.append_all(quote! { pub mod #lib_name; });

            lib_items.push(StorageItem::Folder {
                name: format!("ffi_{}", library_name),
                items: vec![
                    StorageItem::Source {
                        name: "mod.rs".to_owned(),
                        txt: ffi_lib.to_string(),
                    },
                    StorageItem::Source {
                        name: "ffi.rs".to_owned(),
                        txt: ffi_server.to_string(),
                    },
                    StorageItem::Source {
                        name: "ffi_impl.rs".to_owned(),
                        txt: ffi_server_impl.to_string(),
                    },
                    StorageItem::Source {
                        name: "ffi_types.rs".to_owned(),
                        txt: ffi_server_types.to_string(),
                    },
                ],
            })
        }
    }

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
                txt: ffi_cargo.to_string(),
            },
        ],
        name: "idl_ffi".to_owned(),
    }
}

pub(crate) struct FFILayer {
    server_name: String,
}

impl FFILayer {
    pub(crate) fn new(server_name: String) -> Self {
        Self { server_name }
    }
}

impl LayerBuilder for FFILayer {
    fn build(
        &self,
        analyzers: &[idl::analyzer::Analyzer],
        ids_analyzer: &ids::analyzer::Analyzer,
    ) -> anyhow::Result<Vec<StorageItem>> {
        let dir = tempdir()?;
        let path = dir.path();
        let mut files = vec![];

        if analyzers.iter().any(|v| v.has_interface()) {
            let item = ffi_server_files(analyzers, ids_analyzer, &self.server_name);
            item.write_items(path, true)?;

            let package_path = path.join("idl_ffi/Cargo.toml");

            let config = Config::default()?;
            let ws = Workspace::new(&package_path, &config)?;

            let mut compile_options = CompileOptions::new(&config, CompileMode::Build)?;
            compile_options.build_config.requested_profile = InternedString::new("release");

            let comp = cargo::ops::compile(&ws, &compile_options)?;

            for (_, path) in comp.cdylibs {
                // lib file
                files.push(StorageItem::BinarySource {
                    name: path.file_name().unwrap().to_str().unwrap().to_owned(),
                    data: read_bytes(&path)?,
                });

                let so_path = path.with_extension("so");
                files.push(StorageItem::BinarySource {
                    name: so_path.file_name().unwrap().to_str().unwrap().to_owned(),
                    data: read_bytes(&so_path)?,
                })
            }

            dir.close()?;
        }

        Ok(files)
    }
}
