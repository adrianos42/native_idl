use super::FFIClientError;
use crate::rust::con_idl::{get_rust_ty_name, get_rust_ty_ref};
use crate::rust::layers::ffi::*;
use crate::rust::string_pros::{StringPros, StringRustFmt};
use idl::analyzer::Analyzer;
use idl::idl_nodes::*;
use proc_macro2::{self, Literal, TokenStream};
use quote::format_ident;
use quote::{ToTokens, TokenStreamExt};
use std::i64;
use std::{collections::HashMap, fmt};
use thiserror::Error;
pub struct FFIClientTypes {
    module: Vec<TokenStream>,
}

impl ToTokens for FFIClientTypes {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(&self.module);
    }
}

impl fmt::Display for FFIClientTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result_code = String::new();

        self.module.iter().for_each(|value| {
            result_code += &value.to_string();
        });

        write!(f, "{}", result_code.rust_fmt())
    }
}

impl FFIClientTypes {
    pub fn generate(package_name: &str, analyzer: &Analyzer) -> Result<Self, FFIClientError> {
        let mut context = FFIClientTypes::new();

        let library_name = analyzer.library_name();
        let library_ident = format_ident!("{}", library_name);
        let lib_ident = if package_name == analyzer.library_name() {
            quote! { #library_ident }
        } else {
            let package_ident = format_ident!("{}", package_name);
            quote! { #package_ident::#library_ident }
        };

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
    ) -> Result<(), FFIClientError> {
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

                    let con_ty = field.ty.conv_value_to_ffi(&field_value, false, analyzer);
                    let con_value_ty = field.ty.conv_ffi_to_value(&field_value, false, analyzer);
                    fields_conv.push(quote! { #field_name: #con_ty });
                    fields_value_conv.push(quote! { #field_name: #con_value_ty });
                }
                StructNode::Comment(_) => {}
            }
        }

        self.module.push(quote! {
            #[repr(C)]
            pub struct #struct_ident { #( #fields )* }

            impl #struct_ident {
                pub fn dispose() {

                }
            }

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
    ) -> Result<(), FFIClientError> {
        let ident = &type_list.ident;

        let ty_name_ident = format_ident!("{}", &ident);
        let list_ty_name = quote! { idl_types::#ty_name_ident };

        let mut conv_value_to_ffi = vec![];
        let mut conv_ffi_to_value = vec![];
        let mut dispose_ffi = vec![];
        let mut index = 0;

        for field_node in &type_list.fields {
            match field_node {
                TypeListNode::TypeListField(field) => {
                    let f_ident = format_ident!("{}", &field.ident);
                    let f_name = Literal::i64_unsuffixed(index as i64);

                    let mut field_name = quote! { _value_field };

                    let uses_box = match &field.ty {
                        TypeName::StructTypeName(_) | TypeName::ListTypeName(_) => true,
                        _ => false,
                    };

                    if uses_box {
                        field_name = quote! { *#field_name }
                    }

                    let ty_ident = field
                        .ty
                        .conv_value_to_ffi_boxed(&field_name, false, analyzer);

                    conv_value_to_ffi.push(quote! {
                        #list_ty_name::#f_ident(_value_field) => {
                            (#f_name as i64, #ty_ident as *const ::core::ffi::c_void)
                        }
                    });

                    let mut ty_ident =
                        field
                            .ty
                            .conv_ffi_ptr_to_value(&(quote! { value.data }), false, analyzer);

                    if uses_box {
                        ty_ident = quote! { Box::new(#ty_ident) }
                    }

                    conv_ffi_to_value.push(quote! {
                        #f_name => { #list_ty_name::#f_ident(#ty_ident) }
                    });

                    let value_ty = field.ty.get_ffi_ty_ref_mut(false, analyzer);
                    let value_name = quote! { value.data as #value_ty };
                    let ty_ident = field.ty.dispose_ffi_boxed(&value_name, false, analyzer);
                    dispose_ffi.push(quote! { #f_name => { #ty_ident } });

                    index += 1;
                }
                TypeListNode::Comment(_) => {}
            }
        }

        self.module.push(quote! {
            pub struct #ty_name_ident(AbiVariant);

            impl #ty_name_ident {
                pub fn dispose(value: AbiVariant) {
                    match value.index {
                        #( #dispose_ffi )*
                        _ => panic!("Invalid variant value for type list")
                    }
                }
            }

            impl From<#ty_name_ident> for AbiVariant {
                fn from(value: #ty_name_ident) -> Self {
                    value.0
                }
            }

            impl From<AbiVariant> for #ty_name_ident {
                fn from(value: AbiVariant) -> Self {
                    Self(value)
                }
            }

            impl From<idl_types::#ty_name_ident> for #ty_name_ident {
                #[allow(unused_braces)]
                fn from(value: idl_types::#ty_name_ident) -> Self {
                    let (_name, _data) = match value { #( #conv_value_to_ffi )* };
                    AbiVariant { index: _name, data: _data }.into()
                }
            }

            impl From<#ty_name_ident> for idl_types::#ty_name_ident {
                #[allow(unused_braces)]
                fn from(value: #ty_name_ident) -> Self {
                    let value: AbiVariant = value.into();
                    match value.index {
                        #( #conv_ffi_to_value )*
                        _ => panic!("Invalid variant value for type list")
                    }
                }
            }
        });

        Ok(())
    }

    fn add_enum(&mut self, ty_enum: &TypeEnum, _analyzer: &Analyzer) -> Result<(), FFIClientError> {
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
                #[allow(unused_braces)]
                fn from(value: idl_types::#enum_name_ident) -> Self {
                    match value { #( #m_tys )* }.into()
                }
            }

            impl From<#enum_name_ident> for idl_types::#enum_name_ident {
                #[allow(unused_braces)]
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
