use super::ffi_types::*;

#[no_mangle]
#[allow(unused_braces)]
pub extern "C" fn idl_interface_shit(
    _instance: *mut super::idl_ffi_impl::IdlInterface,
    _stream: *const AbiStream,
) -> i64 {
    match ::std::panic::catch_unwind(::std::panic::AssertUnwindSafe(move || {
        let mut _ins = unsafe { Box::from_raw(_instance) };
        let mut _instance = &mut _ins.instance;
        let _stream_val = {
            let _result = unsafe { (_stream as *const AbiStream).read() };
            _result.handle as i64
        };
        _instance.shit(_stream_val);
        std::mem::forget(_ins);
    })) {
        Ok(_) => AbiInternalError::Ok as i64,
        Err(_) => AbiInternalError::UndefinedException as i64,
    }
}
#[no_mangle]
#[allow(unused_braces)]
pub extern "C" fn idl_interface_shit_stream(_stream_function: *const AbiVariant) -> i64 {
    match ::std::panic::catch_unwind(::std::panic::AssertUnwindSafe(move || {
        let _instance = 0 as *mut super::idl_ffi_impl::IdlInterfaceStatic;
        let mut _ins = unsafe { Box::from_raw(_instance) };
        let _stream_function_val = unsafe { _stream_function.read() };
        match _stream_function_val.index {
            0 => _ins.shit_stream = None,
            1 => _ins.shit_stream = Some(unsafe { std::mem::transmute(_stream_function_val.data) }),
            _ => panic!(),
        }
        std::mem::forget(_ins);
    })) {
        Ok(_) => AbiInternalError::Ok as i64,
        Err(_) => AbiInternalError::UndefinedException as i64,
    }
}
#[no_mangle]
#[allow(unused_braces)]
pub extern "C" fn idl_interface_get_value(
    _instance: *mut super::idl_ffi_impl::IdlInterface,
    value: *const super::idl_ffi_types::TypeStruct,
    name: *const AbiArray,
    meh_not: f64,
    _stream: *const AbiStream,
) -> i64 {
    match ::std::panic::catch_unwind(::std::panic::AssertUnwindSafe(move || {
        let mut _ins = unsafe { Box::from_raw(_instance) };
        let mut _instance = &mut _ins.instance;
        let _value_arg_val = {
            super::idl_types::TypeStruct::from(unsafe {
                (value as *const super::idl_ffi_types::TypeStruct).read()
            })
        };
        let _name_arg_val = {
            let _array = unsafe { (name as *const AbiArray).read() };
            let _data = _array.data as *const AbiString;
            let _length = _array.length as isize;
            let mut _array_vec = vec![];
            for _index in 0.._length {
                let _array_item = unsafe { _data.offset(_index).read() };
                _array_vec.push({ _array_item.to_string() });
            }
            _array_vec
        };
        let _meh_not_arg_val = { meh_not } as f64;
        let _stream_val = {
            let _result = unsafe { (_stream as *const AbiStream).read() };
            _result.handle as i64
        };
        _instance.get_value(_value_arg_val, _name_arg_val, _meh_not_arg_val, _stream_val);
        std::mem::forget(_ins);
    })) {
        Ok(_) => AbiInternalError::Ok as i64,
        Err(_) => AbiInternalError::UndefinedException as i64,
    }
}
#[no_mangle]
#[allow(unused_braces)]
pub extern "C" fn idl_interface_get_value_stream(_stream_function: *const AbiVariant) -> i64 {
    match ::std::panic::catch_unwind(::std::panic::AssertUnwindSafe(move || {
        let _instance = 0 as *mut super::idl_ffi_impl::IdlInterfaceStatic;
        let mut _ins = unsafe { Box::from_raw(_instance) };
        let _stream_function_val = unsafe { _stream_function.read() };
        match _stream_function_val.index {
            0 => _ins.get_value_stream = None,
            1 => {
                _ins.get_value_stream =
                    Some(unsafe { std::mem::transmute(_stream_function_val.data) })
            }
            _ => panic!(),
        }
        std::mem::forget(_ins);
    })) {
        Ok(_) => AbiInternalError::Ok as i64,
        Err(_) => AbiInternalError::UndefinedException as i64,
    }
}
#[no_mangle]
#[allow(unused_braces)]
pub extern "C" fn idl_interface_get_name(
    _instance: *mut super::idl_ffi_impl::IdlInterface,
    name: *const AbiString,
    _result: *mut i64,
) -> i64 {
    match ::std::panic::catch_unwind(::std::panic::AssertUnwindSafe(move || {
        let mut _ins = unsafe { Box::from_raw(_instance) };
        let mut _instance = &mut _ins.instance;
        let _name_arg_val = { unsafe { (name as *const AbiString).read() }.to_string() };
        let _result_val: i64 = _instance.get_name(_name_arg_val);
        unsafe {
            *_result = { _result_val } as i64;
        }
        std::mem::forget(_ins);
    })) {
        Ok(_) => AbiInternalError::Ok as i64,
        Err(_) => AbiInternalError::UndefinedException as i64,
    }
}
#[no_mangle]
#[allow(unused_braces)]
pub extern "C" fn idl_interface_create(
    _result: *mut *const super::idl_ffi_impl::IdlInterface,
) -> i64 {
    match ::std::panic::catch_unwind(::std::panic::AssertUnwindSafe(move || {
        let _instance = 0 as *mut super::idl_ffi_impl::IdlInterfaceStatic;
        let _ins = unsafe { Box::from_raw(_instance) };
        let _instance = &_ins.instance;
        let _result_val: Box<dyn super::idl_impl::IdlInterface> = _instance.create();
        unsafe {
            *_result = {
                Box::into_raw(Box::new({
                    super::idl_ffi_impl::IdlInterface::from(_result_val)
                })) as *const super::idl_ffi_impl::IdlInterface
            };
        }
        std::mem::forget(_ins);
    })) {
        Ok(_) => AbiInternalError::Ok as i64,
        Err(_) => AbiInternalError::UndefinedException as i64,
    }
}
