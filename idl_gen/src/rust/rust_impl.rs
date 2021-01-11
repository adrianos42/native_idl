use idl::idl::analyzer::Analyzer;
use idl::idl::idl_nodes::*;

use super::con_idl::get_rust_ty_ref;

use super::string_pros::StringPros;
use proc_macro2::{self, Ident, Span, TokenStream};
use std::fmt;

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

                    let mut stream_arg: Option<(&TypeName, &TypeName)> = None;
                    let mut stream_ret: Option<(&TypeName, &TypeName)> = None;

                    if let Some(args) = args {
                        for arg in args {
                            let arg_ident = Ident::new(&arg.ident, Span::call_site());
                            let arg_ty_ident = get_rust_ty_ref(&arg.ty, true);
                            args_name.push(quote! { #arg_ident: #arg_ty_ident });

                            if let TypeName::TypeStream(value) = &arg.ty {
                                stream_arg = Some((&arg.ty, &value.s_ty));
                            }
                        }
                    }

                    let ret_value_ident = match ret_ty {
                        TypeName::Types(Types::NatNone) | TypeName::TypeStream(_) => {
                            if let TypeName::TypeStream(value) = ret_ty {
                                let stream_ty_ident = get_rust_ty_ref(&ret_ty, true);
                                args_name.push(quote! { stream_instance: #stream_ty_ident });
                                stream_ret = Some((ret_ty, &value.s_ty));
                            }
                            quote! {}
                        }
                        _ => {
                            let ret_ty_ident = get_rust_ty_ref(&ret_ty, true);
                            quote! { -> #ret_ty_ident }
                        }
                    };

                    let (fields_add, self_ident) = if field.is_static {
                        (&mut static_fields, quote! { &self })
                    } else {
                        (&mut fields, quote! { &mut self })
                    };

                    fields_add.push(quote! {
                        fn #func_ident(#self_ident, #( #args_name ),* )#ret_value_ident;
                    });

                    if let Some((arg_ty, stream_ty)) = stream_arg {
                        let ident = Ident::new(
                            &format!("{}_stream_sender", &field.ident),
                            Span::call_site(),
                        );
                        let a_ty = get_rust_ty_ref(arg_ty, true);
                        let s_ty = get_rust_ty_ref(stream_ty, true);
                        fields_add.push(quote! {
                            fn #ident(#self_ident, stream_instance: #a_ty, stream: StreamSender<#s_ty>) -> StreamReceiver;
                        });
                    }
                    if let Some((ret_ty, stream_ty)) = stream_ret {
                        let ident =
                            Ident::new(&format!("{}_stream", &field.ident), Span::call_site());
                        let r_ty = get_rust_ty_ref(ret_ty, true);
                        let s_ty = get_rust_ty_ref(stream_ty, true);
                        fields_add.push(quote! {
                            fn #ident(#self_ident, stream_instance: #r_ty, stream: StreamReceiver) -> StreamSender<#s_ty>;
                        });
                    }
                }
                InterfaceNode::Comment(_) => {}
            }
        }

        if !fields.is_empty() {
            let interface_ident = Ident::new(&format!("{}Instance", &ident), Span::call_site());
            self.module
                .push(quote! { pub trait #interface_ident { #( #fields )* } });
        }

        if !static_fields.is_empty() {
            let interface_static_ident =
                Ident::new(&format!("{}Static", &ident), Span::call_site());
            self.module
                .push(quote! { pub trait #interface_static_ident { #( #static_fields )* } });
        }

        Ok(())
    }

    fn new() -> Self {
        Self { module: vec![] }
    }
}
