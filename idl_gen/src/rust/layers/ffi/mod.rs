use crate::rust::string_pros::StringRustFmt;
use idl::analyzer::Analyzer;
use idl::idl_nodes::*;
use proc_macro2::{self, TokenStream};
use quote::{format_ident, ToTokens, TokenStreamExt};
use std::fmt;

pub(crate) mod client;
pub(crate) mod server;

pub(crate) trait FFITypeName {
    fn get_ffi_ty_ref(&self, references: bool, analyzer: &Analyzer) -> TokenStream;
    fn get_ffi_ty_ref_mut(&self, references: bool, analyzer: &Analyzer) -> TokenStream;
    fn is_ptr_ffi(&self, analyzer: &Analyzer) -> bool;
    fn get_ptr_ffi_ty_ref(&self, references: bool, analyzer: &Analyzer) -> TokenStream;
    fn get_ptr_ffi_ty_ref_mut(&self, references: bool, analyzer: &Analyzer) -> TokenStream;
    fn get_value_ffi_ty_ref(&self, references: bool, analyzer: &Analyzer) -> TokenStream;
    fn conv_ffi_to_value(
        &self,
        ffi_name: &TokenStream,
        references: bool,
        analyzer: &Analyzer,
    ) -> TokenStream;
    fn conv_ffi_ptr_to_value(
        &self,
        ffi_name: &TokenStream,
        references: bool,
        analyzer: &Analyzer,
    ) -> TokenStream;
    fn conv_ffi_value_to_value(
        &self,
        ffi_name: &TokenStream,
        references: bool,
        analyzer: &Analyzer,
    ) -> TokenStream;
    fn conv_value_to_ffi(
        &self,
        value_name: &TokenStream,
        references: bool,
        analyzer: &Analyzer,
    ) -> TokenStream;
    fn conv_value_to_ffi_boxed(
        &self,
        value_name: &TokenStream,
        references: bool,
        analyzer: &Analyzer,
    ) -> TokenStream;
    fn conv_value_to_ffi_value(
        &self,
        value_name: &TokenStream,
        references: bool,
        analyzer: &Analyzer,
    ) -> TokenStream;
    fn dispose_ffi_boxed(
        &self,
        value_name: &TokenStream,
        references: bool,
        analyzer: &Analyzer,
    ) -> TokenStream;
    fn dispose_ffi_ptr(
        &self,
        value_name: &TokenStream,
        references: bool,
        analyzer: &Analyzer,
    ) -> TokenStream;
    fn dispose_ffi(
        &self,
        value_name: &TokenStream,
        references: bool,
        analyzer: &Analyzer,
    ) -> TokenStream;
}

impl FFITypeName for TypeName {
    fn get_ffi_ty_ref(&self, references: bool, analyzer: &Analyzer) -> TokenStream {
        let ident_ty = self.get_value_ffi_ty_ref(references, analyzer);

        match self {
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
                    ConstTypes::NatString | ConstTypes::NatUuid => quote! { *const #ident_ty },
                    ConstTypes::NatInt | ConstTypes::NatFloat => ident_ty,
                }
            }
            TypeName::TypeFunction(_) | TypeName::TypeTuple(_) => {
                panic!("Invalid type `{:?}`", self)
            }
            _ => quote! { *const #ident_ty },
        }
    }

