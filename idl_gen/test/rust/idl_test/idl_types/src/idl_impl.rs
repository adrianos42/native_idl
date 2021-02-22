use idl_internal::{StreamInstance, StreamReceiver, StreamSender, Uuid};
pub trait TestInstance {
    fn test_int(&mut self, value: i64) -> i64;
    fn test_bool(&mut self, value: bool) -> bool;
    fn test_float(&mut self, value: f64) -> f64;
    fn test_string(&mut self, value: String) -> String;
    fn test_none(&mut self, value: ());
    fn test_bytes(&mut self, value: Vec<u8>) -> Vec<u8>;
    fn test_uuid(&mut self, value: Uuid) -> Uuid;
    fn test_int_array(&mut self, value: Vec<i64>) -> Vec<i64>;
    fn test_point_struct(&mut self, value: crate::Point) -> crate::Point;
    fn test_int_map(
        &mut self,
        value: ::std::collections::HashMap<i64, String>,
    ) -> ::std::collections::HashMap<i64, String>;
    fn test_int_int_map(
        &mut self,
        value: ::std::collections::HashMap<i64, i64>,
    ) -> ::std::collections::HashMap<i64, i64>;
    fn test_types(&mut self, value: crate::Types) -> crate::Types;
    fn test_option(&mut self, value: Option<i64>) -> Option<i64>;
    fn test_none_option(&mut self) -> Option<i64>;
    fn test_result(&mut self, value: String) -> Result<String, bool>;
    fn test_result_err(&mut self) -> Result<String, bool>;
    fn test_int_stream(&mut self, stream_instance: Box<dyn StreamInstance + Send>);
    fn test_int_stream_stream(
        &mut self,
        stream_instance: Box<dyn StreamInstance + Send>,
        stream: StreamReceiver,
    ) -> StreamSender<String>;
    fn test_names(&mut self, value: crate::Names) -> crate::Names;
    fn test_values(&mut self, value: crate::ValuesFloat) -> crate::ValuesFloat;
}
