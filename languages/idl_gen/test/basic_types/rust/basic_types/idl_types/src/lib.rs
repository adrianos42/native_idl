pub mod idl_impl;
pub use idl_internal;
pub use idl_internal::Uuid;
#[derive(Debug, Clone)]
pub struct Point {
    pub x: i64,
    pub y: i64,
    pub name: String,
}
#[derive(Debug, Clone)]
pub enum Types {
    Integers(i64),
    Sttrings(String),
    Ntsst(Box<Point>),
}
#[derive(Debug, Copy, Clone)]
pub enum Names {
    Pamela,
    Bianca,
    Dominique,
    Stephany,
}
