//use calc_manager_types::{idl_impl::*, idl_types::*};
use std::{collections::HashMap, thread::sleep, time::Duration};
use std::sync::{Arc, RwLock};

pub use idl_types;
use idl_types::{idl_impl::TestInstance, idl_internal::*};

pub struct Test {
    
}

impl Test {
    pub fn new() -> Self {
        Self {}
    }
}

impl TestInstance for Test {
    fn test_int(&mut self, value: i64) -> i64 {
        value
    }

    fn test_bool(&mut self, value: bool) -> bool {
        value
    }

    fn test_float(&mut self, value: f64) -> f64 {
        value
    }

    fn test_string(&mut self, value: String) -> String {
        value
    }

    fn test_none(&mut self, value: ()) {
        value
    }

    fn test_bytes(&mut self, value: Vec<u8>) -> Vec<u8> {
        value
    }

    fn test_uuid(&mut self, value: Uuid) -> Uuid {
        value
    }

    fn test_int_array(&mut self, value: Vec<i64>) -> Vec<i64> {
        value
    }

    fn test_point_struct(&mut self, value: idl_types::Point) -> idl_types::Point {
        value
    }

    fn test_int_map(
        &mut self,
        value: HashMap<i64, String>,
    ) -> HashMap<i64, String> {
        value
    }

    fn test_int_int_map(
        &mut self,
        value: HashMap<i64, i64>,
    ) -> HashMap<i64, i64> {
        value
    }

    fn test_types(&mut self, value: idl_types::Types) -> idl_types::Types {
        value
    }

    fn test_option(&mut self, value: Option<i64>) -> Option<i64> {
        value
    }

    fn test_none_option(&mut self) -> Option<i64> {
        Some(42)
    }

    fn test_result(&mut self, value: String) -> Result<String, bool> {
        Ok("value result".to_owned())
    }

    fn test_result_err(&mut self) -> Result<String, bool> {
        Err(false)
    }

    fn test_int_stream(&mut self, stream_instance: Box<dyn StreamInstance + Send>) {
    }

    fn test_int_stream_stream(
        &mut self,
        stream_instance: Box<dyn StreamInstance + Send>,
        stream: StreamReceiver,
    ) -> StreamSender<String> {
        StreamSender::Done
    }

    fn test_names(&mut self, value: idl_types::Names) -> idl_types::Names {
        value
    }

    fn test_values(&mut self, value: idl_types::ValuesFloat) -> idl_types::ValuesFloat {
        value
    }
}