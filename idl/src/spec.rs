use serde::Deserialize;
use std::fmt;
use toml::Deserializer;
use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum SpecError {
    #[error("Undefined `{0}`")]
    Abort(String),
    #[error(transparent)]
    TomlParser(#[from] toml::de::Error),
    #[error("Closed")]
    Closed,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Spec {
    pub library: Library,
    pub server: Server,
    pub client: Client,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Server {
    pub layers: Vec<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Client {
    pub layers: Vec<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Library {
    pub name: String,
    pub authors: Option<String>,
    pub version: String,
}

impl Spec {
    pub(crate) fn new(text: &str) -> Result<Spec, SpecError> {
        let result = toml::from_str(text)?;

        Ok(result)
    }
}
