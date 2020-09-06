use crate::idl::*;
use crate::idl_enums::*;

use crate::string_pros::StringPros;
use proc_macro2::{self, Ident, Span, TokenStream};
use quote::TokenStreamExt;
use std::io::Write;
use std::process::{Command, Stdio};

use std::convert::TryFrom;
use std::str::FromStr;

mod enums;
mod server_code;

const NEW_LINE: &str = "\n\n";

#[derive(Debug)]
enum IdlNatType {
    None,
    Int,
    Bool,
    Float,
    Bytes,
    String,
    Void,
    Interface(Box<String>),
    Struct(Box<String>),
    Enum(Box<String>),
    Array(Box<IdlType>),
    Function(Box<IdlFunction>),
    Tuple(Box<IdlTypeTuple>),
    TypeList(Box<IdlTypeList>),
}

impl IdlNatType {
    pub fn get_type_name(&self) -> proc_macro2::TokenStream {
        match self {
            IdlNatType::Int => quote! { i64 },
            IdlNatType::Bool => quote! { bool },
            IdlNatType::Float => quote! { f64 },
            IdlNatType::Bytes => quote! { Vec<u8> },
            IdlNatType::String => quote! { String },
            IdlNatType::Void => quote! {}, // TODO ??
            IdlNatType::Interface(value) | IdlNatType::Struct(value) | IdlNatType::Enum(value) => {
                let token: TokenStream = TokenStream::from_str(value).unwrap();
                token
            }
            IdlNatType::Array(ty) => {
                //let token = ty.get_type_name();
                //quote! { Vec<#token> }
                quote! {}
            }
            IdlNatType::TypeList(tlist) => {
                let ident = Ident::new(
                    tlist.enum_name.as_ref().map_or("", |value| value.as_str()),
                    Span::call_site(),
                );
                quote! { #ident }
            }
            _ => panic!("Invalid type: {:?}", self),
        }
    }
}

impl TryFrom<IdlType> for IdlNatType {
    type Error = ();

    fn try_from(value: IdlType) -> Result<Self, Self::Error> {
        match value.object_type {
            IdlTypes::NInt => Ok(IdlNatType::Int),
            IdlTypes::NFloat => Ok(IdlNatType::Float),
            IdlTypes::NBool => Ok(IdlNatType::Bool),
            IdlTypes::NString => Ok(IdlNatType::String),
            IdlTypes::NBytes => Ok(IdlNatType::Bytes),
            IdlTypes::NVoid => Ok(IdlNatType::Void),
            IdlTypes::NNone => Ok(IdlNatType::None),
            IdlTypes::NFunction => { 
                let function_value = match value.object {
                    IdlTypeObjectTypeList::IdlFunction(idl_function) => idl_function,
                    _ => panic!("Error function object."),
                };

                Ok(IdlNatType::Function(function_value))
            },
            _ => panic!()
            // IdlTypes::NInterface => {}
            // IdlTypes::NStruct => {}
            // IdlTypes::NEnum => {}
            // IdlTypes::NTypeList => Ok(IdlNatType::TypeList(type_list)),
            // IdlTypes::NTuple => Ok(IdlNatType::Tuple(type_tuple)),
            // IdlTypes::NArray => Ok(IdlNatType::Array(type_array)),
        }
    }
}

pub fn create_instance() -> Box<dyn IdlLanguageServer> {
    Box::new(IdlLanguageServerImpl {
        enums: None,
        floats: None,
        structs: None,
        interfaces: None,
        ints: None,
        strings: None,
        enum_module: Vec::new(),
        private_struct_module: Vec::new(),
        struct_module: Vec::new(),
        //        code: String::new(),
    })
}

struct IdlLanguageServerImpl {
    enums: Option<Vec<IdlEnum>>,
    structs: Option<Vec<IdlStruct>>,
    interfaces: Option<Vec<IdlInterface>>,
    strings: Option<Vec<IdlConst>>,
    ints: Option<Vec<IdlConst>>,
    floats: Option<Vec<IdlConst>>,

    enum_module: Vec<TokenStream>,
    private_struct_module: Vec<TokenStream>,
    struct_module: Vec<TokenStream>,
    // code: Vec<TokenStream>,
}

impl IdlLanguageServer for IdlLanguageServerImpl {
    fn add_enums(&mut self, enums: Vec<IdlEnum>) {
        self.enums = Some(enums);
    }

