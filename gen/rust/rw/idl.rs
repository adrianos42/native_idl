use crate::idl_enums::*;

use crate::ffi_internal::{NativeString, Type};
use proc_macro2::{Ident, Span, TokenStream};
use quote::TokenStreamExt;
use std::convert::TryFrom;
use std::str::FromStr;

pub trait IdlLanguageServer {
    fn add_enums(&mut self, enums: Vec<IdlEnum>);
    fn add_structs(&mut self, structs: Vec<IdlStruct>);
    fn add_interfaces(&mut self, interfaces: Vec<IdlInterface>);
    fn add_strings(&mut self, strings: Vec<IdlConst>);
    fn add_ints(&mut self, ints: Vec<IdlConst>);
    fn add_floats(&mut self, floats: Vec<IdlConst>);
    fn finalize(&mut self);
    fn get_as_string(&self) -> String;
}

#[derive(Debug)]
pub struct IdlInterface {
    pub name: String,
    pub fields: Vec<IdlInterfaceField>,
}

#[derive(Debug)]
pub struct IdlInterfaceField {
    pub field_name: String,
    pub idl_type: IdlType,
    pub interface_attributes: Vec<IdlInterfaceAttributes>,
}

#[derive(Debug)]
pub struct IdlStruct {
    pub name: String,
    pub fields: Vec<IdlStructField>,
}

impl TryFrom<*const crate::AbiIdlStruct> for IdlStruct {
    type Error = ();

    fn try_from(value: *const crate::AbiIdlStruct) -> Result<Self, Self::Error> {
        let name = unsafe { value.read().name.read().to_string() };
        let mut fields = vec![];

        let fields_array = unsafe { value.read().fields.read() };
        let fields_data = fields_array.data as *const *const crate::AbiIdlStructField;
        let fields_length = fields_array.length as isize;

        for i in 0..fields_length {
            let idl_struct_field = unsafe { fields_data.offset(i).read() };
            fields.push(IdlStructField::try_from(idl_struct_field).unwrap());
        }

        Ok(IdlStruct { name, fields })
    }
}

#[derive(Debug)]
pub struct IdlStructField {
    pub field_name: String,
    pub idl_type: IdlType,
}

impl TryFrom<*const crate::AbiIdlStructField> for IdlStructField {
    type Error = ();

    fn try_from(value: *const crate::AbiIdlStructField) -> Result<Self, Self::Error> {
        let field_name = unsafe { value.read().field_name.read().to_string() };
        let field_type = unsafe { value.read().idl_type as *const crate::AbiIdlType };

        let idl_type = IdlType::try_from(field_type).expect("Invalid value in types enum.");

        Ok(IdlStructField {
            field_name,
            idl_type,
        })
    }
}

#[derive(Debug)]
pub struct IdlEnum {
    pub name: String,
    pub field_names: Vec<IdlEnumField>,
}

impl TryFrom<*const crate::AbiIdlEnum> for IdlEnum {
    type Error = ();

    fn try_from(value: *const crate::AbiIdlEnum) -> Result<Self, Self::Error> {
        let name = unsafe { value.read().name.read().to_string() };
        let mut field_names = vec![];

        let fields_array = unsafe { value.read().field_names.read() };
        let fields_data = fields_array.data as *const *const crate::AbiIdlEnumField;
        let fields_length = fields_array.length as isize;

        for i in 0..fields_length {
            let idl_enum_field = unsafe { fields_data.offset(i).read().read() };
            let field_name = unsafe { idl_enum_field.field_name.read().to_string() };
            field_names.push(IdlEnumField { field_name });
        }

        Ok(IdlEnum { field_names, name })
    }
}

#[derive(Debug)]
pub struct IdlEnumField {
    pub field_name: String,
}

#[derive(Debug)]
pub struct IdlConst {
    pub name: String,
    pub fields: Vec<IdlConstField>,
}

#[derive(Debug)]
pub struct IdlConstField {
    pub field_name: String,
    pub field_value: String,
}


#[derive(Debug)]
pub struct IdlType {
    pub object: IdlTypeObjectTypeList,
    pub object_type: IdlTypes,
}

