use idl::analyzer::Analyzer;
use idl::idl_nodes::*;
use idl::ids;

use crate::rust::string_pros::{to_hex_string, StringPros, StringRustFmt};
use proc_macro2::{self, TokenStream};
use quote::format_ident;
use quote::{ToTokens, TokenStreamExt};
use std::fmt;

use super::super::create_hash_idents;
use super::BytesServerError;

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
    ) -> Result<Self, BytesServerError> {
        let mut context = BytesPackage::new();

        let package_name = package.name();
        let package_ident = format_ident!("{}", package_name.to_pascal_case());

        let input_ident = quote! { _input };
        let output_ident = quote! { _output };
        let parse_func_ident = quote! { parse_request_bytes };

        let ret_type = quote! { Result<(), ()> };

        let interface_instances: Vec<TokenStream> = analyzers
            .iter()
            .filter(|v| v.any_interface_has_non_static_field())
            .map(|analyzer| {
                let instance = format_ident!("_instance_{}", analyzer.library_name());
                let ident_type = format_ident!("{}", analyzer.library_name().to_pascal_case());
                let ident_mod = format_ident!("{}", analyzer.library_name());
                let ident_mod = if analyzer.library_name() == package_name {
                    quote! { ws:: }
                } else {
                    quote! { #ident_mod::ws:: }
                };

                quote! { #instance: #ident_mod#ident_type }
            })
            .collect();

        let hash_library_match: Vec<TokenStream> = analyzers
            .iter()
            .map(|analyzer| {
                let library_call = if analyzer.any_interface_has_non_static_field() {
                    let instance = format_ident!("_instance_{}", analyzer.library_name());
                    quote! { self.#instance.#parse_func_ident }
                } else {
                    quote! { Self::#parse_func_ident }
                };
                let library_digest_ident: TokenStream =
                    create_hash_idents(&analyzer.library_hash());
                quote! { [#library_digest_ident] => { #library_call(#input_ident, #output_ident) } }
            })
            .collect();

        let package_digest_ident: TokenStream = create_hash_idents(&package.hash);

        context.module.push(quote! {
            pub struct #package_ident { #( #interface_instances ),* }

            impl #package_ident {
                pub fn new() -> Self { Self { #( #interface_instances::new() ),* } }

                pub fn #parse_func_ident<R: ::std::io::Read, W: ::std::io::Write>(
                    &mut self,
                    #input_ident: &mut R,
                    #output_ident: &mut W,
                ) -> #ret_type {
                    let mut _package_hash: [u8; 0x10] = [0x0; 0x10];
                    #input_ident.read_exact(&mut _package_hash[..]).unwrap(); // Package hash

                    match _package_hash[..] {
                        [#package_digest_ident] => {
                            let mut _hash: [u8; 0x10] = [0x0; 0x10];
                            #input_ident.read_exact(&mut _hash[..]).unwrap(); // Library hash

                            match _hash[..] {
                                #( #hash_library_match )*
                                _ => panic!("Interface not defined"),
                            }
                         }
                        _ => panic!("Invalid package hash value")
                    }
                }
            }
        });

        Ok(context)
    }

    fn new() -> Self {
        Self { module: vec![] }
    }
}
