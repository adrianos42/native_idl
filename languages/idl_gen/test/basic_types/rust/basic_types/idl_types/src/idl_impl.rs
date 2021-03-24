use idl_internal::{StreamInstance, StreamReceiver, StreamSender, Uuid};
pub trait TestInstance {
    fn test_int(&mut self, value: i64) -> i64;
    fn test_bool(&mut self, value: bool) -> bool;
    fn test_float(&mut self, value: f64) -> f64;
    fn test_string(&mut self, value: String) -> String;
    fn test_bytes(&mut self, value: Vec<u8>) -> Vec<u8>;
    fn test_uuid(&mut self, value: Uuid) -> Uuid;
    fn test_int_array(&mut self, value: Vec<i64>) -> Vec<i64>;
    fn test_map(
        &mut self,
        value: ::std::collections::HashMap<String, f64>,
    ) -> ::std::collections::HashMap<String, f64>;
    fn test_struct(&mut self, value: super::Point) -> super::Point;
    fn test_types(&mut self, value: super::Types) -> super::Types;
    fn test_enum(&mut self, value: super::Names) -> super::Names;
    fn test_stream(&mut self, stream_instance: Box<dyn StreamInstance + Send>);
    fn test_stream_stream(
        &mut self,
        stream_instance: Box<dyn StreamInstance + Send>,
        stream_receiver: StreamReceiver,
    ) -> StreamSender<i64>;
}
pub trait TestStatic {
    fn test_static(value: i64) -> i64;
}
pub trait TetStStatic {
    fn test_int(value: i64) -> i64;
}
