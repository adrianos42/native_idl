//use super::{server_impl::FFIServerImpl, server_mod::FFIMod, server_types::FFIServerTypes};
use crate::rust::string_pros::StringRustFmt;
use crate::{lang::StorageItem, rust::layers::Layer};
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
    input_dir: &str,
) -> StorageItem {
    StorageItem::Source {
        name: "n".to_owned(),
        txt: "".to_owned(),
    }
}

pub(crate) struct WebSocketLayer {
    server_name: String,
    input_dir: String,
    debug_mode: bool,
}

impl WebSocketLayer {
    pub(crate) fn new(server_name: String, input_dir: String, debug_mode: bool) -> Self {
        Self {
            server_name,
            input_dir,
            debug_mode,
        }
    }
}

impl Layer for WebSocketLayer {
    fn build(
        &self,
        analyzers: &[idl::analyzer::Analyzer],
        ids_analyzer: &ids::analyzer::Analyzer,
    ) -> anyhow::Result<Vec<StorageItem>> {
        let dir = tempdir()?;
        let path = dir.path();
        let mut files = vec![];

        if analyzers.iter().any(|v| v.has_interface()) {
            let item =
                ffi_server_files(analyzers, ids_analyzer, &self.server_name, &self.input_dir);
            item.write_items(path, true)?;

            panic!("wfupy;wufp;yuw;pyuwfpq");

            let package_path = path.join("idl_ffi/Cargo.toml");

            let profile = if self.debug_mode { "debug" } else { "release" };

            let config = Config::default()?;
            let ws = Workspace::new(&package_path, &config)?;

            let mut compile_options = CompileOptions::new(&config, CompileMode::Build)?;
            compile_options.build_config.requested_profile = InternedString::new(profile);

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
