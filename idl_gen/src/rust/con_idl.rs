use idl::idl::idl_nodes::*;
use proc_macro2::{self, Ident, Span, TokenStream};

pub(crate) fn get_rust_ty_ref(ty: &TypeName, references: bool) -> TokenStream {
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
            let args = get_rust_ty_ref(&value.args, references);
            let ret = get_rust_ty_ref(&value.return_ty, references);
            quote! { #args -> #ret }
        }
        TypeName::TypeTuple(value) => {
            // Not really a tuple :/
            let mut fields_t = vec![];
            for ty in &value.fields {
                let ident = Ident::new(&ty.ident, Span::call_site());
                let ty_ident = get_rust_ty_ref(&ty.ty, references);
                fields_t.push(quote! { #ident: #ty_ident })
            }
            let fields = fields_t.into_iter();
            quote! { ( #( #fields ),* ) }
        }
        TypeName::TypeArray(value) => {
            let ty_ident = get_rust_ty_ref(&value.ty, references);
            quote! { Vec<#ty_ident> }
        }
        TypeName::TypeMap(value) => {
            let ty_ident = get_rust_ty_ref(&value.map_ty, references);
            let index_ty_ident = get_rust_ty_ref(&value.index_ty, references);
            quote! { ::std::collections::HashMap<#index_ty_ident, #ty_ident> }
        }
        TypeName::TypeResult(value) => {
            let ok_ty = get_rust_ty_ref(&value.ok_ty, references);
            let err_ty = get_rust_ty_ref(&value.err_ty, references);
            quote! { Result<#ok_ty, #err_ty> }
        }
        TypeName::TypeOption(value) => {
            let some_ty = get_rust_ty_ref(&value.some_ty, references);
            quote! { Option<#some_ty> }
        }
        TypeName::ListTypeName(value)
        | TypeName::EnumTypeName(value)
        | TypeName::StructTypeName(value)
        | TypeName::ConstTypeName(value) => {
            let ident = Ident::new(&value, Span::call_site());
            if references {
                quote! { super::idl_types::#ident }
            } else {
                quote! { #ident }
            }
        }
        TypeName::TypeStream(value) => {
            let stream_ty = get_rust_ty_ref(&value.s_ty, references);
            quote! { Box<dyn StreamInstance<#stream_ty> + Send> }
        } 
        TypeName::InterfaceTypeName(value) => {
            let ident = Ident::new(&format!("{}Instance", value), Span::call_site());
            quote! { Box<dyn super::idl_impl::#ident> }
        }
    }
}
