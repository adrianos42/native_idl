use super::ffi_impl;
use super::ffi_types;
use idl_internal::{ffi::ffi_types::*, StreamSender, Uuid};
use idl_test::idl_types;
#[no_mangle]
#[allow(unused_braces)]
pub extern "C" fn idl_test_method_test_test_int(
    _instance: *mut ffi_impl::TestInstance,
    value: i64,
    _result: *mut i64,
) -> i64 {
    match ::std::panic::catch_unwind(::std::panic::AssertUnwindSafe(move || {
        let mut _ins = unsafe { Box::from_raw(_instance) };
        let mut _instance = &mut _ins.instance;
        let _value_arg_val = { value } as i64;
        let _result_val: i64 = _instance.test_int(_value_arg_val);
        unsafe {
            *_result = { _result_val } as i64;
        }
        std::mem::forget(_ins);
        return AbiInternalError::Ok;
    })) {
        Ok(value) => value as i64,
        Err(_) => AbiInternalError::UndefinedException as i64,
    }
}
#[no_mangle]
#[allow(unused_braces)]
pub extern "C" fn idl_test_method_test_test_bool(
    _instance: *mut ffi_impl::TestInstance,
    value: i64,
    _result: *mut i64,
) -> i64 {
    match ::std::panic::catch_unwind(::std::panic::AssertUnwindSafe(move || {
        let mut _ins = unsafe { Box::from_raw(_instance) };
        let mut _instance = &mut _ins.instance;
        let _value_arg_val = { value } as i64 == 1;
        let _result_val: bool = _instance.test_bool(_value_arg_val);
        unsafe {
            *_result = { _result_val } as i64;
        }
        std::mem::forget(_ins);
        return AbiInternalError::Ok;
    })) {
        Ok(value) => value as i64,
        Err(_) => AbiInternalError::UndefinedException as i64,
    }
}
#[no_mangle]
#[allow(unused_braces)]
pub extern "C" fn idl_test_method_test_test_float(
    _instance: *mut ffi_impl::TestInstance,
    value: f64,
    _result: *mut f64,
) -> i64 {
    match ::std::panic::catch_unwind(::std::panic::AssertUnwindSafe(move || {
        let mut _ins = unsafe { Box::from_raw(_instance) };
        let mut _instance = &mut _ins.instance;
        let _value_arg_val = { value } as f64;
        let _result_val: f64 = _instance.test_float(_value_arg_val);
        unsafe {
            *_result = { _result_val } as f64;
        }
        std::mem::forget(_ins);
        return AbiInternalError::Ok;
    })) {
        Ok(value) => value as i64,
        Err(_) => AbiInternalError::UndefinedException as i64,
    }
}
#[no_mangle]
#[allow(unused_braces)]
pub extern "C" fn idl_test_method_test_test_string(
    _instance: *mut ffi_impl::TestInstance,
    value: *const AbiString,
    _result: *mut *const AbiString,
) -> i64 {
    match ::std::panic::catch_unwind(::std::panic::AssertUnwindSafe(move || {
        let mut _ins = unsafe { Box::from_raw(_instance) };
        let mut _instance = &mut _ins.instance;
        let _value_arg_val = { unsafe { (value as *const AbiString).read() }.to_string() };
        let _result_val: String = _instance.test_string(_value_arg_val);
        unsafe {
            *_result =
                { Box::into_raw(Box::new({ AbiString::from(_result_val) })) as *const AbiString };
        }
        std::mem::forget(_ins);
        return AbiInternalError::Ok;
    })) {
        Ok(value) => value as i64,
        Err(_) => AbiInternalError::UndefinedException as i64,
    }
}
#[no_mangle]
#[allow(unused_braces)]
pub extern "C" fn idl_test_dispose_test_test_string(
    _instance: *mut ffi_impl::TestInstance,
    _result_disp: *mut AbiString,
) -> i64 {
    match ::std::panic::catch_unwind(::std::panic::AssertUnwindSafe(move || {
        {
            let mut _value_disps: Box<AbiString> = unsafe { Box::from_raw(_result_disp) };
            _value_disps.dispose();
        }
        return AbiInternalError::Ok;
    })) {
        Ok(value) => value as i64,
        Err(_) => AbiInternalError::UndefinedException as i64,
    }
}
#[no_mangle]
#[allow(unused_braces)]
pub extern "C" fn idl_test_method_test_test_none(
    _instance: *mut ffi_impl::TestInstance,
    value: i64,
) -> i64 {
    match ::std::panic::catch_unwind(::std::panic::AssertUnwindSafe(move || {
        let mut _ins = unsafe { Box::from_raw(_instance) };
        let mut _instance = &mut _ins.instance;
        let _value_arg_val = {
            assert!({ value } as i64 == 0);
            ()
        };
        _instance.test_none(_value_arg_val);
        std::mem::forget(_ins);
        return AbiInternalError::Ok;
    })) {
        Ok(value) => value as i64,
        Err(_) => AbiInternalError::UndefinedException as i64,
    }
}
#[no_mangle]
#[allow(unused_braces)]
pub extern "C" fn idl_test_method_test_test_bytes(
    _instance: *mut ffi_impl::TestInstance,
    value: *const AbiBytes,
    _result: *mut *const AbiBytes,
) -> i64 {
    match ::std::panic::catch_unwind(::std::panic::AssertUnwindSafe(move || {
        let mut _ins = unsafe { Box::from_raw(_instance) };
        let mut _instance = &mut _ins.instance;
        let _value_arg_val = { unsafe { (value as *const AbiBytes).read() }.to_vec() };
        let _result_val: Vec<u8> = _instance.test_bytes(_value_arg_val);
        unsafe {
            *_result =
                { Box::into_raw(Box::new({ AbiBytes::from(_result_val) })) as *const AbiBytes };
        }
        std::mem::forget(_ins);
        return AbiInternalError::Ok;
    })) {
        Ok(value) => value as i64,
        Err(_) => AbiInternalError::UndefinedException as i64,
    }
}
#[no_mangle]
#[allow(unused_braces)]
pub extern "C" fn idl_test_dispose_test_test_bytes(
    _instance: *mut ffi_impl::TestInstance,
    _result_disp: *mut AbiBytes,
) -> i64 {
    match ::std::panic::catch_unwind(::std::panic::AssertUnwindSafe(move || {
        {
            let mut _value_disps: Box<AbiBytes> = unsafe { Box::from_raw(_result_disp) };
            _value_disps.dispose();
        }
        return AbiInternalError::Ok;
    })) {
        Ok(value) => value as i64,
        Err(_) => AbiInternalError::UndefinedException as i64,
    }
}
#[no_mangle]
#[allow(unused_braces)]
pub extern "C" fn idl_test_method_test_test_uuid(
    _instance: *mut ffi_impl::TestInstance,
    value: *const AbiUuid,
    _result: *mut *const AbiUuid,
) -> i64 {
    match ::std::panic::catch_unwind(::std::panic::AssertUnwindSafe(move || {
        let mut _ins = unsafe { Box::from_raw(_instance) };
        let mut _instance = &mut _ins.instance;
        let _value_arg_val = { unsafe { (value as *const AbiUuid).read() }.to_uuid() };
        let _result_val: Uuid = _instance.test_uuid(_value_arg_val);
        unsafe {
            *_result =
                { Box::into_raw(Box::new({ AbiUuid::from(_result_val) })) as *const AbiUuid };
        }
        std::mem::forget(_ins);
        return AbiInternalError::Ok;
    })) {
        Ok(value) => value as i64,
        Err(_) => AbiInternalError::UndefinedException as i64,
    }
}
#[no_mangle]
#[allow(unused_braces)]
pub extern "C" fn idl_test_dispose_test_test_uuid(
    _instance: *mut ffi_impl::TestInstance,
    _result_disp: *mut AbiUuid,
) -> i64 {
    match ::std::panic::catch_unwind(::std::panic::AssertUnwindSafe(move || {
        {
            let mut _value_disps: Box<AbiUuid> = unsafe { Box::from_raw(_result_disp) };
            _value_disps.dispose();
        }
        return AbiInternalError::Ok;
    })) {
        Ok(value) => value as i64,
        Err(_) => AbiInternalError::UndefinedException as i64,
    }
}
#[no_mangle]
#[allow(unused_braces)]
pub extern "C" fn idl_test_method_test_test_int_array(
    _instance: *mut ffi_impl::TestInstance,
    value: *const AbiArray,
    _result: *mut *const AbiArray,
) -> i64 {
    match ::std::panic::catch_unwind(::std::panic::AssertUnwindSafe(move || {
        let mut _ins = unsafe { Box::from_raw(_instance) };
        let mut _instance = &mut _ins.instance;
        let _value_arg_val = {
            let _array = unsafe { (value as *const AbiArray).read() };
            let _data = _array.data as *const i64;
            let _length = _array.length as isize;
            let mut _array_vec = vec![];
            for _index in 0.._length {
                let _array_item = unsafe { _data.offset(_index).read() };
                _array_vec.push({ _array_item } as i64);
            }
            _array_vec
        };
        let _result_val: Vec<i64> = _instance.test_int_array(_value_arg_val);
        unsafe {
            *_result = {
                Box::into_raw(Box::new({
                    let mut _array_value_items = Vec::<i64>::new();
                    for _array_item in _result_val {
                        _array_value_items.push({ _array_item } as i64);
                    }
                    let mut _array = _array_value_items.into_boxed_slice();
                    let _inn_array = AbiArray {
                        length: _array.len() as i64,
                        data: _array.as_mut_ptr() as *const ::core::ffi::c_void,
                    };
                    std::mem::forget(_array);
                    _inn_array
                })) as *const AbiArray
            };
        }
        std::mem::forget(_ins);
        return AbiInternalError::Ok;
    })) {
        Ok(value) => value as i64,
        Err(_) => AbiInternalError::UndefinedException as i64,
    }
}
#[no_mangle]
#[allow(unused_braces)]
pub extern "C" fn idl_test_dispose_test_test_int_array(
    _instance: *mut ffi_impl::TestInstance,
    _result_disp: *mut AbiArray,
) -> i64 {
    match ::std::panic::catch_unwind(::std::panic::AssertUnwindSafe(move || {
        {
            let mut _value_disps: Box<AbiArray> = unsafe { Box::from_raw(_result_disp) };
            {
                let mut _sl_array: Box<[i64]> = unsafe {
                    Box::from_raw(std::slice::from_raw_parts_mut(
                        _value_disps.data as *mut i64,
                        _value_disps.length as usize,
                    ))
                };
            }
        }
        return AbiInternalError::Ok;
    })) {
        Ok(value) => value as i64,
        Err(_) => AbiInternalError::UndefinedException as i64,
    }
}
#[no_mangle]
#[allow(unused_braces)]
pub extern "C" fn idl_test_method_test_test_point_struct(
    _instance: *mut ffi_impl::TestInstance,
    value: *const crate::ffi_types::Point,
    _result: *mut *const crate::ffi_types::Point,
) -> i64 {
    match ::std::panic::catch_unwind(::std::panic::AssertUnwindSafe(move || {
        let mut _ins = unsafe { Box::from_raw(_instance) };
        let mut _instance = &mut _ins.instance;
        let _value_arg_val = {
            idl_types::Point::from(crate::ffi_types::Point::from(unsafe {
                (value as *const crate::ffi_types::Point).read()
            }))
        };
        let _result_val: idl_types::Point = _instance.test_point_struct(_value_arg_val);
        unsafe {
            *_result = {
                Box::into_raw(Box::new({
                    crate::ffi_types::Point::from(_result_val).into()
                })) as *const crate::ffi_types::Point
            };
        }
        std::mem::forget(_ins);
        return AbiInternalError::Ok;
    })) {
        Ok(value) => value as i64,
        Err(_) => AbiInternalError::UndefinedException as i64,
    }
}
#[no_mangle]
#[allow(unused_braces)]
pub extern "C" fn idl_test_dispose_test_test_point_struct(
    _instance: *mut ffi_impl::TestInstance,
    _result_disp: *mut crate::ffi_types::Point,
) -> i64 {
    match ::std::panic::catch_unwind(::std::panic::AssertUnwindSafe(move || {
        {
            let mut _value_disps: Box<crate::ffi_types::Point> =
                unsafe { Box::from_raw(_result_disp) };
            {
                crate::ffi_types::Point::dispose(_value_disps)
            }
        }
        return AbiInternalError::Ok;
    })) {
        Ok(value) => value as i64,
        Err(_) => AbiInternalError::UndefinedException as i64,
    }
}
#[no_mangle]
#[allow(unused_braces)]
pub extern "C" fn idl_test_method_test_test_int_map(
    _instance: *mut ffi_impl::TestInstance,
    value: *const AbiMap,
    _result: *mut *const AbiMap,
) -> i64 {
    match ::std::panic::catch_unwind(::std::panic::AssertUnwindSafe(move || {
        let mut _ins = unsafe { Box::from_raw(_instance) };
        let mut _instance = &mut _ins.instance;
        let _value_arg_val = {
            let _map = unsafe { (value as *const AbiMap).read() };
            let _length = _map.length as isize;
            let _data = _map.data as *const AbiString;
            let _key = _map.key as *const i64;
            let mut _map_result = ::std::collections::HashMap::new();
            for _index in 0.._length {
                let _map_data = unsafe { _data.offset(_index).read() };
                let _map_key = unsafe { _key.offset(_index).read() };
                if let Some(_) = _map_result.insert({ _map_key } as i64, { _map_data.to_string() })
                {
                    panic!()
                }
            }
            _map_result
        };
        let _result_val: ::std::collections::HashMap<i64, String> =
            _instance.test_int_map(_value_arg_val);
        unsafe {
            *_result = {
                Box::into_raw(Box::new({
                    let mut _array_map_values = Vec::<AbiString>::new();
                    let mut _array_map_keys = Vec::<i64>::new();
                    for (_map_key, _map_data) in _result_val {
                        _array_map_values.push({ AbiString::from(_map_data) });
                        _array_map_keys.push({ _map_key } as i64);
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
                })) as *const AbiMap
            };
        }
        std::mem::forget(_ins);
        return AbiInternalError::Ok;
    })) {
        Ok(value) => value as i64,
        Err(_) => AbiInternalError::UndefinedException as i64,
    }
}
#[no_mangle]
#[allow(unused_braces)]
pub extern "C" fn idl_test_dispose_test_test_int_map(
    _instance: *mut ffi_impl::TestInstance,
    _result_disp: *mut AbiMap,
) -> i64 {
    match ::std::panic::catch_unwind(::std::panic::AssertUnwindSafe(move || {
        {
            let mut _value_disps: Box<AbiMap> = unsafe { Box::from_raw(_result_disp) };
            {
                let mut _sl_data: Box<[AbiString]> = unsafe {
                    Box::from_raw(std::slice::from_raw_parts_mut(
                        _value_disps.data as *mut AbiString,
                        _value_disps.length as usize,
                    ))
                };
                let mut _sl_key: Box<[i64]> = unsafe {
                    Box::from_raw(std::slice::from_raw_parts_mut(
                        _value_disps.key as *mut i64,
                        _value_disps.length as usize,
                    ))
                };
                for _value_data in _sl_data.iter_mut() {
                    _value_data.dispose();
                }
            }
        }
        return AbiInternalError::Ok;
    })) {
        Ok(value) => value as i64,
        Err(_) => AbiInternalError::UndefinedException as i64,
    }
}
#[no_mangle]
#[allow(unused_braces)]
pub extern "C" fn idl_test_method_test_test_int_int_map(
    _instance: *mut ffi_impl::TestInstance,
    value: *const AbiMap,
    _result: *mut *const AbiMap,
) -> i64 {
    match ::std::panic::catch_unwind(::std::panic::AssertUnwindSafe(move || {
        let mut _ins = unsafe { Box::from_raw(_instance) };
        let mut _instance = &mut _ins.instance;
        let _value_arg_val = {
            let _map = unsafe { (value as *const AbiMap).read() };
            let _length = _map.length as isize;
            let _data = _map.data as *const i64;
            let _key = _map.key as *const i64;
            let mut _map_result = ::std::collections::HashMap::new();
            for _index in 0.._length {
                let _map_data = unsafe { _data.offset(_index).read() };
                let _map_key = unsafe { _key.offset(_index).read() };
                if let Some(_) = _map_result.insert({ _map_key } as i64, { _map_data } as i64) {
                    panic!()
                }
            }
            _map_result
        };
        let _result_val: ::std::collections::HashMap<i64, i64> =
            _instance.test_int_int_map(_value_arg_val);
        unsafe {
            *_result = {
                Box::into_raw(Box::new({
                    let mut _array_map_values = Vec::<i64>::new();
                    let mut _array_map_keys = Vec::<i64>::new();
                    for (_map_key, _map_data) in _result_val {
                        _array_map_values.push({ _map_data } as i64);
                        _array_map_keys.push({ _map_key } as i64);
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
                })) as *const AbiMap
            };
        }
        std::mem::forget(_ins);
        return AbiInternalError::Ok;
    })) {
        Ok(value) => value as i64,
        Err(_) => AbiInternalError::UndefinedException as i64,
    }
}
#[no_mangle]
#[allow(unused_braces)]
pub extern "C" fn idl_test_dispose_test_test_int_int_map(
    _instance: *mut ffi_impl::TestInstance,
    _result_disp: *mut AbiMap,
) -> i64 {
    match ::std::panic::catch_unwind(::std::panic::AssertUnwindSafe(move || {
        {
            let mut _value_disps: Box<AbiMap> = unsafe { Box::from_raw(_result_disp) };
            {
                let mut _sl_data: Box<[i64]> = unsafe {
                    Box::from_raw(std::slice::from_raw_parts_mut(
                        _value_disps.data as *mut i64,
                        _value_disps.length as usize,
                    ))
                };
                let mut _sl_key: Box<[i64]> = unsafe {
                    Box::from_raw(std::slice::from_raw_parts_mut(
                        _value_disps.key as *mut i64,
                        _value_disps.length as usize,
                    ))
                };
            }
        }
        return AbiInternalError::Ok;
    })) {
        Ok(value) => value as i64,
        Err(_) => AbiInternalError::UndefinedException as i64,
    }
}
#[no_mangle]
#[allow(unused_braces)]
pub extern "C" fn idl_test_method_test_test_types(
    _instance: *mut ffi_impl::TestInstance,
    value: *const AbiVariant,
    _result: *mut *const AbiVariant,
) -> i64 {
    match ::std::panic::catch_unwind(::std::panic::AssertUnwindSafe(move || {
        let mut _ins = unsafe { Box::from_raw(_instance) };
        let mut _instance = &mut _ins.instance;
        let _value_arg_val = {
            idl_types::Types::from(crate::ffi_types::Types::from(unsafe {
                (value as *const AbiVariant).read()
            }))
        };
        let _result_val: idl_types::Types = _instance.test_types(_value_arg_val);
        unsafe {
            *_result = {
                Box::into_raw(Box::new({
                    crate::ffi_types::Types::from(_result_val).into()
                })) as *const AbiVariant
            };
        }
        std::mem::forget(_ins);
        return AbiInternalError::Ok;
    })) {
        Ok(value) => value as i64,
        Err(_) => AbiInternalError::UndefinedException as i64,
    }
}
#[no_mangle]
#[allow(unused_braces)]
pub extern "C" fn idl_test_dispose_test_test_types(
    _instance: *mut ffi_impl::TestInstance,
    _result_disp: *mut AbiVariant,
) -> i64 {
    match ::std::panic::catch_unwind(::std::panic::AssertUnwindSafe(move || {
        {
            let mut _value_disps: Box<AbiVariant> = unsafe { Box::from_raw(_result_disp) };
            {
                crate::ffi_types::Types::dispose(_value_disps)
            }
        }
        return AbiInternalError::Ok;
    })) {
        Ok(value) => value as i64,
        Err(_) => AbiInternalError::UndefinedException as i64,
    }
}
#[no_mangle]
#[allow(unused_braces)]
pub extern "C" fn idl_test_method_test_test_option(
    _instance: *mut ffi_impl::TestInstance,
    value: *const AbiVariant,
    _result: *mut *const AbiVariant,
) -> i64 {
    match ::std::panic::catch_unwind(::std::panic::AssertUnwindSafe(move || {
        let mut _ins = unsafe { Box::from_raw(_instance) };
        let mut _instance = &mut _ins.instance;
        let _value_arg_val = {
            let _option = unsafe { (value as *const AbiVariant).read() };
            match _option.index {
                0 => {
                    let _some_value = _option.data;
                    Some({ unsafe { (_some_value as *const i64).read() } } as i64)
                }
                1 => None,
                _ => panic!(),
            }
        };
        let _result_val: Option<i64> = _instance.test_option(_value_arg_val);
        unsafe {
            *_result = {
                Box::into_raw(Box::new({
                    let (_option_value, _index) = match _result_val {
                        Some(_value) => (
                            { Box::into_raw(Box::new({ _value } as i64)) as *const i64 }
                                as *const ::core::ffi::c_void,
                            0 as i64,
                        ),
                        None => (
                            { Box::into_raw(Box::new({ 0 } as i64)) as *const i64 }
                                as *const ::core::ffi::c_void,
                            1 as i64,
                        ),
                    };
                    AbiVariant {
                        index: _index,
                        data: _option_value,
                    }
                })) as *const AbiVariant
            };
        }
        std::mem::forget(_ins);
        return AbiInternalError::Ok;
    })) {
        Ok(value) => value as i64,
        Err(_) => AbiInternalError::UndefinedException as i64,
    }
}
#[no_mangle]
#[allow(unused_braces)]
pub extern "C" fn idl_test_dispose_test_test_option(
    _instance: *mut ffi_impl::TestInstance,
    _result_disp: *mut AbiVariant,
) -> i64 {
    match ::std::panic::catch_unwind(::std::panic::AssertUnwindSafe(move || {
        {
            let mut _value_disps: Box<AbiVariant> = unsafe { Box::from_raw(_result_disp) };
            {
                match _value_disps.index {
                    0 => {}
                    1 => {}
                    _value => panic!("Invaild option variant `{}`", _value),
                }
            }
        }
        return AbiInternalError::Ok;
    })) {
        Ok(value) => value as i64,
        Err(_) => AbiInternalError::UndefinedException as i64,
    }
}
#[no_mangle]
#[allow(unused_braces)]
pub extern "C" fn idl_test_method_test_test_none_option(
    _instance: *mut ffi_impl::TestInstance,
    _result: *mut *const AbiVariant,
) -> i64 {
    match ::std::panic::catch_unwind(::std::panic::AssertUnwindSafe(move || {
        let mut _ins = unsafe { Box::from_raw(_instance) };
        let mut _instance = &mut _ins.instance;
        let _result_val: Option<i64> = _instance.test_none_option();
        unsafe {
            *_result = {
                Box::into_raw(Box::new({
                    let (_option_value, _index) = match _result_val {
                        Some(_value) => (
                            { Box::into_raw(Box::new({ _value } as i64)) as *const i64 }
                                as *const ::core::ffi::c_void,
                            0 as i64,
                        ),
                        None => (
                            { Box::into_raw(Box::new({ 0 } as i64)) as *const i64 }
                                as *const ::core::ffi::c_void,
                            1 as i64,
                        ),
                    };
                    AbiVariant {
                        index: _index,
                        data: _option_value,
                    }
                })) as *const AbiVariant
            };
        }
        std::mem::forget(_ins);
        return AbiInternalError::Ok;
    })) {
        Ok(value) => value as i64,
        Err(_) => AbiInternalError::UndefinedException as i64,
    }
}
#[no_mangle]
#[allow(unused_braces)]
pub extern "C" fn idl_test_dispose_test_test_none_option(
    _instance: *mut ffi_impl::TestInstance,
    _result_disp: *mut AbiVariant,
) -> i64 {
    match ::std::panic::catch_unwind(::std::panic::AssertUnwindSafe(move || {
        {
            let mut _value_disps: Box<AbiVariant> = unsafe { Box::from_raw(_result_disp) };
            {
                match _value_disps.index {
                    0 => {}
                    1 => {}
                    _value => panic!("Invaild option variant `{}`", _value),
                }
            }
        }
        return AbiInternalError::Ok;
    })) {
        Ok(value) => value as i64,
        Err(_) => AbiInternalError::UndefinedException as i64,
    }
}
#[no_mangle]
#[allow(unused_braces)]
pub extern "C" fn idl_test_method_test_test_result(
    _instance: *mut ffi_impl::TestInstance,
    value: *const AbiString,
    _result: *mut *const AbiVariant,
) -> i64 {
    match ::std::panic::catch_unwind(::std::panic::AssertUnwindSafe(move || {
        let mut _ins = unsafe { Box::from_raw(_instance) };
        let mut _instance = &mut _ins.instance;
        let _value_arg_val = { unsafe { (value as *const AbiString).read() }.to_string() };
        let _result_val: Result<String, bool> = _instance.test_result(_value_arg_val);
        unsafe {
            *_result = {
                Box::into_raw(Box::new({
                    let (_result_value, _index) = match _result_val {
                        Ok(_ok) => (
                            {
                                Box::into_raw(Box::new({ AbiString::from(_ok) }))
                                    as *const AbiString
                            } as *const ::core::ffi::c_void,
                            0 as i64,
                        ),
                        Err(_err) => (
                            { Box::into_raw(Box::new({ _err } as i64)) as *const i64 }
                                as *const ::core::ffi::c_void,
                            1 as i64,
                        ),
                    };
                    AbiVariant {
                        index: _index,
                        data: _result_value,
                    }
                })) as *const AbiVariant
            };
        }
        std::mem::forget(_ins);
        return AbiInternalError::Ok;
    })) {
        Ok(value) => value as i64,
        Err(_) => AbiInternalError::UndefinedException as i64,
    }
}
#[no_mangle]
#[allow(unused_braces)]
pub extern "C" fn idl_test_dispose_test_test_result(
    _instance: *mut ffi_impl::TestInstance,
    _result_disp: *mut AbiVariant,
) -> i64 {
    match ::std::panic::catch_unwind(::std::panic::AssertUnwindSafe(move || {
        {
            let mut _value_disps: Box<AbiVariant> = unsafe { Box::from_raw(_result_disp) };
            {
                match _value_disps.index {
                    0 => {
                        let mut _value_disps: Box<AbiString> =
                            unsafe { Box::from_raw(_value_disps.data as *mut AbiString) };
                        _value_disps.dispose();
                    }
                    1 => {}
                    _value => panic!("Invaild result variant `{}`", _value),
                }
            }
        }
        return AbiInternalError::Ok;
    })) {
        Ok(value) => value as i64,
        Err(_) => AbiInternalError::UndefinedException as i64,
    }
}
#[no_mangle]
#[allow(unused_braces)]
pub extern "C" fn idl_test_method_test_test_result_err(
    _instance: *mut ffi_impl::TestInstance,
    _result: *mut *const AbiVariant,
) -> i64 {
    match ::std::panic::catch_unwind(::std::panic::AssertUnwindSafe(move || {
        let mut _ins = unsafe { Box::from_raw(_instance) };
        let mut _instance = &mut _ins.instance;
        let _result_val: Result<String, bool> = _instance.test_result_err();
        unsafe {
            *_result = {
                Box::into_raw(Box::new({
                    let (_result_value, _index) = match _result_val {
                        Ok(_ok) => (
                            {
                                Box::into_raw(Box::new({ AbiString::from(_ok) }))
                                    as *const AbiString
                            } as *const ::core::ffi::c_void,
                            0 as i64,
                        ),
                        Err(_err) => (
                            { Box::into_raw(Box::new({ _err } as i64)) as *const i64 }
                                as *const ::core::ffi::c_void,
                            1 as i64,
                        ),
                    };
                    AbiVariant {
                        index: _index,
                        data: _result_value,
                    }
                })) as *const AbiVariant
            };
        }
        std::mem::forget(_ins);
        return AbiInternalError::Ok;
    })) {
        Ok(value) => value as i64,
        Err(_) => AbiInternalError::UndefinedException as i64,
    }
}
#[no_mangle]
#[allow(unused_braces)]
pub extern "C" fn idl_test_dispose_test_test_result_err(
    _instance: *mut ffi_impl::TestInstance,
    _result_disp: *mut AbiVariant,
) -> i64 {
    match ::std::panic::catch_unwind(::std::panic::AssertUnwindSafe(move || {
        {
            let mut _value_disps: Box<AbiVariant> = unsafe { Box::from_raw(_result_disp) };
            {
                match _value_disps.index {
                    0 => {
                        let mut _value_disps: Box<AbiString> =
                            unsafe { Box::from_raw(_value_disps.data as *mut AbiString) };
                        _value_disps.dispose();
                    }
                    1 => {}
                    _value => panic!("Invaild result variant `{}`", _value),
                }
            }
        }
        return AbiInternalError::Ok;
    })) {
        Ok(value) => value as i64,
        Err(_) => AbiInternalError::UndefinedException as i64,
    }
}
#[no_mangle]
#[allow(unused_braces)]
pub extern "C" fn idl_test_method_test_test_int_stream(
    _instance: *mut ffi_impl::TestInstance,
    _stream: *const AbiStream,
) -> i64 {
    match ::std::panic::catch_unwind(::std::panic::AssertUnwindSafe(move || {
        let mut _ins = unsafe { Box::from_raw(_instance) };
        let mut _instance = &mut _ins.instance;
        let _stream_val = {
            let _value = unsafe { (_stream as *const AbiStream).read() };
            match _value.state.into() {
                AbiStreamReceiverState::Start => Box::new(ffi_impl::TestTest_int_stream {
                    callback: unsafe { std::mem::transmute(_value.wake_callback) },
                    handle: _value.wake_handle,
                    object: _value.wake_object,
                }),
                _ => {
                    std::mem::forget(_ins);
                    return AbiInternalError::InvalidArg;
                }
            }
        };
        _instance.test_int_stream(_stream_val);
        std::mem::forget(_ins);
        return AbiInternalError::Ok;
    })) {
        Ok(value) => value as i64,
        Err(_) => AbiInternalError::UndefinedException as i64,
    }
}
#[no_mangle]
#[allow(unused_braces)]
pub extern "C" fn stream_test_test_int_stream(
    _instance: *mut ffi_impl::TestInstance,
    _stream: *const AbiStream,
    _stream_result: *mut *mut AbiStream,
) -> i64 {
    match ::std::panic::catch_unwind(::std::panic::AssertUnwindSafe(move || {
        let mut _ins = unsafe { Box::from_raw(_instance) };
        let mut _instance = &mut _ins.instance;
        let _stream_val = unsafe { (_stream as *const AbiStream).read() };
        let _result = _instance.test_int_stream_stream(
            Box::new(ffi_impl::TestTest_int_stream {
                callback: unsafe { std::mem::transmute(_stream_val.wake_callback) },
                handle: _stream_val.wake_handle,
                object: _stream_val.wake_object,
            }),
            _stream_val.into(),
        );
        unsafe {
            *_stream_result = { Box::into_raw(Box::new({ _result.into_abi() })) as *mut AbiStream };
        }
        std::mem::forget(_ins);
        return AbiInternalError::Ok;
    })) {
        Ok(value) => value as i64,
        Err(_) => AbiInternalError::UndefinedException as i64,
    }
}
#[no_mangle]
#[allow(unused_braces)]
pub extern "C" fn dispose_stream_test_test_int_stream(
    _instance: *mut ffi_impl::TestInstance,
    _stream: *mut AbiStream,
) -> i64 {
    match ::std::panic::catch_unwind(::std::panic::AssertUnwindSafe(move || {
        let _stream_val: Box<AbiStream> = unsafe { Box::from_raw(_stream) };
        _stream_val.dispose();
        return AbiInternalError::Ok;
    })) {
        Ok(value) => value as i64,
        Err(_) => AbiInternalError::UndefinedException as i64,
    }
}
#[no_mangle]
#[allow(unused_braces)]
pub extern "C" fn idl_test_method_test_test_pair(
    _instance: *mut ffi_impl::TestInstance,
    value: *const AbiPair,
    _result: *mut *const AbiPair,
) -> i64 {
    match ::std::panic::catch_unwind(::std::panic::AssertUnwindSafe(move || {
        let mut _ins = unsafe { Box::from_raw(_instance) };
        let mut _instance = &mut _ins.instance;
        let _value_arg_val = {
            let _pair = unsafe { (value as *const AbiPair).read() };
            let _first_value = _pair.first_data as *const AbiString;
            let _second_value = _pair.second_data as *const i64;
            (
                { unsafe { (_first_value as *const AbiString).read() }.to_string() },
                { unsafe { (_second_value as *const i64).read() } } as i64,
            )
        };
        let _result_val: (String, i64) = _instance.test_pair(_value_arg_val);
        unsafe {
            *_result = {
                Box::into_raw(Box::new({
                    let _first_data = {
                        Box::into_raw(Box::new({ AbiString::from(_result_val.0) }))
                            as *const AbiString
                    } as *const ::core::ffi::c_void;
                    let _second_data =
                        { Box::into_raw(Box::new({ _result_val.1 } as i64)) as *const i64 }
                            as *const ::core::ffi::c_void;
                    AbiPair {
                        first_data: _first_data,
                        second_data: _second_data,
                    }
                })) as *const AbiPair
            };
        }
        std::mem::forget(_ins);
        return AbiInternalError::Ok;
    })) {
        Ok(value) => value as i64,
        Err(_) => AbiInternalError::UndefinedException as i64,
    }
}
#[no_mangle]
#[allow(unused_braces)]
pub extern "C" fn idl_test_dispose_test_test_pair(
    _instance: *mut ffi_impl::TestInstance,
    _result_disp: *mut AbiPair,
) -> i64 {
    match ::std::panic::catch_unwind(::std::panic::AssertUnwindSafe(move || {
        {
            let mut _value_disps: Box<AbiPair> = unsafe { Box::from_raw(_result_disp) };
            {
                {
                    let mut _value_disps: Box<AbiString> =
                        unsafe { Box::from_raw(_value_disps.first_data as *mut AbiString) };
                    _value_disps.dispose();
                }
            }
        }
        return AbiInternalError::Ok;
    })) {
        Ok(value) => value as i64,
        Err(_) => AbiInternalError::UndefinedException as i64,
    }
}
#[no_mangle]
#[allow(unused_braces)]
pub extern "C" fn idl_test_method_test_test_names(
    _instance: *mut ffi_impl::TestInstance,
    value: *const AbiString,
    _result: *mut *const AbiString,
) -> i64 {
    match ::std::panic::catch_unwind(::std::panic::AssertUnwindSafe(move || {
        let mut _ins = unsafe { Box::from_raw(_instance) };
        let mut _instance = &mut _ins.instance;
        let _value_arg_val = { unsafe { (value as *const AbiString).read() }.to_string() };
        let _result_val: idl_types::Names = _instance.test_names(_value_arg_val);
        unsafe {
            *_result =
                { Box::into_raw(Box::new({ AbiString::from(_result_val) })) as *const AbiString };
        }
        std::mem::forget(_ins);
        return AbiInternalError::Ok;
    })) {
        Ok(value) => value as i64,
        Err(_) => AbiInternalError::UndefinedException as i64,
    }
}
#[no_mangle]
#[allow(unused_braces)]
pub extern "C" fn idl_test_dispose_test_test_names(
    _instance: *mut ffi_impl::TestInstance,
    _result_disp: *mut AbiString,
) -> i64 {
    match ::std::panic::catch_unwind(::std::panic::AssertUnwindSafe(move || {
        {
            let mut _value_disps: Box<AbiString> = unsafe { Box::from_raw(_result_disp) };
            _value_disps.dispose();
        }
        return AbiInternalError::Ok;
    })) {
        Ok(value) => value as i64,
        Err(_) => AbiInternalError::UndefinedException as i64,
    }
}
#[no_mangle]
#[allow(unused_braces)]
pub extern "C" fn idl_test_method_test_test_values(
    _instance: *mut ffi_impl::TestInstance,
    value: f64,
    _result: *mut f64,
) -> i64 {
    match ::std::panic::catch_unwind(::std::panic::AssertUnwindSafe(move || {
        let mut _ins = unsafe { Box::from_raw(_instance) };
        let mut _instance = &mut _ins.instance;
        let _value_arg_val = { value } as f64;
        let _result_val: idl_types::ValuesFloat = _instance.test_values(_value_arg_val);
        unsafe {
            *_result = { _result_val } as f64;
        }
        std::mem::forget(_ins);
        return AbiInternalError::Ok;
    })) {
        Ok(value) => value as i64,
        Err(_) => AbiInternalError::UndefinedException as i64,
    }
}
#[no_mangle]
#[allow(unused_braces)]
pub extern "C" fn idl_test_create_test(_result: *mut *const ffi_impl::TestInstance) -> i64 {
    match ::std::panic::catch_unwind(::std::panic::AssertUnwindSafe(move || {
        unsafe {
            *_result = Box::into_raw(Box::new({ ffi_impl::TestInstance::new() }))
                as *const ffi_impl::TestInstance;
        }
        return AbiInternalError::Ok;
    })) {
        Ok(value) => value as i64,
        Err(_) => AbiInternalError::UndefinedException as i64,
    }
}
#[no_mangle]
#[allow(unused_braces)]
pub extern "C" fn idl_test_dispose_test(_result: *mut *const ffi_impl::TestInstance) -> i64 {
    match ::std::panic::catch_unwind(::std::panic::AssertUnwindSafe(move || {
        return AbiInternalError::Ok;
    })) {
        Ok(value) => value as i64,
        Err(_) => AbiInternalError::UndefinedException as i64,
    }
}
trait StreamSenderIntoAbiStream<T> {
    fn into_abi(self) -> AbiStream;
}
trait StreamAbiSenderDispose<T> {
    fn dispose(self);
}
impl StreamSenderIntoAbiStream<String> for StreamSender<String> {
    #[allow(unused_braces)]
    fn into_abi(self) -> AbiStream {
        match self {
            StreamSender::Ok => AbiStream::new(AbiStreamSenderState::Ok as i64),
            StreamSender::Value(value) => {
                let mut _result = AbiStream::new(AbiStreamSenderState::Value as i64);
                _result.data =
                    { Box::into_raw(Box::new({ AbiString::from(value) })) as *const AbiString }
                        as *const ::core::ffi::c_void;
                _result
            }
            StreamSender::Request => AbiStream::new(AbiStreamSenderState::Request as i64),
            StreamSender::Waiting => AbiStream::new(AbiStreamSenderState::Waiting as i64),
            StreamSender::Done => AbiStream::new(AbiStreamSenderState::Done as i64),
        }
    }
}
impl StreamAbiSenderDispose<String> for AbiStream {
    fn dispose(self) {
        match self.state.into() {
            AbiStreamSenderState::Value => {}
            _ => {}
        }
    }
}
