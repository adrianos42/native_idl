pub mod ffi;

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
    fn get_handle(&self) -> i64;
}
