// This would be the same as the `AbiStream` defined for ffi, except without the callback fields.
#[derive(Debug)]
pub struct AbiStreamObject {
    pub state: i64,
    // Handle used by the client for internal usage, e.g., a thread id or port for message exchange.
    pub wake_handle: i64,
    // Id for server usage, may be the same as the wake object when dereferencing it.
    pub wake_object_id: i64,
}

#[repr(i64)]
#[derive(Debug)]
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

impl From<i64> for AbiInternalError {
    fn from(value: i64) -> Self {
        match value {
            0x0 => AbiInternalError::Ok,
            0x1 => AbiInternalError::InvalidArg,
            0x2 => AbiInternalError::NullPtr,
            0x3 => AbiInternalError::Abort,
            0x4 => AbiInternalError::CallbackException,
            0x5 => AbiInternalError::UndefinedException,
            0x6 => AbiInternalError::Unimplemented,
            0x7 => AbiInternalError::Type,
            0x8 => AbiInternalError::NotAllowedOperation,
            _ => panic!("Invalid error value: `{}`", value),
        }
    }
}

impl From<AbiInternalError> for i64 {
    fn from(value: AbiInternalError) -> Self {
        match value {
            AbiInternalError::Ok => 0x0,
            AbiInternalError::InvalidArg => 0x1,
            AbiInternalError::NullPtr => 0x2,
            AbiInternalError::Abort => 0x3,
            AbiInternalError::CallbackException => 0x4,
            AbiInternalError::UndefinedException => 0x5,
            AbiInternalError::Unimplemented => 0x6,
            AbiInternalError::Type => 0x7,
            AbiInternalError::NotAllowedOperation => 0x8,
        }
    }
}

#[repr(i64)]
#[derive(Debug)]
pub enum AbiStreamSenderState {
    Ok = 0x0,       // Nothing.
    Value = 0x1,    // Only one value.
    Request = 0x2,  // Request to retrieve the last state from the server.
    Awaiting = 0x3, // Send a response that the server is awaiting for client to send a request.
    Done = 0x4,     // Stream is finished.
}

impl From<i64> for AbiStreamSenderState {
    fn from(value: i64) -> Self {
        match value {
            0x0 => AbiStreamSenderState::Ok,
            0x1 => AbiStreamSenderState::Value,
            0x2 => AbiStreamSenderState::Request,
            0x3 => AbiStreamSenderState::Awaiting,
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
            AbiStreamSenderState::Awaiting => 0x3,
            AbiStreamSenderState::Done => 0x4,
        }
    }
}

impl From<AbiStreamReceiverState> for crate::StreamReceiver {
    fn from(value: AbiStreamReceiverState) -> Self {
        match value {
            AbiStreamReceiverState::Ok => crate::StreamReceiver::Ok,
            AbiStreamReceiverState::Close => crate::StreamReceiver::Close,
            AbiStreamReceiverState::Start => crate::StreamReceiver::Start,
            AbiStreamReceiverState::Pause => crate::StreamReceiver::Pause,
            AbiStreamReceiverState::Resume => crate::StreamReceiver::Resume,
            AbiStreamReceiverState::Request => crate::StreamReceiver::Request,
        }
    }
}

impl From<crate::StreamReceiver> for AbiStreamReceiverState {
    fn from(value: crate::StreamReceiver) -> Self {
        match value {
            crate::StreamReceiver::Ok => AbiStreamReceiverState::Ok,
            crate::StreamReceiver::Close => AbiStreamReceiverState::Close,
            crate::StreamReceiver::Start => AbiStreamReceiverState::Start,
            crate::StreamReceiver::Pause => AbiStreamReceiverState::Pause,
            crate::StreamReceiver::Resume => AbiStreamReceiverState::Resume,
            crate::StreamReceiver::Request => AbiStreamReceiverState::Request,
        }
    }
}

#[repr(i64)]
#[derive(Debug)]
pub enum AbiStreamReceiverState {
    Ok = 0x0,      // Nothing
    Close = 0x1,   // Closes the stream.
    Start = 0x2,   // Sent by the client with the function arguments. The first call made.
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

#[repr(i64)]
#[derive(Debug)]
pub enum MethodType {
    CreateInstance = 0x10,
    DisposeInstance = 0x11,
    MethodCall = 0x12,
    StreamValue = 0x13, // Basically, a stream sender response.
}

impl From<i64> for MethodType {
    fn from(value: i64) -> Self {
        match value {
            0x10 => MethodType::CreateInstance,
            0x11 => MethodType::DisposeInstance,
            0x12 => MethodType::MethodCall,
            0x13 => MethodType::StreamValue,
            _ => panic!("Invalid type value: `{}`", value),
        }
    }
}

impl From<MethodType> for i64 {
    fn from(value: MethodType) -> Self {
        match value {
            MethodType::CreateInstance => 0x10,
            MethodType::DisposeInstance => 0x11,
            MethodType::MethodCall => 0x12,
            MethodType::StreamValue => 0x13,
        }
    }
}

#[repr(i64)]
pub enum MethodCallType {
    Stream = 0x20,
    Method = 0x21,
}

impl From<i64> for MethodCallType {
    fn from(value: i64) -> Self {
        match value {
            0x20 => MethodCallType::Stream,
            0x21 => MethodCallType::Method,
            _ => panic!("Invalid type value: `{}`", value),
        }
    }
}

impl From<MethodCallType> for i64 {
    fn from(value: MethodCallType) -> Self {
        match value {
            MethodCallType::Stream => 0x20,
            MethodCallType::Method => 0x21,
        }
    }
}
