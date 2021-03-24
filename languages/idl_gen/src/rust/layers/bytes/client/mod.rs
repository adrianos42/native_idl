use idl::analyzer::Analyzer;
use idl::idl_nodes::*;
use idl::ids;

use crate::rust::con_idl::{get_rust_ty_name, get_rust_ty_ref};

use super::{create_hash_idents, BytesTypeName};
use crate::rust::string_pros::{StringPros, StringRustFmt};
use proc_macro2::{self, Literal, Punct, Spacing, TokenStream};
use quote::format_ident;
use quote::{ToTokens, TokenStreamExt};
use std::fmt;

pub(crate) mod package;

#[derive(Debug)]
pub enum BytesServerError {
    UnexpectedType,
    Undefined,
}

pub struct BytesInterface {
    module: Vec<TokenStream>,
}

impl ToTokens for BytesInterface {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(&self.module);
    }
}

impl fmt::Display for BytesInterface {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result_code = String::new();

        self.module.iter().for_each(|value| {
            result_code += &value.to_string();
        });

        write!(f, "{}", result_code.rust_fmt())
    }
}

impl BytesInterface {
    pub fn generate(
        package: &ids::package::Package,
        analyzer: &Analyzer,
        is_async: bool,
        new_body: TokenStream,
        new_prefix_body: TokenStream,
        write_prefix_type: TokenStream,
        write_prefix_body: TokenStream,
        method_body: TokenStream,
    ) -> Result<Self, BytesServerError> {
        let mut context = Self::new();

        let library_name = analyzer.library_name();
        let library_ident = format_ident!("{}", library_name);
        let package_name = package.name();
        let lib_ident = if package_name == analyzer.library_name() {
            quote! { #library_ident }
        } else {
            let package_ident = format_ident!("{}", package_name);
            quote! { #package_ident::#library_ident }
        };

        let has_field_returns_stream = analyzer.any_interface_field_returns_stream(None);
        let has_interface_field_sends_stream = analyzer.any_interface_field_sends_stream(None);

        let mut ffi_deps = vec![];
        let mut deps = vec![];

        if has_interface_field_sends_stream || has_field_returns_stream {
            deps.push(quote! { StreamSender, StreamReceiver, });
            ffi_deps.push(quote! { AbiStreamReceiverState, AbiStreamSenderState, });
        }

        for node in &analyzer.nodes {
            match node {
                IdlNode::TypeInterface(value) => {
                    context.add_interface(
                        value, 
                        package, 
                        analyzer, 
                        is_async, 
                        &new_body, 
                        &new_prefix_body, 
                        &write_prefix_type, 
                        &write_prefix_body,
                        &method_body
                    )?
                }
                IdlNode::InterfaceComment(_) => {}
                _ => {}
            }
        }

        Ok(context)
    }

    //// New
    ///```
    ///// Prefix begin
    /// ws::WS_INSTANCE.wait_context().await;
    ///
    ///// Prefix end
    /// let (mut input, response_event) = Self::write_prefix().await;
    ///
    /// input
    ///     .write_i64::<BigEndian>(MethodType::CreateInstance as i64)
    ///     .unwrap();
    ///// Body begin
    /// let mut write_ref = ws::WS_INSTANCE.write.write().await; // Request ws stream to be sent
    /// let write = write_ref.as_mut().expect("Invalid locked value");
    /// write.send(Message::binary(input.into_boxed_slice())).await.unwrap();
    ///
    /// let response = response_event.await.unwrap();
    /// let mut _response_data = &response[..];
    ///// Body end
    ///
    /// let _call_id = Uuid::from_u128(_response_data.read_u128::<BigEndian>().unwrap());
    ///
    /// Self { id: _call_id }
    ///
    ///```
    //// Write prefix function
    ///```
    /// let mut input: Vec<u8> = Vec::with_capacity(0x1000);
    ///
    /// const _PACKAGE_HASH: [u8; 0x10] = [
    ///     0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ///     0x00, 0x00,
    /// ];
    ///
    /// input.write_all(&_PACKAGE_HASH[..]).unwrap(); // Package id
    ///
    /// const _LIBRARY_HASH: [u8; 0x10] = [
    ///     0xd1, 0xcd, 0xbf, 0xe7, 0x4d, 0x2a, 0x7a, 0x18, 0x19, 0x97, 0x96, 0xed, 0x53, 0xca,
    ///     0xf3, 0x4a,
    /// ];
    ///
    /// input.write_all(&_LIBRARY_HASH[..]).unwrap(); // Library id
    ///
    /// const _INTERFACE_HASH: [u8; 0x10] = [
    ///     0x1e, 0xa9, 0x54, 0x2d, 0xe3, 0xef, 0x88, 0x0b, 0xe2, 0xaa, 0x87, 0xa4, 0x0c, 0xf3,
    ///     0x56, 0x6e,
    /// ];
    ///
    /// input.write_all(&_INTERFACE_HASH[..]).unwrap(); // Interface id
    ///
    /// let call_id = Uuid::new_v4();
    /// input.write_u128::<BigEndian>(call_id.as_u128()).unwrap(); // Call id
    ///
    /// input
    ///     .write_i64::<BigEndian>(MethodType::MethodCall as i64)
    ///     .unwrap();
    ///
    ///// Write prefix body begin
    /// let (sender, receiver) = oneshot::channel(); // Creates a channel for sender listening
    /// let dispatch = ws::WS_INSTANCE.dispatch.clone();
    /// dispatch.write().await.insert(call_id, sender);
    ///// Write prefix body end
    ///
    /// (input, response_event)
    ///
    ///// Function body
    /// let (mut _input, _response_event) = Self::_write_prefix().await;
    ///
    /// _input.write_u128::<BigEndian>(self.instance_id.as_u128()).unwrap();
    ///
    /// const _METHOD_HASH: [u8; 0x10] = [
    ///    0x59, 0x70, 0xaa, 0x1a, 0x89, 0xd1, 0x70, 0x8d, 0x98, 0xca, 0x6a, 0xef, 0x85, 0x5a, 
    ///    0xc2, 0x9c,
    /// ];
    /// _input.write_all(&_METHOD_HASH[..]).unwrap();
    ///
    ///// Value conversion begin
    /// _input.write_i64::<BigEndian>(value).unwrap();
    ///
    ///// Value conversion end
    ///// Function body begin
    /// let mut _write_ref = ws::WS_INSTANCE.write.write().await;
    /// let _write = write_ref.as_mut().expect("Invalid locked value");
    /// _write.send(Message::binary(_input.into_boxed_slice())).await.unwrap();
    ///
    /// let _response = _response_event.await.unwrap();
    /// let mut _response_data = &response[..];
    ///// Function body end
    ///// Return conversion begin
    /// _response_data.read_i64::<BigEndian>().unwrap()
    ///// Return conversion end```
    fn add_interface(
        &mut self,
        ty_interface: &TypeInterface,
        package: &ids::package::Package,
        analyzer: &Analyzer,
        is_async: bool,
        new_body: &TokenStream,
        new_prefix_body: &TokenStream,
        write_prefix_type: &TokenStream,
        write_prefix_body: &TokenStream,
        method_body: &TokenStream,
    ) -> Result<(), BytesServerError> {
        let ident = &ty_interface.ident;

        let interface_ident = format_ident!("{}", &ident);
        let (async_ident, await_ident) = if is_async {
            (quote! { async }, quote! { .await })
        } else {
            (quote! {}, quote! {})
        };
        let instance_id = quote! { instance_id };

        let mut fields = vec![];
        let input_ident = quote! { _input };
        let input_ident_mut = quote! { #input_ident };
        let response_event_ident = quote! { _response_event };

        let (interface_self_arg, interface_self_call) =
            if Analyzer::interface_has_non_static_field(ty_interface) {
                (quote! { &mut self, }, quote! { self. })
            } else {
                (quote! {}, quote! { Self:: })
            };

        for field_node in &ty_interface.fields {
            if let InterfaceNode::InterfaceField(field) = field_node {
                let func_ident = format_ident!("{}", field.ident);
                let mut args_ty_names = vec![];
                let mut args_names = vec![];

                let (args, ret_ty) = match &field.ty {
                    TypeName::TypeFunction(value) => (
                        Some(match &value.args {
                            TypeName::TypeTuple(tuple) => &tuple.fields,
                            _ => return Err(BytesServerError::UnexpectedType),
                        }),
                        &value.return_ty,
                    ),
                    TypeName::TypeTuple(tuple) => {
                        (Some(&tuple.fields), &TypeName::Types(Types::NatNone))
                    }
                    any => (None, any),
                };

                let mut stream_arg: Option<(&TypeName, &TypeName)> = None;
                let mut stream_ret: Option<(&TypeName, &TypeName)> = None;
                
                let mut args_conv = vec![];

                if let Some(args) = args {
                    for arg in args {
                        let arg_ident = format_ident!("{}", &arg.ident);
                        let arg_ty_ident = get_rust_ty_ref(&arg.ty, false);
                        args_ty_names.push(quote! { #arg_ident: #arg_ty_ident });
                        args_names.push(quote! { #arg_ident });

                        match &arg.ty {
                            TypeName::TypeStream(value) => stream_arg = Some((&arg.ty, &value.s_ty)),
                            _ => {
                                let arg_conv = arg.ty.conv_value_to_bytes(
                                    &quote! { #arg_ident }, 
                                    &input_ident, 
                                    false,
                                    true,
                                    analyzer,
                                );
                                args_conv.push(arg_conv);
                            }
                        }
                    }
                }

                let (ret_value_ident, ret_conv) = match ret_ty {
                    TypeName::TypeStream(_) => {
                        if let TypeName::TypeStream(value) = ret_ty {
                            let stream_ty_ident = get_rust_ty_ref(&ret_ty, false);
                            args_ty_names.push(quote! { stream_instance: #stream_ty_ident });
                            args_names.push(quote! { stream_instance });
                            stream_ret = Some((ret_ty, &value.s_ty));
                        }
                        (quote! {}, quote! {})
                    }
                    _ => {
                        let result_val_ident = quote! { _response_data };

                        let result_conv = ret_ty.conv_bytes_to_value(
                            &result_val_ident,
                            false,
                            true,
                            analyzer,
                        );

                        let ret_ty_ident = get_rust_ty_ref(&ret_ty, false);
                        (quote! { -> #ret_ty_ident }, quote! { #result_conv })
                    }
                };

                let (self_ident, instance_id_body) = if field.is_static {
                    (quote! {}, quote! {})
                } else {
                    (
                        quote! { &mut self, }, 
                        quote! { 
                            _input.write_u128::<::idl_internal::byteorder::BigEndian>(
                                self.#instance_id.as_u128()
                            )
                            .unwrap(); 
                        },
                    )
                };

                let method_hash_ident: TokenStream = create_hash_idents(&field.hash);

                fields.push(quote! {
                    #[allow(unused_braces)]
                    pub #async_ident fn #func_ident(#self_ident #( #args_ty_names ),* )#ret_value_ident { 
                        let (mut #input_ident, #response_event_ident) = Self::_write_prefix()#await_ident;
                        #input_ident
                            .write_i64::<::idl_internal::byteorder::BigEndian>(
                                ::idl_internal::MethodType::MethodCall as i64
                            )
                            .unwrap();
                        #instance_id_body
                        const _METHOD_HASH: [u8; 0x10] = [#method_hash_ident];
                        #input_ident.write_all(&_METHOD_HASH[..]).unwrap();
                        #( #args_conv )*
                        #method_body
                        #ret_conv
                    }
                });
            }
        }

        let (st_fields, st_method_body) = if Analyzer::interface_has_non_static_field(ty_interface)
        {
            fields.push(quote! {
                pub #async_ident fn new() -> Self {
                    #new_prefix_body
                    let (mut input, response_event) = Self::_write_prefix()#await_ident;
                    input.write_i64::<::idl_internal::byteorder::BigEndian>(
                        ::idl_internal::MethodType::CreateInstance as i64
                    )
                    .unwrap();
                    #new_body
                    let call_id = Uuid::from_u128(
                        response_data.read_u128::<::idl_internal::byteorder::BigEndian>().unwrap()
                    );
                    Self { #instance_id: call_id }
                }
            });
            (quote! { #instance_id: Uuid, }, quote! {})
        } else {
            (quote! {}, quote! {})
        };

        let package_digest_ident: TokenStream = create_hash_idents(&package.hash);
        let library_digest_ident: TokenStream = create_hash_idents(&analyzer.library_hash());
        let interface_digest_ident: TokenStream = create_hash_idents(&ty_interface.hash);

        fields.push(quote! {
            #async_ident fn _write_prefix() -> (Vec<u8>, #write_prefix_type) {
                let mut input: Vec<u8> = Vec::with_capacity(0x1000);

                const PACKAGE_HASH: [u8; 0x10] = [#package_digest_ident];
                input.write_all(&PACKAGE_HASH[..]).unwrap();
                const LIBRARY_HASH: [u8; 0x10] = [#library_digest_ident];
                input.write_all(&LIBRARY_HASH[..]).unwrap();
                const INTERFACE_HASH: [u8; 0x10] = [#interface_digest_ident];
                input.write_all(&INTERFACE_HASH[..]).unwrap();

                let call_id = Uuid::new_v4();
                input.write_u128::<::idl_internal::byteorder::BigEndian>(call_id.as_u128()).unwrap(); // Call id

                #write_prefix_body

                (input, response_event)
            }
        });

        self.module.push(quote! {
            pub struct #interface_ident { #st_fields }

            impl #interface_ident {
                #( #fields )*
            }
        });

        Ok(())
    }

    fn new() -> Self {
        Self { module: vec![] }
    }
}