impl TryFrom<*const crate::AbiIdlType> for IdlType {
    type Error = ();

    fn try_from(value: *const crate::AbiIdlType) -> Result<Self, Self::Error> {
        let object_type = crate::IdlTypes::try_from(unsafe { value.read().object_type })
            .expect("Invalid enum value in IdlTypes.");
        let value_object = unsafe { value.read().object };
        match object_type {
            _ => panic!()
            // IdlTypes::NInt => Ok(IdlType::Int),
            // IdlTypes::NFloat => Ok(IdlType::Float),
            // IdlTypes::NBool => Ok(IdlType::Bool),
            // IdlTypes::NString => Ok(IdlType::String),
            // IdlTypes::NBytes => Ok(IdlType::Bytes),
            // IdlTypes::NFunction => {
            //     let type_object = value_object as *const Type;
            //     let type_function = match IdlTypeObjectTypeList::try_from(type_object).unwrap() {
            //         IdlTypeObjectTypeList::IdlFunction(function_value) => function_value,
            //         _ => panic!("Invalid type list "),
            //     };

            //     Ok(IdlType::Function(type_function))
            // }
            // IdlTypes::NInterface | IdlTypes::NStruct | IdlTypes::NEnum => {
            //     let type_object = value_object as *const Type;
            //     let name = match IdlTypeObjectTypeList::try_from(type_object).unwrap() {
            //         IdlTypeObjectTypeList::String(string_value) => string_value,
            //         _ => panic!("Invalid type list for inter type."),
            //     };

            //     if object_type == IdlTypes::NInterface {
            //         Ok(IdlType::Interface(name))
            //     } else if object_type == IdlTypes::NStruct {
            //         Ok(IdlType::Struct(name))
            //     } else {
            //         Ok(IdlType::Enum(name))
            //     }
            // }
            // IdlTypes::NTypeList => {
            //     let type_object = value_object as *const Type;
            //     let type_list = match IdlTypeObjectTypeList::try_from(type_object).unwrap() {
            //         IdlTypeObjectTypeList::IdlTypeList(type_list_value) => type_list_value,
            //         _ => panic!("Invalid type list "),
            //     };

            //     Ok(IdlType::TypeList(type_list))
            // }
            // IdlTypes::NTuple => {
            //     let type_object = value_object as *const Type;
            //     let type_tuple = match IdlTypeObjectTypeList::try_from(type_object).unwrap() {
            //         IdlTypeObjectTypeList::IdlTypeTuple(type_tuple_value) => type_tuple_value,
            //         _ => panic!("Invalid type list "),
            //     };

            //     Ok(IdlType::Tuple(type_tuple))
            // }
            // IdlTypes::NVoid => Ok(IdlType::Void),
            // IdlTypes::NNone => Ok(IdlType::None),
            // IdlTypes::NArray => {
            //     let type_object = value_object as *const Type;
            //     let type_array = match IdlTypeObjectTypeList::try_from(type_object).unwrap() {
            //         IdlTypeObjectTypeList::IdlType(type_array_value) => type_array_value,
            //         _ => panic!("Invalid type list "),
            //     };

            //     Ok(IdlType::Array(type_array))
            // }
        }
    }
}

#[derive(Debug)]
pub enum IdlTypeObjectTypeList {
    None,
    String(Box<String>),
    IdlType(Box<IdlType>),
    IdlFunction(Box<IdlFunction>),
    IdlTypeList(Box<IdlTypeList>),
    IdlTypeTuple(Box<IdlTypeTuple>),
}

impl TryFrom<*const Type> for IdlTypeObjectTypeList {
    type Error = ();

