use std::ffi::c_void;

#[repr(C)]
pub struct NativeString {
    pub length: i64,
    pub data: *mut u8,
}

impl NativeString {
    pub fn from_string(value: &str) -> *mut NativeString {
        let mut s = value.as_bytes().to_vec().into_boxed_slice();

        let result = Box::new(NativeString {
            length: s.len() as i64,
            data: s.as_mut_ptr(),
        });

        std::mem::forget(s);

        Box::into_raw(result)
    }

    pub fn release(value: *mut NativeString) {
        unsafe {
            let native_string = Box::from_raw(value);
            //let s =
              //  std::slice::from_raw_parts_mut(native_string.data, native_string.length as usize);
            //Box::from_raw(s as *mut [u8]);
        }
    }
}

impl std::fmt::Display for NativeString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = unsafe { std::slice::from_raw_parts(self.data, self.length as usize) };
        match std::str::from_utf8(s) {
            Ok(string) => write!(f, "{}", string),
            Err(err) => panic!(err),
        }
    }
}

#[repr(C)]
pub struct Bytes {
    pub length: i64,
    pub data: *mut u8,
}

impl Bytes {
    pub fn from_bytes(value: &[u8]) -> *mut Bytes {
        let mut s = value.to_vec().into_boxed_slice();

        let result = Box::new(Bytes {
            length: s.len() as i64,
            data: s.as_mut_ptr(),
        });

        std::mem::forget(s);

        Box::into_raw(result)
    }

    pub fn release(value: *mut Bytes) {
        unsafe {
            let bytes = Box::from_raw(value);
            let s = std::slice::from_raw_parts_mut(bytes.data, bytes.length as usize);
            Box::from_raw(s as *mut [u8]);
        }
    }
}

#[repr(C)]
pub struct Type {
    pub name: *mut NativeString,
    pub ptr_data: *mut c_void,
}

#[repr(C)]
pub struct CallbackValue {
    pub handle: *mut c_void,
    pub value: *mut c_void,
}

#[repr(C)]
pub struct ArrayType {
    pub length: i64,
    pub data: *mut c_void,
}