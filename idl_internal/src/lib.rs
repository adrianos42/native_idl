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
    Value(R),
    Request,
    Waiting,
    Done,
    Closed,
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
    fn get_handle(&self) -> i64;
}
