use crate::string_pros::StringPros;
use idl::analyzer::Analyzer;
use idl::idl_types::*;
use proc_macro2::{self, Ident, Span, TokenStream};
use std::fmt;

pub mod client;
pub mod ffi_types;
pub mod server;

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
           mod idl_types; // rust types
           mod idl_ffi; // interface and static functions
           mod idl_ffi_types; // ffi types
           mod idl_impl; // rust interface type
           mod idl_ffi_impl; // ffi interface type
           mod ffi_types; // base types
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
        TypeName::TypeMap(_) => quote! { AbiArray },
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
        TypeName::StructTypeName(value)
        | TypeName::ListTypeName(value)
        | TypeName::EnumTypeName(value) => {
            let ident = Ident::new(&value, Span::call_site());
            if references {
                // TODO ?? Would it be better?
                quote! { super::idl_ffi_types::#ident }
            } else {
                quote! { #ident }
            }
        }
        TypeName::InterfaceTypeName(value) => {
            let ident = Ident::new(&value, Span::call_site());
            quote! { super::idl_ffi_impl::#ident }
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
            Types::NatBool => quote! { { #ffi_name } as bool },
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
            let map_to_value =
                conv_ffi_ptr_to_value(&value.map_ty, &map_data, references, analyzer);
            let map_to_key =
                conv_ffi_ptr_to_value(&value.index_ty, &map_key, references, analyzer);

            quote! { {
                let _array = #ffi_name;
                let _data = _array.data as *const AbiMap;
                let _length = _array.length as isize;
                let mut _map = ::std::collections::HashMap::new();
                for _index in 0.._length {
                    let _map_item = unsafe { _data.offset(_index).read() };
                    let #map_data = _map_item.data;
                    let #map_key = _map_item.key;
                    let _key = #map_to_key;
                    let _value = #map_to_value;
                    if let Some(_) = _map.insert(_key, _value) { panic!() }
                }
                _map
            } }
        }

        TypeName::TypeOption(value) => {
            let some_value = quote! { _some_value };
            let con_name =
                conv_ffi_ptr_to_value(&value.some_ty, &some_value, references, analyzer);
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
            let con_ok_name =
                conv_ffi_ptr_to_value(&value.ok_ty, &ok_value, references, analyzer);
            let con_err_name =
                conv_ffi_ptr_to_value(&value.err_ty, &err_value, references, analyzer);
            quote! { {
                let _result = #ffi_name;
                match _result.index {
                    0 => { let #ok_value = _result.data; Ok(#con_ok_name) }
                    1 => { let #err_value = _result.data; Err(#con_err_name) }
                }
            } }
        }
        TypeName::TypeStream(_) => {
            quote! { {
                let _result = #ffi_name;
                _result.handle as i64
            } }
        }
        TypeName::ListTypeName(value)
        | TypeName::StructTypeName(value)
        | TypeName::EnumTypeName(value) => {
            let ident = Ident::new(&value, Span::call_site());
            quote! { { super::idl_types::#ident::from(#ffi_name) } }
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
                conv_value_to_ffi_boxed(&value.map_ty, &map_data, references, analyzer);
            let map_to_key =
                conv_value_to_ffi_boxed(&value.index_ty, &map_key, references, analyzer);

            quote! { {
                let mut _array_map_value = vec![];
                for (#map_key, #map_data) in #value_name {
                    _array_map_value.push(AbiMap {
                        key: #map_to_key as *const ::core::ffi::c_void,
                        data: #map_to_value as *const ::core::ffi::c_void,
                    });
                }
                let mut _array_map = _array_map_value.into_boxed_slice();
                let _inn_array_map = AbiArray {
                    length: _array_map.len() as i64,
                    data: _array_map.as_mut_ptr() as *const ::core::ffi::c_void,
                };
                std::mem::forget(_array_map);
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
            let ident = Ident::new(value, Span::call_site());
            if references {
                quote! { { super::idl_ffi_types::#ident::from(#value_name) } }
            } else {
                quote! { { #ident::from(#value_name) } }
            }
        }
        TypeName::InterfaceTypeName(value) => {
            let ident = Ident::new(value, Span::call_site());
            quote! { { super::idl_ffi_impl::#ident::from(#value_name) } }
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
