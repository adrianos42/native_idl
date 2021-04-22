use regex::{Captures, Regex};
use std::io::Write;
use std::process::{Command, Stdio};

pub(super) trait StringPros {
    fn to_pascal_case(&self) -> String;
    fn to_snake_case(&self) -> String;
    fn to_snake_case_upper(&self) -> String;
}

impl StringPros for str {
    fn to_pascal_case(&self) -> String {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(^[a-z])|(?:_([a-z]))|(_)").unwrap();
        }

        RE.replace_all(&self, |caps: &Captures| {
            if let Some(cap) = caps.get(1) {
                cap.as_str().to_uppercase()
            } else if let Some(cap) = caps.get(2) {
                cap.as_str().to_uppercase()
            } else {
                "".to_owned()
            }
        })
        .to_string()
    }

    fn to_snake_case(&self) -> String {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(^[A-Z])|([A-Z]$)|([A-Z])").unwrap();
        }

        RE.replace_all(self, |caps: &Captures| {
            if let Some(cap) = caps.get(1) {
                cap.as_str().to_lowercase()
            } else if let Some(cap) = caps.get(2) {
                cap.as_str().to_lowercase()
            } else {
                "_".to_owned() + &caps.get(3).unwrap().as_str().to_lowercase()
            }
        })
        .to_string()
    }

    fn to_snake_case_upper(&self) -> String {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(^[A-Z])|([A-Z]$)|([A-Z])|([a-z])").unwrap();
        }

        RE.replace_all(self, |caps: &Captures| {
            if let Some(cap) = caps.get(1) {
                cap.as_str().to_owned()
            } else if let Some(cap) = caps.get(2) {
                cap.as_str().to_owned()
            } else if let Some(cap) = caps.get(3) {
                "_".to_owned() + cap.as_str()
            } else {
                caps.get(4).unwrap().as_str().to_uppercase()
            }
        })
        .to_string()
    }
}

// pub fn to_hex_string(bytes: &[u8]) -> String {
//     let result: Vec<String> = bytes.iter().map(|b| format!("{:02x}", b)).collect();
//     result.join("")
// }

pub(super) trait StringRustFmt {
    fn rust_fmt(self) -> String;
}

impl StringRustFmt for String {
    fn rust_fmt(self) -> String {
        let mut cmd = Command::new("rustfmt");
        cmd.arg("--edition=2018");

        cmd.stdin(Stdio::piped()).stdout(Stdio::piped());

        let mut child = cmd.spawn().unwrap();
        let mut child_stdin = child.stdin.take().unwrap();
        let mut child_stdout = child.stdout.take().unwrap();

        let stdin_handle = std::thread::spawn(move || {
            let _ = child_stdin.write_all(self.as_bytes());
            self
        });

        let mut output = vec![];
        std::io::copy(&mut child_stdout, &mut output).unwrap();

        let status = child.wait().unwrap();
        let source = stdin_handle.join().unwrap();

        match String::from_utf8(output) {
            Ok(bindings) => match status.code() {
                Some(0) => bindings,
                _ => "".to_owned(),
            },
            _ => source,
        }
    }
}