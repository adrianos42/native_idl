#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate quote;

mod binary_json;
pub mod lang;
pub mod rust;
pub(crate) mod cargo_md;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum IdlGenError {
    #[error(transparent)]
    LanguageBinary(#[from] binary_json::BinaryGenError),
    #[error(transparent)]
    RustGenError(#[from] rust::RustGenError),
}

pub fn for_language(language: &str) -> Result<Box<dyn IdlGen>, IdlGenError> {
    Ok(match language {
        "rust" => Box::new(rust::RustGen::new()?),
        name => Box::new(binary_json::BinaryGen::new(name)?),
    })
}

pub trait IdlGen {
    fn send_request(
        &self,
        request: lang::LanguageRequest,
    ) -> Result<lang::LanguageResponse, IdlGenError>;
}
