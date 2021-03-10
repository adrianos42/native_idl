use idl::analyzer::Analyzer;
use idl::idl_nodes::*;

use crate::rust::con_idl::{get_rust_ty_name, get_rust_ty_ref};

use crate::rust::string_pros::{StringPros, StringRustFmt};
use proc_macro2::{self, Literal, TokenStream};
use quote::format_ident;
use quote::{ToTokens, TokenStreamExt};
use std::{collections::HashMap, fmt};

pub(crate) mod layer;
pub(super) mod ws_mod;
pub(super) mod ws_types;

#[derive(Debug)]
pub enum WSServerError {
    UnexpectedType,
    Undefined,
}

pub struct WSServer {
    module: Vec<TokenStream>,
}

impl ToTokens for WSServer {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(&self.module);
    }
}

impl fmt::Display for WSServer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result_code = String::new();

        self.module.iter().for_each(|value| {
            result_code += &value.to_string();
        });

        write!(f, "{}", result_code.rust_fmt())
    }
}

impl WSServer {
    pub fn generate(package_name: &str, analyzer: &Analyzer) -> Result<Self, WSServerError> {
        let mut context = WSServer::new();

        let mut deps = vec![];

        let library_name = analyzer.library_name();
        let library_ident = format_ident!("{}", library_name);
        let lib_ident = if package_name == analyzer.library_name() {
            quote! { #library_ident }
        } else {
            let package_ident = format_ident!("{}", package_name);
            quote! { #package_ident::#library_ident }
        };

        let has_field_returns_stream = analyzer.any_interface_field_returns_stream(None);
        let has_interface_field_sends_stream = analyzer.any_interface_field_sends_stream(None);

        if has_field_returns_stream | has_interface_field_sends_stream {
            deps.push(quote! { StreamSender });
        }

        context.module.push(quote! {
            use idl_internal::{Uuid, #( #deps ),* };
            use super::ws_impl;
            use super::ws_types;
            use #lib_ident::idl_types;
        });

        let nodes: &[IdlNode] = &analyzer.nodes;
        let mut interfaces: Vec<&TypeInterface> = vec![];
        for node in nodes {
            match node {
                IdlNode::TypeInterface(value) => {
                    context.add_interface(value, analyzer)?;
                    interfaces.push(value);
                }
                _ => {}
            }
        }

        Ok(context)
    }

    fn add_interface(
        &mut self,
        ty_interface: &TypeInterface,
        analyzer: &Analyzer,
    ) -> Result<(), WSServerError> {
        let ident = &ty_interface.ident;

        let mut fields = vec![];
        let interface_ident = format_ident!("{}", &ident);
        let interface_instance_ident = format_ident!("{}Instance", &ident);
        let interface_static_ident = format_ident!("{}Static", &ident);
        let library_ident_ws_impl = quote! { ws_impl };
        fields.push(quote! {});

        let create_fn = |func_ident, args, body| {
            quote! {
                #[no_mangle]
                #[allow(unused_braces)]
                pub fn #func_ident( #args ) -> i64 {
                    match ::std::panic::catch_unwind(::std::panic::AssertUnwindSafe(move || {
                        #body
                    })) {
                        Ok(value) => value as i64,
                        Err(_) => AbiInternalError::UndefinedException as i64,
                    }
                }
            }
        };

        let ins_ident = quote! { _ins };
        let instance_ident = quote! { _instance };

        for field_node in &ty_interface.fields {
            match field_node {
                InterfaceNode::InterfaceField(field) => {
                    let func_ident = format_ident!("{}", field.ident);
                    create_fn(func_ident, quote! {}, quote! {});
                }
                InterfaceNode::Comment(_) => {}
            }
        }

        if Analyzer::interface_has_non_static_field(ty_interface) {}

        self.module.push(quote! {
            pub(crate) struct #interface_ident {
                #( #fields )*
            }
        });

        Ok(())
    }

    fn new() -> Self {
        Self { module: vec![] }
    }
}
