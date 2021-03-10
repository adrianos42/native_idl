//use calc_manager_types::{idl_impl::*, idl_types::*};
use std::{collections::HashMap, thread::sleep, time::Duration};
use std::sync::{Arc, RwLock};

pub use idl_types;
use idl_types::{idl_impl::{TestInstance, TestStatic}, idl_internal::*};

pub struct Test {
    
}

impl Test {
    pub fn new() -> Self {
        Self {}
    }
}

impl TestStatic for Test {
    fn test_static(value: i64) -> i64 {
        value
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
}