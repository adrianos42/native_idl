use crate::lang::StorageItem;
mod ffi;

pub trait LayerBuilder {
    fn build(
        &self,
        analyzers: &[idl::analyzer::Analyzer],
        ids_analyzer: &idl::ids::analyzer::Analyzer,
    ) -> anyhow::Result<Vec<StorageItem>>;
}

pub struct Layer;

impl Layer {
    pub fn layer_builder(server_name: String) -> impl LayerBuilder {
        ffi::server::compile::FFILayer::new(server_name)
    }
}
