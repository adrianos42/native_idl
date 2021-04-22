pub mod ffi;
pub mod abi;

pub use uuid::Uuid;
pub use async_trait::async_trait;

pub enum StreamSender<R> {
    Ok,
    Value(R),
    Request,
    Awaiting,
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

#[async_trait]
pub trait StreamInstance<R>: Send + Sync {
    async fn send(&self, value: StreamSender<R>) -> StreamReceiver;
    async fn id(&self) -> i64;
}