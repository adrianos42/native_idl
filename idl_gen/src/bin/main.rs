use tokio::fs;

use idl::module::Module;

use idl_gen::layers::ffi::{
    server::{FFIServer, FFIServerCargo, FFIServerImpl, FFIServerTypes},
    FFIMod,
};
use idl_gen::rust_impl::RustImpl;
use idl_gen::rust_types::RustTypes;

#[tokio::main]
async fn main() {
    if let Ok(text) = fs::read_to_string("./idl_types.idl").await {
        let module = Module::new();
        if let Err(_) = module.add_document("idl_types").await {
            return;
        }

        module.update_parse("idl_types", &text).await;
        module.update_analyze("idl_types").await;

        let analyz = module.get_analyzer("idl_types").await.unwrap();

        match &*analyz {
            Ok(analyzer) => {
                let ffi_mod = FFIMod::generate(analyzer).unwrap();
                let _ =
                    fs::write("./src/gen/src/lib.rs", ffi_mod.to_string().as_str().as_bytes()).await.unwrap();

                let rus_ty = RustTypes::generate(analyzer).unwrap();
                let _ = fs::write(
                    "./src/gen/src/idl_types.rs",
                    rus_ty.to_string().as_str().as_bytes(),
                )
                .await.unwrap();

                let rus_impl = RustImpl::generate(analyzer).unwrap();
                let _ = fs::write(
                    "./src/gen/src/idl_impl.rs",
                    rus_impl.to_string().as_str().as_bytes(),
                )
                .await.unwrap();

                let ffi_server_types = FFIServerTypes::generate(analyzer).unwrap();
                let _ = fs::write(
                    "./src/gen/src/idl_ffi_types.rs",
                    ffi_server_types.to_string().as_str().as_bytes(),
                )
                .await.unwrap();

                let ffi_server_impl = FFIServerImpl::generate(analyzer).unwrap();
                let _ = fs::write(
                    "./src/gen/src/idl_ffi_impl.rs",
                    ffi_server_impl.to_string().as_str().as_bytes(),
                )
                .await.unwrap();

                let ffi_server = FFIServer::generate(analyzer).unwrap();
                let _ = fs::write(
                    "./src/gen/src/idl_ffi.rs",
                    ffi_server.to_string().as_str().as_bytes(),
                )
                .await.unwrap();

                let ffi_cargo = FFIServerCargo::generate(analyzer).unwrap();
                let _ = fs::write(
                    "./src/gen/Cargo.toml",
                    ffi_cargo.to_string().as_str().as_bytes(),
                )
                .await.unwrap();
            }
            Err(_) => {
                panic!()
            }
        }
    }
}
