//use calc_manager_types::{idl_impl::*, idl_types::*};
use std::sync::{Arc, RwLock};
use std::{collections::HashMap, thread::sleep, time::Duration};

pub use idl_types;
use idl_types::{
    idl_impl::TestInstance,
    idl_internal::{StreamInstance, StreamReceiver, StreamSender},
};

pub struct Test {
    count: i64,
}

impl Test {
    pub fn new() -> Self {
        Self { count: 0 }
    }
}

// impl TestStatic for Test {
//     fn test_static(value: i64) -> i64 {
//         value
//     }
// }

#[idl_types::async_trait]
impl TestInstance for Test {
    // fn test_int(&mut self, value: i64) -> i64 {
    //     value
    // }

    // fn test_bool(&mut self, value: bool) -> bool {
    //     value
    // }

    // fn test_float(&mut self, value: f64) -> f64 {
    //     value
    // }

    // fn test_string(&mut self, value: String) -> String {
    //     value
    // }

    // fn test_bytes(&mut self, value: Vec<u8>) -> Vec<u8> {
    //     value
    // }

    // fn test_uuid(&mut self, value: Uuid) -> Uuid {
    //     value
    // }

    // fn test_int_array(&mut self, value: Vec<i64>) -> Vec<i64> {
    //     value
    // }

    // fn test_map(&mut self, value: HashMap<String, f64>) -> HashMap<String, f64> {
    //     value
    // }

    // fn test_struct(&mut self, value: Point) -> Point {
    //     value
    // }

    // fn test_types(&mut self, value: Types) -> Types {
    //     value
    // }

    // fn test_enum(&mut self, value: Names) -> Names {
    //     value
    // }

    async fn test_stream(&mut self, stream_instance: Box<dyn StreamInstance<i64>>) {
        tokio::spawn(async move {
            let mut count = 0;
            loop {
                count += 1;
                //tokio::time::sleep(Duration::from_nanos(1)).await;
                if let StreamReceiver::Close =
                    stream_instance.send(StreamSender::Value(count)).await
                {
                    return;
                }
            }
        });
    }

    async fn test_stream_stream(
        &mut self,
        stream_instance: Box<dyn StreamInstance<i64>>,
        stream_receiver: StreamReceiver,
    ) {
        match stream_receiver {
            StreamReceiver::Ok | StreamReceiver::Request => {}
            StreamReceiver::Close => {}
            StreamReceiver::Start => {}
            StreamReceiver::Pause => {}
            StreamReceiver::Resume => {}
        }
    }
}
