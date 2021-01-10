#[macro_use]
extern crate lazy_static;

pub mod ffi;

pub enum StreamError {
    Undefined,
    UnknownState,
    Schedule,
}

pub enum StreamSender<R> {
    Ok,
    Error(StreamError),
    Value(R),
    Partial {
        index: i64,
        length: i64,
        value: R,
    },
    Waiting {
        length: i64,
    },
    Request,
    Done,
}

pub enum StreamReceiver {
    Ok,
    Error(StreamError),
    Close,
    Send { index: i64, length: i64 },
    Create,
    Pause,
    Resume,
    Request,
}

pub trait StreamInstance<T> {
    fn wake_client(&self);
    fn get_handle(&self) -> i64;
}
