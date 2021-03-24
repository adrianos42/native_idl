use super::BytesTypeName;

use idl::analyzer::Analyzer;
use idl::idl_nodes::*;

use crate::rust::string_pros::StringRustFmt;
use proc_macro2::{self, Literal, TokenStream};
use quote::format_ident;
use quote::{ToTokens, TokenStreamExt};
use std::fmt;
use std::i64;

use super::BytesError;

use crate::rust::layers::bytes::*;

pub struct BytesTypes {
    module: Vec<TokenStream>,
}

impl ToTokens for BytesTypes {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(&self.module);
    }
}

impl fmt::Display for BytesTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result_code = String::new();

        self.module.iter().for_each(|value| {
            result_code += &value.to_string();
        });

        write!(f, "{}", result_code.rust_fmt())
    }
}

impl BytesTypes {
    pub fn generate(
        references: bool,
        analyzer: &Analyzer,
    ) -> Result<Self, BytesError> {
        let mut context = Self::new();

        let library_name = analyzer.library_name();

        let nodes: &[IdlNode] = &analyzer.nodes;
        for node in nodes {
            match node {
                IdlNode::TypeStruct(value) => context.add_struct(value, references, analyzer)?,
                IdlNode::TypeList(value) => context.add_type_list(value, references, analyzer)?,
                IdlNode::TypeEnum(value) => context.add_enum(value, references, analyzer)?,
                _ => {}
            }
        }

        Ok(context)
    }

