use idl::analyzer::Analyzer;
use idl::idl_types::*;

use super::con_idl::get_rust_ty_ref;

use super::string_pros::StringPros;
use proc_macro2::{self, Ident, Span, TokenStream};
use std::fmt;

const NEW_LINE: &str = "\n\n";

#[derive(Debug)]
pub enum RustImplError {
    UnexpectedType,
    InvalidLiteral,
}

pub struct RustImpl {
    module: Vec<TokenStream>,
}

impl fmt::Display for RustImpl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result_code = String::new();

        self.module.iter().for_each(|value| {
            result_code += &value.to_string();
            result_code += NEW_LINE;
        });

        write!(f, "{}", result_code.as_str().rust_fmt())
    }
}

impl RustImpl {
    pub fn generate(analyzer: &Analyzer) -> Result<Self, RustImplError> {
        let mut context = RustImpl::new();
        let nodes: &[TypeNode] = &analyzer.nodes;

        for node in nodes {
            match node {
                TypeNode::InterfaceComment(_) => {}
                TypeNode::TypeInterface(value) => context.add_interface(value, analyzer)?,
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
        let interface_ident = Ident::new(&ident, Span::call_site());
        let interface_static_ident = Ident::new(&(format!("{}Static", &ident)), Span::call_site());

        for field_node in &ty_interface.fields {
            match field_node {
                InterfaceNode::InterfaceField(field) => {
                    let func_ident = Ident::new(&field.ident, Span::call_site());
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

                    let mut stream_arg: Option<TokenStream> = None;

                    if let Some(args) = args {
                        for arg in args {
                            match &arg.ty {
                                TypeName::TypeStream(_) => {
                                    stream_arg = Some(get_rust_ty_ref(&arg.ty, true))
                                }
                                _ => {
                                    let arg_ident = Ident::new(&arg.ident, Span::call_site());
                                    let arg_ty_ident = get_rust_ty_ref(&arg.ty, true);
                                    args_name.push(quote! { #arg_ident: #arg_ty_ident });
                                }
                            }
                        }
                    }

                    let ret_value_ident = match ret_ty {
                        TypeName::Types(Types::NatNone) | TypeName::TypeStream(_) => {
                            if let TypeName::TypeStream(_) = ret_ty {
                                let stream_ty_ident = get_rust_ty_ref(&ret_ty, true);
                                args_name.push(quote! { stream_handle: #stream_ty_ident })
                            }

                            if let Some(stream_arg) = stream_arg {
                                quote! { -> #stream_arg }
                            } else {
                                quote! {}
                            }
                        }
                        _ => {
                            let ret_ty_ident = get_rust_ty_ref(&ret_ty, true);
                            if let Some(stream_arg) = stream_arg {
                                quote! { -> (#stream_arg, #ret_ty_ident) }
                            } else {
                                quote! { -> #ret_ty_ident }
                            }
                        }
                    }; 

                    if field.is_static {
                        static_fields.push(quote! {
                            fn #func_ident(&self, #( #args_name ),* )#ret_value_ident;
                        });
                    } else {
                        fields.push(quote! {
                            fn #func_ident(&mut self, #( #args_name ),* )#ret_value_ident;
                        });
                    }
                }
                InterfaceNode::Comment(_) => {}
            }
        }

        if !fields.is_empty() {
            self.module
                .push(quote! { pub trait #interface_ident { #( #fields )* } });
        }

        if !Analyzer::interface_has_constructor_field(&ty_interface) {
            let ret_value_ident = {
                let ret_ty_ident =
                    get_rust_ty_ref(&TypeName::InterfaceTypeName(ident.to_owned()), true);
                quote! { -> #ret_ty_ident }
            };

            static_fields.push(quote! {
                fn create(&self)#ret_value_ident;
            })
        }

        self.module
            .push(quote! { pub trait #interface_static_ident { # ( #static_fields )* } });

        Ok(())
    }

    fn new() -> Self {
        Self { module: vec![] }
    }
}
