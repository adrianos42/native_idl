use idl::idl_nodes::*;
use proc_macro2::{self, TokenStream};
use quote::format_ident;

pub(crate) fn get_rust_ty_ref(ty: &TypeName, references: bool) -> TokenStream {
    match ty {
        TypeName::Types(types) => match types {
            Types::NatInt => quote! { i64 },
            Types::NatFloat => quote! { f64 },
            Types::NatString => quote! { String },
            Types::NatBytes => quote! { Vec<u8> },
            Types::NatBool => quote! { bool },
            Types::NatUUID => quote! { Uuid },
            Types::NatNone => quote! { () },
        },
        TypeName::TypeFunction(value) => {
            let args = get_rust_ty_ref(&value.args, references);
            let ret = get_rust_ty_ref(&value.return_ty, references);
            quote! { #args -> #ret }
        }
        TypeName::TypeTuple(value) => {
            // Not really a tuple :/
            let mut fields_t = vec![];
            for ty in &value.fields {
                let ident = format_ident!("{}", &ty.ident);
                let ty_ident = get_rust_ty_ref(&ty.ty, references);
                fields_t.push(quote! { #ident: #ty_ident })
            }
            let fields = fields_t.into_iter();
            quote! { ( #( #fields ),* ) }
        }
        TypeName::TypeArray(value) => {
            let ty = get_rust_ty_ref(&value.ty, references);
            quote! { Vec<#ty> }
        }
        TypeName::TypeMap(value) => {
            let ty = get_rust_ty_ref(&value.map_ty, references);
            let index_ty = get_rust_ty_ref(&value.index_ty, references);
            quote! { ::std::collections::HashMap<#index_ty, #ty> }
        }
        TypeName::TypeResult(value) => {
            let ok_ty = get_rust_ty_ref(&value.ok_ty, references);
            let err_ty = get_rust_ty_ref(&value.err_ty, references);
            quote! { Result<#ok_ty, #err_ty> }
        }
        TypeName::TypePair(value) => {
            let first_ty = get_rust_ty_ref(&value.first_ty, references);
            let second_ty = get_rust_ty_ref(&value.second_ty, references);
            quote! { (#first_ty, #second_ty) }
        }
        TypeName::TypeOption(value) => {
            let some_ty = get_rust_ty_ref(&value.some_ty, references);
            quote! { Option<#some_ty> }
        }
        TypeName::ListTypeName(value)
        | TypeName::EnumTypeName(value)
        | TypeName::StructTypeName(value)
        | TypeName::ConstTypeName(value) => {
            let ident = format_ident!("{}", &value);
            if references {
                quote! { idl_types::#ident }
            } else {
                quote! { crate::#ident }
            }
        }
        TypeName::TypeStream(_) => {
            quote! { Box<dyn StreamInstance + Send> }
        }
        TypeName::InterfaceTypeName(value) => {
            let ident = format_ident!("{}Instance", value);
            quote! { Box<dyn super::idl_impl::#ident> }
        }
    }
}

pub(crate) fn get_rust_ty_name(ty: &TypeName) -> String {
    match ty {
        TypeName::Types(types) => match types {
            Types::NatInt => "i64".to_owned(),
            Types::NatFloat => "f64".to_owned(),
            Types::NatString => "String".to_owned(),
            Types::NatBytes => "Vecu8".to_owned(),
            Types::NatBool => "bool".to_owned(),
            Types::NatUUID => "Uuid".to_owned(),
            Types::NatNone => "none".to_owned(),
        },
        TypeName::TypeFunction(value) => {
            let args = get_rust_ty_name(&value.args);
            let ret = get_rust_ty_name(&value.return_ty);
            format!("Func{}Ret{}_", args, ret)
        }
        TypeName::TypeTuple(value) => {
            // Not really a tuple :/
            let mut fields_t = String::new();
            for ty in &value.fields {
                let ty_ident = get_rust_ty_name(&ty.ty);
                fields_t.push_str(&ty_ident);
            }
            format!("Args{}_", fields_t)
        }
        TypeName::TypeArray(value) => {
            let ty = get_rust_ty_name(&value.ty);
            format!("Vec{}_", ty)
        }
        TypeName::TypeMap(value) => {
            let ty = get_rust_ty_name(&value.map_ty);
            let index_ty = get_rust_ty_name(&value.index_ty);
            format!("Map{}{}_", ty, index_ty)
        }
        TypeName::TypeResult(value) => {
            let ok_ty = get_rust_ty_name(&value.ok_ty);
            let err_ty = get_rust_ty_name(&value.err_ty);
            format!("Result{}{}_", ok_ty, err_ty)
        }
        TypeName::TypePair(value) => {
            let first_ty = get_rust_ty_name(&value.first_ty);
            let second_ty = get_rust_ty_name(&value.second_ty);
            format!("Pair{}{}_", first_ty, second_ty)
        }
        TypeName::TypeOption(value) => {
            let some_ty = get_rust_ty_name(&value.some_ty);
            format!("Option{}_", some_ty)
        }
        TypeName::ListTypeName(value)
        | TypeName::EnumTypeName(value)
        | TypeName::StructTypeName(value)
        | TypeName::ConstTypeName(value) => value.to_owned(),
        TypeName::TypeStream(value) => {
            let stream_ty = get_rust_ty_name(&value.s_ty);
            format!("Stream{}_", stream_ty)
        }
        TypeName::InterfaceTypeName(value) => value.to_owned(),
    }
}