pub mod idl_impl;
pub use idl_internal;
pub use idl_internal::Uuid;
#[derive(Debug, Clone)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}
#[derive(Debug, Clone)]
pub enum Types {
    IntType(i64),
    StringType(String),
}
pub type Names = String;
pub const NAMES_NAME: &str = "Adriano";
pub const NAMES_SURNAME: &str = "Souza";
pub type ValuesFloat = f64;
pub const VALUES_FLOAT_PI: f64 = 3.14;
pub const VALUES_FLOAT_SQRT: f64 = 2.55;
pub const VALUES_FLOAT_ONEDW: f64 = 122332.0;
pub type ValuesInteger = i64;
pub const VALUES_INTEGER_ONE: i64 = 1;
pub const VALUES_INTEGER_TWO: i64 = 2;
pub const VALUES_INTEGER_THREE: i64 = 3;
