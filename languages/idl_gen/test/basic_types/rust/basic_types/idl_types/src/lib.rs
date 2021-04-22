pub mod idl_impl;
pub use idl_internal;
pub use idl_internal::{StreamInstance, StreamReceiver, StreamSender, Uuid};
pub use idl_internal::async_trait;

pub use tokio;
pub use futures;
pub use tungstenite;
pub use url;
pub use tokio_tungstenite;
pub use byteorder;

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
