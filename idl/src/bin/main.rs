use regex::Regex;
use std::borrow::Cow;
use std::fmt;
use std::path::Path;

use tokio::fs;
use tokio::io;

use native_idl::analyzer;
use native_idl::parser::Parser;

#[tokio::main]
async fn main() {
    if let Ok(text) = fs::read_to_string("./idl_types.idl").await {
        match Parser::parse(text.as_str()) {
            Ok(parser) => {
                println!(
                    "{}",
                    native_idl::formatting::format_document(&parser).expect("Invalid document.")
                );
              //  println!("Parsed: {:?}", parser);
            }
            Err(err) => println!("Parser error: {:?}", err),
        }
    }
}
