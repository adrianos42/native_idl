use idl_internal::{StreamInstance, StreamReceiver, StreamSender, Uuid};
pub trait TestInstance {
    fn test_int(&mut self, value: i64) -> i64;
    fn test_bool(&mut self, value: bool) -> bool;
    fn test_float(&mut self, value: f64) -> f64;
    fn test_string(&mut self, value: String) -> String;
    fn test_none(&mut self, value: ());
    fn test_bytes(&mut self, value: Vec<u8>) -> Vec<u8>;
    fn test_uuid(&mut self, value: Uuid) -> Uuid;
}
pub trait TestStatic {
    fn test_static(value: i64) -> i64;
}
