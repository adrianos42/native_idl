use tokio::fs;

use native_idl::module::Module;

use idl_rust_gen::rus_types::RusTypes;

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
                if let Ok(rus_ty) = RusTypes::generate(&analyzer.nodes) {
                    let _ =
                        fs::write("./idl_types.rs", rus_ty.to_string().as_str().as_bytes()).await;
                }
            }
            Err(_) => {}
        }
    }
}