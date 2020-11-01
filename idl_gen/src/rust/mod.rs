pub mod layers;
pub mod rust_impl;
pub mod rust_types;

mod string_pros;

pub(crate) mod con_idl;

use idl::idl::analyzer::Analyzer;
use idl::module::Module;
use thiserror::Error;

use crate::lang::*;
use layers::ffi::{
    server::{FFIServer, FFIServerCargo, FFIServerImpl, FFIServerTypes},
    FFIMod,
};
use rust_impl::RustImpl;
use rust_types::RustTypes;

#[derive(Debug, Error)]
pub enum RustGenError {}

pub struct RustGen {}

impl RustGen {
    pub fn new() -> Result<Self, RustGenError> {
        Ok(Self {})
    }
}

impl crate::IdlGen for RustGen {
    fn send_request(
        &self,
        request: crate::lang::LanguageRequest,
    ) -> Result<crate::lang::LanguageResponse, crate::IdlGenError> {
        let analyzer = Analyzer::from_nodes(request.nodes);

        let mut dir = crate::lang::Folder {
            items: vec![],
            name: "gen".to_owned(),
        };

        let mut src_dir = crate::lang::Folder {
            items: vec![],
            name: "src".to_owned(),
        };

        let ffi_mod = FFIMod::generate(&analyzer).unwrap();
        src_dir
            .items
            .push(crate::lang::StorageItem::Source(crate::lang::Source {
                name: "lib.rs".to_owned(),
                txt: ffi_mod.to_string(),
            }));

        let rus_ty = RustTypes::generate(&analyzer).unwrap();
        src_dir
            .items
            .push(crate::lang::StorageItem::Source(crate::lang::Source {
                name: "idl_types.rs".to_owned(),
                txt: rus_ty.to_string(),
            }));

        let rus_impl = RustImpl::generate(&analyzer).unwrap();
        src_dir
            .items
            .push(crate::lang::StorageItem::Source(crate::lang::Source {
                name: "idl_impl.rs".to_owned(),
                txt: rus_impl.to_string(),
            }));

        let ffi_server_types = FFIServerTypes::generate(&analyzer).unwrap();
        src_dir
            .items
            .push(crate::lang::StorageItem::Source(crate::lang::Source {
                name: "idl_ffi_types.rs".to_owned(),
                txt: ffi_server_types.to_string(),
            }));

        let ffi_server_impl = FFIServerImpl::generate(&analyzer).unwrap();
        src_dir
            .items
            .push(crate::lang::StorageItem::Source(crate::lang::Source {
                name: "idl_ffi_impl.rs".to_owned(),
                txt: ffi_server_impl.to_string(),
            }));

        let ffi_server = FFIServer::generate(&analyzer).unwrap();
        src_dir
            .items
            .push(crate::lang::StorageItem::Source(crate::lang::Source {
                name: "idl_ffi.rs".to_owned(),
                txt: ffi_server.to_string(),
            }));

        let ffi_cargo = FFIServerCargo::generate(&analyzer).unwrap();
        dir.items
            .push(crate::lang::StorageItem::Source(crate::lang::Source {
                name: "Cargo.toml".to_owned(),
                txt: ffi_cargo.to_string(),
            }));

        dir.items.push(crate::lang::StorageItem::Folder(src_dir));

        Ok(LanguageResponse {
            gen_response: ResponseType::Generated(dir),
        })
    }
}
