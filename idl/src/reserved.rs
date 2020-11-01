use super::range::Range;
use regex::Regex;
use thiserror::Error;
use std::fmt;

static RESERVED: &'static [&str] = &[
    "abstract",
    "and",
    "as",
    "asm",
    "assert",
    "async",
    "attribute",
    "auto",
    "await",
    "become",
    "begin",
    "box",
    "break",
    "case",
    "catch",
    "class",
    "contexpr",
    "continue",
    "covariant",
    "crate",
    "decltype",
    "def",
    "default",
    "deferred",
    "define",
    "delete",
    "do",
    "dyn",
    "dynamic",
    "else",
    "end",
    "error",
    "exception",
    "explicit",
    "export",
    "extends",
    "extension",
    "extern",
    "external",
    "false",
    "final",
    "finally",
    "fn",
    "for",
    "friend",
    "get",
    "goto",
    "hashcode",
    "hide",
    "if",
    "impl",
    "implements",
    "in",
    "inline",
    "instance",
    "is",
    "let",
    "loop",
    "macro",
    "match",
    "mixin",
    "mod",
    "move",
    "mut",
    "mutable",
    "nand",
    "new",
    "nil",
    "noexpect",
    "nor",
    "nosuchfunction",
    "null",
    "nullptr",
    "on",
    "operator",
    "option",
    "or",
    "out",
    "override",
    "part",
    "priv",
    "private",
    "protected",
    "pub",
    "public",
    "ref",
    "register",
    "result",
    "rethrow",
    "return",
    "runtimetype",
    "self",
    "service",
    "set",
    "short",
    "show",
    "signed",
    "sizeof",
    "static",
    "structure",
    "super",
    "switch",
    "sync",
    "this",
    "throw",
    "time",
    "tostring",
    "trait",
    "true",
    "try",
    "typedef",
    "typeof",
    "union",
    "unsafe",
    "unsigned",
    "unsized",
    "use",
    "var",
    "version",
    "virtual",
    "void",
    "volatile",
    "where",
    "while",
    "with",
    "xor",
    "yield",
];

static RESERVED_ATTRIBUTES_NAMES: &'static [&str] = &[
    "attribute",
    "default",
    "deferred",
    "export",
    "external",
    "hide",
    "in",
    "is",
    "on",
    "operator",
    "service",
    "time",
    "version",
];

static RESERVED_TYPES: &'static [&str] = &[
    "void",
    "uint",
    "time",
    "u8",
    "u16",
    "u32",
    "u64",
    "u128",
    "i8",
    "i16",
    "i32",
    "i64",
    "i128",
    "f16",
    "f32",
    "f64",
    "f128",
    "double",
    "decimal",
    "date",
    "char",
    "arrayi8",
    "arrayi16",
    "arrayi32",
    "arrayi64",
    "arrayi128",
    "arrayu8",
    "arrayu16",
    "arrayu32",
    "arrayu64",
    "arrayu128",
    "arrayf16",
    "arrayf32",
    "arrayf64",
];

#[derive(Error, Debug, Clone)]
pub struct NameError(
    pub(super) NameErrorKind,
    pub(super) Range,
    pub(super) String,
);

#[derive(Debug, Copy, Clone)]
pub enum NameErrorKind {
    InvalidFieldName,
    InvalidTypeName,
    ReservedName,
    ReservedType,
    ReservedAttributeName,
}

impl fmt::Display for NameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let errstr = match &self.0 {
            NameErrorKind::InvalidFieldName => format!("Invalid field name `{}`", self.2),
            NameErrorKind::InvalidTypeName => format!("Invalid type name `{}`", self.2),
            NameErrorKind::ReservedName => format!("Reserved name `{}`", self.2),
            NameErrorKind::ReservedType => format!("Reserved type `{}`", self.2),
            NameErrorKind::ReservedAttributeName => {
                format!("Reserved attribute name `{}`", self.2)
            }
        };

        write!(f, "{}", errstr)
    }
}

pub(super) fn type_name_is_valid(name: &str, range: Range) -> Result<(), NameError> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^[A-Z]+[A-Za-z0-9]*$").unwrap();
    }

    if RE.is_match(name) {
        Ok(())
    } else {
        Err(NameError(
            NameErrorKind::InvalidTypeName,
            range,
            name.to_owned(),
        ))
    }
}

pub(super) fn field_name_is_valid(name: &str, range: Range) -> Result<(), NameError> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^((?:[a-z]+[0-9]*)+(?:_[a-z0-9]+)*)$").unwrap();
    }

    if RE.is_match(name) {
        Ok(())
    } else {
        Err(NameError(
            NameErrorKind::InvalidFieldName,
            range,
            name.to_owned(),
        ))
    }
}

pub(super) fn is_reserved_word(name: &str, range: Range) -> Result<(), NameError> {
    if RESERVED.contains(&name) {
        Err(NameError(
            NameErrorKind::ReservedName,
            range,
            name.to_owned(),
        ))
    } else {
        Ok(())
    }
}

pub(super) fn is_reserved_attribute(name: &str, range: Range) -> Result<(), NameError> {
    if RESERVED_ATTRIBUTES_NAMES.contains(&name) {
        Err(NameError(
            NameErrorKind::ReservedAttributeName,
            range,
            name.to_owned(),
        ))
    } else {
        Ok(())
    }
}

pub(super) fn is_reserved_type(name: &str, range: Range) -> Result<(), NameError> {
    if RESERVED_TYPES.contains(&name) {
        Err(NameError(
            NameErrorKind::ReservedType,
            range,
            name.to_owned(),
        ))
    } else {
        Ok(())
    }
}
