use crate::lang::StorageItem;
use thiserror::Error;
mod ffi;

pub trait LayerBuilder {
    fn build(
        &self,
        analyzer: &idl::analyzer::Analyzer,
        ids_analyzer: &idl::ids::analyzer::Analyzer,
    ) -> anyhow::Result<Vec<StorageItem>>;
}

pub struct Layer;

impl Layer {
    pub fn layer_builder(server_name: String) -> impl LayerBuilder {
        ffi::server::compile::FFILayer::new(server_name)
    }
}
