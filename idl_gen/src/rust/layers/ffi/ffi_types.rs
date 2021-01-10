use super::idl_internal::{StreamReceiver, StreamSender, StreamError};

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

#[repr(C)]
pub struct AbiStreamPartial {
    pub index: i64,  // ! This field cannot be negative.
    pub length: i64, // ! This field cannot be negative.
    pub data: *const ::core::ffi::c_void,
}

#[repr(C)]
pub struct AbiStreamSize {
    pub index: i64,
    pub length: i64,
}

#[repr(i64)]
pub enum AbiStreamSenderState {
    Ok = 0x0,      // Nothing.
    Error = 0x1,   // Error State.
    Value = 0x2,   // Only one value.
    Partial = 0x3, // Partial data sent from the server.
    Request = 0x4, // Request to retrieve the last state from the server.
    Waiting = 0x5, // Send a response that the server is awaiting for client to send a request.
    Done = 0x6,
}

#[repr(i64)]
pub enum AbiStreamReceiverState {
    Ok = 0x0,      // Nothing
    Error = 0x1,   // Error state.
    Close = 0x2,   // Closes the stream.
    Send = 0x3,    // Requests the server to start the stream.
    Create = 0x4,  // Sent by the client with the function arguments. Only one call allowed.
    Pause = 0x5,   // Pauses the stream.
    Resume = 0x6,  // Resumes a previous paused stream.
    Request = 0x7, // Request to retrieve pending data.
}

#[repr(i64)]
pub enum AbiStreamError {
    Undefined = 0x0,
    UnknownState = 0x1,
    Schedule = 0x2,
}

impl From<AbiStreamSenderState> for i64 {
    fn from(value: AbiStreamSenderState) -> Self {
        match value {
            AbiStreamSenderState::Ok => 0x0,
            AbiStreamSenderState::Error => 0x1,
            AbiStreamSenderState::Value => 0x2,
            AbiStreamSenderState::Partial => 0x3,
            AbiStreamSenderState::Request => 0x4,
            AbiStreamSenderState::Waiting => 0x5,
            AbiStreamSenderState::Done => 0x6,
        }
    }
}

impl From<i64> for AbiStreamSenderState {
    fn from(value: i64) -> Self {
        match value {
            0x0 => AbiStreamSenderState::Ok,
            0x1 => AbiStreamSenderState::Error,
            0x2 => AbiStreamSenderState::Value,
            0x3 => AbiStreamSenderState::Partial,
            0x4 => AbiStreamSenderState::Request,
            0x5 => AbiStreamSenderState::Waiting,
            0x6 => AbiStreamSenderState::Done,
            _ => panic!("Invalid state value: `{}`", value),
        }
    }
}

impl From<AbiStreamReceiverState> for i64 {
    fn from(value: AbiStreamReceiverState) -> Self {
        match value {
            AbiStreamReceiverState::Ok => 0x0,
            AbiStreamReceiverState::Error => 0x1,
            AbiStreamReceiverState::Close => 0x2,
            AbiStreamReceiverState::Send => 0x3,
            AbiStreamReceiverState::Create => 0x4,
            AbiStreamReceiverState::Pause => 0x5,
            AbiStreamReceiverState::Resume => 0x6,
            AbiStreamReceiverState::Request => 0x7,
        }
    }
}

