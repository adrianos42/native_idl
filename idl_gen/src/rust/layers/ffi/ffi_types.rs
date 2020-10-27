#[repr(i64)]
pub enum AbiInternalError {
    Ok = 0x0,
    InvalidArg = 0x1,
    NullPtr = 0x2,
    Abort = 0x3,
    CallbackException = 0x4,
    UndefinedException = 0x5,
    Unimplemented = 0x6,
    Type = 0x7,
    NotAllowedOperation = 0x8,
}

#[repr(C)]
pub struct AbiStreamValue {
    pub handle: i64,
    pub value: *const ::core::ffi::c_void,
}

#[repr(C)]
pub struct AbiStream {
    pub handle: i64,
}

#[repr(C)]
pub struct AbiVariant {
    pub index: i64,
    pub data: *const ::core::ffi::c_void,
}

#[repr(C)]
pub struct AbiArray {
    pub length: i64,
    pub data: *const ::core::ffi::c_void,
}

#[repr(C)]
pub struct AbiMap {
    pub length: i64,
    pub key: *const ::core::ffi::c_void,
    pub data: *const ::core::ffi::c_void,
}

#[repr(C)]
pub struct AbiBytes {
    pub length: i64,
    pub data: *const u8,
}

impl AbiBytes {
    // This makes a copy from bytes
    pub fn to_vec(&self) -> Vec<u8> {
        let sl = unsafe {
            let sl = std::slice::from_raw_parts_mut(self.data as *mut u8, self.length as usize);
            Box::from_raw(sl)
        };
        let result = sl.to_vec();
        std::mem::forget(sl);
        result
    }
}

impl From<Vec<u8>> for AbiBytes {
    fn from(value: Vec<u8>) -> Self {
        let mut sl = value.into_boxed_slice();
        let result = AbiBytes {
            length: sl.len() as i64,
            data: sl.as_mut_ptr(),
        };
        std::mem::forget(sl);
        result
    }
}

#[repr(C)]
pub struct AbiString {
    pub length: i64,
    pub data: *const u8,
}

impl AbiString {
    pub fn release(&mut self) {
        unsafe {
            let sl = std::slice::from_raw_parts_mut(self.data as *mut u8, self.length as usize);
            let _ = Box::from_raw(sl);
        }
    }
}

impl std::fmt::Display for AbiString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let sl = unsafe { std::slice::from_raw_parts(self.data, self.length as usize) };
        match std::str::from_utf8(sl) {
            Ok(string) => write!(f, "{}", string),
            Err(err) => panic!(err),
        }
    }
}

impl From<String> for AbiString {
    fn from(value: String) -> Self {
        let mut sl = value.as_bytes().to_vec().into_boxed_slice();
        let result = Self {
            length: sl.len() as i64,
            data: sl.as_mut_ptr(),
        };
        std::mem::forget(sl);
        result
    }
}
    