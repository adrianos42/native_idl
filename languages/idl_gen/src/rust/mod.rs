pub mod layers;
pub mod rust_server;
pub mod rust_types;

mod string_pros;

pub(crate) mod con_idl;

use crate::{
    lang::{BuildType, LanguageRequest, LanguageResponse, RequestType, ResponseType, ServerArg},
    IdlGenError,
};

use anyhow::Result;
use layers::{layer_builder_client, layer_builder_server, layer_runner_server, Layer};
use rust_server::rust_impl_files;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RustGenError {
    #[error("{0}")]
    Undefined(String),
    #[error("{0}")]
    Layer(String),
    #[error("{0}")]
    Server(String),
}

pub struct RustGen {}

impl RustGen {
    pub fn new() -> Result<Self, RustGenError> {
        Ok(Self {})
    }
}

impl crate::IdlGen for RustGen {
    fn send_request(&self, request: LanguageRequest) -> Result<LanguageResponse, IdlGenError> {
        match request.request_type {
            RequestType::Server(name) => {
                let mut root_items = vec![];

                let analyzers: Vec<idl::analyzer::Analyzer> = request
                    .libraries
                    .into_iter()
                    .map(|library| idl::analyzer::Analyzer::from_nodes(library.nodes))
                    .collect();

                let ids_analyzer = idl::ids::analyzer::Analyzer::from_nodes(request.ids_nodes);

                match name.args {
                    ServerArg::Build => {
                        let layer = layer_builder_server(
                            "ffi",
                            &name.server_name,
                            &name.input_path,
                            match name.build_type {
                                BuildType::Release => false,
                                BuildType::Debug => true,
                            },
                        );
                        root_items.append(
                            &mut layer
                                .build(&analyzers, &ids_analyzer)
                                .map_err(|err| RustGenError::Undefined(err.to_string()))?,
                        );
                    }
                    ServerArg::Run => {
                        let layer = layer_runner_server(
                            "ws",
                            name.server_name.to_owned(),
                            name.input_path,
                            match name.build_type {
                                BuildType::Release => false,
                                BuildType::Debug => true,
                            },
                        );
                        root_items.append(
                            &mut layer
                                .build(&analyzers, &ids_analyzer)
                                .map_err(|err| RustGenError::Undefined(err.to_string()))?,
                        );
                    }
                    ServerArg::Generate => {
                        root_items.push(rust_impl_files(&analyzers, &ids_analyzer));
                    }
                }
                Ok(LanguageResponse {
                    gen_response: ResponseType::Generated(root_items),
                    response_messages: vec![],
                })
            }
            RequestType::Client(name) => {
                let mut root_items = vec![];

                let analyzers: Vec<idl::analyzer::Analyzer> = request
                    .libraries
                    .into_iter()
                    .map(|library| idl::analyzer::Analyzer::from_nodes(library.nodes))
                    .collect();

                let ids_analyzer = idl::ids::analyzer::Analyzer::from_nodes(request.ids_nodes);

                let client = ids_analyzer
                    .find_client(&name.client_name)
                    .ok_or_else(|| RustGenError::Undefined("".to_owned()))?;

                let server = client
                    .get_field("servers")
                    .and_then(|servers| servers.as_values())
                    .and_then(|v| {
                        v.into_iter().find(|p| {
                            p.as_server_name().map_or(false, |server_name| {
                                server_name == name.server_name.as_str()
                            })
                        })
                    })
                    .and_then(|server_name| server_name.as_server_name())
                    .and_then(|v| ids_analyzer.find_server(&v))
                    .ok_or(RustGenError::Server("Could not find server".to_owned()))?;

                let layer_name = server
                    .get_field("layers")
                    .and_then(|layers| layers.as_values())
                    .and_then(|mut v| v.pop())
                    .and_then(|layer_name| layer_name.as_identifier())
                    .ok_or(RustGenError::Layer(
                        "Could not find defined layer".to_owned(),
                    ))?;

                let layer = layer_builder_client(&layer_name, &name.client_name);

                root_items.append(
                    &mut layer
                        .build(&analyzers, &ids_analyzer)
                        .map_err(|err| RustGenError::Undefined(err.to_string()))?,
                );

                Ok(LanguageResponse {
                    gen_response: ResponseType::Generated(root_items),
                    response_messages: vec![],
                })
            }
        }
    }
}

