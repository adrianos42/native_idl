pub mod ffi;

pub use uuid::Uuid;

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
