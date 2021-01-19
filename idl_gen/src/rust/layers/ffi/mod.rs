use crate::{lang::StorageItem, rust::string_pros::StringPros};
use idl::analyzer::Analyzer;
use idl::idl_nodes::*;
use idl::ids;
use proc_macro2::{self, TokenStream};
use quote::format_ident;
use std::fmt;

use super::LayerBuilder;

pub mod client;
pub mod server;
pub mod server_cargo;

// TODO Check to see if arguments and returned values are valid const values defined in idl.

pub fn ffi_server_files(
    analyzer: &idl::analyzer::Analyzer,
    ids_analyzer: &ids::analyzer::Analyzer,
    server_name: &str,
) -> StorageItem {
    let ffi_server_impl = server::FFIServerImpl::generate(&analyzer).unwrap();
    let ffi_server_types = server::FFIServerTypes::generate(&analyzer).unwrap();
    let ffi_server = server::FFIServer::generate(&analyzer).unwrap();
    let ffi_lib = FFIMod::generate(&analyzer).unwrap();
    let ffi_cargo = server_cargo::FFIServerCargo::generate(&ids_analyzer, server_name).unwrap();

    StorageItem::Folder {
        items: vec![
            StorageItem::Folder {
                name: "src".to_owned(),
                items: vec![
                    StorageItem::Source {
                        name: "ffi.rs".to_owned(),
                        txt: ffi_server.to_string(),
                    },
                    StorageItem::Source {
                        name: "ffi_impl.rs".to_owned(),
                        txt: ffi_server_impl.to_string(),
                    },
                    StorageItem::Source {
                        name: "ffi_types.rs".to_owned(),
                        txt: ffi_server_types.to_string(),
                    },
                    StorageItem::Source {
                        name: "lib.rs".to_owned(),
                        txt: ffi_lib.to_string(),
                    },
                ],
            },
            StorageItem::Source {
                name: "Cargo.toml".to_owned(),
                txt: ffi_cargo.to_string(),
            },
        ],
        name: "idl_ffi".to_owned(),
    }
}

pub(crate) struct FFILayer {
    server_name: String,
}

impl FFILayer {
    pub(crate) fn new(server_name: String) -> Self {
        Self { server_name }
    }
}

impl LayerBuilder for FFILayer {
    fn build(
        &self,
        analyzer: &idl::analyzer::Analyzer,
        ids_analyzer: &ids::analyzer::Analyzer,
    ) -> Vec<StorageItem> {
        let mut files = vec![];
        files.push(ffi_server_files(analyzer, ids_analyzer, &self.server_name));
        files
    }
}

pub struct FFIMod {
    module: TokenStream,
}

impl fmt::Display for FFIMod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.module.to_string().as_str().rust_fmt())
    }
}

impl FFIMod {
    pub fn generate(_analyzer: &Analyzer) -> Result<Self, ()> {
        let module = quote! {
            mod ffi; // interface and static functions
            mod ffi_types; // ffi types
            mod ffi_impl; // ffi interface type
        };

        Ok(FFIMod { module })
    }
}

