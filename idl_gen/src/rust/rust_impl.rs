use idl::analyzer::Analyzer;
use idl::idl_nodes::*;
use idl::ids;
use crate::lang::StorageItem;

use super::{con_idl::get_rust_ty_ref, rust_types::RustTypes};

use super::string_pros::StringPros;
use proc_macro2::{self, TokenStream};
use quote::format_ident;
use quote::{ToTokens, TokenStreamExt};
use std::fmt;

#[derive(Debug)]
pub enum RustImplError {
    UnexpectedType,
    InvalidLiteral,
}

pub struct RustImpl {
    module: Vec<TokenStream>,
}

impl ToTokens for RustImpl {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(&self.module);
    }
}

impl fmt::Display for RustImpl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result_code = String::new();

        self.module.iter().for_each(|value| {
            result_code += &value.to_string();
        });

        write!(f, "{}", result_code.as_str().rust_fmt())
    }
}

impl RustImpl {
    pub fn generate(analyzer: &Analyzer) -> Result<Self, RustImplError> {
        let mut context = RustImpl::new();

        context.module.push(quote! {
            use idl_internal::{StreamReceiver, StreamSender, StreamInstance};
        });

        let nodes: &[IdlNode] = &analyzer.nodes;

        for node in nodes {
            match node {
                IdlNode::InterfaceComment(_) => {}
                IdlNode::TypeInterface(value) => context.add_interface(value, analyzer)?,
                _ => {}
            }
        }

        Ok(context)
    }

