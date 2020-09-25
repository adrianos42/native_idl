use idl::analyzer::Analyzer;
use idl::idl_types::*;

use super::con_idl::get_rust_ty_ref;

use super::string_pros::StringPros;
use proc_macro2::{self, Ident, Literal, Punct, Spacing, Span, TokenStream};
use regex::Regex;
use std::f64;
use std::i64;
use std::{fmt, str::FromStr};

const NEW_LINE: &str = "\n\n";

#[derive(Debug)]
pub enum RustTypeError {
    UnexpectedType,
    InvalidLiteral,
}

pub struct RustTypes {
    module: Vec<TokenStream>,
}

impl fmt::Display for RustTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result_code = String::new();

        self.module.iter().for_each(|value| {
            result_code += &value.to_string();
            result_code += NEW_LINE;
        });

        write!(f, "{}", result_code.as_str().rust_fmt())
    }
}

impl RustTypes {
    pub fn generate(analyzer: &Analyzer) -> Result<Self, RustTypeError> {
        let mut context = RustTypes::new();
        let nodes: &[TypeNode] = &analyzer.nodes;

        for node in nodes {
            match node {
                TypeNode::Comment(_) => {}
                TypeNode::StructComment(_) => {}
                TypeNode::TypeStruct(value) => context.add_struct(value)?,
                TypeNode::EnumComment(_) => {}
                TypeNode::TypeEnum(value) => context.add_enum(value)?,
                TypeNode::TypeListComment(_) => {}
                TypeNode::TypeList(value) => context.add_type_list(value)?,
                TypeNode::ConstComment(_) => {}
                TypeNode::TypeConst(value) => context.add_const(value)?,
                _ => {}
            }
        }

        Ok(context)
    }

    fn add_enum(&mut self, ty_enum: &TypeEnum) -> Result<(), RustTypeError> {
        let ident = Ident::new(&ty_enum.ident, Span::call_site());

        let mut fields_result = vec![];

        for field_node in &ty_enum.fields {
            match field_node {
                EnumNode::EnumField(field) => {
                    let field_ident = Ident::new(&field.ident, Span::call_site());

                    fields_result.push(field_ident);
                }
                EnumNode::Comment(_) => {}
            }
        }

        let fields = fields_result.into_iter();

        self.module.push(quote! {
            #[derive(Debug, Copy, Clone)]
            pub enum #ident { #( #fields ),* }
        });

        Ok(())
    }

