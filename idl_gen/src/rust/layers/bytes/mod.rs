pub(crate) mod server;
pub(crate) mod client;
pub(crate) mod types;

use idl::analyzer::Analyzer;
use idl::idl_nodes::*;
use proc_macro2::{self, Punct, Spacing, TokenStream};

#[derive(Debug)]
pub enum BytesError {
    UnexpectedType,
    Undefined,
}

pub(crate) trait BytesTypeName {
    fn conv_bytes_to_value(
        &self,
        bytes_name: &TokenStream,
        references: bool,
        analyzer: &Analyzer,
    ) -> TokenStream;
    fn calc_number_bytes(
        &self,
        value_name: &TokenStream,
        references: bool,
        analyzer: &Analyzer,
    ) -> TokenStream;
    fn conv_value_to_bytes(
        &self,
        value_name: &TokenStream,
        bytes_name: &TokenStream,
        references: bool,
        analyzer: &Analyzer,
    ) -> TokenStream;
}

impl BytesTypeName for TypeName {
    // From value types and ptr struct
    fn conv_bytes_to_value(
        &self,
        bytes_name: &TokenStream,
        references: bool,
        analyzer: &Analyzer,
    ) -> TokenStream {
        match self {
            TypeName::Types(value) => match value {
                Types::NatInt => quote! {{ #bytes_name.read_i64::<BigEndian>().unwrap() }},
                Types::NatBool => quote! {{ #bytes_name.read_i64::<BigEndian>().unwrap() == 1 }},
                Types::NatFloat => quote! {{ #bytes_name.read_f64::<BigEndian>().unwrap() }},
                Types::NatNone => {
                    quote! {{ assert!(#bytes_name.read_i64::<BigEndian>().unwrap() == 0); () }}
                }
                Types::NatString => quote! {{
                    let _length = #bytes_name.read_i64::<BigEndian>().unwrap();
                    let mut _data = vec![0; _length as usize];
                    #bytes_name.read_exact(&mut _data[..]).unwrap();
                    unsafe { String::from_utf8_unchecked(_data) }
                }},
                Types::NatBytes => quote! {{
                    let _length = #bytes_name.read_i64::<BigEndian>().unwrap();
                    let mut _data = vec![0; _length as usize];
                    #bytes_name.read_exact(&mut _data[..]).unwrap();
                    _data
                }},
                Types::NatUUID => quote! {{
                    let mut _data: [u8; 0x10] = [0x0; 0x10];
                    #bytes_name.read_exact(&mut _data[..]).unwrap();
                    Uuid::from_bytes(_data)
                }},
            },
            TypeName::ConstTypeName(value) => {
                let const_ty = analyzer
                    .find_ty_const(&value)
                    .expect("Could not reference const type");

                match const_ty.const_type {
                    ConstTypes::NatInt => TypeName::Types(Types::NatInt)
                        .conv_bytes_to_value(bytes_name, references, analyzer),
                    ConstTypes::NatFloat => TypeName::Types(Types::NatFloat)
                        .conv_bytes_to_value(bytes_name, references, analyzer),
                    ConstTypes::NatString => TypeName::Types(Types::NatString)
                        .conv_bytes_to_value(bytes_name, references, analyzer),
                    ConstTypes::NatUuid => TypeName::Types(Types::NatUUID)
                        .conv_bytes_to_value(bytes_name, references, analyzer),
                }
            }
            TypeName::TypeArray(_) => quote! {},
            TypeName::TypeMap(_) => quote! {},
            TypeName::TypeOption(_) => quote! {},
            TypeName::TypeResult(_) => quote! {},
            TypeName::TypePair(_) => quote! {},
            TypeName::ListTypeName(_) => quote! {},
            TypeName::EnumTypeName(_) => quote! {},
            TypeName::StructTypeName(_) => quote! {},
            _ => panic!("Invalid type for conversion"),
        }
    }

    fn conv_value_to_bytes(
        &self,
        value_name: &TokenStream,
        bytes_name: &TokenStream,
        references: bool,
        analyzer: &Analyzer,
    ) -> TokenStream {
        match self {
            TypeName::Types(value) => match value {
                Types::NatInt => {
                    quote! { #bytes_name.write_i64::<BigEndian>(#value_name).unwrap(); }
                }
                Types::NatBool => {
                    quote! { #bytes_name.write_i64::<BigEndian>(#value_name as i64).unwrap(); }
                }
                Types::NatFloat => {
                    quote! { #bytes_name.write_f64::<BigEndian>(#value_name).unwrap(); }
                }
                Types::NatNone => {
                    quote! { assert!(#bytes_name.write_i64::<BigEndian>(0).unwrap()); }
                }
                Types::NatString => quote! {
                    let _length = #value_name.len() as i64;
                    let mut _data = #value_name.into_bytes();
                    #bytes_name.write_i64::<BigEndian>(_length).unwrap();
                    #bytes_name.write_all(&mut _data[..]).unwrap();
                },
                Types::NatBytes => quote! {
                    let _length = #value_name.len() as i64;
                    #bytes_name.write_i64::<BigEndian>(_length).unwrap();
                    #bytes_name.write_all(&mut #value_name[..]).unwrap();
                },
                Types::NatUUID => quote! {
                    let mut _data = #value_name.as_bytes().to_vec();
                    #bytes_name.write_all(&mut _data[..]).unwrap();
                },
            },
            TypeName::ConstTypeName(value) => {
                let const_ty = analyzer
                    .find_ty_const(&value)
                    .expect("Could not reference const type");

                match const_ty.const_type {
                    ConstTypes::NatInt => TypeName::Types(Types::NatInt)
                        .conv_value_to_bytes(value_name, bytes_name, references, analyzer),
                    ConstTypes::NatFloat => TypeName::Types(Types::NatFloat)
                        .conv_value_to_bytes(value_name, bytes_name, references, analyzer),
                    ConstTypes::NatString => TypeName::Types(Types::NatString)
                        .conv_value_to_bytes(value_name, bytes_name, references, analyzer),
                    ConstTypes::NatUuid => TypeName::Types(Types::NatUUID)
                        .conv_value_to_bytes(value_name, bytes_name, references, analyzer),
                }
            }
            TypeName::TypeArray(_) => quote! {},
            TypeName::TypeMap(_) => quote! {},
            TypeName::TypeOption(_) => quote! {},
            TypeName::TypeResult(_) => quote! {},
            TypeName::TypePair(_) => quote! {},
            TypeName::ListTypeName(_) => quote! {},
            TypeName::EnumTypeName(_) => quote! {},
            TypeName::StructTypeName(_) => quote! {},
            _ => panic!("Invalid type for conversion"),
        }
    }

    fn calc_number_bytes(
        &self,
        value_name: &TokenStream,
        references: bool,
        analyzer: &Analyzer,
    ) -> TokenStream {
        match self {
            TypeName::Types(value) => match value {
                Types::NatNone | Types::NatInt | Types::NatBool | Types::NatFloat => quote! { 8 },
                Types::NatString => quote! { #value_name.len() },
                Types::NatBytes => quote! { #value_name.len() },
                Types::NatUUID => quote! { 16 },
            },
            TypeName::ConstTypeName(value) => {
                let const_ty = analyzer
                    .find_ty_const(&value)
                    .expect("Could not reference const type");

                match const_ty.const_type {
                    ConstTypes::NatInt => TypeName::Types(Types::NatInt)
                        .calc_number_bytes(value_name, references, analyzer),
                    ConstTypes::NatFloat => TypeName::Types(Types::NatFloat)
                        .calc_number_bytes(value_name, references, analyzer),
                    ConstTypes::NatString => TypeName::Types(Types::NatString)
                        .calc_number_bytes(value_name, references, analyzer),
                    ConstTypes::NatUuid => TypeName::Types(Types::NatUUID)
                        .calc_number_bytes(value_name, references, analyzer),
                }
            }
            TypeName::EnumTypeName(_) => quote! { 8 },
            TypeName::TypeArray(_) => quote! { 0 },
            TypeName::TypeMap(_) => quote! { 0 },
            TypeName::TypeOption(_) => quote! { 0 },
            TypeName::TypeResult(_) => quote! { 0 },
            TypeName::TypePair(value) => {
                let first = value.first_ty.calc_number_bytes(
                    &quote! { #value_name.0 },
                    references,
                    analyzer,
                );
                let second = value.second_ty.calc_number_bytes(
                    &quote! { #value_name.1 },
                    references,
                    analyzer,
                );
                quote! { #first + #second }
            }
            TypeName::ListTypeName(_) => quote! { 0 },
            TypeName::StructTypeName(_) => quote! { 0 },
            _ => panic!("Invalid type for conversion"),
        }
    }
}


pub(crate) fn create_hash_idents(hash: &[u8]) -> TokenStream {
    let result: Vec<TokenStream> = hash.iter()
        .map(|v| {
            let numb_str = format!("{:x}", v);
            let puncf = Punct::new('0', Spacing::Joint);
            let punc = Punct::new('x', Spacing::Joint);

            let numb0 = Punct::new(numb_str.chars().nth(0).unwrap(), Spacing::Joint);

            if numb_str.chars().count() > 2 {
                panic!("Invalid number");
            }

            match numb_str.chars().nth(1) {
                Some(ch) => {
                    let numb1 = Punct::new(ch, Spacing::Joint);
                    quote! { #puncf#punc#numb0#numb1 }
                }
                None => {
                    quote! { #puncf#punc#puncf#numb0 }
                }
            }
        })
        .collect();

    quote! { #( #result ),* }
}
