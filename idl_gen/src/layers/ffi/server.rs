use idl::analyzer::Analyzer;
use idl::idl_types::*;

use crate::con_idl::get_rust_ty_ref;

use crate::string_pros::StringPros;
use proc_macro2::{self, Ident, Literal, Span, TokenStream};
use std::fmt;
use std::i64;

use super::*;

#[derive(Debug)]
pub enum FFIServerError {
    UnexpectedType,
    InvalidLiteral,
}

pub struct FFIServer {
    module: Vec<TokenStream>,
}

impl fmt::Display for FFIServer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result_code = String::new();

        self.module.iter().for_each(|value| {
            result_code += &value.to_string();
            result_code += "\n\n";
        });

        write!(f, "{}", result_code.as_str().rust_fmt())
    }
}

impl FFIServer {
    pub fn generate(analyzer: &Analyzer) -> Result<Self, FFIServerError> {
        let mut context = FFIServer::new();

        context.module.push(quote! { use super::ffi_types::*; });

        let nodes: &[TypeNode] = &analyzer.nodes;
        for node in nodes {
            match node {
                TypeNode::TypeInterface(value) => context.add_interface(value, analyzer)?,
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

        let mut fields = vec![];
        let interface_ident = Ident::new(&ident, Span::call_site());
        let interface_static_ident = Ident::new(&(format!("{}Static", &ident)), Span::call_site());
        let library_ident_ffi_impl = quote! { super::idl_ffi_impl };

        let create_fn = |func_ffi_ident, args, body| {
            quote! {
                #[no_mangle]
                #[allow(unused_braces)]
                pub extern "C" fn #func_ffi_ident( #args ) -> i64 {
                    match ::std::panic::catch_unwind(::std::panic::AssertUnwindSafe(move || { #body })) {
                        Ok(_) => AbiInternalError::Ok as i64,
                        Err(_) => AbiInternalError::UndefinedException as i64,
                    }
                }
            }
        };

        for field_node in &ty_interface.fields {
            match field_node {
                InterfaceNode::InterfaceField(field) => {
                    let field_name = format!("{}_{}", ident.to_snake_case(), field.ident);
                    let func_ffi_ident = Ident::new(&field_name, Span::call_site());
                    let func_ident = Ident::new(&field.ident, Span::call_site());

                    let mut args_value = vec![];
                    let mut args_ffi = vec![];

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
                    let mut arg_stream: Option<(&TypeName, &str)> = None;

                    // Create function arg names and conversion between ffi and rust types.
                    if let Some(args) = args {
                        for arg in args {
                            let arg_ident = Ident::new(&arg.ident, Span::call_site());
                            let arg_ty_ident = get_ffi_ty_ref(&arg.ty, true, analyzer);

                            // This is to convert a stream argument to a result value.
                            if let TypeName::TypeStream(_) = &arg.ty {
                                arg_stream = Some((&arg.ty, &arg.ident));
                            } else {
                                let arg_call_name = format!("_{}_arg_val", arg.ident);
                                let arg_value_ident = Ident::new(&arg_call_name, Span::call_site());
                                args_value.push(quote! { #arg_value_ident });

                                args_ffi.push(quote! { #arg_ident: #arg_ty_ident });

                                let arg_ident = quote! { #arg_ident };
                                let result_arg =
                                    conv_ffi_to_value(&arg.ty, &arg_ident, true, analyzer);
                                args_conv_ffi.push(quote! { let #arg_value_ident = #result_arg; });
                            };
                        }
                    }

                    let mut ret_conv_ffi: Vec<TokenStream> = vec![];

                    let ret_value_ident = match ret_ty {
                        TypeName::Types(Types::NatNone) => quote! {},
                        TypeName::TypeStream(_) => {
                            // Add the argument that is converted to a rust value
                            let stream_ident = quote! { _stream };
                            let stream_value_ident = quote! { _stream_val };
                            let arg_ty_ident = get_ffi_ty_ref(&ret_ty, true, analyzer);
                            args_value.push(quote! { #stream_value_ident });
                            args_ffi.push(quote! { #stream_ident: #arg_ty_ident });
                            let result_stream =
                                conv_ffi_to_value(&ret_ty, &stream_ident, true, analyzer);
                            args_conv_ffi
                                .push(quote! { let #stream_value_ident = #result_stream; });

                            // If the function returns a stream, it means the rust function returns
                            // nothing, so it may only return the stream to be set by the client,
                            // passed as an argument, like `(value: stream int) -> stream int`.
                            if let Some((arg_stream_ty, arg_stream_name)) = arg_stream {
                                // Add the argument that is passed to set the stream
                                let result_stream_ident =
                                    Ident::new(arg_stream_name, Span::call_site());

                                let result_val_ident = quote! { _result_stream_val };

                                let result_conv = conv_value_to_ffi(
                                    arg_stream_ty,
                                    &result_val_ident,
                                    true,
                                    analyzer,
                                );
                                ret_conv_ffi.push(quote! {
                                    unsafe { *#result_stream_ident = #result_conv; }
                                });
                                let r_ty = get_rust_ty_ref(arg_stream_ty, true);
                                quote! { let #result_val_ident: #r_ty= }
                            } else {
                                quote! {}
                            }
                        }
                        _ => {
                            let result_ident = quote! { _result };
                            let result_ty_ident = get_ffi_ty_ref(ret_ty, true, analyzer);
                            args_ffi.push(quote! { #result_ident: *mut #result_ty_ident });

                            let result_val_ident = quote! { _result_val };

                            let result_conv =
                                conv_value_to_ffi(&ret_ty, &result_val_ident, true, analyzer);

                            let r_ty = get_rust_ty_ref(ret_ty, true);

                            // When it returns any other type, but needs to also return a stream value,
                            // (value: stream int) -> int
                            // then the rust function returns the tuple (stream, result)
                            if let Some((arg_stream_ty, arg_stream_name)) = arg_stream {
                                let result_stream_ident =
                                    Ident::new(arg_stream_name, Span::call_site());
                                let result_stream_val_ident = quote! { _result_stream_val };
                                let s_ty = get_rust_ty_ref(arg_stream_ty, true);
                                let result_stream_conv = conv_value_to_ffi(
                                    arg_stream_ty,
                                    &result_stream_val_ident,
                                    true,
                                    analyzer,
                                );
                                ret_conv_ffi.push(quote! {
                                    unsafe {
                                        *#result_ident = #result_conv;
                                        *#result_stream_ident = #result_stream_conv;
                                     }
                                });
                                quote! { let (#result_stream_val_ident, #result_val_ident): (#s_ty, #r_ty)= }
                            } else {
                                ret_conv_ffi.push(quote! {
                                    unsafe { *#result_ident = #result_conv; }
                                });
                                quote! { let #result_val_ident: #r_ty= }
                            }
                        }
                    };

                    // The stream arg is always the last argument.
                    if let Some((arg_stream_ty, arg_stream_name)) = arg_stream {
                        let arg_ident = Ident::new(arg_stream_name, Span::call_site());
                        let arg_ty_ident = get_ffi_ty_ref(arg_stream_ty, true, analyzer);
                        args_ffi.push(quote! { #arg_ident: *mut #arg_ty_ident });
                    }

                    let ins_ident = quote! { _ins };
                    let instance_ident = quote! { _instance };

                    let (instance_mut, instance_args, instance_body) = if field.is_static {
                        (
                            quote! {},
                            quote! {},
                            quote! { let #instance_ident =
                            0 as *mut #library_ident_ffi_impl::#interface_static_ident; },
                        )
                    } else {
                        (
                            quote! { mut },
                            quote! { #instance_ident: *mut #library_ident_ffi_impl::#interface_ident, },
                            quote! {},
                        )
                    };

                    let body_ident = quote! {
                        #instance_body
                        let #instance_mut #ins_ident = unsafe { Box::from_raw(#instance_ident) };
                        let #instance_mut #instance_ident = &#instance_mut #ins_ident.instance;
                        #( #args_conv_ffi );*
                        #ret_value_ident #instance_ident.#func_ident( #( #args_value ),* );
                        #( #ret_conv_ffi );*
                        std::mem::forget(#ins_ident);
                    };

                    let args_ident = quote! {
                        #instance_args
                        #( #args_ffi ),*
                    };

                    fields.push(create_fn(func_ffi_ident, args_ident, body_ident));

                    // Add the function to set the stream callback
                    if let TypeName::TypeStream(_) = ret_ty {
                        let func_ffi_ident =
                            Ident::new(&(format!("{}_stream", field_name)), Span::call_site());
                        let instance_body = quote! { let #instance_ident =
                        0 as *mut #library_ident_ffi_impl::#interface_static_ident; };
                        let args_ident = quote! { _stream_function: *const AbiVariant  };
                        let stream_ident =
                            Ident::new(&(format!("{}_stream", &field.ident)), Span::call_site());
                        let body_ident = quote! {
                            #instance_body
                            let mut #ins_ident = unsafe { Box::from_raw(#instance_ident) };
                            let _stream_function_val = unsafe { _stream_function.read() };

                            match _stream_function_val.index {
                                0 => #ins_ident.#stream_ident = None,
                                1 => #ins_ident.#stream_ident = Some(unsafe {
                                    std::mem::transmute(_stream_function_val.data)
                                }),
                                _ => panic!(),
                            }

                            std::mem::forget(#ins_ident);
                        };
                        fields.push(create_fn(func_ffi_ident, args_ident, body_ident));
                    }

                    if !field.is_static && is_boxed_ffi(ret_ty, analyzer) {
                        let func_ffi_ident =
                            Ident::new(&(format!("{}_release", field_name)), Span::call_site());

                        let result_ident = quote! { _result };
                        let result_ty_ident = get_ffi_ty_ref(ret_ty, true, analyzer);
                        let args_ident =
                            quote! { #instance_args #result_ident: *mut #result_ty_ident };
                        let body_ident = quote! {
                            //let _ = unsafe { Box::from_raw() };
                            std::mem::forget(#ins_ident);
                        };

                        //fields.push(create_fn(func_ffi_ident, args_ident, body_ident));
                    }
                }
                InterfaceNode::Comment(_) => {}
            }
        }

        if !Analyzer::interface_has_constructor_field(&ty_interface) {
            let (ret_value_ident, ret_conv_ffi, arg_ident) = {
                let interface_ty_ident = TypeName::InterfaceTypeName(ident.to_owned());
                let result_ident = quote! { _result };
                let result_val_ident = quote! { _result_val };

                let result_ty_ident = get_ffi_ty_ref(&interface_ty_ident, true, analyzer);

                let result_conv =
                    conv_value_to_ffi(&interface_ty_ident, &result_val_ident, true, analyzer);
                let r_ty = get_rust_ty_ref(&interface_ty_ident, true);
                (
                    quote! { let #result_val_ident: #r_ty= },
                    quote! { unsafe { *#result_ident = #result_conv; } },
                    quote! { #result_ident: *mut #result_ty_ident },
                )
            };

            let field_name = format!("{}_{}", ident.to_snake_case(), "create");
            let func_ffi_ident = Ident::new(&field_name, Span::call_site());

            let ins_ident = quote! { _ins };
            let instance_ident = quote! { _instance };

            let instance_body = quote! { let #instance_ident =
            0 as *mut #library_ident_ffi_impl::#interface_static_ident; };

            let body_ident = quote! {
                #instance_body
                let #ins_ident = unsafe { Box::from_raw(#instance_ident) };
                let #instance_ident = &#ins_ident.instance;
                #ret_value_ident #instance_ident.create();
                #ret_conv_ffi
                std::mem::forget(#ins_ident);
            };

            fields.push(create_fn(func_ffi_ident, arg_ident, body_ident));
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

impl fmt::Display for FFIServerTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result_code = String::new();

        self.module.iter().for_each(|value| {
            result_code += &value.to_string();
            result_code += "\n\n";
        });

        write!(f, "{}", result_code.as_str().rust_fmt())
    }
}

impl FFIServerTypes {
    pub fn generate(analyzer: &Analyzer) -> Result<Self, FFIServerError> {
        let mut context = FFIServerTypes::new();

        context.module.push(quote! {
            use super::ffi_types::*;
        });

        let nodes: &[TypeNode] = &analyzer.nodes;
        for node in nodes {
            match node {
                TypeNode::TypeStruct(value) => context.add_struct(value, analyzer)?,
                TypeNode::TypeList(value) => context.add_type_list(value, analyzer)?,
                TypeNode::TypeEnum(value) => context.add_enum(value, analyzer)?,
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

        let struct_ident = Ident::new(&ident, Span::call_site());

        for field_node in &ty_struct.fields {
            match field_node {
                StructNode::StructField(field) => {
                    let field_name = Ident::new(&field.ident, Span::call_site());
                    let field_ty_name = get_ffi_ty_ref(&field.ty, false, analyzer);
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

            impl From<super::idl_types::#struct_ident> for #struct_ident {
                #[allow(unused_braces)]
                fn from(value: super::idl_types::#struct_ident) -> Self {
                    Self { #( #fields_conv ),* }
                }
            }

            impl From<#struct_ident> for super::idl_types::#struct_ident {
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

        let ty_name_ident = Ident::new(&ident, Span::call_site());
        let list_ty_name = quote! { super::idl_types::#ty_name_ident };

        let mut m_tys = vec![];
        let mut m_tys_value = vec![];
        let mut index = 0;

        for field_node in &type_list.fields {
            match field_node {
                TypeListNode::TypeListField(field) => {
                    let f_ident = Ident::new(&field.ident, Span::call_site());
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

            impl From<super::idl_types::#ty_name_ident> for #ty_name_ident {
                #[allow(unused_braces)]
                fn from(value: super::idl_types::#ty_name_ident) -> Self {
                    let (_name, _data) = match value { #( #m_tys )* };
                    AbiVariant { index: _name, data: _data }
                }
            }

            impl From<#ty_name_ident> for super::idl_types::#ty_name_ident {
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

        let enum_name_ident = Ident::new(&ident, Span::call_site());
        let enum_ty_name = quote! { super::idl_types::#enum_name_ident };

        let mut m_tys = vec![];
        let mut m_tys_value = vec![];
        let mut index = 0;

        for field_node in &ty_enum.fields {
            match field_node {
                EnumNode::EnumField(field) => {
                    let f_ident = Ident::new(&field.ident, Span::call_site());
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
            pub type #enum_name_ident = i64;

            impl From<super::idl_types::#enum_name_ident> for #enum_name_ident {
                fn from(value: super::idl_types::#enum_name_ident) -> Self {
                    match value { #( #m_tys )* }
                }
            }

            impl From<#enum_name_ident> for super::idl_types::#enum_name_ident {
                fn from(value: #enum_name_ident) -> Self {
                    match value {
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

impl fmt::Display for FFIServerImpl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result_code = String::new();

        self.module.iter().for_each(|value| {
            result_code += &value.to_string();
            result_code += "\n\n";
        });

        write!(f, "{}", result_code.as_str().rust_fmt())
    }
}

impl FFIServerImpl {
    pub fn generate(analyzer: &Analyzer) -> Result<Self, FFIServerError> {
        let mut context = FFIServerImpl::new();

        context.module.push(quote! { use super::ffi_types::*; });

        let nodes: &[TypeNode] = &analyzer.nodes;
        for node in nodes {
            match node {
                TypeNode::TypeInterface(value) => context.add_interface(value, analyzer)?,
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

        let interface_ident = Ident::new(&ident, Span::call_site());
        let interface_static_ident = Ident::new(&(format!("{}Static", &ident)), Span::call_site());
        let library_ident_impl = quote! { super::idl_impl };

        let stream_fns: Vec<(Ident, &TypeName)> = ty_interface
            .fields
            .iter()
            .filter_map(|node| match node {
                InterfaceNode::InterfaceField(field) => {
                    let s_ty = match &field.ty {
                        TypeName::TypeFunction(value) => match &value.return_ty {
                            TypeName::TypeStream(value) => Some(&value.s_ty),
                            _ => None,
                        },
                        TypeName::TypeStream(value) => Some(&value.s_ty),
                        _ => None,
                    };

                    if let Some(s_ty) = s_ty {
                        Some((
                            Ident::new(&(format!("{}_stream", &field.ident)), Span::call_site()),
                            s_ty,
                        ))
                    } else {
                        None
                    }
                }
                _ => None,
            })
            .collect();

        let streams_fns_idents: Vec<&Ident> = stream_fns.iter().map(|idn| &idn.0).collect();
        let streams_fns_tys: Vec<TokenStream> = stream_fns
            .iter()
            .map(|idn| {
                let s_ty_ident = get_ffi_ty_ref(idn.1, true, analyzer);
                quote! { extern "C" fn(i64, #s_ty_ident) }
            })
            .collect();

        let streams: TokenStream = if !stream_fns.is_empty() {
            let f_names = stream_fns
                .iter()
                .map(|idn| Ident::new(&(format!("set_{}", idn.0.to_string())), Span::call_site()));
            let r_stream_ty = stream_fns.iter().map(|idn| get_rust_ty_ref(idn.1, true));
            let r_stream_con = stream_fns.iter().map(|idn| {
                let val_ident = quote! { value };
                conv_value_to_ffi(&idn.1, &val_ident, true, analyzer)
            });

            quote! { impl #interface_static_ident {
                #(
                    #[allow(unused_braces)]
                    pub fn #f_names(&self, handle: i64, value: #r_stream_ty) {
                        let _result = #r_stream_con;
                        self.#streams_fns_idents.unwrap()(handle, _result);
                    }
                )*
                }
            }
        } else {
            quote! {}
        };

        self.module.push(quote! {
           pub struct #interface_static_ident {
               pub(super) instance: Box<dyn #library_ident_impl::#interface_static_ident>,
               #( pub(super) #streams_fns_idents: Option<#streams_fns_tys>, )*
           }
        });

        self.module.push(quote! {
           pub struct #interface_ident {
               pub(super) instance: Box<dyn #library_ident_impl::#interface_ident>,
           }

           impl From<Box<dyn #library_ident_impl::#interface_ident>> for #interface_ident {
               fn from(value: Box<dyn #library_ident_impl::#interface_ident>) -> Self {
                   Self {
                       instance: value,
                       // #( #streams_fns_idents: None, )*
                   }
               }
           }

           #streams
        });

        Ok(())
    }

    fn new() -> Self {
        Self { module: vec![] }
    }
}

pub struct FFIServerCargo {
    cargo_toml: String,
}

impl fmt::Display for FFIServerCargo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.cargo_toml)
    }
}

impl FFIServerCargo {
    pub fn generate(analyzer: &Analyzer) -> Result<Self, FFIServerError> {
        let mut context = FFIServerCargo::new();

        let pkg_name = analyzer.get_library_name();
        let version = "0.1.0";
        let author = "Adriano Souza";
        let edition = "2018";
        let lib_name = analyzer.get_library_name();

        context.cargo_toml = format!(
r#"[package]
name = "{}"
version = "{}"
authors = ["{}"]
edition = "{}"
    
[lib]
name = "{}"
crate-type = ["staticlib", "cdylib"]
"#,
            pkg_name, version, author, edition, lib_name,
        );

        Ok(context)
    }

    fn new() -> Self {
        Self {
            cargo_toml: String::new(),
        }
    }
}
