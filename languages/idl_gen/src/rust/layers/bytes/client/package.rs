use idl::analyzer::Analyzer;
use idl::idl_nodes::*;
use idl::ids;

use crate::rust::string_pros::{StringPros, StringRustFmt};
use proc_macro2::{self, TokenStream};
use quote::format_ident;
use quote::{ToTokens, TokenStreamExt};
use std::fmt;

use super::super::create_hash_idents;
use super::BytesServerError;

// Generates only the function used to parse the data received 
// by the server and then sending the data through the response 
// body using the `_call_id` variable and the `_response` value.
pub struct BytesPackage {
    module: Vec<TokenStream>,
}

impl ToTokens for BytesPackage {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(&self.module);
    }
}

impl fmt::Display for BytesPackage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result_code = String::new();

        self.module.iter().for_each(|value| {
            result_code += &value.to_string();
        });

        write!(f, "{}", result_code.rust_fmt())
    }
}

impl BytesPackage {
    pub fn generate(
        package: &ids::package::Package,
        analyzers: &[idl::analyzer::Analyzer],
        response_body: TokenStream,
        is_async: bool,
    ) -> Result<Self, BytesServerError> {
        let mut context = Self::new();

        let input_ident = quote! { _input };
        let call_id_ident = quote! { _call_id };
        let parse_func_ident = quote! { parse_response_bytes };

        let ret_type = quote! { Result<(), ::std::io::Error> };

        let hash_library_match: Vec<TokenStream> = analyzers
            .iter()
            .map(|analyzer| {
                let mut hash_interface_match = vec![];

                for node in &analyzer.nodes {
                    if let IdlNode::TypeInterface(value) = node {
                        let interface_digest_ident: TokenStream = create_hash_idents(&value.hash);
                        hash_interface_match.push(quote! { [#interface_digest_ident] });
                    }
                }

                let library_digest_ident: TokenStream =
                    create_hash_idents(&analyzer.library_hash());
                quote! { [#library_digest_ident] => {
                    match _interface_hash[..] {
                        #( #hash_interface_match )|* => { #response_body }
                        _ => panic!("Invalid interface hash value"),
                    }
                } }
            })
            .collect();

        let package_digest_ident: TokenStream = create_hash_idents(&package.hash);

        let async_ident = if is_async { quote! { async } } else { quote! {} };

        context.module.push(quote! {
            #async_ident fn #parse_func_ident<R: ::std::io::Read>(
                #input_ident: &mut R,
            ) -> #ret_type {
                let mut _package_hash: [u8; 0x10] = [0x0; 0x10];
                #input_ident.read_exact(&mut _package_hash[..])?; // Package hash
                match _package_hash[..] {
                    [#package_digest_ident] => {
                        let mut _hash: [u8; 0x10] = [0x0; 0x10];
                        #input_ident.read_exact(&mut _hash[..])?; // Library hash
                        let mut _interface_hash: [u8; 0x10] = [0x0; 0x10];
                        #input_ident.read_exact(&mut _interface_hash[..])?; // Interface hash
                        let #call_id_ident = Uuid::from_u128(_input.read_u128::<BigEndian>()?);
                        let mut _response = Vec::new();
                        _input.read_to_end(&mut _response).unwrap();
                        match _hash[..] {
                            #( #hash_library_match )*
                            _ => panic!("Invalid library hash value"),
                        }
                     }
                    _ => panic!("Invalid package hash value")
                }
                Ok(())
            }
        });

        Ok(context)
    }

    fn new() -> Self {
        Self { module: vec![] }
    }
}
