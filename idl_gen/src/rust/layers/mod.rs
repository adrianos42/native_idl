use crate::lang::StorageItem;

mod ffi;

pub trait LayerBuilder {
    fn build(
        &self,
        analyzer: &idl::analyzer::Analyzer,
        ids_analyzer: &idl::ids::analyzer::Analyzer,
    ) -> Vec<StorageItem>;
}

pub struct Layer;

impl Layer {
    pub fn layer_builder(server_name: String) -> impl LayerBuilder {
        ffi::FFILayer::new(server_name)
    }
}
