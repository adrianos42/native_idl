use idl::analyzer::Analyzer;
use idl::idl_nodes::*;

use crate::{rust::con_idl::{get_rust_ty_name, get_rust_ty_ref}};

use crate::rust::string_pros::StringPros;
use proc_macro2::{self, Literal, TokenStream};
use quote::{TokenStreamExt, ToTokens};
use quote::format_ident;
use std::i64;
use std::{collections::HashMap, fmt};

use super::*;

pub(crate) mod compile;

#[derive(Debug)]
pub enum FFIServerError {
    UnexpectedType,
    InvalidLiteral,
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

        write!(f, "{}", result_code.as_str().rust_fmt())
    }
}

impl FFIServer {
    pub fn generate(analyzer: &Analyzer) -> Result<Self, FFIServerError> {
        let mut context = FFIServer::new();

        let mut deps = vec![];

        let lib_ident =  format_ident!("{}", analyzer.get_library_name());

        let has_field_returns_stream = analyzer.any_interface_field_returns_stream(None);
        let has_interface_field_sends_stream = analyzer.any_interface_field_sends_stream(None);

        if has_field_returns_stream | has_interface_field_sends_stream {
            deps.push(quote! { StreamSender });
        }

        context.module.push(quote! {
            use idl_internal::{ffi::ffi_types::*, #( #deps ),* };
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
                            // Returns none type, since it's accepted anyway.
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
                                let con = conv_ffi_to_value(&arg.ty, &arg_ident, true, analyzer);
                                conv_stream_to_val(&con, false, &field)
                            } else {
                                conv_ffi_to_value(&arg.ty, &arg_ident, true, analyzer)
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
                            let con = conv_ffi_to_value(&ret_ty, &stream_ident, true, analyzer);
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
                                conv_value_to_ffi(&ret_ty, &result_val_ident, true, analyzer);

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
                        "{}_{}",
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
                        let con = conv_ffi_to_value(&ret_ty, &stream_ident, true, analyzer);
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
                    } else if is_boxed_ffi(ret_ty, analyzer) {
                        // FIXME change release
                        let func_ffi_ident = format_ident!("dispose_{}", field_name);

                        let result_ident = quote! { _result };
                        let result_ty_ident = ret_ty.get_ffi_ty_ref(true, analyzer);
                        let args_ident =
                            quote! { #instance_args #result_ident: *mut #result_ty_ident };
                        let body_ident = quote! {
                            //let _ = unsafe { Box::from_raw() };
                            //std::mem::forget(#ins_ident);

                            return AbiInternalError::Ok;
                        };

                        fields.push(create_fn(func_ffi_ident, args_ident, body_ident));
                    }

                    if let Some(stream_arg_ty) = stream_arg {
                        let func_ffi_ident = format_ident!("stream_sender_{}", field_name);
                        let func_ident = format_ident!("{}_stream_sender", &field.ident);
                        let stream_ident = quote! { _stream };
                        let stream_result_ident = quote! { _stream_result };
                        let stream_value_ident = quote! { _stream_val };
                        let args = quote! {
                            #instance_args
                            #stream_ident: *const AbiStream,
                            #stream_result_ident: *mut *mut AbiStream,
                        };
                        let con = conv_ffi_to_value(stream_arg_ty, &stream_ident, true, analyzer);
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

                        let func_ffi_ident = format_ident!("dispose_stream_sender_{}", field_name);
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
            let func_ffi_ident = format_ident!("{}", &field_name);

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

            let field_name = format!("dispose_{}", ident.to_snake_case());
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
                let cont_val = conv_value_to_ffi_boxed(&ret_ty, &quote! { value }, true, analyzer);

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
                    conv_ffi_ptr_to_value(&send_ty, &quote! { self.data }, true, analyzer);

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

pub struct FFIServerTypes {
    module: Vec<TokenStream>,
}

impl ToTokens for FFIServerTypes {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(&self.module);
    }
}

impl fmt::Display for FFIServerTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result_code = String::new();

        self.module.iter().for_each(|value| {
            result_code += &value.to_string();
        });

        write!(f, "{}", result_code.as_str().rust_fmt())
    }
}

impl FFIServerTypes {
    pub fn generate(analyzer: &Analyzer) -> Result<Self, FFIServerError> {
        let mut context = FFIServerTypes::new();

        let lib_ident =  format_ident!("{}", analyzer.get_library_name());

        context.module.push(quote! {
            use idl_internal::ffi::ffi_types::*;
            use #lib_ident::idl_types;
        });

        let nodes: &[IdlNode] = &analyzer.nodes;
        for node in nodes {
            match node {
                IdlNode::TypeStruct(value) => context.add_struct(value, analyzer)?,
                IdlNode::TypeList(value) => context.add_type_list(value, analyzer)?,
                IdlNode::TypeEnum(value) => context.add_enum(value, analyzer)?,
                _ => {}
            }
        }

        Ok(context)
    }

    fn add_struct(
        &mut self,
        ty_struct: &TypeStruct,
        analyzer: &Analyzer,
    ) -> Result<(), FFIServerError> {
        let ident = &ty_struct.ident;

        let mut fields = vec![];
        let mut fields_conv = vec![];
        let mut fields_value_conv = vec![];

        let struct_ident = format_ident!("{}", &ident);

        for field_node in &ty_struct.fields {
            match field_node {
                StructNode::StructField(field) => {
                    let field_name = format_ident!("{}", &field.ident);
                    let field_ty_name = field.ty.get_ffi_ty_ref(false, analyzer);
                    let field_value = quote! { value.#field_name };
                    fields.push(quote! { pub #field_name: #field_ty_name, });

                    let con_ty = conv_value_to_ffi(&field.ty, &field_value, false, analyzer);
                    let con_value_ty = conv_ffi_to_value(&field.ty, &field_value, false, analyzer);
                    fields_conv.push(quote! { #field_name: #con_ty });
                    fields_value_conv.push(quote! { #field_name: #con_value_ty });
                }
                StructNode::Comment(_) => {}
            }
        }

        self.module.push(quote! {
            #[repr(C)]
            pub struct #struct_ident { #( #fields )* }

            impl From<idl_types::#struct_ident> for #struct_ident {
                #[allow(unused_braces)]
                fn from(value: idl_types::#struct_ident) -> Self {
                    Self { #( #fields_conv ),* }
                }
            }

            impl From<#struct_ident> for idl_types::#struct_ident {
                #[allow(unused_braces)]
                fn from(value: #struct_ident) -> Self {
                    Self { #( #fields_value_conv ),* }
                }
            }
        });

        Ok(())
    }

    fn add_type_list(
        &mut self,
        type_list: &TypeList,
        analyzer: &Analyzer,
    ) -> Result<(), FFIServerError> {
        let ident = &type_list.ident;

        let ty_name_ident = format_ident!("{}", &ident);
        let list_ty_name = quote! { idl_types::#ty_name_ident };

        let mut m_tys = vec![];
        let mut m_tys_value = vec![];
        let mut index = 0;

        for field_node in &type_list.fields {
            match field_node {
                TypeListNode::TypeListField(field) => {
                    let f_ident = format_ident!("{}", &field.ident);
                    let f_name = Literal::i64_unsuffixed(index as i64);

                    let mut field_name = quote! { _value_field };

                    let is_boxed = match &field.ty {
                        TypeName::StructTypeName(_) | TypeName::ListTypeName(_) => true,
                        _ => false,
                    };

                    if is_boxed {
                        field_name = quote! { *#field_name }
                    }

                    let ty_ident = conv_value_to_ffi_boxed(&field.ty, &field_name, false, analyzer);

                    m_tys.push(quote! {
                        #list_ty_name::#f_ident(_value_field) => {
                            (#f_name as i64, #ty_ident as *const ::core::ffi::c_void)
                        }
                    });

                    let mut ty_ident =
                        conv_ffi_ptr_to_value(&field.ty, &(quote! { value.data }), false, analyzer);

                    if is_boxed {
                        ty_ident = quote! { Box::new(#ty_ident) }
                    }

                    m_tys_value.push(quote! {
                        #f_name => { #list_ty_name::#f_ident(#ty_ident) }
                    });

                    index += 1;
                }
                TypeListNode::Comment(_) => {}
            }
        }

        self.module.push(quote! {
            pub type #ty_name_ident = AbiVariant;

            impl From<idl_types::#ty_name_ident> for #ty_name_ident {
                #[allow(unused_braces)]
                fn from(value: idl_types::#ty_name_ident) -> Self {
                    let (_name, _data) = match value { #( #m_tys )* };
                    AbiVariant { index: _name, data: _data }
                }
            }

            impl From<#ty_name_ident> for idl_types::#ty_name_ident {
                #[allow(unused_braces)]
                fn from(value: #ty_name_ident) -> Self {
                    match value.index {
                        #( #m_tys_value )*
                        _ => panic!()
                    }
                }
            }
        });

        Ok(())
    }

    fn add_enum(&mut self, ty_enum: &TypeEnum, _analyzer: &Analyzer) -> Result<(), FFIServerError> {
        let ident = &ty_enum.ident;

        let enum_name_ident = format_ident!("{}", &ident);
        let enum_ty_name = quote! { idl_types::#enum_name_ident };

        let mut m_tys = vec![];
        let mut m_tys_value = vec![];
        let mut index = 0;

        for field_node in &ty_enum.fields {
            match field_node {
                EnumNode::EnumField(field) => {
                    let f_ident = format_ident!("{}", &field.ident);
                    let f_name = Literal::i64_unsuffixed(index as i64);

                    m_tys.push(quote! {
                        #enum_ty_name::#f_ident => { #f_name }
                    });

                    m_tys_value.push(quote! {
                        #f_name => { #enum_ty_name::#f_ident }
                    });

                    index += 1;
                }
                EnumNode::Comment(_) => {}
            }
        }

        self.module.push(quote! {
            pub struct #enum_name_ident(i64);

            impl From<#enum_name_ident> for i64 {
                fn from(value: #enum_name_ident) -> Self {
                    value.0
                }
            }

            impl From<i64> for #enum_name_ident {
                fn from(value: i64) -> Self {
                    Self(value)
                }
            }

            impl From<idl_types::#enum_name_ident> for #enum_name_ident {
                fn from(value: idl_types::#enum_name_ident) -> Self {
                    match value { #( #m_tys )* }.into()
                }
            }

            impl From<#enum_name_ident> for idl_types::#enum_name_ident {
                fn from(value: #enum_name_ident) -> Self {
                    match value.0 {
                        #( #m_tys_value )*
                        _ => panic!()
                    }
                }
            }
        });

        Ok(())
    }

    fn new() -> Self {
        Self { module: vec![] }
    }
}

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

        write!(f, "{}", result_code.as_str().rust_fmt())
    }
}

impl FFIServerImpl {
    pub fn generate(analyzer: &Analyzer) -> Result<Self, FFIServerError> {
        let mut context = FFIServerImpl::new();

        let lib_ident = format_ident!("{}", analyzer.get_library_name());

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
                    pub(super) handle: i64,
                }

                unsafe impl Send for #stream_name_ident {} // TODO

                impl StreamInstance for #stream_name_ident {
                    fn wake_client(&self) {
                        let run = self.callback;
                        run(self.handle, self.object);
                    }

                    fn get_handle(&self) -> i64 {
                        self.handle
                    }
                }
            });
        }

        self.module.push(quote! { #( #stream_im )* });

        let interface_ident = format_ident!("{}", &ident);
            let lib_ident = format_ident!("{}", analyzer.get_library_name());

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
                            pub(crate) fn #f_ident(stream_instance: #a_ty, stream: StreamSender<#s_ty>) -> StreamReceiver {
                                #interface_static_instance_ident::#f_ident::<#lib_ident::#interface_ident>(stream_instance, stream)
                            }
                        });
                        static_instance_fields.push(quote! {
                            fn #f_ident<T: #library_ident_impl::#interface_static_ident>(stream_instance: #a_ty, stream: StreamSender<#s_ty>) -> StreamReceiver {
                                T::#f_ident(stream_instance,stream)
                            }
                        });
                    }
                    if let Some((ret_ty, stream_ty)) = stream_ret {
                        let f_ident = format_ident!("{}_stream", &field.ident);
                        let r_ty = get_rust_ty_ref(ret_ty, true);
                        let s_ty = get_rust_ty_ref(stream_ty, true);
                        static_fields.push(quote! {
                            pub(crate) fn #f_ident(stream_instance: #r_ty, stream: StreamReceiver) -> StreamSender<#s_ty> {
                                #interface_static_instance_ident::#f_ident::<#lib_ident::#interface_ident>(stream_instance, stream)
                            }
                        });

                        static_instance_fields.push(quote! {
                            fn #f_ident<T: #library_ident_impl::#interface_static_ident>(stream_instance: #r_ty, stream: StreamReceiver) -> StreamSender<#s_ty> {
                                T::#f_ident(stream_instance,stream)
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


mod server_cargo {
    use super::FFIServerError;
    use crate::cargo_md::*;
    use core::fmt;
    use idl::ids;
    use std::collections::HashMap;

    pub struct FFIServerCargo {
        cargo_toml: Option<String>,
    }

    impl fmt::Display for FFIServerCargo {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.cargo_toml.as_ref().unwrap())
        }
    }

    impl FFIServerCargo {
        pub fn generate(
            analyzer: &ids::analyzer::Analyzer,
            server_name: &str,
        ) -> Result<Self, FFIServerError> {
            let lib_name = analyzer.library_name().unwrap();
            let target_server = analyzer
                .find_server(server_name)
                .ok_or(FFIServerError::Undefined)?;

            let mut context = FFIServerCargo::new();

            let mut dependencies = HashMap::<String, HashMap<String, String>>::new();

            let path = target_server
                .get_field("path")
                .and_then(|v| v.as_string())
                .or_else(|| target_server.get_field("git").and_then(|v| v.as_string()))
                .ok_or(FFIServerError::Undefined)?;

            let mut git = HashMap::<String, String>::new();
            git.insert(
                "git".to_owned(),
                "https://github.com/adrianos42/native_idl".to_owned(),
            );
            dependencies.insert("idl_internal".to_owned(), git);

            let mut lib_path = HashMap::<String, String>::new();
            lib_path.insert("path".to_owned(), path);
            dependencies.insert(lib_name.to_owned(), lib_path);

            let fields = CargoFields {
                package: CargoPackage {
                    name: "idl_ffi".to_owned(),
                    authors: None,
                    edition: Some("2018".to_owned()),
                    version: "0.1.0".to_owned(),
                },
                lib: Some(CargoLib {
                    crate_type: Some(vec!["staticlib".to_owned(), "cdylib".to_owned()]),
                    name: lib_name.to_owned(),
                }),
                dependencies: Some(dependencies),
            };

            context.cargo_toml = Some(create_cargo(fields));

            Ok(context)
        }

        fn new() -> Self {
            Self { cargo_toml: None }
        }
    }
}