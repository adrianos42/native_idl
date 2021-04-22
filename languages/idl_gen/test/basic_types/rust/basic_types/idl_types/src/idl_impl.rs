use idl_internal::{StreamInstance, StreamReceiver, StreamSender, Uuid};
#[idl_internal::async_trait]
pub trait TestInstance: Send + Sync {
    async fn test_stream(&mut self, stream_instance: Box<dyn StreamInstance<i64>>);
    async fn test_stream_stream(
        &mut self,
        stream_instance: Box<dyn StreamInstance<i64>>,
        stream_receiver: StreamReceiver,
    );
}
