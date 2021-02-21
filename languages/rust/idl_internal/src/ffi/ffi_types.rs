use crate::StreamReceiver;
use std::convert::TryInto;

#[repr(i64)]
pub enum AbiInternalError {
    Ok = 0x0,                  // Success.
    InvalidArg = 0x1,          // Invalid argument.
    NullPtr = 0x2,             // Null pointer.
    Abort = 0x3,               // Critical error.
    CallbackException = 0x4,   // Error occurred when calling back on the client function.
    UndefinedException = 0x5,  // Undefined error.
    Unimplemented = 0x6,       // Unimplemented function.
    Type = 0x7,                // Type error.
    NotAllowedOperation = 0x8, // Not allowed operation error.
}

#[repr(i64)]
pub enum AbiStreamSenderState {
    Ok = 0x0,      // Nothing.
    Value = 0x1,   // Only one value.
    Request = 0x2, // Request to retrieve the last state from the server.
    Waiting = 0x3, // Send a response that the server is awaiting for client to send a request.
    Done = 0x4,    // Stream is finished.
}

impl From<i64> for AbiStreamSenderState {
    fn from(value: i64) -> Self {
        match value {
            0x0 => AbiStreamSenderState::Ok,
            0x1 => AbiStreamSenderState::Value,
            0x2 => AbiStreamSenderState::Request,
            0x3 => AbiStreamSenderState::Waiting,
            0x4 => AbiStreamSenderState::Done,
            _ => panic!("Invalid state value: `{}`", value),
        }
    }
}

impl From<AbiStreamSenderState> for i64 {
    fn from(value: AbiStreamSenderState) -> Self {
        match value {
            AbiStreamSenderState::Ok => 0x0,
            AbiStreamSenderState::Value => 0x1,
            AbiStreamSenderState::Request => 0x2,
            AbiStreamSenderState::Waiting => 0x3,
            AbiStreamSenderState::Done => 0x4,
        }
    }
}

#[repr(i64)]
pub enum AbiStreamReceiverState {
    Ok = 0x0,      // Nothing
    Close = 0x1,   // Closes the stream.
    Start = 0x2,   // Sent by the client with the function arguments. Only one call allowed.
    Pause = 0x3,   // Pauses the stream.
    Resume = 0x4,  // Resumes a previous paused stream.
    Request = 0x5, // Request to retrieve pending data.
}

impl From<AbiStreamReceiverState> for i64 {
    fn from(value: AbiStreamReceiverState) -> Self {
        match value {
            AbiStreamReceiverState::Ok => 0x0,
            AbiStreamReceiverState::Close => 0x1,
            AbiStreamReceiverState::Start => 0x2,
            AbiStreamReceiverState::Pause => 0x3,
            AbiStreamReceiverState::Resume => 0x4,
            AbiStreamReceiverState::Request => 0x5,
        }
    }
}

impl From<i64> for AbiStreamReceiverState {
    fn from(value: i64) -> Self {
        match value {
            0x0 => AbiStreamReceiverState::Ok,
            0x1 => AbiStreamReceiverState::Close,
            0x2 => AbiStreamReceiverState::Start,
            0x3 => AbiStreamReceiverState::Pause,
            0x4 => AbiStreamReceiverState::Resume,
            0x5 => AbiStreamReceiverState::Request,
            _ => panic!("Invalid state value: `{}`", value),
        }
    }
}

#[repr(C)]
pub struct AbiStream {
    pub state: i64,
    pub wake_handle: i64,
    pub wake_object: *const ::core::ffi::c_void,
    pub wake_callback: *const ::core::ffi::c_void,
    pub data: *const ::core::ffi::c_void,
}

impl AbiStream {
    pub fn new(state: i64) -> Self {
        Self {
            wake_callback: 0 as *const ::core::ffi::c_void,
            data: 0 as *const ::core::ffi::c_void,
            wake_object: 0 as *const ::core::ffi::c_void,
            wake_handle: 0,
            state: state,
        }
    }
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
pub struct AbiPair {
    pub first_data: *const ::core::ffi::c_void,
    pub second_data: *const ::core::ffi::c_void,
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

impl AbiBytes {
    pub fn free(&mut self) {
        //   unsafe {}
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
pub struct AbiUuid {
    pub data: *const u8,
}

impl AbiUuid {
    pub fn to_uuid(&self) -> uuid::Uuid {
        let sl = unsafe {
            let sl = std::slice::from_raw_parts_mut(self.data as *mut u8, 0x10);
            Box::from_raw(sl)
        };

        let bytes: [u8; 0x10] = sl
            .to_vec()
            .try_into()
            .unwrap_or_else(|_| panic!("Error converting `uuid`"));

        std::mem::forget(sl);

        uuid::Uuid::from_bytes(bytes)
    }
}

impl AbiUuid {
    pub fn free(&mut self) {
        //   unsafe {}
    }
}

impl From<uuid::Uuid> for AbiUuid {
    fn from(value: uuid::Uuid) -> Self {
        let bytes = value.as_bytes().to_vec();
        let mut data = bytes.into_boxed_slice();
        let result = AbiUuid {
            data: data.as_mut_ptr(),
        };
        std::mem::forget(data);
        result
    }
}

#[repr(C)]
pub struct AbiString {
    pub length: i64,
    pub data: *const u8,
}

impl AbiString {
    pub fn free(&mut self) {
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

impl From<AbiStream> for StreamReceiver {
    fn from(value: AbiStream) -> Self {
        match value.state.into() {
            AbiStreamReceiverState::Close => StreamReceiver::Close,
            AbiStreamReceiverState::Start => StreamReceiver::Start,
            AbiStreamReceiverState::Pause => StreamReceiver::Pause,
            AbiStreamReceiverState::Resume => StreamReceiver::Resume,
            AbiStreamReceiverState::Request => StreamReceiver::Request,
            AbiStreamReceiverState::Ok => StreamReceiver::Ok,
        }
    }
}

impl From<StreamReceiver> for AbiStream {
    #[allow(unused_braces)]
    fn from(value: StreamReceiver) -> Self {
        match value {
            StreamReceiver::Close => AbiStream::new(AbiStreamReceiverState::Close as i64),
            StreamReceiver::Start => AbiStream::new(AbiStreamReceiverState::Start as i64),
            StreamReceiver::Resume => AbiStream::new(AbiStreamReceiverState::Resume as i64),
            StreamReceiver::Request => AbiStream::new(AbiStreamReceiverState::Request as i64),
            StreamReceiver::Pause => AbiStream::new(AbiStreamReceiverState::Pause as i64),
            StreamReceiver::Ok => AbiStream::new(AbiStreamReceiverState::Ok as i64),
        }
    }
}
