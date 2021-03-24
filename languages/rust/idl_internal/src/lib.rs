pub mod ffi;
pub mod abi;

pub use uuid::Uuid;
pub use byteorder;
pub use tungstenite;
pub use url;
pub use tokio;
pub use tokio_tungstenite;
pub use lazy_static;
pub use futures;

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
