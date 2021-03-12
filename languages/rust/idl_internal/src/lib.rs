pub mod ffi;

pub use uuid::Uuid;
pub use byteorder;
pub use tungstenite;
pub use url;

pub enum StreamSender<R> {
    Ok,
    Value(R),
    Request,
    Waiting,
    Done,
}

pub enum StreamReceiver {
    Ok,
    Close,
    Start,
    Pause,
    Resume,
    Request,
}

pub trait StreamInstance {
    fn wake_client(&self);
    fn get_id(&self) -> i64;
}

pub enum MethodType {
    CreateInstance = 0x10,
    DisposeInstance = 0x11,
    MethodCall = 0x12,
}

impl From<i64> for MethodType {
    fn from(value: i64) -> Self {
        match value {
            0x10 => MethodType::CreateInstance,
            0x11 => MethodType::DisposeInstance,
            0x12 => MethodType::MethodCall,
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
        }
    }
}

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