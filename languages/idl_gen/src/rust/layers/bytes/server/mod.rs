use idl::analyzer::Analyzer;
use idl::idl_nodes::*;

use crate::rust::con_idl::{get_rust_ty_name, get_rust_ty_ref};

use super::{create_hash_idents, BytesTypeName};
use crate::rust::string_pros::{StringPros, StringRustFmt};
use proc_macro2::{self, Literal, Punct, Spacing, TokenStream};
use quote::format_ident;
use quote::{ToTokens, TokenStreamExt};
use std::{collections::HashMap, fmt};

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
        package_name: &str,
        impl_ident: TokenStream,
        analyzer: &Analyzer,
    ) -> Result<Self, BytesServerError> {
        let mut context = Self::new();

        let library_name = analyzer.library_name();
        let library_ident = format_ident!("{}", library_name);

        let has_field_returns_stream = analyzer.any_interface_field_returns_stream(None);
        let has_interface_field_sends_stream = analyzer.any_interface_field_sends_stream(None);

        let mut deps = vec![];

        if has_interface_field_sends_stream || has_field_returns_stream {
            deps.push(quote! { StreamSender, StreamReceiver, });
        }

        context.module.push(quote! {
            use ::std::collections::HashMap;
            use idl_internal::{
                MethodType, Uuid,
                byteorder::{BigEndian, ReadBytesExt, WriteBytesExt}, #( #deps )*
            };
        });

        let mut hash_interface_match = vec![];
        let input_ident = quote! { _input };
        let output_ident = quote! { _output };

        let mut interface_instances = vec![];
        let mut interface_instances_create = vec![];

        for node in &analyzer.nodes {
            if let IdlNode::TypeInterface(value) = node {
                let interface_digest_ident: TokenStream = create_hash_idents(&value.hash);
                let interface_ident = format_ident!("{}", &value.ident);
                let interface_sn_ident = format_ident!("instance_{}", value.ident.to_snake_case());
                let hash_body = if Analyzer::interface_has_non_static_field(value) {
                    interface_instances.push(quote! {
                        #interface_sn_ident: #interface_ident
                    });
                    interface_instances_create.push(quote! {
                        #interface_sn_ident: #interface_ident::new()
                    });

                    quote! { self.#interface_sn_ident.parse_request_bytes(#input_ident, #output_ident) }
                } else {
                    quote! { #interface_ident::parse_request_bytes(#input_ident, #output_ident) }
                };

                hash_interface_match.push(quote! { [#interface_digest_ident] => { #hash_body } });
                context.add_interface(value, &impl_ident, analyzer)?;
            }
        }

        let (instances_self, instances_new) = if analyzer.any_interface_has_non_static_field() {
            (
                quote! { &mut self, },
                quote! {
                    pub(crate) fn new() -> Self {
                        Self { #( #interface_instances_create ),* }
                    }
                },
            )
        } else {
            (quote! {}, quote! {})
        };

        let library_ident = format_ident!("{}Library", library_name.to_pascal_case());
        let ret_type = quote! { Result<(), ::std::io::Error> };

        context.module.push(quote! {
            pub(crate) struct #library_ident {
                #( #interface_instances ),*
            }

            impl #library_ident {
                #instances_new
                pub(crate) fn parse_request_bytes<R: ::std::io::Read, W: ::std::io::Write>(
                    #instances_self
                    #input_ident: &mut R,
                    #output_ident: &mut W,
                ) -> #ret_type {
                    let mut _hash: [u8; 0x10] = [0x0; 0x10];
                    #input_ident.read_exact(&mut _hash[..])?; // Interface hash
                    #output_ident.write_all(&_hash[..])?;
                    match _hash[..] {
                        #( #hash_interface_match ),*
                        _ => panic!("Interface not defined"),
                    }
                }
            }
        });

        Ok(context)
    }

    fn add_interface(
        &mut self,
        ty_interface: &TypeInterface,
        impl_ident: &TokenStream,
        analyzer: &Analyzer,
    ) -> Result<(), BytesServerError> {
        let ident = &ty_interface.ident;

        let mut fields = vec![];
        let mut hash_fields_match = vec![];
        let interface_instance_ident = format_ident!("{}Instance", &ident);
        let interface_static_ident = format_ident!("{}Static", &ident);
        let interface_ident = format_ident!("{}", &ident);
        let instances_ident = quote! { instances };
        let input_ident = quote! { _input };
        let output_ident = quote! { _output };

        let ret_type = quote! { Result<(), ::std::io::Error> };

        let (interface_self_arg, interface_self_call) =
            if Analyzer::interface_has_non_static_field(ty_interface) {
                (quote! { &mut self, }, quote! { self. })
            } else {
                (quote! {}, quote! { Self:: })
            };

        for field_node in &ty_interface.fields {
            if let InterfaceNode::InterfaceField(field) = field_node {
                let instance_ident = quote! { _instance };
                let func_ident = format_ident!("{}", field.ident);
                let field_digest_ident: TokenStream = create_hash_idents(&field.hash);

                let instance_call = if field.is_static {
                    quote! { super::#impl_ident::#interface_static_ident:: }
                } else {
                    quote! { #instance_ident.instance. }
                };

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

                let mut args_conv = vec![];
                let mut args_value = vec![];
                let mut stream_arg: Option<&TypeName> = None;

                if let Some(args) = args {
                    for arg in args {
                        let arg_value_ident = format_ident!("_{}_arg_val", &arg.ident);
                        args_value.push(quote! { #arg_value_ident });

                        let result_arg = if let TypeName::TypeStream(_) = &arg.ty {
                            stream_arg = Some(&arg.ty);
                            let con = arg.ty.conv_bytes_to_value(&input_ident, true, false,analyzer);
                            //conv_stream_to_val(&con, false, &field) // TODO Stream args
                            quote! {}
                        } else {
                            arg.ty
                                .conv_bytes_to_value(&input_ident, true, false, analyzer)
                        };
                        args_conv.push(quote! { let #arg_value_ident = #result_arg; });
                    }
                }

                let (ret_value_ident, ret_conv) = match ret_ty {
                    TypeName::Types(Types::NatNone) => (quote! {}, quote! {}),
                    TypeName::TypeStream(_) => {
                        let stream_value_ident = quote! { _stream_val };
                        args_value.push(quote! { #stream_value_ident });
                        args_conv.push(quote! {
                            let _stream_state = #input_ident.read_i64::<::idl_internal::byteorder::BigEndian>().unwrap();
                        });

                        (quote! {}, quote! {})
                    }
                    _ => {
                        let result_val_ident = quote! { _result_val };

                        let result_conv = ret_ty.conv_value_to_bytes(
                            &result_val_ident,
                            &output_ident,
                            true,
                            false,
                            analyzer,
                        );

                        let r_ty = get_rust_ty_ref(ret_ty, true);

                        (quote! { let mut #result_val_ident: #r_ty= }, result_conv)
                    }
                };

                let body = quote! {
                    #( #args_conv )*
                    #ret_value_ident #instance_call#func_ident( #( #args_value ),* );
                    #ret_conv
                    Ok(())
                };

                let (field_arg, field_instance) = if field.is_static {
                    (quote! {}, quote! {})
                } else {
                    (
                        quote! { #instance_ident: &mut super::#impl_ident::#interface_instance_ident, },
                        quote! { #instance_ident, },
                    )
                };

                hash_fields_match.push(quote! { [#field_digest_ident] => {
                        Self::#func_ident(#field_instance #input_ident, #output_ident)
                    }
                });

                fields.push(quote! {
                    #[allow(unused_braces)]
                    fn #func_ident<R: ::std::io::Read, W: ::std::io::Write>(
                        #field_arg
                        #input_ident: &mut R,
                        #output_ident: &mut W
                    ) -> #ret_type { #body }
                });
            }
        }

        let method_type_ident = quote! { _method_type };

        let (st_fields, st_method_body) = if Analyzer::interface_has_non_static_field(ty_interface)
        {
            fields.push(quote! {
                fn new() -> Self {
                    Self { #instances_ident: HashMap::new() }
                }
            });
            (
                quote! { #instances_ident: HashMap<Uuid, super::#impl_ident::#interface_instance_ident>, },
                quote! {
                    MethodType::CreateInstance => {
                        let _instance_id = Uuid::new_v4();
                        #output_ident.write_u128::<BigEndian>(_instance_id.as_u128())?;
                        self.#instances_ident.insert(_instance_id, super::#impl_ident::#interface_instance_ident::new());
                        Ok(())
                    },
                    MethodType::DisposeInstance => {
                        let _instance_id = Uuid::from_u128(#input_ident.read_u128::<BigEndian>()?);
                        self.#instances_ident.remove(&_instance_id).unwrap();
                        Ok(())
                    }
                    MethodType::MethodCall => {
                        let _instance_id = Uuid::from_u128(#input_ident.read_u128::<BigEndian>()?);
                        let mut _method_hash: [u8; 0x10] = [0x0; 0x10];
                        #input_ident.read_exact(&mut _method_hash[..])?;
                        match self.#instances_ident.get_mut(&_instance_id) {
                            Some(_instance) => {
                                match _method_hash[..] {
                                    #( #hash_fields_match )*
                                    _ => panic!("Method not defined"),
                                }
                            }
                            None => panic!("Instance not found")
                        }
                    }
                },
            )
        } else {
            (
                quote! {},
                quote! {
                    MethodType::MethodCall => {
                        let mut _method_hash: [u8; 0x10] = [0x0; 0x10];
                        #input_ident.read_exact(&mut _method_hash[..])?;
                        match _method_hash[..] {
                            #( #hash_fields_match )*
                            _ => panic!("Invalid method call"),
                        }
                    }
                },
            )
        };

        fields.push(quote! {
            fn parse_request_bytes<R: ::std::io::Read, W: ::std::io::Write>(
                #interface_self_arg
                #input_ident: &mut R,
                #output_ident: &mut W,
            ) -> #ret_type {
                #output_ident.write_u128::<BigEndian>(#input_ident.read_u128::<BigEndian>()?)?;
                let #method_type_ident = MethodType::from(#input_ident.read_i64::<BigEndian>()?);

                match #method_type_ident {
                    #st_method_body
                    _ => panic!("Invalid method type"),
                }
            }
        });

        self.module.push(quote! {
            struct #interface_ident { #st_fields }

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
