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
    pub fn layer_builder_server(server_name: String) -> impl LayerBuilder {
        ffi::server::layer::FFILayer::new(server_name)
    }

    pub fn layer_builder_client(client_name: String) -> impl LayerBuilder {
        ffi::client::layer::FFILayer::new(client_name)
    }
}
