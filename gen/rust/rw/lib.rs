use std::ffi::c_void;

#[macro_use]
extern crate quote;

#[macro_use]
extern crate lazy_static;

mod ffi_internal;
mod gen;
mod idl;
pub mod idl_enums;
mod idl_impl;

pub mod scanner;
pub mod parser;

pub mod string_pros;

use crate::ffi_internal::*;
use crate::idl_enums::*;
use std::convert::TryFrom;

#[repr(C)]
pub struct AbiIdlLanguageServer {
    instance: Box<dyn crate::idl::IdlLanguageServer>,
}

#[no_mangle]
fn release_idl_language_server(instance: *mut AbiIdlLanguageServer) -> i64 {
    IdlAbiError::Ok as i64
}

#[no_mangle]
fn factory_idl_language_server_create_instance(value: *mut *const AbiIdlLanguageServer) -> i64 {
    unsafe {
        *value = Box::into_raw(Box::new(AbiIdlLanguageServer {
            instance: gen::create_instance(),
        }));
    }
    IdlAbiError::Ok as i64
}

#[no_mangle]
fn function_idl_language_server_sync_add_enums(
    instance: *mut AbiIdlLanguageServer,
    enums: *mut ArrayType,
) -> i64 {
    let mut ins = unsafe { Box::from_raw(instance) };

    let enums_array = unsafe { enums.read() };
    let data = enums_array.data as *const *const AbiIdlEnum;
    let length = enums_array.length as isize;

    let mut enums_vec = vec![];
    for i in 0..length {
        let idl_enum = unsafe { data.offset(i).read() };
        enums_vec.push(idl::IdlEnum::try_from(idl_enum).unwrap());
    }

    ins.instance.add_enums(enums_vec);

    let _ = Box::into_raw(ins);

    IdlAbiError::Ok as i64
}

#[no_mangle]
fn function_idl_language_server_sync_add_structs(
    instance: *mut AbiIdlLanguageServer,
    structs: *mut ArrayType,
) -> i64 {
    let mut ins = unsafe { Box::from_raw(instance) };

    let structs_array = unsafe { structs.read() };
    let data = structs_array.data as *const *const AbiIdlStruct;
    let length = structs_array.length as isize;

    let mut structs_vec = vec![];
    for i in 0..length {
        let idl_struct = unsafe { data.offset(i).read() };
        structs_vec.push(idl::IdlStruct::try_from(idl_struct).unwrap());
    }

    ins.instance.add_structs(structs_vec);

    let _ = Box::into_raw(ins);

    IdlAbiError::Ok as i64
}

#[no_mangle]
fn function_idl_language_server_sync_add_interfaces(
    instance: *mut AbiIdlLanguageServer,
    interfaces: *mut ArrayType,
) -> i64 {
    IdlAbiError::Ok as i64
}

#[no_mangle]
fn function_idl_language_server_sync_add_strings(
    instance: *mut AbiIdlLanguageServer,
    strings: *mut ArrayType,
) -> i64 {
    IdlAbiError::Ok as i64
}

#[no_mangle]
fn function_idl_language_server_sync_add_ints(
    instance: *mut AbiIdlLanguageServer,
    ints: *mut ArrayType,
) -> i64 {
    IdlAbiError::Ok as i64
}

#[no_mangle]
fn function_idl_language_server_sync_add_floats(
    instance: *mut AbiIdlLanguageServer,
    floats: *mut ArrayType,
) -> i64 {
    IdlAbiError::Ok as i64
}

#[no_mangle]
fn function_idl_language_server_sync_finalize(instance: *mut AbiIdlLanguageServer) -> i64 {
    let mut ins = unsafe { Box::from_raw(instance) };
    ins.instance.finalize();

    let _ = Box::into_raw(ins);

    IdlAbiError::Ok as i64
}

#[no_mangle]
fn function_idl_language_server_get_as_string(
    instance: *mut AbiIdlLanguageServer,
    value: *mut *const NativeString,
) -> i64 {
    let ins = unsafe { Box::from_raw(instance) };
    let w = ins.instance.get_as_string();

    unsafe {
        *value = NativeString::from_string(w.as_str());
    }

    let _ = Box::into_raw(ins);

    IdlAbiError::Ok as i64
}

