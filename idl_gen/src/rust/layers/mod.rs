use crate::lang::StorageItem;
pub(crate) mod bytes;
mod ffi;
mod ws;

pub trait Layer {
    fn build(
        &self,
        analyzers: &[idl::analyzer::Analyzer],
        ids_analyzer: &idl::ids::analyzer::Analyzer,
    ) -> anyhow::Result<Vec<StorageItem>>;
}

pub fn layer_runner_server(layer: &str, server_name: String, input_dir: String, debug_mode: bool) -> Box<dyn Layer> {
    match layer {
        "ws" => Box::new(ws::server::layer::WSLayer::new(server_name, input_dir, debug_mode)),
        _ => panic!("Layer not supported"),
    }
}

// The input path is the working diretory, e.g. the same path as the idl files and therefore the source code to be compiled.
pub fn layer_builder_server(
    layer: &str,
    server_name: &str,
    input_dir: &str,
    debug_mode: bool,
) -> Box<dyn Layer> {
    match layer {
        "ffi" => Box::new(ffi::server::layer::FFILayer::new(
            server_name.to_owned(),
            input_dir.to_owned(),
            debug_mode.to_owned(),
        )),
        "ws" => Box::new(ws::server::layer::WSLayer::new(
            server_name.to_owned(),
            input_dir.to_owned(),
            debug_mode.to_owned(),
        )),
        _ => panic!("Layer not supported"),
    }
}

pub fn layer_builder_client(layer: &str, client_name: &str) -> Box<dyn Layer> {
    match layer {
        "ffi" => Box::new(ffi::client::layer::FFILayer::new(
            client_name.to_owned(),
        )),
        "ws" => Box::new(ws::client::layer::WSLayer::new(
            client_name.to_owned(),
            false, // TODO
        )),
        _ => panic!("Layer not supported"),
    }
}
