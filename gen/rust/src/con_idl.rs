use native_idl::idl_types::*;
use proc_macro2::{self, Ident, Span, TokenStream};

pub(crate) fn get_rust_ty_ref(ty: &TypeName) -> TokenStream {
    match ty {
        TypeName::Types(types) => match types {
            Types::NatInt => quote! { i64 },
            Types::NatFloat => quote! { f64 },
            Types::NatString => quote! { String },
            Types::NatBytes => quote! { Vec<u8> },
            Types::NatBool => quote! { bool },
            Types::NatNone => quote! { () },
        },
        TypeName::TypeFunction(value) => {
            let args = get_rust_ty_ref(&value.args);
            let ret = get_rust_ty_ref(&value.return_ty);
            quote! { #args -> #ret }
        }
        TypeName::TypeTuple(value) => {
            // Not really a tuple :/
            let mut fields_t = vec![];
            for ty in &value.ty_list {
                let ident = Ident::new(&ty.ident, Span::call_site());
                let ty_ident = get_rust_ty_ref(&ty.ty);
                fields_t.push(quote! { #ident: #ty_ident })
            }
            let fields = fields_t.into_iter();
            quote! { ( #( #fields ),* ) }
        }
        TypeName::TypeArray(value) => {
            let ty_ident = get_rust_ty_ref(&value.ty);
            quote! { Vec<#ty_ident> }
        }
        TypeName::TypeMap(value) => {
            let ty_ident = get_rust_ty_ref(&value.map_ty);
            let index_ty_ident = get_rust_ty_ref(&value.index_ty);
            quote! { HashMap<#index_ty_ident, #ty_ident> }
        }
        TypeName::TypeResult(value) => {
            let ok_ty = get_rust_ty_ref(&value.ok_ty);
            let err_ty = get_rust_ty_ref(&value.err_ty);
            quote! { Result<#ok_ty, #err_ty> }
        }
        TypeName::TypeOption(value) => {
            let some_ty = get_rust_ty_ref(&value.some_ty);
            quote! { Option<#some_ty> }
        }
        TypeName::ConstTypeName(value)
        | TypeName::ListTypeName(value)
        | TypeName::EnumTypeName(value)
        | TypeName::StructTypeName(value)
        | TypeName::InterfaceTypeName(value)
        | TypeName::FactoryTypeName(value)
        | TypeName::StreamTypeName(value) => {
            let ident = Ident::new(&value, Span::call_site());
            quote! { #ident }
        }
    }
}