    fn add_type_list(&mut self, type_list: &TypeList) -> Result<(), RustTypeError> {
        let ident = Ident::new(&type_list.ident, Span::call_site());

        let mut fields = vec![];

        for field_node in &type_list.fields {
            match field_node {
                TypeListNode::TypeListField(field) => {
                    let field_ident = Ident::new(&field.ident, Span::call_site());

                    match &field.ty {
                        TypeName::Types(value) => match value {
                            Types::NatNone => {
                                fields.push(quote! { #field_ident, });
                            }
                            _ => {
                                let ty_ident = get_rust_ty_ref(&field.ty, false);
                                fields.push(quote! { #field_ident(#ty_ident), });
                            }
                        },
                        TypeName::TypeFunction(_)
                        | TypeName::TypeTuple(_)
                        | TypeName::TypeResult(_)
                        | TypeName::TypeOption(_)
                        | TypeName::InterfaceTypeName(_) => {
                            return Err(RustTypeError::UnexpectedType)
                        }
                        TypeName::StructTypeName(value) | TypeName::ListTypeName(value) => {
                            let ident = Ident::new(&value, Span::call_site());
                            fields.push(quote! { #field_ident(Box<#ident>), });
                        }
                        ty => {
                            let ty_ident = get_rust_ty_ref(&ty, false);
                            fields.push(quote! { #field_ident(#ty_ident), });
                        }
                    }
                }
                TypeListNode::Comment(_) => {}
            }
        }

        self.module.push(quote! {
            #[derive(Debug, Clone)]
            pub enum #ident { #( #fields )* }
        });

        Ok(())
    }

    fn add_struct(&mut self, ty_struct: &TypeStruct) -> Result<(), RustTypeError> {
        let ident = Ident::new(&ty_struct.ident, Span::call_site());

        let mut fields = vec![];

        for field_node in &ty_struct.fields {
            match field_node {
                StructNode::StructField(field) => {
                    let field_ident = Ident::new(&field.ident, Span::call_site());

                    match &field.ty {
                        TypeName::TypeFunction(_)
                        | TypeName::TypeTuple(_)
                        | TypeName::TypeResult(_)
                        | TypeName::TypeOption(_)
                        | TypeName::InterfaceTypeName(_) => {
                            return Err(RustTypeError::UnexpectedType)
                        }
                        ty => {
                            let ty_ident = get_rust_ty_ref(&ty, false);
                            fields.push(quote! { pub #field_ident: #ty_ident, });
                        }
                    }
                }
                StructNode::Comment(_) => {}
            }
        }

        self.module.push(quote! {
            #[derive(Debug, Clone)]
            pub struct #ident { #( #fields )* }
        });

        Ok(())
    }

    fn add_const(&mut self, ty_const: &TypeConst) -> Result<(), RustTypeError> {
        let ident = &ty_const.ident;

        let mut fields = vec![];

        for field_node in &ty_const.fields {
            match field_node {
                ConstNode::ConstField(field) => {
                    let field_name = format!(
                        "{}_{}",
                        ident.to_snake_case_upper(),
                        &field.ident.as_str().to_snake_case_upper()
                    );
                    let fd_ident = Ident::new(&field_name, Span::call_site());

                    match ty_const.const_type {
                        ConstTypes::NatString => {
                            let literal = Literal::string(&field.value);
                            fields.push(quote! { pub const #fd_ident: &str = #literal; })
                        }
                        ConstTypes::NatInt => {
                            lazy_static! {
                                static ref RE: Regex =
                                    Regex::new(r"^((-)?0x[0-9a-fA-F]+)|(-?[0-9]+)$").unwrap();
                            }

                            let caps = RE.captures(&field.value).expect("Invalid number.");

                            if let Some(cap) = caps.get(3) {
                                let value = i64::from_str_radix(cap.as_str(), 10)
                                    .expect("Invalid number value");

                                let literal = Literal::i64_unsuffixed(value);
                                fields.push(quote! { pub const #fd_ident: i64 = #literal; })
                            } else if let Some(cap) = caps.get(1) {
                                let z_lit = Punct::new('0', Spacing::Joint);
                                let c_lit = Punct::new('x', Spacing::Joint);
                                if let Some(_) = caps.get(2) {
                                    let value = i64::from_str_radix(
                                        cap.as_str().trim_start_matches("-0x"),
                                        0x10,
                                    )
                                    .expect("Invalid hex number value");

                                    let num = Literal::i64_unsuffixed(value);

                                    let s_lit = Punct::new('-', Spacing::Joint);

                                    fields.push(
                                    quote! { pub const #fd_ident: i64 = #s_lit#z_lit#c_lit#num; },
                                )
                                } else {
                                    let value = i64::from_str_radix(
                                        cap.as_str().trim_start_matches("0x"),
                                        0x10,
                                    )
                                    .expect("Invalid hex number value");

                                    let num = Literal::i64_unsuffixed(value);

                                    fields.push(
                                        quote! { pub const #fd_ident: i64 = #z_lit#c_lit#num; },
                                    )
                                }
                            } else {
                                panic!("Expected number.");
                            };
                        }
                        ConstTypes::NatFloat => {
                            let value =
                                f64::from_str(&field.value).expect("Invalid floating number.");
                            let num = Literal::f64_unsuffixed(value);
                            fields.push(quote! { pub const #fd_ident: f64 = #num; })
                        }
                    }
                }
                ConstNode::Comment(_) => {}
            }
        }

        let const_def = match ty_const.const_type {
            ConstTypes::NatInt => quote! { i64 },
            ConstTypes::NatFloat => quote! { f64 },
            ConstTypes::NatString => quote! { String },
        };
        let const_ident = Ident::new(&ident, Span::call_site());

        self.module.push(quote! {
            pub type #const_ident = #const_def;
            #( #fields )*
        });

        Ok(())
    }

    fn new() -> Self {
        Self { module: vec![] }
    }
}
