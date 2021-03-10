use crate::lang::StorageItem;
mod ffi;
mod ws;
pub(crate) mod bytes;

pub trait Layer {
    fn build(
        &self,
        analyzers: &[idl::analyzer::Analyzer],
        ids_analyzer: &idl::ids::analyzer::Analyzer,
    ) -> anyhow::Result<Vec<StorageItem>>;
}

pub fn layer_runner_server(server_name: String, input_dir: String, debug_mode: bool) -> impl Layer {
    ws::server::layer::WSLayer::new(server_name, input_dir, debug_mode)
}

// The input path is the working diretory, e.g. the same path as the idl files and therefore the source code to be compiled.
pub fn layer_builder_server(
    server_name: String,
    input_dir: String,
    debug_mode: bool,
) -> impl Layer {
    ffi::server::layer::FFILayer::new(server_name, input_dir, debug_mode)
}

pub fn layer_builder_client(client_name: String) -> impl Layer {
    ffi::client::layer::FFILayer::new(client_name)
}