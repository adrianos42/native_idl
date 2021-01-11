#[macro_use]
extern crate lazy_static;

pub mod ffi;

pub enum StreamError {
    Undefined,
    InvalidState,
    Schedule,
    Closed,
}

pub enum StreamSender<R> {
    Ok,
    Waiting,
    Done,
    Value(R),
    Request,
    Error(StreamError),
}

pub enum StreamReceiver {
    Ok,
    Close,
    Start,
    Pause,
    Resume,
    Request,
    Error(StreamError),
}

pub trait StreamInstance {
    fn wake_client(&self);
    fn get_handle(&self) -> i64;
}
