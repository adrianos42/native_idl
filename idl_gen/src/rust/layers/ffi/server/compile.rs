use idl::ids;
use crate::{lang::StorageItem, rust::layers::LayerBuilder};
use std::{collections::HashMap, io::{self, Write}};
use std::{fs::File, path::PathBuf, str::FromStr};
use tempfile::tempdir;
use cargo::{Config, core::{Verbosity, compiler::Executor}, ops::CompileOptions, util::{ConfigValue, config::Definition, interning::InternedString, paths::read_bytes}};
use cargo::core::compiler::{CompileKind, CompileMode, CompileTarget};
use cargo::core::manifest::TargetSourcePath;
use cargo::core::{PackageId, Target, TargetKind, Workspace};
use cargo::util::command_prelude::{ArgMatchesExt, ProfileChecking};
use cargo::util::{process, CargoResult, ProcessBuilder, dylib_path};

pub fn ffi_server_files(
    analyzer: &idl::analyzer::Analyzer,
    ids_analyzer: &ids::analyzer::Analyzer,
    server_name: &str,
) -> StorageItem {
    let ffi_server_impl = super::FFIServerImpl::generate(&analyzer).unwrap();
    let ffi_server_types = super::FFIServerTypes::generate(&analyzer).unwrap();
    let ffi_server = super::FFIServer::generate(&analyzer).unwrap();
    let ffi_lib = super::FFIMod::generate(&analyzer).unwrap();
    let ffi_cargo =
        super::server_cargo::FFIServerCargo::generate(&ids_analyzer, server_name).unwrap();

    StorageItem::Folder {
        items: vec![
            StorageItem::Folder {
                name: "src".to_owned(),
                items: vec![
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
                    StorageItem::Source {
                        name: "lib.rs".to_owned(),
                        txt: ffi_lib.to_string(),
                    },
                ],
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
        analyzer: &idl::analyzer::Analyzer,
        ids_analyzer: &ids::analyzer::Analyzer,
    ) -> anyhow::Result<Vec<StorageItem>> {
        let dir = tempdir()?;
        let path = dir.path();

        let item = ffi_server_files(analyzer, ids_analyzer, &self.server_name);
        item.write_items(path, true)?;

        let package_path = path.join("idl_ffi/Cargo.toml");

        let config = Config::default()?;
        let ws = Workspace::new(&package_path, &config)?;

        let mut compile_options = CompileOptions::new(&config, CompileMode::Build)?;
        compile_options.build_config.requested_profile = InternedString::new("release");

        let comp = cargo::ops::compile(&ws, &compile_options)?;

        let mut files = vec![];

        for (unit, path) in comp.cdylibs {
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
        Ok(files)
    }
}
