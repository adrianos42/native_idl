use regex::{Captures, Regex};

pub trait StringPros {
    fn to_pascal_case(&self) -> String;
    fn to_snake_case(&self) -> String;
}
 
impl StringPros for str {
    fn to_pascal_case(&self) -> String {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(^[a-z])|(?:_(a-z))").unwrap();
        }

        RE.replace_all(&self, |caps: &Captures| {
            if let Some(cap) = caps.get(1) {
                cap.as_str().to_uppercase()
            } else {
                caps.get(2).unwrap().as_str().to_lowercase()
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
                "_".to_owned() + caps.get(3).unwrap().as_str()
            }
        }).to_string()
    }
}
