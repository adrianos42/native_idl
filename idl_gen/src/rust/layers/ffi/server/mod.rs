use idl::{analyzer::Analyzer};
use idl::idl_nodes::*;

use crate::{rust::con_idl::{get_rust_ty_name, get_rust_ty_ref}};

use crate::rust::string_pros::{StringPros, StringRustFmt};
use proc_macro2::{self, Literal, TokenStream};
use quote::{TokenStreamExt, ToTokens};
use quote::format_ident;
use std::{collections::HashMap, fmt};

use super::*;

pub(crate) mod layer;
pub(super) mod server_cargo;
pub(super) mod server_impl;
pub(super) mod server_types;
pub(super) mod server_mod;

#[derive(Debug)]
pub enum FFIServerError {
    UnexpectedType,
    Undefined,
}

pub struct FFIServer {
    module: Vec<TokenStream>,
}

impl ToTokens for FFIServer {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(&self.module);
    }
}

impl fmt::Display for FFIServer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result_code = String::new();

        self.module.iter().for_each(|value| {
            result_code += &value.to_string();
        });

        write!(f, "{}", result_code.rust_fmt())
    }
}

impl FFIServer {
    pub fn generate(package_name: &str, analyzer: &Analyzer) -> Result<Self, FFIServerError> {
        let mut context = FFIServer::new();

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
            use idl_internal::{Uuid, ffi::ffi_types::*, #( #deps ),* };
            use super::ffi_impl;
            use super::ffi_types;
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

        context.add_interfaces_conv(&interfaces, analyzer)?;

        Ok(context)
    }

    fn add_interface(
        &mut self,
        ty_interface: &TypeInterface,
        analyzer: &Analyzer,
    ) -> Result<(), FFIServerError> {
        let ident = &ty_interface.ident;

        let mut fields = vec![];
        //let interface_ident = format_ident!("{}", &ident);
        let interface_instance_ident = format_ident!("{}Instance", &ident);
        let interface_static_ident = format_ident!("{}Static", &ident);
        let library_ident_ffi_impl = quote! { ffi_impl };

        let create_fn = |func_ffi_ident, args, body| {
            quote! {
                #[no_mangle]
                #[allow(unused_braces)]
                pub extern "C" fn #func_ffi_ident( #args ) -> i64 {
                    match ::std::panic::catch_unwind(::std::panic::AssertUnwindSafe(move || { #body })) {
                        Ok(value) => value as i64,
                        Err(_) => AbiInternalError::UndefinedException as i64,
                    }
                }
            }
        };

        let ins_ident = quote! { _ins };
        let instance_ident = quote! { _instance };

        let conv_stream_to_val = |value_ident: &TokenStream,
                                  is_receiving,
                                  field: &Box<InterfaceField>| {
            let field_ident = format_ident!("{}{}", ident, field.ident.to_pascal_case());

            let conv_b = quote! {
                Box::new(ffi_impl::#field_ident {
                    callback: unsafe { std::mem::transmute(_value.wake_callback) },
                    handle: _value.wake_handle,
                    object: _value.wake_object,
                })
            };

            let ret_match = if field.is_static {
                quote! { return AbiInternalError::InvalidArg, }
            } else {
                quote! { { std::mem::forget(#ins_ident); return AbiInternalError::InvalidArg; } }
            };

            let matchs = if is_receiving {
                quote! {
                    match _value.state.into() {
                        AbiStreamReceiverState::Start => { #conv_b }
                        _ => #ret_match
                    }
                }
            } else {
                quote! {
                    match _value.state.into() {
                        AbiStreamSenderState::Waiting => { #conv_b }
                        _ => #ret_match
                    }
                }
            };

            quote! { {
                let _value = #value_ident;
                #matchs
            } }
        };

        let conv_stream_body_to_val = |value_name: &TokenStream, field_name: &String| {
            let field_ident = format_ident!("{}{}", ident, field_name.to_pascal_case());

            quote! {
                Box::new(ffi_impl::#field_ident {
                    callback: unsafe { std::mem::transmute(#value_name.wake_callback) },
                    handle: #value_name.wake_handle,
                    object: #value_name.wake_object,
                })
            }
        };

        for field_node in &ty_interface.fields {
            match field_node {
                InterfaceNode::InterfaceField(field) => {
                    let field_name = format!("{}_{}", ident.to_snake_case(), field.ident);
                    let func_ident = format_ident!("{}", &field.ident);

                    let mut args_value = vec![];
                    let mut args_ffi = vec![];

                    let instance_call = if field.is_static {
                        quote! { ffi_impl::#interface_static_ident:: }
                    } else {
                        quote! { #instance_ident. }
                    };

                    let (args, ret_ty) = match &field.ty {
                        TypeName::TypeFunction(value) => (
                            Some(match &value.args {
                                TypeName::TypeTuple(tuple) => &tuple.fields,
                                _ => return Err(FFIServerError::UnexpectedType),
                            }),
                            &value.return_ty,
                        ),
                        TypeName::TypeTuple(tuple) => {
                            // Returns none type, since it's accepted anyways.
                            (Some(&tuple.fields), &TypeName::Types(Types::NatNone))
                        }
                        _ => (None, &field.ty),
                    };

                    let mut args_conv_ffi: Vec<TokenStream> = vec![];
                    let mut stream_arg: Option<&TypeName> = None;

                    // Create function arg names and conversion between ffi and rust types.
                    if let Some(args) = args {
                        for arg in args {
                            let arg_ident = format_ident!("{}", &arg.ident);
                            let arg_ty_ident = arg.ty.get_ffi_ty_ref(true, analyzer);

                            let arg_call_name = format!("_{}_arg_val", arg.ident);
                            let arg_value_ident = format_ident!("{}", &arg_call_name);
                            args_value.push(quote! { #arg_value_ident });

                            args_ffi.push(quote! { #arg_ident: #arg_ty_ident });

                            let arg_ident = quote! { #arg_ident };
                            let result_arg = if let TypeName::TypeStream(_) = &arg.ty {
                                stream_arg = Some(&arg.ty);
                                let con = arg.ty.conv_ffi_to_value(&arg_ident, true, analyzer);
                                conv_stream_to_val(&con, false, &field)
                            } else {
                                arg.ty.conv_ffi_to_value(&arg_ident, true, analyzer)
                            };
                            args_conv_ffi.push(quote! { let #arg_value_ident = #result_arg; });
                        }
                    }

                    let mut ret_conv_ffi: Vec<TokenStream> = vec![];

                    let ret_value_ident = match ret_ty {
                        TypeName::Types(Types::NatNone) => quote! {},
                        TypeName::TypeStream(_) => {
                            // Add the argument that is converted to a rust value
                            let stream_ident = quote! { _stream };
                            let stream_value_ident = quote! { _stream_val };
                            let arg_ty_ident = ret_ty.get_ffi_ty_ref(true, analyzer);
                            args_value.push(quote! { #stream_value_ident });
                            args_ffi.push(quote! { #stream_ident: #arg_ty_ident });
                            let con = ret_ty.conv_ffi_to_value(&stream_ident, true, analyzer);
                            let result_stream = conv_stream_to_val(&con, true, &field);
                            args_conv_ffi
                                .push(quote! { let #stream_value_ident = #result_stream; });

                            quote! {}
                        }
                        _ => {
                            let result_ident = quote! { _result };
                            let result_ty_ident = ret_ty.get_ffi_ty_ref(true, analyzer);
                            args_ffi.push(quote! { #result_ident: *mut #result_ty_ident });

                            let result_val_ident = quote! { _result_val };

                            let result_conv =
                                ret_ty.conv_value_to_ffi(&result_val_ident, true, analyzer);

                            let r_ty = get_rust_ty_ref(ret_ty, true);

                            ret_conv_ffi.push(quote! {
                                unsafe { *#result_ident = #result_conv; }
                            });
                            quote! { let #result_val_ident: #r_ty= }
                        }
                    };

                    let (body_ident, instance_args) = if field.is_static {
                        (
                            quote! {
                                #( #args_conv_ffi );*
                                #ret_value_ident #instance_call#func_ident( #( #args_value ),* );
                                #( #ret_conv_ffi );*
                                return AbiInternalError::Ok;
                            },
                            quote! {},
                        )
                    } else {
                        (
                            quote! {
                                let mut #ins_ident = unsafe { Box::from_raw(#instance_ident) };
                                let mut #instance_ident = &mut #ins_ident.instance;
                                #( #args_conv_ffi );*
                                #ret_value_ident #instance_call#func_ident( #( #args_value ),* );
                                #( #ret_conv_ffi );*
                                std::mem::forget(#ins_ident);
                                return AbiInternalError::Ok;
                            },
                            quote! { #instance_ident: *mut #library_ident_ffi_impl::#interface_instance_ident, },
                        )
                    };

                    let args = quote! {
                        #instance_args
                        #( #args_ffi ),*
                    };

                    let func_ffi_ident = format_ident!(
                        "{}_{}_{}",
                        analyzer.library_name(),
                        if field.is_static { "static" } else { "method" },
                        field_name
                    );
                    fields.push(create_fn(func_ffi_ident, args, body_ident));

                    // Add the function to set the stream callback
                    if let TypeName::TypeStream(_) = ret_ty {
                        let func_ffi_ident = format_ident!("stream_{}", field_name);
                        let func_ident = format_ident!("{}_stream", &field.ident);
                        let stream_ident = quote! { _stream };
                        let stream_result_ident = quote! { _stream_result };
                        let stream_value_ident = quote! { _stream_val };
                        let args = quote! {
                            #instance_args
                            #stream_ident: *const AbiStream,
                            #stream_result_ident: *mut *mut AbiStream,
                        };
                        let con = ret_ty.conv_ffi_to_value(&stream_ident, true, analyzer);
                        let conv = conv_stream_body_to_val(&stream_value_ident, &field.ident);

                        let body_stream = quote! {
                            let #stream_value_ident = #con;
                            let _result = #instance_call#func_ident(#conv,#stream_value_ident.into());
                            unsafe {
                                *#stream_result_ident = { Box::into_raw(Box::new({ _result.into_abi() })) as *mut AbiStream };
                            }

                        };

                        let body_ident = if field.is_static {
                            quote! {
                                #body_stream
                                return AbiInternalError::Ok;
                            }
                        } else {
                            quote! {
                                let mut #ins_ident = unsafe { Box::from_raw(#instance_ident) };
                                let mut #instance_ident = &mut #ins_ident.instance;
                                #body_stream
                                std::mem::forget(#ins_ident);
                                return AbiInternalError::Ok;
                            }
                        };

                        fields.push(create_fn(func_ffi_ident, args, body_ident));

                        let func_ffi_ident = format_ident!("dispose_stream_{}", field_name);
                        let args_ident = quote! {
                                #instance_args
                                #stream_ident: *mut AbiStream,
                        };
                        let body_ident = quote! {
                            let #stream_value_ident: Box<AbiStream> = unsafe { Box::from_raw(#stream_ident) };
                            #stream_value_ident.dispose();
                            return AbiInternalError::Ok;
                        };

                        fields.push(create_fn(func_ffi_ident, args_ident, body_ident));
                    } else if ret_ty.is_ptr_ffi(analyzer) {
                        let func_ffi_ident = format_ident!("{}_dispose_{}", analyzer.library_name(), field_name);

                        let result_ident = quote! { _result_disp };
                        let result_ty_ident = ret_ty.get_ffi_ty_ref_mut(true, analyzer);
                        let args_ident =
                            quote! { 
                                #instance_args 
                                #result_ident: #result_ty_ident 
                        };
                        let result_dispose = ret_ty.dispose_ffi_ptr(&result_ident, true, analyzer);
                        let body_ident = quote! {
                            #result_dispose
                            return AbiInternalError::Ok;
                        };

                        fields.push(create_fn(func_ffi_ident, args_ident, body_ident));
                    }

                    if let Some(stream_arg_ty) = stream_arg {
                        let func_ffi_ident = format_ident!("{}_stream_sender_{}", analyzer.library_name(), field_name);
                        let func_ident = format_ident!("{}_stream_sender", &field.ident);
                        let stream_ident = quote! { _stream };
                        let stream_result_ident = quote! { _stream_result };
                        let stream_value_ident = quote! { _stream_val };
                        let args = quote! {
                            #instance_args
                            #stream_ident: *const AbiStream,
                            #stream_result_ident: *mut *mut AbiStream,
                        };
                        let con = stream_arg_ty.conv_ffi_to_value(&stream_ident, true, analyzer);
                        let conv = conv_stream_body_to_val(&stream_value_ident, &field.ident);

                        let body_stream = quote! {
                            let #stream_value_ident = #con;
                            let _result = #instance_call#func_ident(#conv,#stream_value_ident.into_sender());
                            unsafe {
                                *#stream_result_ident = { Box::into_raw(Box::new({ _result.into() })) as *mut AbiStream };
                            }
                        };

                        let body_ident = if field.is_static {
                            quote! {
                                #body_stream
                                return AbiInternalError::Ok;
                            }
                        } else {
                            quote! {
                                let mut #ins_ident = unsafe { Box::from_raw(#instance_ident) };
                                let mut #instance_ident = &mut #ins_ident.instance;
                                #body_stream
                                std::mem::forget(#ins_ident);
                                return AbiInternalError::Ok;
                            }
                        };

                        fields.push(create_fn(func_ffi_ident, args, body_ident));

                        let func_ffi_ident = format_ident!("{}_dispose_stream_sender_{}", analyzer.library_name(), field_name);
                        let args_ident = quote! {
                            #instance_args
                            #stream_ident: *mut AbiStream,
                        };
                        let body_ident = quote! {
                            let #stream_value_ident: Box<AbiStream> = unsafe { Box::from_raw(#stream_ident) };
                            return AbiInternalError::Ok;
                        };

                        fields.push(create_fn(func_ffi_ident, args_ident, body_ident));
                    }
                }
                InterfaceNode::Comment(_) => {}
            }
        }

        if Analyzer::interface_has_non_static_field(ty_interface) {
            // instance creation
            let (ret_conv_ffi, arg_ident) = {
                let interface_ty_ident = TypeName::InterfaceTypeName(ident.to_owned());
                let result_ident = quote! { _result };
                // let result_val_ident = quote! { _result_val };

                let result_ty_ident = interface_ty_ident.get_ffi_ty_ref(true, analyzer);

                let result_conv = quote! {
                    Box::into_raw(Box::new({
                            ffi_impl::#interface_instance_ident::new()
                        })) as *const ffi_impl::#interface_instance_ident
                };
                (
                    quote! { unsafe { *#result_ident = #result_conv; } },
                    quote! { #result_ident: *mut #result_ty_ident },
                )
            };

            let field_name = format!("create_{}", ident.to_snake_case());
            let func_ffi_ident = format_ident!("{}_{}", analyzer.library_name(), &field_name);

            let body_ident = quote! {
                #ret_conv_ffi
                return AbiInternalError::Ok;
            };

            fields.push(create_fn(func_ffi_ident, arg_ident, body_ident));

            // instance release
            let arg_ident = {
                let interface_ty_ident = TypeName::InterfaceTypeName(ident.to_owned());
                let result_ident = quote! { _result };
               // let result_val_ident = quote! { _result_val };

                let result_ty_ident = interface_ty_ident.get_ffi_ty_ref(true, analyzer);

                quote! { #result_ident: *mut #result_ty_ident }
            };

            let field_name = format!("{}_dispose_{}", analyzer.library_name(), ident.to_snake_case());
            let func_ffi_ident = format_ident!("{}", &field_name);

            // let ins_ident = quote! { _ins };
            // let instance_ident = quote! { _instance };

            // let instance_body = quote! { let #instance_ident = 0 as *mut #library_ident_ffi_impl::#interface_static_ident; };

            let body_ident = quote! {
               // #instance_body
                //let #ins_ident = unsafe { Box::from_raw(#instance_ident) };
                //let #instance_ident = &#ins_ident.instance;
                return AbiInternalError::Ok;
            };

            fields.push(create_fn(func_ffi_ident, arg_ident, body_ident));
        }

        self.module.push(quote! { #( #fields )* });

        Ok(())
    }

    fn add_interfaces_conv(
        &mut self,
        interfaces: &[&TypeInterface],
        analyzer: &Analyzer,
    ) -> Result<(), FFIServerError> {
        let mut stream_ret_types = HashMap::new();
        let mut stream_send_types = HashMap::new();

        for node in interfaces {
            for node in &node.fields {
                match node {
                    InterfaceNode::InterfaceField(field) => {
                        if let Some(stream_ty) = Analyzer::field_stream_return_ty(field) {
                            let ty_name = get_rust_ty_name(stream_ty);
                            stream_ret_types.insert(ty_name, stream_ty);
                        }
                        if let Some(stream_send_ty) = Analyzer::field_stream_send_ty(field) {
                            let ty_name = get_rust_ty_name(stream_send_ty);
                            stream_send_types.insert(ty_name, stream_send_ty);
                        }
                    }
                    _ => {}
                }
            }
        }

        let mut fields = vec![];

        if !stream_ret_types.is_empty() {
            self.module.push(quote! {
                trait StreamSenderIntoAbiStream<T> {
                    fn into_abi(self) -> AbiStream;
                }
                trait StreamAbiSenderDispose<T> {
                    fn dispose(self);
                }
            });

            for ret_ty in stream_ret_types.values() {
                let r_ty = get_rust_ty_ref(ret_ty, true);
                let cont_val = ret_ty.conv_value_to_ffi_boxed(&quote! { value }, true, analyzer);

                fields.push(quote! {
                    impl StreamSenderIntoAbiStream<#r_ty> for StreamSender<#r_ty> {
                        #[allow(unused_braces)]
                        fn into_abi(self) -> AbiStream {
                            match self {
                                StreamSender::Ok => AbiStream::new(AbiStreamSenderState::Ok as i64),
                                StreamSender::Value(value) => {
                                    let mut _result = AbiStream::new(AbiStreamSenderState::Value as i64);
                                    _result.data = #cont_val as *const ::core::ffi::c_void;
                                    _result
                                }
                                StreamSender::Request => AbiStream::new(AbiStreamSenderState::Request as i64),
                                StreamSender::Waiting => AbiStream::new(AbiStreamSenderState::Waiting as i64),
                                StreamSender::Done => AbiStream::new(AbiStreamSenderState::Done as i64),
                            }
                        }
                    }
                    impl StreamAbiSenderDispose<#r_ty> for AbiStream {
                        fn dispose(self) {
                            match self.state.into() {
                                AbiStreamSenderState::Value => {}
                                _ => {}
                            }
                        }
                    }
                });
            }
        }

        if !stream_send_types.is_empty() {
            self.module.push(quote! {
                trait AbiStreamIntoStreamSender<T> {
                    fn into_sender(self) -> StreamSender<T>;
                }
            });

            for send_ty in stream_send_types.values() {
                let s_ty = get_rust_ty_ref(send_ty, true);
                let cont_val =
                    send_ty.conv_ffi_ptr_to_value(&quote! { self.data }, true, analyzer);

                fields.push(quote! {
                    impl AbiStreamIntoStreamSender<#s_ty> for AbiStream {
                        #[allow(unused_braces)]
                        fn into_sender(self) -> StreamSender<#s_ty> {
                            match self.state.into() {
                                AbiStreamSenderState::Ok => StreamSender::Ok,
                                AbiStreamSenderState::Value => StreamSender::Value(#cont_val),
                                AbiStreamSenderState::Request => StreamSender::Request,
                                AbiStreamSenderState::Waiting => StreamSender::Waiting,
                                AbiStreamSenderState::Done => StreamSender::Done,
                            }
                        }
                    }
                });
            }
        }

        self.module.push(quote! { #( #fields )* });

        Ok(())
    }

    fn new() -> Self {
        Self { module: vec![] }
    }
}