pub(crate) fn get_ffi_ty_ref(ty: &TypeName, references: bool, analyzer: &Analyzer) -> TokenStream {
    let ident_ty = get_value_ffi_ty_ref(ty, references, analyzer);

    match ty {
        TypeName::Types(types) => match types {
            Types::NatInt | Types::NatBool | Types::NatNone | Types::NatFloat => ident_ty,
            _ => quote! { *const #ident_ty },
        },
        TypeName::EnumTypeName(_) => ident_ty,
        TypeName::ConstTypeName(value) => {
            let const_ty = analyzer
                .find_ty_const(&value)
                .expect("Could not reference const type");
            match const_ty.const_type {
                ConstTypes::NatString => quote! { *const #ident_ty },
                ConstTypes::NatInt | ConstTypes::NatFloat => ident_ty,
            }
        }
        TypeName::TypeFunction(_) | TypeName::TypeTuple(_) => panic!("Invalid type `{:?}`", ty),
        _ => quote! { *const #ident_ty },
    }
}

pub(crate) fn is_boxed_ffi(ty: &TypeName, analyzer: &Analyzer) -> bool {
    match ty {
        TypeName::Types(types) => match types {
            Types::NatInt | Types::NatFloat | Types::NatBool | Types::NatNone => false,
            Types::NatString | Types::NatBytes => true,
        },
        TypeName::EnumTypeName(_) => true,
        TypeName::ConstTypeName(value) => {
            let const_ty = analyzer
                .find_ty_const(&value)
                .expect("Could not reference const type");

            match const_ty.const_type {
                ConstTypes::NatString => true,
                ConstTypes::NatInt | ConstTypes::NatFloat => false,
            }
        }
        _ => true,
    }
}

pub(crate) fn get_ptr_ffi_ty_ref(
    ty: &TypeName,
    references: bool,
    analyzer: &Analyzer,
) -> TokenStream {
    let ident = get_value_ffi_ty_ref(ty, references, analyzer);
    quote! { *const #ident }
}

pub(crate) fn get_value_ffi_ty_ref(
    ty: &TypeName,
    references: bool,
    analyzer: &Analyzer,
) -> TokenStream {
    match ty {
        TypeName::Types(types) => match types {
            Types::NatInt | Types::NatBool | Types::NatNone => quote! { i64 },
            Types::NatFloat => quote! { f64 },
            Types::NatString => quote! { AbiString },
            Types::NatBytes => quote! { AbiBytes },
        },
        TypeName::TypeArray(_) => quote! { AbiArray },
        TypeName::TypeMap(_) => quote! { AbiMap },
        TypeName::TypeStream(_) => quote! { AbiStream },
        TypeName::TypeOption(_) | TypeName::TypeResult(_) => quote! { AbiVariant },
        TypeName::ConstTypeName(value) => {
            let const_ty = analyzer
                .find_ty_const(&value)
                .expect("Could not reference const type");

            match const_ty.const_type {
                ConstTypes::NatString => quote! { AbiString },
                ConstTypes::NatInt => quote! { i64 },
                ConstTypes::NatFloat => quote! { f64 },
            }
        }
        TypeName::EnumTypeName(_) => quote! { i64 },
        TypeName::StructTypeName(value) | TypeName::ListTypeName(value) => {
            let ident = format_ident!("{}", &value);
            if references {
                // TODO ?? Would it be better?
                quote! { crate::ffi_types::#ident }
            } else {
                quote! { #ident }
            }
        }
        TypeName::InterfaceTypeName(value) => {
            let ident = format_ident!("{}Instance", value);
            quote! { ffi_impl::#ident }
        }

        sw => panic!("Invalid type `{:?}`", sw),
    }
}

// From value types and ptr struct
pub(crate) fn conv_ffi_to_value(
    ty_name: &TypeName,
    ffi_name: &TokenStream,
    references: bool,
    analyzer: &Analyzer,
) -> TokenStream {
    match ty_name {
        TypeName::Types(value) => match value {
            Types::NatString | Types::NatBytes => {
                return conv_ffi_ptr_to_value(ty_name, ffi_name, references, analyzer)
            }
            _ => {}
        },
        TypeName::TypeArray(_)
        | TypeName::TypeMap(_)
        | TypeName::ListTypeName(_)
        | TypeName::StructTypeName(_)
        | TypeName::TypeStream(_)
        | TypeName::TypeOption(_)
        | TypeName::TypeResult(_) => {
            return conv_ffi_ptr_to_value(ty_name, ffi_name, references, analyzer)
        }
        TypeName::ConstTypeName(value) => {
            let const_ty = analyzer
                .find_ty_const(&value)
                .expect("Could not reference const type");

            match const_ty.const_type {
                ConstTypes::NatString => {
                    return conv_ffi_ptr_to_value(ty_name, ffi_name, references, analyzer)
                }
                _ => {}
            };
        }
        _ => {}
    }

    conv_ffi_value_to_value(ty_name, ffi_name, references, analyzer)
}

pub(crate) fn conv_ffi_ptr_to_value(
    ty_name: &TypeName,
    ffi_name: &TokenStream,
    references: bool,
    analyzer: &Analyzer,
) -> TokenStream {
    let c_ffi = get_ptr_ffi_ty_ref(&ty_name, references, analyzer);
    conv_ffi_value_to_value(
        ty_name,
        &(quote! { unsafe { (#ffi_name as #c_ffi).read() } }),
        references,
        analyzer,
    )
}

pub(crate) fn conv_ffi_value_to_value(
    ty_name: &TypeName,
    ffi_name: &TokenStream,
    references: bool,
    analyzer: &Analyzer,
) -> TokenStream {
    match ty_name {
        TypeName::Types(value) => match value {
            Types::NatInt => quote! { { #ffi_name } as i64 },
            Types::NatFloat => quote! { { #ffi_name } as f64 },
            Types::NatString => quote! { { #ffi_name.to_string() } },
            Types::NatBytes => quote! { { #ffi_name.to_vec() } },
            Types::NatBool => quote! { { #ffi_name } as i64 == 1 },
            Types::NatNone => quote! { { assert!({ #ffi_name } as i64 == 0); () } },
        },
        TypeName::TypeArray(value) => {
            let array_item = quote! { _array_item };
            let array_to_value =
                conv_ffi_value_to_value(&value.ty, &array_item, references, analyzer);
            let array_ty_ident = get_value_ffi_ty_ref(&value.ty, references, analyzer);

            quote! { {
                let _array = #ffi_name;
                let _data = _array.data as *const #array_ty_ident;
                let _length = _array.length as isize;
                let mut _array_vec = vec![];
                for _index in 0.._length {
                    let #array_item = unsafe { _data.offset(_index).read() };
                    _array_vec.push(#array_to_value);
                }
                _array_vec
            } }
        }
        TypeName::TypeMap(value) => {
            let map_data = quote! { _map_data };
            let map_key = quote! { _map_key };
            let value_ty_ident = get_value_ffi_ty_ref(&value.map_ty, references, analyzer);
            let key_ty_ident = get_value_ffi_ty_ref(&value.index_ty, references, analyzer);
            let map_to_value =
                conv_ffi_value_to_value(&value.map_ty, &map_data, references, analyzer);
            let map_to_key =
                conv_ffi_value_to_value(&value.index_ty, &map_key, references, analyzer);

            quote! { {
                let _map = #ffi_name;
                let _length = _map.length as isize;
                let _data = _map.data as *const #value_ty_ident;
                let _key = _map.key as *const #key_ty_ident;
                let mut _map_result = ::std::collections::HashMap::new();
                for _index in 0.._length {
                    let #map_data = unsafe { _data.offset(_index).read() };
                    let #map_key = unsafe { _key.offset(_index).read() };
                    if let Some(_) = _map_result.insert(#map_to_key, #map_to_value) { panic!() }
                }
                _map_result
            } }
        }
        TypeName::TypeOption(value) => {
            let some_value = quote! { _some_value };
            let con_name = conv_ffi_ptr_to_value(&value.some_ty, &some_value, references, analyzer);
            quote! { {
                let _option = #ffi_name;
                match _option.index {
                    0 => { let #some_value = _option.data; Some(#con_name) },
                    1 => None,
                    _ => panic!(),
                }
            } }
        }
        TypeName::TypeResult(value) => {
            let ok_value = quote! { _ok_value };
            let err_value = quote! { _err_value };
            let con_ok_name = conv_ffi_ptr_to_value(&value.ok_ty, &ok_value, references, analyzer);
            let con_err_name =
                conv_ffi_ptr_to_value(&value.err_ty, &err_value, references, analyzer);
            quote! { {
                let _result = #ffi_name;
                match _result.index {
                    0 => { let #ok_value = _result.data; Ok(#con_ok_name) }
                    1 => { let #err_value = _result.data; Err(#con_err_name) }
                    _ => panic!(),
                }
            } }
        }
        TypeName::TypeStream(_) => {
            quote! { #ffi_name }
        }
        TypeName::ListTypeName(value)
        | TypeName::StructTypeName(value)
        | TypeName::EnumTypeName(value) => {
            let ident = format_ident!("{}", &value);
            quote! { { idl_types::#ident::from(crate::ffi_types::#ident::from(#ffi_name)) } }
        }
        TypeName::ConstTypeName(value) => {
            let const_ty = analyzer
                .find_ty_const(&value)
                .expect("Could not reference const type");

            let c_ty = match const_ty.const_type {
                ConstTypes::NatInt => &TypeName::Types(Types::NatInt),
                ConstTypes::NatFloat => &TypeName::Types(Types::NatFloat),
                ConstTypes::NatString => &TypeName::Types(Types::NatString),
            };

            conv_ffi_value_to_value(c_ty, ffi_name, references, analyzer)
        }
        sw => panic!("Invalid type `{:?}`", sw),
    }
}

pub(crate) fn conv_value_to_ffi_boxed(
    ty_name: &TypeName,
    value_name: &TokenStream,
    references: bool,
    analyzer: &Analyzer,
) -> TokenStream {
    let b_value = conv_value_to_ffi_value(ty_name, value_name, references, analyzer);
    let ref_ffi = get_ptr_ffi_ty_ref(ty_name, references, analyzer);
    quote! { { Box::into_raw(Box::new(#b_value)) as #ref_ffi  } }
}

// Values types are kept unchanged, meanwhile struct are returned as a ptr
pub(crate) fn conv_value_to_ffi(
    ty_name: &TypeName,
    value_name: &TokenStream,
    references: bool,
    analyzer: &Analyzer,
) -> TokenStream {
    match ty_name {
        TypeName::Types(value) => match value {
            Types::NatBytes | Types::NatString => {
                return conv_value_to_ffi_boxed(ty_name, value_name, references, analyzer)
            }
            _ => {}
        },
        TypeName::TypeArray(_)
        | TypeName::TypeMap(_)
        | TypeName::TypeOption(_)
        | TypeName::TypeResult(_)
        | TypeName::ListTypeName(_)
        | TypeName::StructTypeName(_)
        | TypeName::TypeStream(_)
        | TypeName::InterfaceTypeName(_) => {
            return conv_value_to_ffi_boxed(ty_name, value_name, references, analyzer);
        }
        TypeName::ConstTypeName(value) => {
            let const_ty = analyzer
                .find_ty_const(&value)
                .expect("Could not reference const type");

            match const_ty.const_type {
                ConstTypes::NatString => {
                    return conv_value_to_ffi_boxed(ty_name, value_name, references, analyzer)
                }
                _ => {}
            };
        }
        _ => {}
    }

    conv_value_to_ffi_value(ty_name, value_name, references, analyzer)
}

pub(crate) fn conv_value_to_ffi_value(
    ty_name: &TypeName,
    value_name: &TokenStream,
    references: bool,
    analyzer: &Analyzer,
) -> TokenStream {
    match ty_name {
        TypeName::Types(value) => match value {
            Types::NatInt => quote! { { #value_name } as i64 },
            Types::NatFloat => quote! { { #value_name } as f64 },
            Types::NatString => quote! { { AbiString::from(#value_name) } },
            Types::NatBytes => quote! { { AbiBytes::from(#value_name) } },
            Types::NatBool => quote! { { #value_name } as i64 },
            Types::NatNone => quote! { { 0 } as i64 },
        },
        // TypeName::EnumTypeName(_) => quote! { { #value_name as i64 } },
        TypeName::TypeArray(value) => {
            let array_item = quote! { _array_item };
            let array_to_ffi =
                conv_value_to_ffi_value(&value.ty, &array_item, references, analyzer);

            quote! { {
                let mut _array_value_items = vec![];
                for #array_item in #value_name { _array_value_items.push(#array_to_ffi); }
                let mut _array = _array_value_items.into_boxed_slice();
                let _inn_array = AbiArray {
                    length: _array.len() as i64,
                    data: _array.as_mut_ptr() as *const ::core::ffi::c_void,
                };
                std::mem::forget(_array);
                _inn_array
            } }
        }
        TypeName::TypeMap(value) => {
            let map_data = quote! { _map_data };
            let map_key = quote! { _map_key };

            let map_to_value =
                conv_value_to_ffi_value(&value.map_ty, &map_data, references, analyzer);
            let map_to_key =
                conv_value_to_ffi_value(&value.index_ty, &map_key, references, analyzer);

            quote! { {
                let mut _array_map_values = vec![];
                let mut _array_map_keys = vec![];
                for (#map_key, #map_data) in #value_name {
                    _array_map_values.push(#map_to_value);
                    _array_map_keys.push(#map_to_key);
                }
                let mut _array_map_v = _array_map_values.into_boxed_slice();
                let mut _array_map_k = _array_map_keys.into_boxed_slice();
                let _inn_array_map = AbiMap {
                    length: _array_map_k.len() as i64,
                    data: _array_map_v.as_mut_ptr() as *const ::core::ffi::c_void,
                    key: _array_map_k.as_mut_ptr() as *const ::core::ffi::c_void,
                };
                std::mem::forget(_array_map_k);
                std::mem::forget(_array_map_v);
                _inn_array_map
            } }
        }
        TypeName::TypeOption(value) => {
            let some_data = quote! { _value };
            let to_data = conv_value_to_ffi_boxed(&value.some_ty, &some_data, references, analyzer);
            let to_none = conv_value_to_ffi_boxed(
                &TypeName::Types(Types::NatNone),
                value_name,
                references,
                analyzer,
            );

            quote! { {
                let (_option_value, _index) = match #value_name {
                    Some(#some_data) => (#to_data as *const ::core::ffi::c_void, 0 as i64),
                    None => (#to_none as *const ::core::ffi::c_void, 1 as i64),
                };

                AbiVariant { index: _index, data: _option_value }
            } }
        }
        TypeName::TypeResult(value) => {
            let ok_data = quote! { _ok };
            let err_data = quote! { _err };
            let to_err = conv_value_to_ffi_boxed(&value.err_ty, &err_data, references, analyzer);
            let to_ok = conv_value_to_ffi_boxed(&value.ok_ty, &ok_data, references, analyzer);

            quote! { {
                let (_result_value, _index) = match #value_name {
                    Ok(#ok_data) => (#to_ok as *const ::core::ffi::c_void, 0 as i64),
                    Err(#err_data) => (#to_err as *const ::core::ffi::c_void, 1 as i64),
                };

                AbiVariant { index: _index, data: _result_value }
            } }
        }
        TypeName::TypeStream(_) => quote! { { let _stream_handle = #value_name;
            AbiStream {
                handle: _stream_handle as i64,
            }
        } },
        TypeName::ListTypeName(value)
        | TypeName::StructTypeName(value)
        | TypeName::EnumTypeName(value) => {
            let ident = format_ident!("{}", value);
            if references {
                quote! { { crate::ffi_types::#ident::from(#value_name).into() } }
            } else {
                quote! { { #ident::from(#value_name) } }
            }
        }
        TypeName::InterfaceTypeName(value) => {
            let ident = format_ident!("{}Instance", value);
            quote! { { ffi_impl::#ident::from(#value_name) } }
        }
        TypeName::ConstTypeName(value) => {
            let const_ty = analyzer
                .find_ty_const(&value)
                .expect("Could not reference const type");

            let c_ty = match const_ty.const_type {
                ConstTypes::NatInt => &TypeName::Types(Types::NatInt),
                ConstTypes::NatFloat => &TypeName::Types(Types::NatFloat),
                ConstTypes::NatString => &TypeName::Types(Types::NatString),
            };

            conv_value_to_ffi_value(c_ty, value_name, references, analyzer)
        }
        sw => panic!("Invalid type `{:?}`", sw),
    }
}

// #[no_mangle]
// #[allow(unused_braces)]
// pub extern "C" fn create_test(_result: *mut *const super::idl_ffi_impl::Test) -> i64 {
//     match ::std::panic::catch_unwind(::std::panic::AssertUnwindSafe(move || {
//         let _result_val: Box<dyn super::idl_impl::TestInstance> =
//             Box::new(super::impl_test::Test::new());
//         unsafe {
//             *_result = {
//                 Box::into_raw(Box::new({ super::idl_ffi_impl::Test::from(_result_val) }))
//                     as *const super::idl_ffi_impl::Test
//             };
//         }
//     })) {
//         Ok(_) => AbiInternalError::Ok as i64,
//         Err(_) => AbiInternalError::UndefinedException as i64,
//     }
// }