// pub fn build_server(idl_path: &std::path::Path) -> Result<()> {
//     let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());

//     let module = open_directory(idl_path)?;
//     module.update_module()?;

//     let library_name = module.library_name().unwrap();

//     let analyzer_r = &*module.get_idl_analyzer(&library_name).ok_or(anyhow!(""))?;
//     let analyzer = analyzer_r.as_ref().map_err(|_| anyhow!(""))?;

//     let impl_mod = RustImpl::generate(&analyzer).unwrap();
//     let rus_ty = RustTypes::generate(&analyzer).unwrap();
//     let ffi_server_impl = FFIServerImpl::generate(&analyzer).unwrap();
//     let ffi_server_types = FFIServerTypes::generate(&analyzer).unwrap();
//     let ffi_server = FFIServer::generate(&analyzer).unwrap();

//     let lib_ident = format_ident!("{}", library_name);

//     let interfaces: Vec<TokenStream> = analyzer
//         .nodes
//         .iter()
//         .filter_map(|node| match node {
//             idl::idl_nodes::IdlNode::TypeInterface(ty_interface) => {
//                 let name = format_ident!("{}", &ty_interface.ident);
//                 Some(quote! { type #name = super::#name; })
//             }
//             _ => None,
//         })
//         .collect();

//     let module_body = quote! {
//         pub(crate) mod #lib_ident {
//             #( #interfaces )*
//             #rus_ty

//             pub(crate) mod idl_impl {
//                 #impl_mod
//             }

//             mod ffi_types {
//                 #ffi_server_types
//             }

//             mod ffi_impl {
//                 #ffi_server_impl
//             }

//             mod ffi {
//                 #ffi_server
//             }
//         }
//     };

//     write_items(
//         &StorageItem::Source {
//             name: format!("{}.rs", library_name),
//             txt: module_body.to_string().rust_fmt(),
//         },
//         &out_dir,
//     )?;

//     Ok(())
// }

// fn write_items(storage: &StorageItem, path: &Path) -> Result<()> {
//     match storage {
//         StorageItem::Source { name, txt } => {
//             let mut file = fs::OpenOptions::new()
//                 .write(true)
//                 .truncate(true)
//                 .create(true)
//                 .open(path.join(name.as_str()))?;
//             file.write_all(txt.as_bytes())?;
//         }
//         StorageItem::Folder { name, items } => {
//             let new_path = path.join(&name);
//             fs::create_dir_all(&new_path)?;

//             for item in items {
//                 write_items(item, &new_path)?;
//             }
//         }
//     }

//     Ok(())
// }

// #[derive(Debug)]
// struct Document {
//     text: String,
//     file_stem: String,
//     file_type: String,
// }

// pub(crate) fn open_directory(path: &Path) -> Result<Module> {
//     let module = Module::new();

//     if !path.is_dir() {
//         return Err(anyhow!("Path is not a directory"));
//     }

//     for entry in fs::read_dir(path)? {
//         let item = entry?.path();

//         if item.is_file() {
//             if let Some(doc) = add_file(&item) {
//                 match doc.file_type.as_str() {
//                     "idl" => {
//                         module.add_idl_document_and_update(&doc.file_stem, &doc.text)?;
//                     }
//                     "ids" => {
//                         module.add_ids_document_and_update(&doc.file_stem, &doc.text)?;
//                     }
//                     _ => panic!(),
//                 }
//             }
//         }
//     }

//     Ok(module)
// }

// fn add_file(path: &Path) -> Option<Document> {
//     let file_stem_s = path.file_stem()?;
//     let file_type_s = path.extension()?;

//     let file_stem = file_stem_s.to_str()?.to_owned();
//     let file_type = file_type_s.to_str()?.to_owned();

//     match file_type.as_str() {
//         "idl" | "ids" => {
//             if let Ok(text) = fs::read_to_string(path) {
//                 return Some(Document {
//                     file_stem,
//                     file_type,
//                     text,
//                 });
//             }
//         }
//         _ => {}
//     }

//     None
// }
