use idl_internal::{StreamInstance, StreamReceiver, StreamSender, Uuid};
pub trait TestInstance {
    fn test_stream(&mut self, stream_instance: Box<dyn StreamInstance + Send + Sync>);
    fn test_stream_stream(
        &mut self,
        stream_instance: Box<dyn StreamInstance + Send + Sync>,
        stream_receiver: StreamReceiver,
    ) -> StreamSender<i64>;
}
pub trait TetStStatic {
    fn test_int(value: i64) -> i64;
}