    fn add_structs(&mut self, structs: Vec<IdlStruct>) {
        self.structs = Some(structs);
    }

    fn add_interfaces(&mut self, interfaces: Vec<IdlInterface>) {
        self.interfaces = Some(interfaces);
    }

    fn add_strings(&mut self, strings: Vec<IdlConst>) {
        self.strings = Some(strings);
    }

    fn add_ints(&mut self, ints: Vec<IdlConst>) {
        self.ints = Some(ints);
    }

    fn add_floats(&mut self, floats: Vec<IdlConst>) {
        self.floats = Some(floats);
    }

    fn finalize(&mut self) {
        self.set_strings();
        self.set_ints();
        self.set_floats();
        self.set_enums();
        self.set_structs();
        self.set_interfaces();
    }

    fn get_as_string(&self) -> String {
        let mut result_code = String::new();

        self.private_struct_module.iter().for_each(|value| {
            result_code += value.to_string().as_str();
            result_code += NEW_LINE;
        });

        println!("{}", result_code);

        result_code = IdlLanguageServerImpl::rust_fmt(result_code.as_str());

        result_code
    }
}

impl IdlLanguageServerImpl {
    fn set_enums(&mut self) {
        if let Some(enums) = &self.enums {
            for enum_value in enums.iter() {
                let ident = Ident::new(enum_value.name.as_str(), Span::call_site());

                let mut field_vec = vec![];

                for field in enum_value.field_names.iter() {
                    let field_ident = Ident::new(
                        field.field_name.as_str().to_pascal_case().as_str(),
                        Span::call_site(),
                    );

                    field_vec.push(field_ident);
                }

                let fields = field_vec.into_iter();

                self.enum_module.push(
                    quote! {
                        pub enum #ident {
                            #( #fields ),*
                        }

                    }
                    .into(),
                );
            }
        }
    }

    fn set_structs(&mut self) {
        if let Some(structs) = &self.structs {
            for struct_value in structs.iter() {
                // Creates struct definition for abi, that will be generated with their own files.
                let ident_str = "Abi".to_owned() + struct_value.name.as_str();
                let ident = Ident::new(ident_str.as_str(), Span::call_site());

                let fields: Vec<TokenStream> = struct_value
                    .fields
                    .iter()
                    .map(|value| {
                        let ident = Ident::new(value.field_name.as_str(), Span::call_site());
                        //let ty = value.idl_type.get_type_name();
                        quote! { #ident:  }
                    })
                    .collect();

                self.private_struct_module.push(
                    quote! {
                        #[repr(C)]
                        pub struct #ident {
                            #( #fields ),*
                        }
                    }
                    .into(),
                )
            }

            self.struct_module.push(
                quote! {
                    mod private_structs;
                }
                .into(),
            );

            for struct_value in structs.iter() {
                // Structs that visible to the client, which have no native representation, only pure rust.
                let ident = Ident::new(struct_value.name.as_str(), Span::call_site());

                self.struct_module.push(
                    quote! {
                        pub struct #ident {

                        }
                    }
                    .into(),
                )
            }
        }
    }

    fn set_interfaces(&mut self) {}

    fn set_strings(&mut self) {}

    fn set_ints(&mut self) {}

    fn set_floats(&mut self) {}

    fn rust_fmt(code: &str) -> String {
        // ????? Why not a lib?
        let mut cmd = Command::new("rustfmt");

        cmd.stdin(Stdio::piped()).stdout(Stdio::piped());

        let mut child = cmd.spawn().unwrap();
        let mut child_stdin = child.stdin.take().unwrap();
        let mut child_stdout = child.stdout.take().unwrap();

        let source = code.to_owned();

        let stdin_handle = std::thread::spawn(move || {
            let _ = child_stdin.write_all(source.as_bytes());
            source
        });

        let mut output = vec![];
        std::io::copy(&mut child_stdout, &mut output).unwrap();

        let status = child.wait().unwrap();
        let source = stdin_handle.join().unwrap();

        match String::from_utf8(output) {
            Ok(bindings) => match status.code() {
                Some(0) => bindings,
                Some(2) => "Rustfmt parsing erros.".to_owned(),
                Some(3) => "Rustfmt could not format some lines.".to_owned(),
                _ => "rustfmt error.".to_owned(),
            },
            _ => source,
        }
    }
}