    fn try_from(value: *const Type) -> Result<Self, Self::Error> {
        let name = unsafe { value.read().name.read().to_string() };
        let data = unsafe { value.read().ptr_data };

        match name.as_str() {
            "none" => {
                let value_ptr = data as *const i64;
                let none_value = unsafe { value_ptr.read() };
                assert!(none_value == 0, "None value must be zero.");
                Ok(IdlTypeObjectTypeList::None)
            }
            "string" => {
                let string_value = unsafe { (data as *const NativeString).read().to_string() };
                Ok(IdlTypeObjectTypeList::String(Box::new(string_value)))
            }
            "IdlType" => {
                let value_ptr = data as *const crate::AbiIdlType;
                Ok(IdlTypeObjectTypeList::IdlType(Box::new(
                    IdlType::try_from(value_ptr).expect("Invalid type in abi value."),
                )))
            }
            "IdlFunction" => {
                let value_ptr = data as *const crate::AbiIdlFunction;
                Ok(IdlTypeObjectTypeList::IdlFunction(Box::new(
                    IdlFunction::try_from(value_ptr).expect("Invalid type in abi value."),
                )))
            }
            "IdlTypeList" => {
                let value_ptr = data as *const crate::AbiIdlTypeList;
                Ok(IdlTypeObjectTypeList::IdlTypeList(Box::new(
                    IdlTypeList::try_from(value_ptr).expect("Invalid type in abi value."),
                )))
            }
            "IdlTypeTuple" => {
                let value_ptr = data as *const crate::AbiIdlTypeTuple;
                Ok(IdlTypeObjectTypeList::IdlTypeTuple(Box::new(
                    IdlTypeTuple::try_from(value_ptr).expect("Invalid type in abi value."),
                )))
            }
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub struct IdlTypeList {
    pub enum_name: Option<String>,
    pub type_list: Vec<IdlType>,
}

impl TryFrom<*const crate::AbiIdlTypeList> for IdlTypeList {
    type Error = ();

    fn try_from(value: *const crate::AbiIdlTypeList) -> Result<Self, Self::Error> {
        let type_array = unsafe { value.read().type_list };
        let data = unsafe { type_array.read().data as *const *const crate::AbiIdlType };
        let length = unsafe { type_array.read().length as isize };

        let mut type_list = vec![];

        for i in 0..length {
            let type_item = unsafe { data.offset(i).read() };
            let idl_type = IdlType::try_from(type_item).unwrap();
            type_list.push(idl_type);
        }

        Ok(IdlTypeList {
            enum_name: None,
            type_list,
        })
    }
}

#[derive(Debug)]
pub struct IdlTypeTuple {
    pub type_list: Vec<IdlTupleEntry>,
}

impl TryFrom<*const crate::AbiIdlTypeTuple> for IdlTypeTuple {
    type Error = ();

    fn try_from(value: *const crate::AbiIdlTypeTuple) -> Result<Self, Self::Error> {
        let tuple_array = unsafe { value.read().type_list };
        let data = unsafe { tuple_array.read().data as *const *const crate::AbiIdlTupleEntry };
        let length = unsafe { tuple_array.read().length as isize };

        let mut type_list = vec![];

        for i in 0..length {
            let tuple_item = unsafe { data.offset(i).read() };

            type_list.push(IdlTupleEntry::try_from(tuple_item).unwrap());
        }

        Ok(IdlTypeTuple { type_list })
    }
}

#[derive(Debug)]
pub struct IdlTupleEntry {
    pub name: String,
    pub idl_type: IdlType,
}

impl TryFrom<*const crate::AbiIdlTupleEntry> for IdlTupleEntry {
    type Error = ();

    fn try_from(value: *const crate::AbiIdlTupleEntry) -> Result<Self, Self::Error> {
        let name = unsafe { value.read().name.read().to_string() };
        let idl_type_data = unsafe { value.read().idl_type as *const crate::AbiIdlType };
        let idl_type = IdlType::try_from(idl_type_data).unwrap();

        Ok(IdlTupleEntry { name, idl_type })
    }
}

#[derive(Debug)]
pub struct IdlFunction {
    pub args: IdlType,
    pub return_type: IdlType,
}

impl TryFrom<*const crate::AbiIdlFunction> for IdlFunction {
    type Error = ();

    fn try_from(value: *const crate::AbiIdlFunction) -> Result<Self, Self::Error> {
        let args_data = unsafe { value.read().args as *const crate::AbiIdlType };
        let return_type_data = unsafe { value.read().return_type as *const crate::AbiIdlType };
        let args = IdlType::try_from(args_data).unwrap();
        let return_type = IdlType::try_from(return_type_data).unwrap();

        Ok(IdlFunction { args, return_type })
    }
}