    fn add_interface(
        &mut self,
        ty_interface: &TypeInterface,
        _analyzer: &Analyzer,
    ) -> Result<(), RustImplError> {
        let ident = &ty_interface.ident;

        let mut fields = vec![];
        let mut static_fields = vec![];

        for field_node in &ty_interface.fields {
            match field_node {
                InterfaceNode::InterfaceField(field) => {
                    let func_ident = format_ident!("{}", &field.ident);
                    let mut args_name = vec![];

                    let (args, ret_ty) = match &field.ty {
                        TypeName::TypeFunction(value) => (
                            Some(match &value.args {
                                TypeName::TypeTuple(tuple) => &tuple.fields,
                                _ => return Err(RustImplError::UnexpectedType),
                            }),
                            &value.return_ty,
                        ),
                        TypeName::TypeTuple(tuple) => {
                            (Some(&tuple.fields), &TypeName::Types(Types::NatNone))
                        }
                        _ => (None, &field.ty),
                    };

                    let mut stream_arg: Option<(&TypeName, &TypeName)> = None;
                    let mut stream_ret: Option<(&TypeName, &TypeName)> = None;

                    if let Some(args) = args {
                        for arg in args {
                            let arg_ident = format_ident!("{}", &arg.ident);
                            let arg_ty_ident = get_rust_ty_ref(&arg.ty, false);
                            args_name.push(quote! { #arg_ident: #arg_ty_ident });

                            if let TypeName::TypeStream(value) = &arg.ty {
                                stream_arg = Some((&arg.ty, &value.s_ty));
                            }
                        }
                    }

                    let ret_value_ident = match ret_ty {
                        TypeName::Types(Types::NatNone) | TypeName::TypeStream(_) => {
                            if let TypeName::TypeStream(value) = ret_ty {
                                let stream_ty_ident = get_rust_ty_ref(&ret_ty, false);
                                args_name.push(quote! { stream_instance: #stream_ty_ident });
                                stream_ret = Some((ret_ty, &value.s_ty));
                            }
                            quote! {}
                        }
                        _ => {
                            let ret_ty_ident = get_rust_ty_ref(&ret_ty, false);
                            quote! { -> #ret_ty_ident }
                        }
                    };

                    let (fields_add, self_ident) = if field.is_static {
                        (&mut static_fields, quote! {})
                    } else {
                        (&mut fields, quote! { &mut self, })
                    };

                    fields_add.push(quote! {
                        fn #func_ident(#self_ident #( #args_name ),* )#ret_value_ident;
                    });

                    if let Some((arg_ty, stream_ty)) = stream_arg {
                        let ident = format_ident!("{}_stream_sender", &field.ident);
                        let a_ty = get_rust_ty_ref(arg_ty, false);
                        let s_ty = get_rust_ty_ref(stream_ty, false);
                        fields_add.push(quote! {
                            fn #ident(#self_ident stream_instance: #a_ty, stream: StreamSender<#s_ty>) -> StreamReceiver;
                        });
                    }
                    if let Some((ret_ty, stream_ty)) = stream_ret {
                        let ident = format_ident!("{}_stream", &field.ident);
                        let r_ty = get_rust_ty_ref(ret_ty, false);
                        let s_ty = get_rust_ty_ref(stream_ty, false);
                        fields_add.push(quote! {
                            fn #ident(#self_ident stream_instance: #r_ty, stream: StreamReceiver) -> StreamSender<#s_ty>;
                        });
                    }
                }
                InterfaceNode::Comment(_) => {}
            }
        }

        if !fields.is_empty() {
            let interface_ident = format_ident!("{}Instance", &ident);
            self.module
                .push(quote! { pub trait #interface_ident { #( #fields )* } });
        }

        if !static_fields.is_empty() {
            let interface_static_ident = format_ident!("{}Static", &ident);
            self.module
                .push(quote! { pub trait #interface_static_ident { #( #static_fields )* } });
        }

        Ok(())
    }

    fn new() -> Self {
        Self { module: vec![] }
    }
}

pub struct RustImplMod {
    module: TokenStream,
}

impl fmt::Display for RustImplMod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.module.to_string().as_str().rust_fmt())
    }
}

impl ToTokens for RustImplMod {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(self.module.to_token_stream());
    }
}

impl RustImplMod {
    pub fn generate(_analyzer: &Analyzer) -> Result<Self, ()> {
        let module = quote! {
            pub mod idl_impl; // rust interface type
            pub use idl_internal;
        };

        Ok(RustImplMod { module })
    }
}

mod impl_cargo {
    use crate::cargo_md::*;
    use core::fmt;
    use idl::ids;
    use serde::Serialize;

    pub struct RustImplCargo {
        cargo_toml: Option<String>,
    }

    impl fmt::Display for RustImplCargo {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.cargo_toml.as_ref().unwrap())
        }
    }

    #[derive(Serialize)]
    struct CargoFFIDeps {
        idl_internal: CargoGit,
    }

    impl RustImplCargo {
        pub fn generate(analyzer: &ids::analyzer::Analyzer) -> Result<Self, ()> {
            let mut context = RustImplCargo::new();
            //let lib_name = analyzer.library_name().unwrap();

            let fields = CargoFields {
                package: CargoPackage {
                    name: "idl_types".to_owned(),
                    authors: None,
                    edition: Some("2018".to_owned()),
                    version: "0.1.0".to_owned(),
                },
                lib: None,
                dependencies: Some(CargoFFIDeps {
                    idl_internal: CargoGit {
                        git: "https://github.com/adrianos42/native_idl".to_owned(),
                    },
                }),
            };

            context.cargo_toml = Some(create_cargo(fields));

            Ok(context)
        }

        fn new() -> Self {
            Self {
                cargo_toml: None,
            }
        }
    }
}

pub fn rust_impl_files(analyzer: &idl::analyzer::Analyzer, ids_analyzer: &ids::analyzer::Analyzer) -> StorageItem {
    let rus_ty = RustTypes::generate(&analyzer).unwrap();
    let impl_mod = RustImpl::generate(&analyzer).unwrap();
    let impl_lib = RustImplMod::generate(&analyzer).unwrap();
    let impl_cargo = impl_cargo::RustImplCargo::generate(&ids_analyzer).unwrap();

    StorageItem::Folder {
        items: vec![
            StorageItem::Folder {
                name: "src".to_owned(),
                items: vec![
                    StorageItem::Source {
                        name: "idl_impl.rs".to_owned(),
                        txt: impl_mod.to_string(),
                    },
                    StorageItem::Source {
                        name: "lib.rs".to_owned(),
                        txt: format!("{} {}", impl_lib.to_string(), rus_ty.to_string()),
                    },
                ],
            },
            StorageItem::Source {
                name: "Cargo.toml".to_owned(),
                txt: impl_cargo.to_string(),
            },
        ],
        name: "idl_types".to_owned(),
    }
}
