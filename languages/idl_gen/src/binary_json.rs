use std::cell::RefCell;
use std::process::{Command, Stdio};
use std::io::Write;
use thiserror::Error;

use crate::lang::*;

#[derive(Debug, Error)]
pub enum BinaryGenError {
    #[error("Json error.")]
    JsonError,
    #[error("Process creation.")]
    ProcessCreation,
    #[error("Process error `{0}`.")]
    ProcessError(String)
}

pub struct BinaryGen {
    child: RefCell<std::process::Child>,
}

impl BinaryGen {
    pub fn new(name: &str) -> Result<Self, BinaryGenError> {
        let mut cmd = Command::new(format!("idl_{}", name));

        cmd.stdin(Stdio::piped()).stdout(Stdio::piped());

        let child = cmd.spawn().map_err(|_| BinaryGenError::ProcessCreation)?;

        Ok(Self {
            child: RefCell::new(child),
        })
    }
}

impl crate::IdlGen for BinaryGen {
    fn send_request(
        &self,
        request: LanguageRequest,
    ) -> Result<LanguageResponse, crate::IdlGenError> {
        let mut child = self.child.borrow_mut();

        let request = serde_json::to_string(&request).map_err(|_| BinaryGenError::JsonError)?;

        let mut child_stdin = child.stdin.take().ok_or(BinaryGenError::ProcessCreation)?;
        let mut child_stdout = child.stdout.take().ok_or(BinaryGenError::ProcessCreation)?;

        let stdin_handle = std::thread::spawn(move || {
            let _ = child_stdin.write_all(request.as_bytes());
            request
        });

        let mut output = vec![];

        std::io::copy(&mut child_stdout, &mut output).unwrap();

        let status = child.wait().unwrap();
        let source = stdin_handle
            .join()
            .map_err(|_| BinaryGenError::ProcessCreation)?;

        let response = match String::from_utf8(output) {
            Ok(bindings) => match status.code() {
                Some(0) => bindings,
                _ => "".to_owned(),
            },
            _ => source,
        };

        let response: LanguageResponse =
            serde_json::from_str(&response).map_err(|_| BinaryGenError::JsonError)?;

        Ok(response)
    }
}