    fn add_struct(
        &mut self,
        ty_struct: &TypeStruct,
        references: bool,
        analyzer: &Analyzer,
    ) -> Result<(), BytesError> {
        let ident = &ty_struct.ident;

        let mut fields_conv = vec![];
        let mut fields_value_conv = vec![];

        let struct_ident = format_ident!("{}", &ident);
        let struct_ident_ex = format_ident!("{}Ex", &ident);
        let input_ident = quote! { _input };
        let output_ident = quote! { _output };

        let lib_ref = if references {
            quote! { super:: }
        } else {
            quote! {}
        };

        for field_node in &ty_struct.fields {
            if let StructNode::StructField(field) = field_node {
                let field_name = format_ident!("{}", &field.ident);
                let field_value = quote! { self.#field_name };

                let con_ty = field.ty.conv_value_to_bytes(
                    &field_value,
                    &output_ident,
                    references,
                    false,
                    analyzer,
                );
                fields_conv.push(quote! { #con_ty });

                let con_value_ty =
                    field
                        .ty
                        .conv_bytes_to_value(&input_ident, references, false, analyzer);
                fields_value_conv.push(quote! { #field_name: #con_value_ty });
            }
        }

        self.module.push(quote! {
            trait #struct_ident_ex {
                fn from_bytes<R: ::std::io::Read>(#input_ident: &mut R) -> #lib_ref#struct_ident;
                fn into_bytes<W: ::std::io::Write>(&self, #output_ident: &mut W);
            }

            impl #struct_ident_ex for #lib_ref#struct_ident {
                #[allow(unused_braces)]
                fn from_bytes<R: ::std::io::Read>(#input_ident: &mut R) -> #lib_ref#struct_ident {
                    #lib_ref#struct_ident { #( #fields_value_conv ),* }
                }

                #[allow(unused_braces)]
                fn into_bytes<W: ::std::io::Write>(&self, #output_ident: &mut W) {
                    #( #fields_conv )*
                }
            }
        });

        Ok(())
    }

    fn add_type_list(
        &mut self,
        type_list: &TypeList,
        references: bool,
        analyzer: &Analyzer,
    ) -> Result<(), BytesError> {
        let ident = &type_list.ident;

        let ty_name_ident = format_ident!("{}", &ident);
        let ty_name_ident_ex = format_ident!("{}Ex", &ident);
        let input_ident = quote! { _input };
        let output_ident = quote! { _output };

        let lib_ref = if references {
            quote! { super:: }
        } else {
            quote! {}
        };

        let mut conv_value_to_bytes = vec![];
        let mut conv_bytes_to_value = vec![];
        let mut index = 0;

        for field_node in &type_list.fields {
            if let TypeListNode::TypeListField(field) = field_node {
                let f_ident = format_ident!("{}", &field.ident);
                let f_name = Literal::i64_unsuffixed(index as i64);

                let mut field_name = quote! { _value_field };

                let uses_box = match &field.ty {
                    TypeName::StructTypeName(_) | TypeName::ListTypeName(_) => true,
                    _ => false,
                };

                if true {
                    field_name = quote! { (*#field_name) }
                }

                let ty_ident = field.ty.conv_value_to_bytes(
                    &field_name,
                    &output_ident,
                    references,
                    false,
                    analyzer,
                );

                conv_value_to_bytes.push(quote! {
                        #lib_ref#ty_name_ident::#f_ident(_value_field) => {
                            #output_ident.write_i64::<::idl_internal::byteorder::BigEndian>(#f_name).unwrap();
                            #ty_ident
                        }
                    });

                let mut ty_ident =
                    field
                        .ty
                        .conv_bytes_to_value(&input_ident, references, false, analyzer);

                if uses_box {
                    ty_ident = quote! { Box::new(#ty_ident) }
                }

                conv_bytes_to_value.push(quote! {
                    #f_name => { #lib_ref#ty_name_ident::#f_ident(#ty_ident) }
                });

                index += 1;
            }
        }

        self.module.push(quote! {
            trait #ty_name_ident_ex {
                fn from_bytes<R: ::std::io::Read>(#input_ident: &mut R) -> #lib_ref#ty_name_ident;
                fn into_bytes<W: ::std::io::Write>(&self, #output_ident: &mut W);
            }

            impl #ty_name_ident_ex for #lib_ref#ty_name_ident {
                #[allow(unused_braces)]
                #[allow(unused_parens)]
                fn into_bytes<W: ::std::io::Write>(&self, #output_ident: &mut W) {
                    match self { #( #conv_value_to_bytes )* }
                }

                #[allow(unused_braces)]
                fn from_bytes<R: ::std::io::Read>(#input_ident: &mut R) -> #lib_ref#ty_name_ident {
                    let variant = #input_ident.read_i64::<::idl_internal::byteorder::BigEndian>().unwrap();
                    match variant {
                        #( #conv_bytes_to_value )*
                        _ => panic!("Invalid variant value for type list")
                    }
                }
            }
        });

        Ok(())
    }

    fn add_enum(
        &mut self,
        ty_enum: &TypeEnum,
        references: bool,
        _analyzer: &Analyzer,
    ) -> Result<(), BytesError> {
        let ident = &ty_enum.ident;

        let enum_name_ident = format_ident!("{}", &ident);
        let enum_name_ident_ex = format_ident!("{}Ex", &ident);
        let input_ident = quote! { _input };
        let output_ident = quote! { _output };

        let lib_ref = if references {
            quote! { super:: }
        } else {
            quote! {}
        };

        let mut m_tys = vec![];
        let mut m_tys_value = vec![];
        let mut index = 0;

        for field_node in &ty_enum.fields {
            if let EnumNode::EnumField(field) = field_node {
                let f_ident = format_ident!("{}", &field.ident);
                let f_name = Literal::i64_unsuffixed(index as i64);

                m_tys.push(quote! {
                    #lib_ref#enum_name_ident::#f_ident => {
                        #f_name
                    }
                });

                m_tys_value.push(quote! {
                    #f_name => { #lib_ref#enum_name_ident::#f_ident }
                });

                index += 1;
            }
        }

        self.module.push(quote! {
            trait #enum_name_ident_ex {
                fn from_bytes<R: ::std::io::Read>(#input_ident: &mut R) -> #lib_ref#enum_name_ident;
                fn into_bytes<W: ::std::io::Write>(&self, #output_ident: &mut W);
            }

            impl #enum_name_ident_ex for #lib_ref#enum_name_ident {
                #[allow(unused_braces)]
                #[allow(unused_parens)]
                fn into_bytes<W: ::std::io::Write>(&self, #output_ident: &mut W) {
                    #output_ident.write_i64::<::idl_internal::byteorder::BigEndian>(
                        match self { #( #m_tys )* }
                    )
                    .unwrap()
                }

                #[allow(unused_braces)]
                fn from_bytes<R: ::std::io::Read>(#input_ident: &mut R) -> #lib_ref#enum_name_ident {
                    let variant = #input_ident.read_i64::<::idl_internal::byteorder::BigEndian>().unwrap();
                    match variant {
                        #( #m_tys_value )*
                        _ => panic!("Invalid variant value for enum"),
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