#[no_mangle]
fn function_idl_language_server_release_as_string(
    instance: *mut AbiIdlLanguageServer,
    value: *mut NativeString,
) -> i64 {
    IdlAbiError::Ok as i64
}

#[repr(C)]
struct AbiIdlInterface {
    name: *mut NativeString,
    fields: *mut ArrayType,
}

#[repr(C)]
struct AbiIdlInterfaceField {
    field_name: *mut NativeString,
    idl_type: *mut AbiIdlType,
    interface_attributes: *mut ArrayType,
}

#[repr(C)]
struct AbiIdlStruct {
    name: *mut NativeString,
    fields: *mut ArrayType,
}

#[repr(C)]
struct AbiIdlStructField {
    field_name: *mut NativeString,
    idl_type: *mut AbiIdlType,
}

#[repr(C)]
struct AbiIdlEnum {
    name: *mut NativeString,
    field_names: *mut ArrayType,
}

#[repr(C)]
struct AbiIdlEnumField {
    field_name: *mut NativeString,
}

#[repr(C)]
struct AbiIdlConst {
    name: *mut NativeString,
    fields: *mut ArrayType,
}

#[repr(C)]
struct AbiIdlConstFIeld {
    field_name: *mut NativeString,
    field_value: *mut NativeString,
}

#[repr(C)]
struct AbiIdlType {
    object: *mut Type,
    object_type: i64,
}

#[repr(C)]
struct AbiIdlTypeList {
    type_list: *mut ArrayType,
}

#[repr(C)]
struct AbiIdlTypeTuple {
    type_list: *mut ArrayType,
}

#[repr(C)]
struct AbiIdlTupleEntry {
    name: *mut NativeString,
    idl_type: *mut AbiIdlType,
}

#[repr(C)]
struct AbiIdlTypeArray {
    array_type: *mut AbiIdlType,
}

#[repr(C)]
struct AbiIdlFunction {
    args: *mut AbiIdlType,
    return_type: *mut AbiIdlType,
}

// #[no_mangle]
// fn function_calculator_sync_fuck(_instance: *mut Calculator, value: *mut *const ArrayType) -> i64 {
//     let value0 = Shit { length: 43 };
//     let value1 = Shit { length: 23 };
//     let value2 = Shit { length: 42 };
//     let value3 = Shit { length: 53489345 };

//     let mut vect = vec![value0, value1, value2, value3].into_boxed_slice();
//     let vect_ptr: *mut c_void = vect.as_mut_ptr() as *mut c_void;
//     let vav = vect_ptr as i64;

//     let inn_arr = ArrayType {
//         length: vect.len() as i64,
//         data: vect_ptr,
//     };

//     std::mem::forget(vect);

//     let value0 = Shit { length: inn_arr.length };
//     let value1 = Shit { length: 4323 };
//     let value2 = Shit { length: 4342 };
//     let value3 = Shit { length: 345 };

//     let mut vect = vec![value0, value1, value2, value3].into_boxed_slice();
//     let vect_ptr: *mut c_void = vect.as_mut_ptr() as *mut c_void;

//     let inn_arr2 = ArrayType {
//         length: vect.len() as i64,
//         data: vect_ptr,
//     };

//     std::mem::forget(vect);

//      let value0 = Shit { length: vav as i64 };
//     let value1 = Shit { length: 4323 };
//     let value2 = Shit { length: 4342 };
//     let value3 = Shit { length: 345 };

//     let mut vect = vec![value0, value1, value2, value3].into_boxed_slice();
//     let vect_ptr: *mut c_void = vect.as_mut_ptr() as *mut c_void;

//     let inn_arr1 = ArrayType {
//         length: vect.len() as i64,
//         data: vect_ptr,
//     };

//     std::mem::forget(vect);

//     let mut vect = vec![inn_arr, inn_arr1, inn_arr2].into_boxed_slice();
//     let vect_ptr: *mut c_void = vect.as_mut_ptr() as *mut c_void;

//     let result = ArrayType {
//         length: vect.len() as i64,
//         data: vect_ptr,
//     };

//     std::mem::forget(vect);

//     unsafe {
//         *value = Box::into_raw(Box::new(result));
//     }

//     0
// }
