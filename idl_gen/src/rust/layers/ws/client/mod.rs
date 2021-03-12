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

#[derive(Debug)]
pub enum WSClientError {
    UnexpectedType,
    Undefined,
}

pub struct WSClient {
    module: Vec<TokenStream>,
}

impl ToTokens for WSClient {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(&self.module);
    }
}

impl fmt::Display for WSClient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result_code = String::new();

        self.module.iter().for_each(|value| {
            result_code += &value.to_string();
        });

        write!(f, "{}", result_code.rust_fmt())
    }
}

impl WSClient {
    pub fn generate(package: &idl::ids::package::Package) -> Result<Self, WSClientError> {
        let mut context = WSClient::new();
        let package_name = package.name();

        let package_ident = format_ident!("{}", package_name.to_pascal_case());

        context.module.push(quote! {
            use idl_internal::tungstenite;
        });

        context.add_main(package)?;

        Ok(context)
    }

    fn add_main(&mut self, package: &idl::ids::package::Package) -> Result<(), WSClientError> {
        let package_name = package.name();
        let package_ident = format_ident!("{}", package_name.to_pascal_case());

        let url_lit = Literal::string("127.0.0.1:3012");

        let buffer_capacity = quote! { 0x1000 };

        self.module.push(quote! {
            fn main() {
                let client = ::std::net::TcpListener::bind(#url_lit).unwrap();
                for stream in client.incoming() {
                    ::std::thread::spawn(move || {
                        let mut package_inst: idl_ws::#package_ident = idl_ws::#package_ident::new();
                        
                        let callback = 
                            |request: &tungstenite::handshake::server::Request,
                             mut response: tungstenite::handshake::server::Response| {
                            // TODO
                            Ok(response)
                        };

                        let mut ws = tungstenite::accept_hdr(stream.unwrap(), callback).unwrap();

                        loop {
                            let message = ws.read_message().unwrap();
                            if message.is_binary() {
                                let data = message.into_data();
                                let mut output: Vec<u8> = Vec::with_capacity(#buffer_capacity);
                                match package_inst.parse_request_bytes(&mut data.as_slice(), &mut output) {
                                    Ok(_) => {} // TODO
                                    Err(err) => {
                                        panic!();
                                    },
                                }
                                ws.write_message(tungstenite::Message::Binary(output)).unwrap();
                            } else {
                                panic!();
                            }
                        }
                    });
                }
            }
        });

        Ok(())
    }

    fn new() -> Self {
        Self { module: vec![] }
    }
}