impl From<i64> for AbiStreamReceiverState {
    fn from(value: i64) -> Self {
        match value {
            0x0 => AbiStreamReceiverState::Ok,
            0x1 => AbiStreamReceiverState::Error,
            0x2 => AbiStreamReceiverState::Close,
            0x3 => AbiStreamReceiverState::Send,
            0x4 => AbiStreamReceiverState::Create,
            0x5 => AbiStreamReceiverState::Pause,
            0x6 => AbiStreamReceiverState::Resume,
            0x7 => AbiStreamReceiverState::Request,
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
    // Complete: null
    // Close: null?
    // Send: `AbiStreamSize`
    // Partial: `AbiStreamPartial`
    // Values: null
    // Event: result type
    // Waiting: the stream length
    // Error: null?
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
        unsafe {}
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
            AbiStreamReceiverState::Send => {
                let _result = unsafe { (value.data as *const AbiStreamSize).read() };
                StreamReceiver::Send {
                    index: _result.index as isize,
                    length: _result.length as isize,
                }
            }
            AbiStreamReceiverState::Error => StreamReceiver::Error(StreamError::Undefined), // TODO
            AbiStreamReceiverState::Create => StreamReceiver::Create,
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
            StreamReceiver::Send { index, length } => {
                let mut _result = AbiStream::new(AbiStreamReceiverState::Send as i64);
                _result.data = {
                    Box::into_raw(Box::new({
                        let _inn = AbiStreamSize {
                            index: index as i64,
                            length: length as i64,
                        };
                        _inn
                    })) as *const AbiStreamSize
                } as *const ::core::ffi::c_void;
                _result
            }

            StreamReceiver::Create => AbiStream::new(AbiStreamReceiverState::Create as i64),
            StreamReceiver::Resume => AbiStream::new(AbiStreamReceiverState::Resume as i64),
            StreamReceiver::Request => AbiStream::new(AbiStreamReceiverState::Request as i64),
            StreamReceiver::Pause => AbiStream::new(AbiStreamReceiverState::Pause as i64),
            StreamReceiver::Error(_error) => {
                let mut _result = AbiStream::new(AbiStreamReceiverState::Error as i64);
                _result.data =
                    { Box::into_raw(Box::new(0)) as *const i64 } as *const ::core::ffi::c_void;
                _result
            }
            StreamReceiver::Ok => AbiStream::new(AbiStreamReceiverState::Ok as i64),
        }
    }
}

impl From<AbiStream> for StreamSender<i64> {
    fn from(value: AbiStream) -> Self {
        match value.state.into() {
            AbiStreamSenderState::Value => {
                let _result = unsafe { (value.data as *const i64).read() };
                StreamSender::Value(_result)
            }
            AbiStreamSenderState::Partial => {
                let _result = unsafe { (value.data as *const AbiStreamPartial).read() };
                StreamSender::Partial {
                    index: _result.index as isize,
                    length: _result.length as isize,
                    value: unsafe { (value.data as *const i64).read() },
                }
            }
            AbiStreamSenderState::Error => StreamSender::Error(StreamError::Undefined),
            AbiStreamSenderState::Waiting => {
                let _result = unsafe { (value.data as *const i64).read() };
                StreamSender::Waiting {
                    length: _result as isize,
                }
            }
            AbiStreamSenderState::Done => StreamSender::Done,
            AbiStreamSenderState::Request => StreamSender::Request,
            AbiStreamSenderState::Ok => StreamSender::Ok,
        }
    }
}

impl From<StreamSender<i64>> for AbiStream {
    #[allow(unused_braces)]
    fn from(value: StreamSender<i64>) -> Self {
        match value {
            StreamSender::Value(_value) => {
                let mut _result = AbiStream::new(AbiStreamSenderState::Value as i64);
                _result.data = { Box::into_raw(Box::new(_value as i64)) as *const i64 }
                    as *const ::core::ffi::c_void;
                _result
            }

            StreamSender::Partial {
                index,
                length,
                value,
            } => {
                let mut _result = AbiStream::new(AbiStreamSenderState::Partial as i64);
                _result.data = {
                    Box::into_raw(Box::new({
                        let _inn = AbiStreamPartial {
                            index: index as i64,
                            length: length as i64,
                            data: Box::into_raw(Box::new(value as i64))
                                as *const ::core::ffi::c_void,
                        };
                        _inn
                    })) as *const AbiStreamPartial
                } as *const ::core::ffi::c_void;
                _result
            }
            StreamSender::Waiting { length } => {
                let mut _result = AbiStream::new(AbiStreamSenderState::Waiting as i64);
                _result.data = { Box::into_raw(Box::new(length as i64)) as *const i64 }
                    as *const ::core::ffi::c_void;
                _result
            }
            StreamSender::Request => AbiStream::new(AbiStreamSenderState::Request as i64),
            StreamSender::Done => AbiStream::new(AbiStreamSenderState::Done as i64),
            StreamSender::Error(_error) => {
                let mut _result = AbiStream::new(AbiStreamSenderState::Error as i64);
                _result.data =
                    { Box::into_raw(Box::new(0)) as *const i64 } as *const ::core::ffi::c_void;
                _result
            }
            StreamSender::Ok => AbiStream::new(AbiStreamSenderState::Ok as i64),
        }
    }
}