    fn get_ffi_ty_ref_mut(&self, references: bool, analyzer: &Analyzer) -> TokenStream {
        let ident_ty = self.get_value_ffi_ty_ref(references, analyzer);

        match self {
            TypeName::Types(types) => match types {
                Types::NatInt | Types::NatBool | Types::NatNone | Types::NatFloat => ident_ty,
                _ => quote! { *mut #ident_ty },
            },
            TypeName::EnumTypeName(_) => ident_ty,
            TypeName::ConstTypeName(value) => {
                let const_ty = analyzer
                    .find_ty_const(&value)
                    .expect("Could not reference const type");
                match const_ty.const_type {
                    ConstTypes::NatString | ConstTypes::NatUuid => quote! { *mut #ident_ty },
                    ConstTypes::NatInt | ConstTypes::NatFloat => ident_ty,
                }
            }
            TypeName::TypeFunction(_) | TypeName::TypeTuple(_) => {
                panic!("Invalid type `{:?}`", self)
            }
            _ => quote! { *mut #ident_ty },
        }
    }

    // Types that need to be released
    fn is_ptr_ffi(&self, analyzer: &Analyzer) -> bool {
        match self {
            TypeName::Types(types) => match types {
                Types::NatInt | Types::NatFloat | Types::NatBool | Types::NatNone => false,
                Types::NatString | Types::NatBytes | Types::NatUUID => true,
            },
            TypeName::EnumTypeName(_) => false,
            TypeName::ConstTypeName(value) => {
                let const_ty = analyzer
                    .find_ty_const(&value)
                    .expect("Could not reference const type");

                match const_ty.const_type {
                    ConstTypes::NatString | ConstTypes::NatUuid => true,
                    ConstTypes::NatInt | ConstTypes::NatFloat => false,
                }
            }
            _ => true,
        }
    }

    fn get_ptr_ffi_ty_ref(&self, references: bool, analyzer: &Analyzer) -> TokenStream {
        let ident = self.get_value_ffi_ty_ref(references, analyzer);
        quote! { *const #ident }
    }

    fn get_ptr_ffi_ty_ref_mut(&self, references: bool, analyzer: &Analyzer) -> TokenStream {
        let ident = self.get_value_ffi_ty_ref(references, analyzer);
        quote! { *mut #ident }
    }

    fn get_value_ffi_ty_ref(&self, references: bool, analyzer: &Analyzer) -> TokenStream {
        match self {
            TypeName::Types(types) => match types {
                Types::NatInt | Types::NatBool | Types::NatNone => quote! { i64 },
                Types::NatFloat => quote! { f64 },
                Types::NatString => quote! { AbiString },
                Types::NatBytes => quote! { AbiBytes },
                Types::NatUUID => quote! { AbiUuid },
            },
            TypeName::TypeArray(_) => quote! { AbiArray },
            TypeName::TypeMap(_) => quote! { AbiMap },
            TypeName::TypePair(_) => quote! { AbiPair },
            TypeName::TypeStream(_) => quote! { AbiStream },
            TypeName::TypeOption(_) | TypeName::TypeResult(_) => quote! { AbiVariant },
            TypeName::ConstTypeName(value) => {
                let const_ty = analyzer
                    .find_ty_const(&value)
                    .expect("Could not reference const type");

                match const_ty.const_type {
                    ConstTypes::NatUuid => quote! { AbiUuid },
                    ConstTypes::NatString => quote! { AbiString },
                    ConstTypes::NatInt => quote! { i64 },
                    ConstTypes::NatFloat => quote! { f64 },
                }
            }
            TypeName::EnumTypeName(_) => quote! { i64 },
            TypeName::ListTypeName(_) => quote! { AbiVariant },
            TypeName::StructTypeName(value) => {
                let ident = format_ident!("{}", &value);
                if references {
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
    fn conv_ffi_to_value(
        &self,
        ffi_name: &TokenStream,
        references: bool,
        analyzer: &Analyzer,
    ) -> TokenStream {
        match self {
            TypeName::Types(value) => match value {
                Types::NatString | Types::NatBytes | Types::NatUUID => {
                    return self.conv_ffi_ptr_to_value(ffi_name, references, analyzer)
                }
                _ => {}
            },
            TypeName::TypeArray(_)
            | TypeName::TypeMap(_)
            | TypeName::ListTypeName(_)
            | TypeName::StructTypeName(_)
            | TypeName::TypeStream(_)
            | TypeName::TypeOption(_)
            | TypeName::TypeResult(_)
            | TypeName::TypePair(_) => {
                return self.conv_ffi_ptr_to_value(ffi_name, references, analyzer)
            }
            TypeName::ConstTypeName(value) => {
                let const_ty = analyzer
                    .find_ty_const(&value)
                    .expect("Could not reference const type");

                match const_ty.const_type {
                    ConstTypes::NatUuid | ConstTypes::NatString => {
                        return self.conv_ffi_ptr_to_value(ffi_name, references, analyzer)
                    }
                    _ => {}
                };
            }
            TypeName::TypeFunction(_)
            | TypeName::TypeTuple(_)
            | TypeName::EnumTypeName(_)
            | TypeName::InterfaceTypeName(_) => {}
        }

        self.conv_ffi_value_to_value(ffi_name, references, analyzer)
    }

    fn conv_ffi_ptr_to_value(
        &self,
        ffi_name: &TokenStream,
        references: bool,
        analyzer: &Analyzer,
    ) -> TokenStream {
        let c_ffi = self.get_ptr_ffi_ty_ref(references, analyzer);
        self.conv_ffi_value_to_value(
            &(quote! { unsafe { (#ffi_name as #c_ffi).read() } }),
            references,
            analyzer,
        )
    }

    fn conv_ffi_value_to_value(
        &self,
        ffi_name: &TokenStream,
        references: bool,
        analyzer: &Analyzer,
    ) -> TokenStream {
        match self {
            TypeName::Types(value) => match value {
                Types::NatInt => quote! { { #ffi_name } as i64 },
                Types::NatFloat => quote! { { #ffi_name } as f64 },
                Types::NatString => quote! { { #ffi_name.to_string() } },
                Types::NatBytes => quote! { { #ffi_name.to_vec() } },
                Types::NatUUID => quote! { { #ffi_name.to_uuid() } },
                Types::NatBool => quote! { { #ffi_name } as i64 == 1 },
                Types::NatNone => quote! { { assert!({ #ffi_name } as i64 == 0); () } },
            },
            TypeName::TypeArray(value) => {
                let array_item = quote! { _array_item };
                let array_to_value =
                    value
                        .ty
                        .conv_ffi_value_to_value(&array_item, references, analyzer);
                let array_ty_ident = value.ty.get_value_ffi_ty_ref(references, analyzer);

                quote! { {
                    let _array = #ffi_name;
                    let _data = _array.data as *const #array_ty_ident;
                    let _length = _array.length as usize;
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
                let value_ty_ident = value.map_ty.get_value_ffi_ty_ref(references, analyzer);
                let key_ty_ident = value.index_ty.get_value_ffi_ty_ref(references, analyzer);
                let map_to_value = value
                    .map_ty
                    .conv_ffi_value_to_value(&map_data, references, analyzer);
                let map_to_key = value
                    .index_ty
                    .conv_ffi_value_to_value(&map_key, references, analyzer);

                quote! { {
                    let _map = #ffi_name;
                    let _length = _map.length as usize;
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
            TypeName::TypePair(value) => {
                let first_value = quote! { _first_value };
                let second_value = quote! { _second_value };
                let first_ty_ident = value.first_ty.get_value_ffi_ty_ref(references, analyzer);
                let second_ty_ident = value.second_ty.get_value_ffi_ty_ref(references, analyzer);
                let con_first_name =
                    value
                        .first_ty
                        .conv_ffi_ptr_to_value(&first_value, references, analyzer);
                let con_second_name =
                    value
                        .second_ty
                        .conv_ffi_ptr_to_value(&second_value, references, analyzer);
                quote! { {
                    let _pair = #ffi_name;
                    let #first_value = _pair.first_data as *const #first_ty_ident;
                    let #second_value = _pair.second_data as *const #second_ty_ident;
                    (#con_first_name, #con_second_name)
                } }
            }
            TypeName::TypeOption(value) => {
                let some_value = quote! { _some_value };
                let con_name =
                    value
                        .some_ty
                        .conv_ffi_ptr_to_value(&some_value, references, analyzer);
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
                let con_ok_name = value
                    .ok_ty
                    .conv_ffi_ptr_to_value(&ok_value, references, analyzer);
                let con_err_name = value
                    .err_ty
                    .conv_ffi_ptr_to_value(&err_value, references, analyzer);
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
            TypeName::StructTypeName(value)
            | TypeName::ListTypeName(value)
            | TypeName::EnumTypeName(value) => {
                let ident = format_ident!("{}", &value);
                if references {
                    quote! { { idl_types::#ident::from(crate::ffi_types::#ident::from(#ffi_name)) } }
                } else {
                    quote! { { idl_types::#ident::from(#ident::from(#ffi_name)) } }
                }
            }
            TypeName::ConstTypeName(value) => {
                let const_ty = analyzer
                    .find_ty_const(&value)
                    .expect("Could not reference const type");

                let c_ty = match const_ty.const_type {
                    ConstTypes::NatInt => &TypeName::Types(Types::NatInt),
                    ConstTypes::NatFloat => &TypeName::Types(Types::NatFloat),
                    ConstTypes::NatString => &TypeName::Types(Types::NatString),
                    ConstTypes::NatUuid => &TypeName::Types(Types::NatUUID),
                };

                let ident = format_ident!("{}", &value);
                let conv_v = c_ty.conv_ffi_value_to_value(ffi_name, references, analyzer);
                quote! { { idl_types::#ident(#conv_v) } }
            }
            TypeName::TypeFunction(_) | TypeName::TypeTuple(_) | TypeName::InterfaceTypeName(_) => {
                panic!("Invalid type")
            }
        }
    }

    fn conv_value_to_ffi_boxed(
        &self,
        value_name: &TokenStream,
        references: bool,
        analyzer: &Analyzer,
    ) -> TokenStream {
        let b_value = self.conv_value_to_ffi_value(value_name, references, analyzer);
        let ref_ffi = self.get_ptr_ffi_ty_ref(references, analyzer);
        quote! { { Box::into_raw(Box::new(#b_value)) as #ref_ffi  } }
    }

    // Values types are kept unchanged, meanwhile struct are returned as a ptr
    fn conv_value_to_ffi(
        &self,
        value_name: &TokenStream,
        references: bool,
        analyzer: &Analyzer,
    ) -> TokenStream {
        match self {
            TypeName::Types(value) => match value {
                Types::NatBytes | Types::NatString | Types::NatUUID => {
                    return self.conv_value_to_ffi_boxed(value_name, references, analyzer)
                }
                _ => {}
            },
            TypeName::TypeArray(_)
            | TypeName::TypeMap(_)
            | TypeName::TypePair(_)
            | TypeName::TypeOption(_)
            | TypeName::TypeResult(_)
            | TypeName::ListTypeName(_)
            | TypeName::StructTypeName(_)
            | TypeName::TypeStream(_)
            | TypeName::InterfaceTypeName(_) => {
                return self.conv_value_to_ffi_boxed(value_name, references, analyzer);
            }
            TypeName::ConstTypeName(value) => {
                let const_ty = analyzer
                    .find_ty_const(&value)
                    .expect("Could not reference const type");

                match const_ty.const_type {
                    ConstTypes::NatUuid | ConstTypes::NatString => {
                        return self.conv_value_to_ffi_boxed(value_name, references, analyzer)
                    }
                    _ => {}
                };
            }
            _ => {}
        }

        self.conv_value_to_ffi_value(value_name, references, analyzer)
    }

    fn conv_value_to_ffi_value(
        &self,
        value_name: &TokenStream,
        references: bool,
        analyzer: &Analyzer,
    ) -> TokenStream {
        match self {
            TypeName::Types(value) => match value {
                Types::NatInt => quote! { { #value_name } as i64 },
                Types::NatFloat => quote! { { #value_name } as f64 }, // TODO
                Types::NatString => quote! { { AbiString::from(#value_name) } },
                Types::NatBytes => quote! { { AbiBytes::from(#value_name) } },
                Types::NatUUID => quote! { { AbiUuid::from(#value_name) } },
                Types::NatBool => quote! { { #value_name } as i64 },
                Types::NatNone => quote! { { 0 } as i64 },
            },
            TypeName::TypeArray(value) => {
                let array_item = quote! { _array_item };
                let array_to_ffi =
                    value
                        .ty
                        .conv_value_to_ffi_value(&array_item, references, analyzer);
                let array_ty_ident = value.ty.get_value_ffi_ty_ref(references, analyzer);

                quote! { {
                    let mut _array_value_items = Vec::<#array_ty_ident>::new();
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

                let map_to_value = value
                    .map_ty
                    .conv_value_to_ffi_value(&map_data, references, analyzer);
                let map_to_key = value
                    .index_ty
                    .conv_value_to_ffi_value(&map_key, references, analyzer);
                let value_ty_ident = value.map_ty.get_value_ffi_ty_ref(references, analyzer);
                let key_ty_ident = value.index_ty.get_value_ffi_ty_ref(references, analyzer);

                quote! { {
                    let mut _array_map_values = Vec::<#value_ty_ident>::new();
                    let mut _array_map_keys = Vec::<#key_ty_ident>::new();
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
            TypeName::TypePair(value) => {
                let first_value = quote! { #value_name.0 };
                let second_value = quote! { #value_name.1 };
                let to_first =
                    value
                        .first_ty
                        .conv_value_to_ffi_boxed(&first_value, references, analyzer);
                let to_second =
                    value
                        .second_ty
                        .conv_value_to_ffi_boxed(&second_value, references, analyzer);

                quote! { {
                    let _first_data = #to_first as *const ::core::ffi::c_void;
                    let _second_data = #to_second as *const ::core::ffi::c_void;
                    AbiPair { first_data: _first_data, second_data: _second_data }
                } }
            }
            TypeName::TypeOption(value) => {
                let some_data = quote! { _value };
                let to_data = value
                    .some_ty
                    .conv_value_to_ffi_boxed(&some_data, references, analyzer);
                let to_none = TypeName::Types(Types::NatNone)
                    .conv_value_to_ffi_boxed(value_name, references, analyzer);

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
                let to_err = value
                    .err_ty
                    .conv_value_to_ffi_boxed(&err_data, references, analyzer);
                let to_ok = value
                    .ok_ty
                    .conv_value_to_ffi_boxed(&ok_data, references, analyzer);

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
            TypeName::StructTypeName(value)
            | TypeName::ListTypeName(value)
            | TypeName::EnumTypeName(value) => {
                let ident = format_ident!("{}", value);
                if references {
                    quote! { { crate::ffi_types::#ident::from(#value_name).into() } }
                } else {
                    quote! { { #ident::from(#value_name).into() } }
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
                    ConstTypes::NatUuid => &TypeName::Types(Types::NatUUID),
                };

                let value_name = quote! { #value_name.0 };
                c_ty.conv_value_to_ffi_value(&value_name, references, analyzer)
            }
            sw => panic!("Invalid type `{:?}`", sw),
        }
    }

    fn dispose_ffi_boxed(
        &self,
        value_name: &TokenStream,
        references: bool,
        analyzer: &Analyzer,
    ) -> TokenStream {
        let dispose_name = quote! { _value_disps };
        let ref_ffi = self.get_value_ffi_ty_ref(references, analyzer);
        let dispose_body = self.dispose_ffi(&dispose_name, true, analyzer);
        quote! { {
            let mut #dispose_name: Box<#ref_ffi> = unsafe { Box::from_raw(#value_name) };
            #dispose_body
        } }
    }

    fn dispose_ffi_ptr(
        &self,
        value_name: &TokenStream,
        references: bool,
        analyzer: &Analyzer,
    ) -> TokenStream {
        if self.is_ptr_ffi(analyzer) {
            let dispose_name = quote! { _value_disps };
            let ref_ffi = self.get_value_ffi_ty_ref(references, analyzer);
            let dispose_body = self.dispose_ffi(&dispose_name, true, analyzer);
            quote! { {
                let mut #dispose_name: Box<#ref_ffi> = unsafe { Box::from_raw(#value_name) };
                #dispose_body
            } }
        } else {
            quote! {}
        }
    }

    fn dispose_ffi(
        &self,
        value_name: &TokenStream,
        references: bool,
        analyzer: &Analyzer,
    ) -> TokenStream {
        match self {
            TypeName::Types(value) => match value {
                Types::NatBytes | Types::NatUUID | Types::NatString => {
                    quote! { #value_name.dispose(); }
                }
                // In case there's a new one, matches all types
                Types::NatInt | Types::NatFloat | Types::NatBool | Types::NatNone => quote! {},
            },
            TypeName::TypeArray(value) => {
                let array_ty = value.ty.get_ffi_ty_ref_mut(references, analyzer);
                let array_data = quote! { #value_name.data as #array_ty };
                let array_length = quote! { #value_name.length as usize };
                let data_value = quote! { _value };
                let array_data_ty = value.ty.get_value_ffi_ty_ref(references, analyzer);
                let dispose_data = value.ty.dispose_ffi(&data_value, references, analyzer);

                let dispose_body = if value.ty.is_ptr_ffi(analyzer) {
                    quote! {
                        for #data_value in _sl_array.iter_mut() {
                            #dispose_data
                        }
                    }
                } else {
                    quote! {}
                };

                quote! { {
                    let mut _sl_array: Box<[#array_data_ty]> = unsafe {
                        Box::from_raw(std::slice::from_raw_parts_mut(#array_data, #array_length))
                    };
                    #dispose_body
                } }
            }
            TypeName::TypeMap(value) => {
                let map_length = quote! { #value_name.length as usize };

                let map_data_ty = value.map_ty.get_ffi_ty_ref_mut(references, analyzer);
                let map_data_data = quote! { #value_name.data as #map_data_ty };

                let map_key_ty = value.index_ty.get_ffi_ty_ref_mut(references, analyzer);
                let map_key_data = quote! { #value_name.key as #map_key_ty };

                let data_value = quote! { _value_data };
                let dispose_data = value.map_ty.dispose_ffi(&data_value, references, analyzer);

                let key_value = quote! { _value_key };
                let dispose_key = value.index_ty.dispose_ffi(&key_value, references, analyzer);

                let dispose_data_body = if value.map_ty.is_ptr_ffi(analyzer) {
                    quote! {
                        for #data_value in _sl_data.iter_mut() {
                            #dispose_data
                        }
                    }
                } else {
                    quote! {}
                };

                let dispose_key_body = if value.index_ty.is_ptr_ffi(analyzer) {
                    quote! {
                        for #key_value in _sl_key.iter_mut() {
                            #dispose_key
                        }
                    }
                } else {
                    quote! {}
                };

                let map_data_ty = value.map_ty.get_value_ffi_ty_ref(references, analyzer);
                let key_data_ty = value.index_ty.get_value_ffi_ty_ref(references, analyzer);

                quote! { {
                    let mut _sl_data: Box<[#map_data_ty]> = unsafe {
                        Box::from_raw(std::slice::from_raw_parts_mut(#map_data_data, #map_length))
                    };
                    let mut _sl_key: Box<[#key_data_ty]> = unsafe {
                        Box::from_raw(std::slice::from_raw_parts_mut(#map_key_data, #map_length))
                    };
                    #dispose_data_body
                    #dispose_key_body
                } }
            }
            TypeName::TypePair(value) => {
                let first_ty = value.first_ty.get_ptr_ffi_ty_ref_mut(references, analyzer);
                let second_ty = value.second_ty.get_ptr_ffi_ty_ref_mut(references, analyzer);
                let first_data = quote! { #value_name.first_data as #first_ty };
                let second_data = quote! { #value_name.second_data as #second_ty };
                let dispose_first =
                    value
                        .first_ty
                        .dispose_ffi_boxed(&first_data, references, analyzer);
                let dispose_second =
                    value
                        .second_ty
                        .dispose_ffi_boxed(&second_data, references, analyzer);

                quote! { {
                    #dispose_first
                    #dispose_second
                } }
            }
            TypeName::TypeOption(value) => {
                let some_ty = value.some_ty.get_ptr_ffi_ty_ref_mut(references, analyzer);
                let none_ty =
                    TypeName::Types(Types::NatNone).get_ptr_ffi_ty_ref_mut(references, analyzer);
                let some_data = quote! { #value_name.data as #some_ty };
                let none_data = quote! { #value_name.data as #none_ty };
                let dispose_some = value
                    .some_ty
                    .dispose_ffi_boxed(&some_data, references, analyzer);
                let dispose_none = TypeName::Types(Types::NatNone)
                    .dispose_ffi_boxed(&none_data, references, analyzer);

                quote! { {
                    match #value_name.index {
                        0 => { #dispose_some },
                        1 => { #dispose_none },
                        _value => panic!("Invaild option variant `{}`", _value),
                    }
                } }
            }
            TypeName::TypeResult(value) => {
                let err_ty = value.err_ty.get_ptr_ffi_ty_ref_mut(references, analyzer);
                let ok_ty = value.ok_ty.get_ptr_ffi_ty_ref_mut(references, analyzer);
                let err_data = quote! { #value_name.data as #err_ty };
                let ok_data = quote! { #value_name.data as #ok_ty };
                let dispose_err = value
                    .err_ty
                    .dispose_ffi_boxed(&err_data, references, analyzer);
                let dispose_ok = value
                    .ok_ty
                    .dispose_ffi_boxed(&ok_data, references, analyzer);

                quote! { {
                    match #value_name.index {
                        0 => { #dispose_ok },
                        1 => { #dispose_err },
                        _value => panic!("Invaild result variant `{}`", _value),
                    }
                } }
            }
            TypeName::StructTypeName(_) => quote! { { #value_name.dispose(); } },
            TypeName::ListTypeName(value) => {
                let ident = format_ident!("{}", value);
                if references {
                    quote! { { crate::ffi_types::#ident::dispose(#value_name.index, #value_name.data); } }
                } else {
                    quote! { { #ident::dispose(#value_name.index, #value_name.data); } }
                }
            }
            TypeName::ConstTypeName(value) => {
                let const_ty = analyzer
                    .find_ty_const(&value)
                    .expect("Could not reference const type");

                match const_ty.const_type {
                    ConstTypes::NatFloat | ConstTypes::NatInt => quote! {},
                    ConstTypes::NatUuid | ConstTypes::NatString => {
                        quote! { #value_name.dispose(); }
                    }
                }
            }
            sw => panic!("Invalid type `{:?}`", sw),
        }
    }
}
