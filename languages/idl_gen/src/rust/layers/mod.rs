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

// The input path is the working directory, e.g. the same path as the idl files and therefore the source code to be compiled.
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
            client_name.to_owned()
        )),
        _ => panic!("Layer not supported"),
    }
}



// uuid = { version = "0.8", features = ["v4"] }
// anyhow = "1.0"
// thiserror = "1.0"
// byteorder = "1.4.2"
// tungstenite = "0.13.0"
// url = "2.2.1"
// lazy_static = "1.4.0"
// tokio-tungstenite = "0.14.0"
// futures = "0.3.13"
// tokio = { version = "1.4.0", features = ["rt", "rt-multi-thread", "net", "macros", "sync"] }
// async-trait = { version = "0.1.48" }