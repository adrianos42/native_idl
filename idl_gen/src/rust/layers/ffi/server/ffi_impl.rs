use idl::{analyzer::Analyzer};
use idl::idl_nodes::*;

use crate::{rust::con_idl::{get_rust_ty_name, get_rust_ty_ref}};

use crate::rust::string_pros::{StringPros, StringRustFmt};
use proc_macro2::{self, Literal, TokenStream};
use quote::{TokenStreamExt, ToTokens};
use quote::format_ident;
use std::fmt;

use super::FFIServerError;

pub struct FFIServerImpl {
    module: Vec<TokenStream>,
}

impl ToTokens for FFIServerImpl {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(&self.module);
    }
}

impl fmt::Display for FFIServerImpl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result_code = String::new();

        self.module.iter().for_each(|value| {
            result_code += &value.to_string();
        });

        write!(f, "{}", result_code.rust_fmt())
    }
}

impl FFIServerImpl {
    pub fn generate(package_name: &str, analyzer: &Analyzer) -> Result<Self, FFIServerError> {
        let mut context = FFIServerImpl::new();

        let library_name = analyzer.library_name();
        let library_ident = format_ident!("{}", library_name);
        let lib_ident = if package_name == analyzer.library_name() { 
            quote! { #library_ident } 
        } else {
            let package_ident = format_ident!("{}", package_name);
            quote! { #package_ident::#library_ident }
        };

        context.module.push(quote! {
            use idl_internal::*;
            use #lib_ident::idl_types::idl_impl;
            use #lib_ident;
        });

        let nodes: &[IdlNode] = &analyzer.nodes;
        for node in nodes {
            match node {
                IdlNode::TypeInterface(value) => context.add_interface(value, analyzer)?,
                _ => {}
            }
        }

        Ok(context)
    }

    fn add_interface(
        &mut self,
        ty_interface: &TypeInterface,
        analyzer: &Analyzer,
    ) -> Result<(), FFIServerError> {
        let ident = &ty_interface.ident;

        let library_ident_impl = quote! { idl_impl };

        let streams: Vec<(&TypeName, String)> = ty_interface
            .fields
            .iter()
            .filter_map(|node| match node {
                InterfaceNode::InterfaceField(field) => match &field.ty {
                    TypeName::TypeFunction(value) => match &value.return_ty {
                        TypeName::TypeStream(value) => Some((&value.s_ty, field.ident.to_owned())),
                        _ => None,
                    },
                    TypeName::TypeStream(value) => Some((&value.s_ty, field.ident.to_owned())),
                    _ => None,
                },
                _ => None,
            })
            .collect();

        let mut stream_im = vec![];
        for (_, name) in streams {
            let stream_name_ident = format_ident!("{}{}", &ident, name.to_pascal_case());
            stream_im.push(quote! {
                pub(super) struct #stream_name_ident {
                    pub(super) callback: extern "C" fn(i64, *const ::core::ffi::c_void),
                    pub(super) object: *const ::core::ffi::c_void,
                    pub(super) object_id: i64,
                    pub(super) handle: i64,
                }

                unsafe impl Send for #stream_name_ident {} // TODO ??

                impl StreamInstance for #stream_name_ident {
                    fn wake_client(&self) {
                        let run = self.callback;
                        run(self.handle, self.object);
                    }

                    fn get_id(&self) -> i64 {
                        self.object_id
                    }
                }
            });
        }

        self.module.push(quote! { #( #stream_im )* });

        let interface_ident = format_ident!("{}", &ident);
            let lib_ident = format_ident!("{}", analyzer.library_name());

        if Analyzer::interface_has_non_static_field(ty_interface) {
            let interface_instance_ident = format_ident!("{}Instance", &ident);

            self.module.push(quote! {
                pub struct #interface_instance_ident {
                    pub(super) instance: Box<dyn #library_ident_impl::#interface_instance_ident>,
                }
                impl #interface_instance_ident {
                    pub(super) fn new() -> Self { 
                        Self {
                            instance: Box::new(#lib_ident::#interface_ident::new()),
                        }
                    }
                }
                impl From<Box<dyn #library_ident_impl::#interface_instance_ident>> for #interface_instance_ident {
                    fn from(value: Box<dyn #library_ident_impl::#interface_instance_ident>) -> Self {
                        Self {
                            instance: value,
                        }
                    }
                }
            });
        }

        if Analyzer::interface_has_static_field(ty_interface) {
            let interface_static_ident = format_ident!("{}Static", &ident);
            let interface_static_instance_ident = format_ident!("{}InstanceStatic", &ident);

            let mut static_fields = vec![];
            let mut static_instance_fields = vec![];

            for field in ty_interface.fields.iter().filter_map(|m| match m {
                InterfaceNode::InterfaceField(field) if field.is_static =>  {
                    Some(field)
                }
                _ => None,
            }) {
                let mut args_name = vec![];
                let mut args_ident = vec![];
                let func_ident = format_ident!("{}", &field.ident);

                    let (args, ret_ty) = match &field.ty {
                        TypeName::TypeFunction(value) => (
                            Some(match &value.args {
                                TypeName::TypeTuple(tuple) => &tuple.fields,
                                _ => panic!("Invalid function type"),
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
                            let arg_ty_ident = get_rust_ty_ref(&arg.ty, true);
                            args_ident.push(quote! { #arg_ident });
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
                                args_ident.push(quote! { stream_instance });
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

                    static_fields.push(quote! {
                        pub fn #func_ident(#( #args_name ),* )#ret_value_ident {
                            #interface_static_instance_ident::#func_ident::<#lib_ident::#interface_ident>(#( #args_ident ),*)
                        }
                    });

                    static_instance_fields.push(quote! {
                        fn #func_ident<T: #library_ident_impl::#interface_static_ident>(#( #args_name ),* )#ret_value_ident {
                            T::#func_ident(#( #args_ident ),*)
                        }
                    });

                    if let Some((arg_ty, stream_ty)) = stream_arg {
                        let f_ident = format_ident!("{}_stream_sender", &field.ident);
                        let a_ty = get_rust_ty_ref(arg_ty, true);
                        let s_ty = get_rust_ty_ref(stream_ty, true);
                        static_fields.push(quote! {
                            pub(crate) fn #f_ident(stream_instance: #a_ty, stream_sender: StreamSender<#s_ty>) -> StreamReceiver {
                                #interface_static_instance_ident::#f_ident::<#lib_ident::#interface_ident>(stream_instance, stream_sender)
                            }
                        });
                        static_instance_fields.push(quote! {
                            fn #f_ident<T: #library_ident_impl::#interface_static_ident>(stream_instance: #a_ty, stream_sender: StreamSender<#s_ty>) -> StreamReceiver {
                                T::#f_ident(stream_instance,stream_sender)
                            }
                        });
                    }
                    if let Some((ret_ty, stream_ty)) = stream_ret {
                        let f_ident = format_ident!("{}_stream", &field.ident);
                        let r_ty = get_rust_ty_ref(ret_ty, true);
                        let s_ty = get_rust_ty_ref(stream_ty, true);
                        static_fields.push(quote! {
                            pub(crate) fn #f_ident(stream_instance: #r_ty, stream_receiver: StreamReceiver) -> StreamSender<#s_ty> {
                                #interface_static_instance_ident::#f_ident::<#lib_ident::#interface_ident>(stream_instance, stream_receiver)
                            }
                        });

                        static_instance_fields.push(quote! {
                            fn #f_ident<T: #library_ident_impl::#interface_static_ident>(stream_instance: #r_ty, stream_receiver: StreamReceiver) -> StreamSender<#s_ty> {
                                T::#f_ident(stream_instance,stream_receiver)
                            }
                        });
                    }
            }
            
            self.module.push(quote! {
                pub (crate) struct #interface_static_ident;
                impl #interface_static_ident { #( #static_fields )* }
                struct #interface_static_instance_ident;
                impl #interface_static_instance_ident { #( #static_instance_fields )* }
            })

        }

        Ok(())
    }

    fn new() -> Self {
        Self { module: vec![] }
    }
}


