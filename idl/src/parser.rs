use std::convert::From;
use std::fmt;
use std::sync::Arc;

use super::scanner::{ContextStream, ScError, WordStream};

pub use super::scanner::{AttributeNames, Keywords, NativeTypes};
pub use super::range::{Position, Range};

#[derive(Debug)]
pub enum ParserNode {
    Library(String),
    Imports(Vec<String>),
    Comment(String),
    InterfaceComment(String),
    Interface(Interface),
    StructComment(String),
    Struct(Struct),
    EnumComment(String),
    Enum(Enum),
    TypeListComment(String),
    TypeList(TypeList),
    ConstComment(String),
    Const(Const),
    StreamComment(String),
    Stream(Stream),
    FactoryComment(String),
    Factory(Factory),
}

#[derive(Debug)]
pub struct Interface {
    pub ident: Arc<Type>,
    pub fields: Vec<InterfaceNode>,
    pub range: Range,
}

#[derive(Debug)]
pub enum InterfaceNode {
    InterfaceField(Box<InterfaceField>),
    Comment(String),
}

#[derive(Debug)]
pub struct InterfaceField {
    pub attributes: Vec<Attribute>,
    pub ident: String,
    pub ty: Arc<Type>,
    pub range: Range,
}

impl fmt::Display for InterfaceField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let attributes = self
            .attributes
            .iter()
            .fold("".to_owned(), |mut acc, value| {
                acc += value.to_string().as_str();
                acc
            });

        if !attributes.is_empty() {
            return write!(f, "{} {}: {}", attributes, self.ident.as_str(), self.ty);
        }

        write!(f, "{}: {}", self.ident.as_str(), self.ty)
    }
}

#[derive(Debug)]
pub enum Type {
    Native(Arc<TypeNative>),
    Name(Arc<TypeName>),
    Function(Arc<TypeFunction>),
    Tuple(Arc<TypeTuple>),
    Array(Arc<TypeArray>),
    Map(Arc<TypeMap>),
    Result(Arc<TypeResult>),
    Option(Arc<TypeOption>),
}

impl Type {
    pub fn get_range(&self) -> Range {
        match self {
            Type::Native(value) => value.range,
            Type::Name(value) => value.range,
            Type::Function(value) => value.range,
            Type::Tuple(value) => value.range,
            Type::Array(value) => value.range,
            Type::Map(value) => value.range,
            Type::Result(value) => value.range,
            Type::Option(value) => value.range,
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Type::Array(value) => value.to_string(),
                Type::Map(value) => value.to_string(),
                Type::Function(value) => value.to_string(),
                Type::Native(value) => value.to_string(),
                Type::Tuple(value) => value.to_string(),
                Type::Name(value) => value.to_string(),
                Type::Result(value) => value.to_string(),
                Type::Option(value) => value.to_string(),
            }
        )
    }
}

#[derive(Debug)]
pub struct TypeNative {
    pub ty: NativeTypes,
    pub range: Range,
}

impl fmt::Display for TypeNative {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self.ty {
                NativeTypes::Int => "int".to_owned(),
                NativeTypes::Bytes => "bytes".to_owned(),
                NativeTypes::Float => "float".to_owned(),
                NativeTypes::String => "string".to_owned(),
                NativeTypes::Bool => "bool".to_owned(),
                NativeTypes::None => "none".to_owned(),
            }
        )
    }
}

#[derive(Debug)]
pub struct TypeName {
    pub ident: String,
    pub range: Range,
}

impl fmt::Display for TypeName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.ident.as_str())
    }
}

#[derive(Debug)]
pub struct TypeArray {
    pub array_ty: Arc<Type>,
    pub range: Range,
}

impl fmt::Display for TypeArray {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}[]", self.array_ty)
    }
}

#[derive(Debug)]
pub struct TypeMap {
    pub m_ty: Arc<Type>,
    pub index_ty: Arc<Type>,
    pub range: Range,
}

impl fmt::Display for TypeMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}[{}]", self.m_ty, self.index_ty,)
    }
}

#[derive(Debug)]
pub struct TypeResult {
    pub ok_ty: Arc<Type>,
    pub err_ty: Arc<Type>,
    pub range: Range,
}

impl fmt::Display for TypeResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}; {}]", self.ok_ty, self.err_ty,)
    }
}

#[derive(Debug)]
pub struct TypeOption {
    pub some_ty: Arc<Type>,
    pub range: Range,
}

impl fmt::Display for TypeOption {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.some_ty)
    }
}

#[derive(Debug)]
pub struct TypeFunction {
    pub args: Arc<Type>,
    pub ret_ty: Arc<Type>,
    pub range: Range,
}

impl fmt::Display for TypeFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} -> {}", self.args, self.ret_ty,)
    }
}

#[derive(Debug)]
pub struct TypeTuple {
    pub ty_list: Vec<TupleEntry>,
    pub range: Range,
}

impl fmt::Display for TypeTuple {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({})",
            self.ty_list.iter().fold("".to_owned(), |acc, li| {
                let appn = if acc.is_empty() { "" } else { ", " };
                acc + appn + li.to_string().as_str()
            })
        )
    }
}

#[derive(Debug)]
pub struct TupleEntry {
    pub ident: String,
    pub ty: Arc<Type>,
    pub range: Range,
}

impl fmt::Display for TupleEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.ident, self.ty)
    }
}

#[derive(Debug)]
pub struct TypeList {
    pub ident: Arc<Type>,
    pub ty_list: Vec<TypeListNode>,
    pub range: Range,
}

#[derive(Debug)]
pub enum TypeListNode {
    TypeListField(TypeListField),
    Comment(String),
}

#[derive(Debug)]
pub struct TypeListField {
    pub ident: String,
    pub ty: Arc<Type>,
    pub range: Range,
}

impl fmt::Display for TypeListField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.ident, self.ty)
    }
}

#[derive(Debug)]
pub struct Stream {
    pub ident: Arc<Type>,
    pub fields: Vec<StructNode>,
    pub range: Range,
}

#[derive(Debug)]
pub struct Struct {
    pub ident: Arc<Type>,
    pub fields: Vec<StructNode>,
    pub range: Range,
}

#[derive(Debug)]
pub enum StructNode {
    StructField(Box<StructField>),
    Comment(String),
}

#[derive(Debug)]
pub struct StructField {
    pub ident: String,
    pub ty: Arc<Type>,
    pub range: Range,
}

impl fmt::Display for StructField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.ident.as_str(), self.ty)
    }
}

#[derive(Debug)]
pub struct Factory {
    pub ident: Arc<Type>,
    pub fields: Vec<InterfaceNode>,
    pub range: Range,
}

#[derive(Debug)]
pub struct Enum {
    pub ident: Arc<Type>,
    pub fields: Vec<EnumNode>,
    pub range: Range,
}

#[derive(Debug)]
pub enum EnumNode {
    EnumField(Box<EnumField>),
    Comment(String),
}

#[derive(Debug)]
pub struct EnumField {
    pub ident: String,
    pub range: Range,
}

impl fmt::Display for EnumField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.ident.as_str())
    }
}

#[derive(Debug)]
pub enum AttributeField {
    Name(AttributeNames),
    UnknownName(String),
    StringField(String),
}

impl fmt::Display for AttributeField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            AttributeField::Name(name) => name.to_string(),
            AttributeField::StringField(field) => {
                let mut field_str = String::from("\"");
                field_str += field.as_str();
                field_str += "\"";
                field_str
            }
            AttributeField::UnknownName(unk_name) => unk_name.to_owned(),
        };

        write!(f, "{}", name)
    }
}

#[derive(Debug)]
pub struct Attribute {
    pub fields: Vec<AttributeField>,
    pub range: Range,
}

impl fmt::Display for Attribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = String::new();

        for attribute in self.fields.iter() {
            if !result.is_empty() {
                result += ", ";
            }
            result += attribute.to_string().as_str();
        }

        write!(f, "[{}]", result,)
    }
}

#[derive(Debug)]
pub struct Const {
    pub ident: Arc<Type>,
    pub fields: Vec<ConstNode>,
    pub const_type: ConstType,
    pub range: Range,
}

#[derive(Debug)]
pub enum ConstNode {
    ConstField(ConstField),
    Comment(String),
}

#[derive(Debug, Copy, Clone)]
pub enum ConstType {
    Int,
    Float,
    String,
}

impl fmt::Display for ConstType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ConstType::Float => "float",
                ConstType::Int => "int",
                ConstType::String => "string",
            }
        )
    }
}

#[derive(Debug)]
pub struct ConstField {
    pub ident: String,
    pub value: String,
    pub range: Range,
    pub const_type: ConstType,
}

impl fmt::Display for ConstField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: {}",
            self.ident.as_str(),
            match self.const_type {
                ConstType::String => format!("\"{}\"", self.value.as_str()),
                _ => self.value.to_owned(),
            }
        )
    }
}

#[derive(Debug)]
pub struct EnumFieldError(pub EnumFieldErrorKind, pub Range);

impl fmt::Display for EnumFieldError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for EnumFieldErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let errstr = match self {
            EnumFieldErrorKind::Undefined => "Enum field error.",
            EnumFieldErrorKind::EmptyBody => "Empty enum field.",
            EnumFieldErrorKind::InvalidSymbol => "Invalid symbol.",
            EnumFieldErrorKind::MissingComma => "Missing `,`.",
            EnumFieldErrorKind::MissingCurlyBracket => "Missing `}`.",
            EnumFieldErrorKind::MissingIdentifier => "Missing identifier.",
        };

        write!(f, "{}", errstr)
    }
}

#[derive(Debug)]
pub enum EnumFieldErrorKind {
    Undefined,
    EmptyBody,
    MissingComma,
    InvalidSymbol,
    MissingIdentifier,
    MissingCurlyBracket,
}

#[derive(Debug)]
pub struct EnumError(pub EnumErrorKind, pub Range);

impl fmt::Display for EnumError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for EnumErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let errstr = match self {
            EnumErrorKind::Undefined => "Enum error.".to_owned(),
            EnumErrorKind::MissingTypeName => "Missing type name.".to_owned(),
            EnumErrorKind::TypeDeclaration => "Invalid type declaration.".to_owned(),
            EnumErrorKind::EnumField(field) => field.to_string(),
        };

        write!(f, "{}", errstr)
    }
}

#[derive(Debug)]
pub enum EnumErrorKind {
    Undefined,
    MissingTypeName,
    TypeDeclaration,
    EnumField(EnumFieldErrorKind),
}

impl From<EnumFieldError> for EnumError {
    fn from(value: EnumFieldError) -> Self {
        EnumError(EnumErrorKind::EnumField(value.0), value.1)
    }
}

#[derive(Debug)]
pub struct FactoryError(pub FactoryErrorKind, pub Range);

impl fmt::Display for FactoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for FactoryErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let errstr = match self {
            FactoryErrorKind::TypeDeclaration => "Invalid type declaration.".to_owned(),
            FactoryErrorKind::Undefined => "Factory error.".to_owned(),
            FactoryErrorKind::InterfaceField(field) => field.to_string(),
            FactoryErrorKind::MissingTypeName => "Missing type name.".to_owned(),
        };

        write!(f, "{}", errstr)
    }
}

#[derive(Debug)]
pub enum FactoryErrorKind {
    Undefined,
    MissingTypeName,
    TypeDeclaration,
    InterfaceField(InterfaceFieldErrorKind),
}

#[derive(Debug)]
pub struct InterfaceError(pub InterfaceErrorKind, pub Range);

impl fmt::Display for InterfaceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for InterfaceErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let errstr = match self {
            InterfaceErrorKind::TypeDeclaration => "Invalid type declaration.".to_owned(),
            InterfaceErrorKind::Undefined => "Interface error.".to_owned(),
            InterfaceErrorKind::InterfaceField(field) => field.to_string(),
            InterfaceErrorKind::MissingTypeName => "Missing type name.".to_owned(),
        };

        write!(f, "{}", errstr)
    }
}

#[derive(Debug)]
pub enum InterfaceErrorKind {
    Undefined,
    MissingTypeName,
    TypeDeclaration,
    InterfaceField(InterfaceFieldErrorKind),
}

#[derive(Debug)]
pub struct InterfaceFieldError(pub InterfaceFieldErrorKind, pub Range);

impl fmt::Display for InterfaceFieldError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for InterfaceFieldErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let errstr = match self {
            InterfaceFieldErrorKind::Undefined => "Interface field error.".to_owned(),
            InterfaceFieldErrorKind::Attribute(att) => att.to_string(),
            InterfaceFieldErrorKind::EmptyBody => "Empty interface field.".to_owned(),
            InterfaceFieldErrorKind::ExpectedFunctionSyntax
            | InterfaceFieldErrorKind::MissingArrowFunction => "Function syntax.".to_owned(),
            InterfaceFieldErrorKind::IncompleteField => "Incomplete field.".to_owned(),
            InterfaceFieldErrorKind::InvalidBracketPlacement => {
                "Invalid bracket placement.".to_owned()
            }
            InterfaceFieldErrorKind::InvalidSymbol => "Invalid symbol.".to_owned(),
            InterfaceFieldErrorKind::MissingAttribute => "Missing attribute.".to_owned(),
            InterfaceFieldErrorKind::MissingColon => "Missing `:`.".to_owned(),
            InterfaceFieldErrorKind::MissingCurlyBracket => "Missing `}`.".to_owned(),
            InterfaceFieldErrorKind::MissingIdentifier => "Missing identifier.".to_owned(),
            InterfaceFieldErrorKind::MultipleIdentifier => "Multiple identifier.".to_owned(),
            InterfaceFieldErrorKind::Type(ty) => ty.to_string(),
            InterfaceFieldErrorKind::TypeDeclaration => "Invalid type declaration.".to_owned(),
            InterfaceFieldErrorKind::TypeTuple(ty) => ty.to_string(),
            InterfaceFieldErrorKind::ExpectedType(ty) => ty.to_owned(),
        };

        write!(f, "{}", errstr)
    }
}

#[derive(Debug)]
pub enum InterfaceFieldErrorKind {
    Undefined,
    EmptyBody,
    IncompleteField,
    MultipleIdentifier,
    MissingIdentifier,
    InvalidSymbol,
    InvalidBracketPlacement,
    TypeDeclaration,
    MissingAttribute,
    MissingColon,
    MissingArrowFunction,
    ExpectedFunctionSyntax,
    MissingCurlyBracket,
    ExpectedType(String),
    Type(TypeErrorKind),
    TypeTuple(TypeTupleErrorKind),
    Attribute(AttributeErrorKind),
}

impl From<InterfaceFieldError> for InterfaceError {
    fn from(value: InterfaceFieldError) -> Self {
        InterfaceError(InterfaceErrorKind::InterfaceField(value.0), value.1)
    }
}

impl From<InterfaceFieldError> for FactoryError {
    fn from(value: InterfaceFieldError) -> Self {
        FactoryError(FactoryErrorKind::InterfaceField(value.0), value.1)
    }
}

impl From<AttributeError> for InterfaceFieldError {
    fn from(value: AttributeError) -> Self {
        InterfaceFieldError(InterfaceFieldErrorKind::Attribute(value.0), value.1)
    }
}

impl From<TypeTupleError> for InterfaceFieldError {
    fn from(value: TypeTupleError) -> Self {
        InterfaceFieldError(InterfaceFieldErrorKind::TypeTuple(value.0), value.1)
    }
}

impl From<TypeError> for InterfaceFieldError {
    fn from(value: TypeError) -> Self {
        InterfaceFieldError(InterfaceFieldErrorKind::Type(value.0), value.1)
    }
}

#[derive(Debug, Eq, PartialEq)]
enum InterfaceFieldParsing {
    ExpectingAttribute,
    ExpectingColon,
    ExpectingComma,
    ExpectingType,
    Type,
    Tuple,
    ExpectingReturnType,
    ReturnType,
}

#[derive(Debug)]
pub struct StreamError(pub StreamErrorKind, pub Range);

impl fmt::Display for StreamError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for StreamErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let errstr = match self {
            StreamErrorKind::Undefined => "Stream error.".to_owned(),
            StreamErrorKind::MissingTypeName => "Missing type name.".to_owned(),
            StreamErrorKind::TypeDeclaration => "Type declaration.".to_owned(),
            StreamErrorKind::StructField(field) => field.to_string(),
        };

        write!(f, "{}", errstr)
    }
}

#[derive(Debug)]
pub enum StreamErrorKind {
    Undefined,
    MissingTypeName,
    TypeDeclaration,
    StructField(StructFieldErrorKind),
}

impl From<StructFieldError> for StreamError {
    fn from(value: StructFieldError) -> Self {
        StreamError(StreamErrorKind::StructField(value.0), value.1)
    }
}

#[derive(Debug)]
pub struct StructError(pub StructErrorKind, pub Range);

impl fmt::Display for StructError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for StructErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let errstr = match self {
            StructErrorKind::Undefined => "Struct error.".to_owned(),
            StructErrorKind::MissingTypeName => "Missing type name.".to_owned(),
            StructErrorKind::TypeDeclaration => "Type declaration.".to_owned(),
            StructErrorKind::StructField(field) => field.to_string(),
        };

        write!(f, "{}", errstr)
    }
}

#[derive(Debug)]
pub enum StructErrorKind {
    Undefined,
    MissingTypeName,
    TypeDeclaration,
    StructField(StructFieldErrorKind),
}

#[derive(Debug)]
pub struct StructFieldError(pub StructFieldErrorKind, pub Range);

impl fmt::Display for StructFieldError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for StructFieldErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let errstr = match self {
            StructFieldErrorKind::Undefined => "Struct field error.".to_owned(),
            StructFieldErrorKind::EmptyBody => "Field empty body.".to_owned(),
            StructFieldErrorKind::IncompleteField => "Incomplete field.".to_owned(),
            StructFieldErrorKind::ExpectedType(ident) => format!("Expected type, `{}`", ident),
            StructFieldErrorKind::InvalidBracketPlacement => {
                "Invalid bracket placement.".to_owned()
            }
            StructFieldErrorKind::InvalidSymbol => "Invalid symbol.".to_owned(),
            StructFieldErrorKind::MissingColon => "Missing `:`.".to_owned(),
            StructFieldErrorKind::MissingComma => "Missing `,`.".to_owned(),
            StructFieldErrorKind::MissingCurlyBracket => "Missing `}`".to_owned(),
            StructFieldErrorKind::MissingIdentifier => "Missing identifier.".to_owned(),
            StructFieldErrorKind::MultipleIdentifier => "Multiple identifier.".to_owned(),
            StructFieldErrorKind::Type(ty) => ty.to_string(),
            StructFieldErrorKind::TypeDeclaration => "Type declaration.".to_owned(),
        };

        write!(f, "{}", errstr)
    }
}

#[derive(Debug)]
pub enum StructFieldErrorKind {
    Undefined,
    EmptyBody,
    IncompleteField,
    MultipleIdentifier,
    InvalidSymbol,
    InvalidBracketPlacement,
    TypeDeclaration,
    MissingIdentifier,
    MissingColon,
    MissingComma,
    MissingCurlyBracket,
    ExpectedType(String),
    Type(TypeErrorKind),
}

#[derive(Debug, Eq, PartialEq)]
enum StructFieldParsing {
    ExpectingIdentifier,
    ExpectingColon,
    ExpectingType,
    Type,
}

impl From<StructFieldError> for StructError {
    fn from(value: StructFieldError) -> Self {
        StructError(StructErrorKind::StructField(value.0), value.1)
    }
}

impl From<TypeError> for StructFieldError {
    fn from(value: TypeError) -> Self {
        StructFieldError(StructFieldErrorKind::Type(value.0), value.1)
    }
}

#[derive(Debug)]
pub struct TypeListError(pub TypeListErrorKind, pub Range);

impl fmt::Display for TypeListError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for TypeListErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let errstr = match self {
            TypeListErrorKind::Undefined => "Type list error.".to_owned(),
            TypeListErrorKind::TypeDeclaration => "Type declaration.".to_owned(),
            TypeListErrorKind::MissingTypeName => "Missing type name.".to_owned(),
            TypeListErrorKind::TypeField(ty) => ty.to_string(),
        };

        write!(f, "{}", errstr)
    }
}

#[derive(Debug)]
pub enum TypeListErrorKind {
    Undefined,
    MissingTypeName,
    TypeDeclaration,
    TypeField(TypeListFieldErrorKind),
}

#[derive(Debug)]
pub struct TypeListFieldError(pub TypeListFieldErrorKind, pub Range);

impl fmt::Display for TypeListFieldError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for TypeListFieldErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let errstr = match self {
            TypeListFieldErrorKind::Undefined => "Type list error.".to_owned(),
            TypeListFieldErrorKind::EmptyBody => "Empty body.".to_owned(),
            TypeListFieldErrorKind::InvalidBracketPlacement => {
                "Invalid bracket placement.".to_owned()
            }
            TypeListFieldErrorKind::InvalidSymbol => "Invalid symbol.".to_owned(),
            TypeListFieldErrorKind::MissingCurlyBracket => "Missing `}`.".to_owned(),
            TypeListFieldErrorKind::Type(ty) => ty.to_string(),
            TypeListFieldErrorKind::TypeDeclaration => "Type declaration.".to_owned(),
            TypeListFieldErrorKind::IncompleteField => "Incomplete field.".to_owned(),
            TypeListFieldErrorKind::MultipleIdentifier => "Multiple identifier.".to_owned(),
            TypeListFieldErrorKind::MissingIdentifier => "Missing identifier.".to_owned(),
            TypeListFieldErrorKind::MissingColon => "Missing colon.".to_owned(),
            TypeListFieldErrorKind::MissingComma => "Missing comma.".to_owned(),
            TypeListFieldErrorKind::ExpectedType(name) => format!("Expected type `{}`.", name),
        };

        write!(f, "{}", errstr)
    }
}

#[derive(Debug)]
pub enum TypeListFieldErrorKind {
    Undefined,
    EmptyBody,
    IncompleteField,
    MultipleIdentifier,
    InvalidSymbol,
    InvalidBracketPlacement,
    TypeDeclaration,
    MissingIdentifier,
    MissingColon,
    MissingComma,
    MissingCurlyBracket,
    ExpectedType(String),
    Type(TypeErrorKind),
}

#[derive(Debug, Eq, PartialEq)]
pub enum TypeListFieldParsing {
    ExpectingIdentifier,
    ExpectingColon,
    ExpectingType,
    Type,
}

impl From<TypeListFieldError> for TypeListError {
    fn from(value: TypeListFieldError) -> Self {
        TypeListError(TypeListErrorKind::TypeField(value.0), value.1)
    }
}

impl From<TypeError> for TypeListFieldError {
    fn from(value: TypeError) -> Self {
        TypeListFieldError(TypeListFieldErrorKind::Type(value.0), value.1)
    }
}

#[derive(Debug)]
pub struct TypeTupleError(pub TypeTupleErrorKind, pub Range);

impl fmt::Display for TypeTupleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for TypeTupleErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let errstr = match self {
            TypeTupleErrorKind::Undefined => "Type tuple error.".to_owned(),
            TypeTupleErrorKind::DuplicateIdentifier => "Duplicate identifier.".to_owned(),
            TypeTupleErrorKind::DuplicateType => "Duplicate type.".to_owned(),
            TypeTupleErrorKind::InvalidTupleDeclaration => "Invalid declaration.".to_owned(),
            TypeTupleErrorKind::MissingColon => "Missing `:`.".to_owned(),
            TypeTupleErrorKind::MissingIdentifier => "Missing identifier.".to_owned(),
            TypeTupleErrorKind::ExpectedType(name) => format!("Expected type `{}`.", name),
            TypeTupleErrorKind::Type(ty) => ty.to_string(),
        };

        write!(f, "{}", errstr)
    }
}

#[derive(Debug)]
pub enum TypeTupleErrorKind {
    Undefined,
    DuplicateIdentifier,
    DuplicateType,
    MissingColon,
    MissingIdentifier,
    InvalidTupleDeclaration,
    ExpectedType(String),
    Type(TypeErrorKind),
}

impl From<TypeError> for TypeTupleError {
    fn from(value: TypeError) -> Self {
        TypeTupleError(TypeTupleErrorKind::Type(value.0), value.1)
    }
}

#[derive(Debug)]
pub struct TypeError(pub TypeErrorKind, pub Range);

impl fmt::Display for TypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for TypeErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let errstr = match self {
            TypeErrorKind::Undefined => "Type error.".to_owned(),
            TypeErrorKind::ArrayOfMapIsNotAllowed => "Array of map is not allowed.".to_owned(),
            TypeErrorKind::InvalidMapDeclaration => "Invalid map declaration.".to_owned(),
            TypeErrorKind::InvalidMapType(name) => format!("Invalid map type `{}`.", name),
            TypeErrorKind::InvalidType(name) => format!("Invalid type `{}`.", name),
            TypeErrorKind::MapOfArrayIsNotAllowed => "Map of array is not allowed.".to_owned(),
            TypeErrorKind::WrongNumberOfTypes => "Wrong number of types.".to_owned(),
        };

        write!(f, "{}", errstr)
    }
}

#[derive(Debug)]
pub enum TypeErrorKind {
    Undefined,
    InvalidType(String),
    InvalidMapType(String),
    WrongNumberOfTypes,
    InvalidMapDeclaration,
    ArrayOfMapIsNotAllowed,
    MapOfArrayIsNotAllowed,
}

#[derive(Debug, Eq, PartialEq)]
enum TupleParsing {
    ExpectingFieldName,
    ExpectingColon,
    ExpectingType,
    Type,
}

#[derive(Debug)]
pub struct AttributeError(pub AttributeErrorKind, pub Range);

impl fmt::Display for AttributeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for AttributeErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let errstr = match self {
            AttributeErrorKind::Undefined => "Attribute error.".to_owned(),
            AttributeErrorKind::Empty => "Empty attribute.".to_owned(),
            AttributeErrorKind::EmptyField => "Empty field.".to_owned(),
            AttributeErrorKind::InvalidField => "Invalid field.".to_owned(),
            AttributeErrorKind::MissingBracket => "Missing bracket.".to_owned(),
            AttributeErrorKind::MissingComma => "Missing `,`.".to_owned(),
        };

        write!(f, "{}", errstr)
    }
}

#[derive(Debug)]
pub enum AttributeErrorKind {
    Undefined,
    EmptyField,
    Empty,
    MissingComma,
    MissingBracket,
    InvalidField,
}

#[derive(Debug)]
pub struct LibraryError(pub LibraryErrorKind, pub Range);

impl fmt::Display for LibraryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for LibraryErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let errstr = match self {
            LibraryErrorKind::InvalidLibraryDeclaration => "Invalid declaration.",
        };

        write!(f, "{}", errstr)
    }
}

#[derive(Debug)]
pub enum LibraryErrorKind {
    InvalidLibraryDeclaration,
}

#[derive(Debug)]
pub struct ImportsError(pub ImportsErrorKind, pub Range);

impl fmt::Display for ImportsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for ImportsErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let errstr = match self {
            ImportsErrorKind::ExpectedBody => "Empty body.",
            ImportsErrorKind::EmptyName => "Empty name.",
            ImportsErrorKind::ExpectedIdentifier => "Missing identifier.",
            ImportsErrorKind::InvalidImportDeclaration => "Invalid declaration.",
        };

        write!(f, "{}", errstr)
    }
}

#[derive(Debug)]
pub enum ImportsErrorKind {
    InvalidImportDeclaration,
    ExpectedIdentifier,
    ExpectedBody,
    EmptyName,
}

#[derive(Debug)]
pub struct ConstFieldError(pub ConstFieldErrorKind, pub Range);

impl fmt::Display for ConstFieldError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for ConstFieldErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let errstr = match self {
            ConstFieldErrorKind::Undefined => "Const field error.",
            ConstFieldErrorKind::ConstTypeMustBeUnique => "Const type must be unique.",
            ConstFieldErrorKind::EmptyBody => "Empty body.",
            ConstFieldErrorKind::IncompleteField => "Incomplete field.",
            ConstFieldErrorKind::InvalidSymbol => "Invalid symbol.",
            ConstFieldErrorKind::MissingAssignment => "Missing `=`.",
            ConstFieldErrorKind::MissingComma => "Missing `,`.",
            ConstFieldErrorKind::MissingCurlyBracket => "Missing `}`.",
            ConstFieldErrorKind::MissingIdentifier => "Missing identifier.",
            ConstFieldErrorKind::MultipleIdentifier => "Multiple identifier.",
        };

        write!(f, "{}", errstr)
    }
}

#[derive(Debug)]
pub enum ConstFieldErrorKind {
    Undefined,
    EmptyBody,
    IncompleteField,
    ConstTypeMustBeUnique,
    MultipleIdentifier,
    InvalidSymbol,
    MissingIdentifier,
    MissingAssignment,
    MissingComma,
    MissingCurlyBracket,
}

#[derive(Debug)]
pub struct ConstError(pub ConstErrorKind, pub Range);

impl fmt::Display for ConstError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for ConstErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let errstr = match self {
            ConstErrorKind::Undefined => "Const error.".to_owned(),
            ConstErrorKind::ConstField(field) => field.to_string(),
            ConstErrorKind::TypeDeclaration => "Type declaration.".to_owned(),
            ConstErrorKind::MissingTypeName => "Missing type name.".to_owned(),
        };

        write!(f, "{}", errstr)
    }
}

#[derive(Debug)]
pub enum ConstErrorKind {
    Undefined,
    MissingTypeName,
    TypeDeclaration,
    ConstField(ConstFieldErrorKind),
}

#[derive(Debug, Eq, PartialEq)]
enum ConstFieldParsing {
    ExpectingIdentifier,
    ExpectingColon,
    ExpectingValue,
    Value,
}

impl From<ConstFieldError> for ConstError {
    fn from(value: ConstFieldError) -> Self {
        ConstError(ConstErrorKind::ConstField(value.0), value.1)
    }
}

impl From<InterfaceError> for ParserError {
    fn from(value: InterfaceError) -> Self {
        ParserError::Interface(value)
    }
}

impl From<StructError> for ParserError {
    fn from(value: StructError) -> Self {
        ParserError::Struct(value)
    }
}

impl From<EnumError> for ParserError {
    fn from(value: EnumError) -> Self {
        ParserError::Enum(value)
    }
}

impl From<LibraryError> for ParserError {
    fn from(value: LibraryError) -> Self {
        ParserError::Library(value)
    }
}

impl From<ImportsError> for ParserError {
    fn from(value: ImportsError) -> Self {
        ParserError::Imports(value)
    }
}

impl From<TypeListError> for ParserError {
    fn from(value: TypeListError) -> Self {
        ParserError::TypeList(value)
    }
}

impl From<ConstError> for ParserError {
    fn from(value: ConstError) -> Self {
        ParserError::Const(value)
    }
}

impl From<StreamError> for ParserError {
    fn from(value: StreamError) -> Self {
        ParserError::Stream(value)
    }
}

impl From<FactoryError> for ParserError {
    fn from(value: FactoryError) -> Self {
        ParserError::Factory(value)
    }
}

impl From<ScError> for ParserError {
    fn from(value: ScError) -> Self {
        let (kind, range) = match value {
            ScError::InvalidCharacter(invalid_char) => (
                ScannerErrorKind::InvalidCharacter,
                Range::from_word_range(&invalid_char),
            ),
            ScError::InvalidComment(invalid_comment) => (
                ScannerErrorKind::InvalidComment,
                Range::from_word_range(&invalid_comment),
            ),
            ScError::InvalidString(invalid_str) => (
                ScannerErrorKind::InvalidString,
                Range::from_word_range(&invalid_str),
            ),
            ScError::Name(invalid_name) => match invalid_name {
                WordStream::Identifier(invalid_identifer) => (
                    ScannerErrorKind::Name(invalid_identifer.get_word().to_owned()),
                    Range::from_word_range(&invalid_identifer),
                ),
                sw => (
                    ScannerErrorKind::Name("".to_owned()),
                    Range::from_wrange(sw.get_range()),
                ), // TODO
            },
            ScError::SymbolMissing(symbol_missing) => match symbol_missing {
                WordStream::RightCurlyBracket(rcurly_range) => (
                    ScannerErrorKind::SymbolMissing(rcurly_range.get_word().to_string()),
                    Range::from_word_range(&rcurly_range),
                ),
                WordStream::RightParenthesis(rparen_range) => (
                    ScannerErrorKind::SymbolMissing(rparen_range.get_word().to_string()),
                    Range::from_word_range(&rparen_range),
                ),
                WordStream::RightSquareBracket(rsquare_range) => (
                    ScannerErrorKind::SymbolMissing(rsquare_range.get_word().to_string()),
                    Range::from_word_range(&rsquare_range),
                ),
                sw => (
                    ScannerErrorKind::SymbolMissing("".to_owned()), // TODO
                    Range::from_wrange(sw.get_range()),
                ),
            },
        };

        ParserError::Text(ScannerError(kind, range))
    }
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let errstr = match self {
            ParserError::Undefined(_) => "Parser error.".to_owned(),
            ParserError::Closed => "Closed.".to_owned(),
            pr => pr.to_string(),
        };

        write!(f, "{}", errstr)
    }
}

#[derive(Debug)]
pub enum ParserError {
    Closed,
    Undefined(Range),
    Enum(EnumError),
    Interface(InterfaceError),
    Factory(FactoryError),
    Struct(StructError),
    Stream(StreamError),
    Library(LibraryError),
    Imports(ImportsError),
    TypeList(TypeListError),
    Const(ConstError),
    Text(ScannerError),
}

impl ParserError {
    pub fn get_message_with_range(&self) -> (String, Range) {
        match self {
            ParserError::Closed => (self.to_string(), Range::default()),
            ParserError::Undefined(range) => (self.to_string(), *range),
            ParserError::Text(ScannerError(kind, range)) => (kind.to_string(), *range),
            ParserError::Const(ConstError(kind, range)) => (kind.to_string(), *range),
            ParserError::Enum(EnumError(kind, range)) => (kind.to_string(), *range),
            ParserError::Imports(ImportsError(kind, range)) => (kind.to_string(), *range),
            ParserError::Interface(InterfaceError(kind, range)) => (kind.to_string(), *range),
            ParserError::Library(LibraryError(kind, range)) => (kind.to_string(), *range),
            ParserError::Struct(StructError(kind, range)) => (kind.to_string(), *range),
            ParserError::TypeList(TypeListError(kind, range)) => (kind.to_string(), *range),
            ParserError::Stream(StreamError(kind, range)) => (kind.to_string(), *range),
            ParserError::Factory(FactoryError(kind, range)) => (kind.to_string(), *range),
        }
    }
}

#[derive(Debug)]
pub struct ScannerError(pub ScannerErrorKind, pub Range);

impl fmt::Display for ScannerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for ScannerErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let errstr = match &self {
            ScannerErrorKind::InvalidCharacter => "Invalid character.".to_owned(),
            ScannerErrorKind::InvalidComment => "Invalid comment.".to_owned(),
            ScannerErrorKind::InvalidString => "Invalid string.".to_owned(),
            ScannerErrorKind::Name(name) => format!("Invalid name: {}", name),
            ScannerErrorKind::ReservedWord(word) => format!("Reserved word: {}", word),
            ScannerErrorKind::SymbolMissing(symbol) => format!("Missing {}", symbol),
        };

        write!(f, "{}", errstr)
    }
}

#[derive(Debug)]
pub enum ScannerErrorKind {
    SymbolMissing(String),
    ReservedWord(String),
    Name(String),
    InvalidCharacter,
    InvalidComment,
    InvalidString,
}

#[derive(Debug, Default)]
pub struct Parser {
    pub nodes: Vec<ParserNode>,
}

impl Parser {
    pub fn closed() -> Result<Self, (Self, ParserError)> {
        Err((Self::default(), ParserError::Closed))
    }

    pub fn parse(text: &str) -> Result<Self, (Self, ParserError)> {
        let lines: Vec<&str> = text.lines().collect();

        let mut context = Self::default();

        let source_stream = match ContextStream::scan_text(&lines) {
            Ok(value) => value,
            Err(err) => return Err((context, err.into())),
        };

        let mut word_stream = source_stream.word_streams.iter();
        let mut comments = vec![];

        while let Some(w_stream) = word_stream.next() {
            match w_stream {
                WordStream::Keyword(keyword) => {
                    let start_position = Range::from_word_range(&keyword).as_position();

                    match &keyword.get_word() {
                        Keywords::Interface => {
                            for comment in comments {
                                context.nodes.push(ParserNode::InterfaceComment(comment));
                            }

                            if let Err(err) =
                                context.consume_interface(&mut word_stream, start_position)
                            {
                                return Err((context, err.into()));
                            }
                        }
                        Keywords::Struct => {
                            for comment in comments {
                                context.nodes.push(ParserNode::StructComment(comment));
                            }
                            if let Err(err) =
                                context.consume_struct(&mut word_stream, start_position)
                            {
                                return Err((context, err.into()));
                            }
                        }
                        Keywords::Enum => {
                            for comment in comments {
                                context.nodes.push(ParserNode::EnumComment(comment));
                            }
                            if let Err(err) = context.consume_enum(&mut word_stream, start_position)
                            {
                                return Err((context, err.into()));
                            }
                        }
                        Keywords::Library => {
                            for comment in comments {
                                context.nodes.push(ParserNode::Comment(comment));
                            }
                            if let Err(err) =
                                context.consume_library(&mut word_stream, start_position)
                            {
                                return Err((context, err.into()));
                            }
                        }
                        Keywords::Import => {
                            for comment in comments {
                                context.nodes.push(ParserNode::Comment(comment));
                            }
                            if let Err(err) =
                                context.consume_import(&mut word_stream, start_position)
                            {
                                return Err((context, err.into()));
                            }
                        }
                        Keywords::Type => {
                            for comment in comments {
                                context.nodes.push(ParserNode::TypeListComment(comment));
                            }
                            if let Err(err) = context.consume_type(&mut word_stream, start_position)
                            {
                                return Err((context, err.into()));
                            }
                        }
                        Keywords::Const => {
                            for comment in comments {
                                context.nodes.push(ParserNode::ConstComment(comment));
                            }
                            if let Err(err) =
                                context.consume_const(&mut word_stream, start_position)
                            {
                                return Err((context, err.into()));
                            }
                        }
                        Keywords::Stream => {
                            for comment in comments {
                                context.nodes.push(ParserNode::StreamComment(comment));
                            }
                            if let Err(err) =
                                context.consume_stream(&mut word_stream, start_position)
                            {
                                return Err((context, err.into()));
                            }
                        }
                        Keywords::Factory => {
                            for comment in comments {
                                context.nodes.push(ParserNode::StreamComment(comment));
                            }

                            if let Err(err) =
                                context.consume_factory(&mut word_stream, start_position)
                            {
                                return Err((context, err.into()));
                            }
                        }
                    }
                    comments = vec![];
                }
                WordStream::Comment(comment) => {
                    comments.push(comment.get_word().to_owned());
                    if let Some(c_stream) = word_stream.next() {
                        match c_stream {
                            WordStream::NewLine(_) => {}
                            sw => {
                                return Err((
                                    context,
                                    ParserError::Undefined(Range::from_wrange(sw.get_range())),
                                ))
                            }
                        }
                    }
                }
                // This should mean only a new line before any other declaration.
                WordStream::NewLine(_) => {
                    for comment in comments {
                        context.nodes.push(ParserNode::Comment(comment));
                    }
                    comments = vec![];
                }
                sw => {
                    return Err((
                        context,
                        ParserError::Undefined(Range::from_wrange(sw.get_range())),
                    ))
                }
            }
        }

        Ok(context)
    }

    fn consume_library<'a, I>(
        &mut self,
        word_stream: &mut I,
        start_position: Position,
    ) -> Result<(), LibraryError>
    where
        I: Iterator<Item = &'a WordStream>,
    {
        let last_range = start_position.as_range();

        while let Some(identifier) = word_stream.next() {
            match identifier {
                WordStream::Identifier(ident) => {
                    let library_name = ident.get_word().to_owned();
                    self.nodes.push(ParserNode::Library(library_name));
                    return Ok(());
                }
                sw => {
                    return Err(LibraryError(
                        LibraryErrorKind::InvalidLibraryDeclaration,
                        last_range.merge(Range::from_wrange(sw.get_range())),
                    ))
                }
            }
        }

        Err(LibraryError(
            LibraryErrorKind::InvalidLibraryDeclaration,
            last_range,
        ))
    }

    fn consume_import<'a, I>(
        &mut self,
        word_stream: &mut I,
        start_position: Position,
    ) -> Result<(), ImportsError>
    where
        I: Iterator<Item = &'a WordStream>,
    {
        let mut imports = vec![];

        let mut cur_name: Option<String> = None;
        let mut has_comma = false;
        let mut last_range = start_position.as_range();

        while let Some(identifier) = word_stream.next() {
            match identifier {
                WordStream::LeftCurlyBracket(lb) => {
                    last_range = Range::from_word_range(lb);

                    while let Some(b_stream) = word_stream.next() {
                        match b_stream {
                            WordStream::CurlyBracketBody(cb_stream) => {
                                let mut w = cb_stream.iter();

                                while let Some(cb_body) = w.next() {
                                    match cb_body {
                                        WordStream::Identifier(ident) => {
                                            has_comma = false;
                                            last_range =
                                                Range::from_word_range(ident).end_as_range();
                                            cur_name = Some(ident.get_word().to_owned());
                                        }
                                        WordStream::Comma(cm) => {
                                            let range = Range::from_word_range(cm);
                                            match cur_name.take() {
                                                Some(ident) => {
                                                    has_comma = true;
                                                    imports.push(ident);
                                                    last_range = range.end_as_range();
                                                }
                                                None => {
                                                    return Err(ImportsError(
                                                        ImportsErrorKind::EmptyName,
                                                        last_range.merge(range),
                                                    ))
                                                }
                                            }
                                        }
                                        WordStream::NewLine(_) => {}
                                        ws => {
                                            return Err(ImportsError(
                                                ImportsErrorKind::ExpectedBody,
                                                last_range
                                                    .merge(Range::from_wrange(ws.get_range())),
                                            ))
                                        }
                                    }
                                }
                            }
                            WordStream::RightCurlyBracket(rc) => {
                                if !has_comma {
                                    match cur_name {
                                        Some(c_name) => imports.push(c_name),
                                        None => {
                                            return Err(ImportsError(
                                                ImportsErrorKind::EmptyName,
                                                last_range.merge(Range::from_word_range(rc)),
                                            ))
                                        }
                                    }
                                }

                                self.nodes.push(ParserNode::Imports(imports));
                                return Ok(());
                            }
                            WordStream::NewLine(_) => {}
                            sw => {
                                return Err(ImportsError(
                                    ImportsErrorKind::ExpectedBody,
                                    last_range.merge(Range::from_wrange(sw.get_range())),
                                ));
                            }
                        }
                    }
                }
                WordStream::NewLine(_) => {}
                sw => {
                    return Err(ImportsError(
                        ImportsErrorKind::ExpectedIdentifier,
                        last_range.merge(Range::from_wrange(sw.get_range())),
                    ))
                }
            }
        }

        Err(ImportsError(
            ImportsErrorKind::ExpectedIdentifier,
            last_range,
        ))
    }

    fn consume_factory<'a, I>(
        &mut self,
        word_stream: &mut I,
        start_position: Position,
    ) -> Result<(), FactoryError>
    where
        I: Iterator<Item = &'a WordStream>,
    {
        let mut factory_name: Option<String> = None;
        let mut name_range = Range::default();
        let last_range = start_position.as_range();

        while let Some(w_stream) = word_stream.next() {
            match w_stream {
                WordStream::LeftCurlyBracket(lc) => match factory_name {
                    Some(ident) => {
                        let (fields, end_position) = Self::push_interface_fields(word_stream)?;

                        self.nodes.push(ParserNode::Factory(Factory {
                            ident: Arc::new(Type::Name(Arc::new(TypeName {
                                ident,
                                range: name_range,
                            }))),
                            fields,
                            range: Range {
                                start: start_position,
                                end: end_position,
                            },
                        }));

                        return Ok(());
                    }
                    None => {
                        return Err(FactoryError(
                            FactoryErrorKind::MissingTypeName,
                            last_range.merge(Range::from_word_range(lc)),
                        ))
                    }
                },
                WordStream::TypeName(type_name) => {
                    name_range = Range::from_word_range(type_name);
                    if factory_name.is_some() {
                        return Err(FactoryError(FactoryErrorKind::TypeDeclaration, name_range));
                    }

                    factory_name = Some(type_name.get_word().to_owned());
                }
                sw => {
                    return Err(FactoryError(
                        FactoryErrorKind::TypeDeclaration,
                        last_range.merge(Range::from_wrange(sw.get_range())),
                    ))
                }
            }
        }

        Err(FactoryError(FactoryErrorKind::Undefined, last_range))
    }

    fn consume_interface<'a, I>(
        &mut self,
        word_stream: &mut I,
        start_position: Position,
    ) -> Result<(), InterfaceError>
    where
        I: Iterator<Item = &'a WordStream>,
    {
        let mut interface_name: Option<String> = None;
        let mut name_range = Range::default();
        let last_range = start_position.as_range();

        while let Some(w_stream) = word_stream.next() {
            match w_stream {
                WordStream::LeftCurlyBracket(lc) => match interface_name {
                    Some(ident) => {
                        let (fields, end_position) = Self::push_interface_fields(word_stream)?;

                        self.nodes.push(ParserNode::Interface(Interface {
                            ident: Arc::new(Type::Name(Arc::new(TypeName {
                                ident,
                                range: name_range,
                            }))),
                            fields,
                            range: Range {
                                start: start_position,
                                end: end_position,
                            },
                        }));

                        return Ok(());
                    }
                    None => {
                        return Err(InterfaceError(
                            InterfaceErrorKind::MissingTypeName,
                            last_range.merge(Range::from_word_range(lc)),
                        ))
                    }
                },
                WordStream::TypeName(type_name) => {
                    name_range = Range::from_word_range(type_name);
                    if interface_name.is_some() {
                        return Err(InterfaceError(
                            InterfaceErrorKind::TypeDeclaration,
                            name_range,
                        ));
                    }

                    interface_name = Some(type_name.get_word().to_owned());
                }
                sw => {
                    return Err(InterfaceError(
                        InterfaceErrorKind::TypeDeclaration,
                        last_range.merge(Range::from_wrange(sw.get_range())),
                    ))
                }
            }
        }

        Err(InterfaceError(InterfaceErrorKind::Undefined, last_range))
    }

    fn push_interface_fields<'a, I>(
        word_stream: &mut I,
    ) -> Result<(Vec<InterfaceNode>, Position), InterfaceFieldError>
    where
        I: Iterator<Item = &'a WordStream>,
    {
        let mut fields = vec![];
        let mut parsing = InterfaceFieldParsing::ExpectingAttribute;
        let mut attributes = vec![];
        let mut name: Option<String> = None;
        let mut ty: Option<Type> = None;
        let mut field_range = Range::default();
        let mut last_range = Range::default();

        while let Some(n_stream) = word_stream.next() {
            match n_stream {
                WordStream::CurlyBracketBody(curly_body) => {
                    let mut curly_word_stream = curly_body.iter();

                    // [attribute...] identifer : (identifier: type) -> type | type | [type],
                    while let Some(w_stream) = curly_word_stream.next() {
                        match w_stream {
                            WordStream::Comma(cm) => {
                                let range = Range::from_word_range(cm);
                                match parsing {
                                    InterfaceFieldParsing::Type
                                    | InterfaceFieldParsing::Tuple
                                    | InterfaceFieldParsing::ReturnType
                                    | InterfaceFieldParsing::ExpectingComma => {
                                        fields.push(InterfaceNode::InterfaceField(Box::new(
                                            InterfaceField {
                                                attributes,
                                                ident: name.take().unwrap(),
                                                ty: Arc::new(ty.take().unwrap()),
                                                range: field_range,
                                            },
                                        )));

                                        attributes = vec![];
                                        field_range = Range::default();
                                        parsing = InterfaceFieldParsing::ExpectingAttribute;
                                        last_range = range.end_as_range();
                                    }
                                    InterfaceFieldParsing::ExpectingType => {
                                        return Err(InterfaceFieldError(
                                            InterfaceFieldErrorKind::ExpectedType("".to_owned()),
                                            last_range.merge(range),
                                        ));
                                    }
                                    _ => {
                                        return Err(InterfaceFieldError(
                                            InterfaceFieldErrorKind::IncompleteField,
                                            last_range.merge(range),
                                        ))
                                    }
                                }
                            }
                            WordStream::Identifier(ident) => {
                                let range = Range::from_word_range(ident);
                                parsing = match parsing {
                                    InterfaceFieldParsing::ExpectingAttribute => {
                                        field_range = Range::from_word_range(ident);
                                        name = Some(ident.get_word().to_owned());
                                        last_range = range.end_as_range();
                                        InterfaceFieldParsing::ExpectingColon
                                    }
                                    InterfaceFieldParsing::ExpectingColon => {
                                        return Err(InterfaceFieldError(
                                            InterfaceFieldErrorKind::MultipleIdentifier,
                                            last_range.merge(range),
                                        ))
                                    }
                                    InterfaceFieldParsing::ExpectingType => {
                                        return Err(InterfaceFieldError(
                                            InterfaceFieldErrorKind::ExpectedType(
                                                ident.get_word().to_owned(),
                                            ),
                                            last_range.merge(range),
                                        ));
                                    }
                                    _ => {
                                        return Err(InterfaceFieldError(
                                            InterfaceFieldErrorKind::MissingAttribute,
                                            last_range.merge(range),
                                        ))
                                    }
                                }
                            }
                            WordStream::Colon(cl) => {
                                let range = Range::from_word_range(cl);
                                parsing = match parsing {
                                    InterfaceFieldParsing::ExpectingColon => {
                                        field_range = field_range.merge(range);
                                        last_range = range.end_as_range();
                                        InterfaceFieldParsing::ExpectingType
                                    }
                                    InterfaceFieldParsing::ExpectingType => {
                                        return Err(InterfaceFieldError(
                                            InterfaceFieldErrorKind::ExpectedType("".to_owned()),
                                            last_range.merge(range),
                                        ));
                                    }
                                    _ => {
                                        return Err(InterfaceFieldError(
                                            InterfaceFieldErrorKind::MissingIdentifier,
                                            last_range.merge(range),
                                        ))
                                    }
                                }
                            }
                            WordStream::LeftSquareBracket(lq) => {
                                let position = Range::from_word_range(lq).as_position();
                                let range = position.as_range();

                                parsing = match parsing {
                                    InterfaceFieldParsing::ExpectingAttribute => {
                                        let (att, range) =
                                            Self::push_attribute(&mut curly_word_stream, position)?;
                                        attributes.push(att);

                                        if parsing == InterfaceFieldParsing::ExpectingAttribute {
                                            field_range = range; // TODO Fix attribute range.
                                        }

                                        field_range = field_range.merge(range);
                                        last_range = range.end_as_range();
                                        InterfaceFieldParsing::ExpectingAttribute
                                    }
                                    InterfaceFieldParsing::Type => {
                                        let (arr_map, range) = Self::get_array_or_map(
                                            &mut curly_word_stream,
                                            Arc::new(ty.unwrap()),
                                            Range::from_word_range(lq).as_position(),
                                        )?;
                                        ty = Some(arr_map);
                                        field_range = field_range.merge(range);
                                        last_range = range.end_as_range();
                                        InterfaceFieldParsing::Type
                                    }
                                    InterfaceFieldParsing::ReturnType => {
                                        match ty {
                                            Some(Type::Function(methd)) => {
                                                let (ret_ty, range) = Self::get_array_or_map(
                                                    &mut curly_word_stream,
                                                    methd.ret_ty.clone(),
                                                    Range::from_word_range(lq).as_position(),
                                                )?;

                                                ty = Some(Type::Function(Arc::new(TypeFunction {
                                                    args: methd.args.clone(),
                                                    ret_ty: Arc::new(ret_ty),
                                                    range: methd.range.merge(range),
                                                })));

                                                field_range = field_range.merge(range);
                                                last_range = range.end_as_range();
                                            }
                                            _ => {
                                                return Err(InterfaceFieldError(
                                                    InterfaceFieldErrorKind::Undefined,
                                                    last_range.merge(range),
                                                ))
                                            }
                                        }
                                        InterfaceFieldParsing::ReturnType
                                    }
                                    InterfaceFieldParsing::ExpectingReturnType => {
                                        let (ret_ty, range) = Self::get_result_or_option(
                                            &mut curly_word_stream,
                                            Range::from_word_range(lq).as_position(),
                                        )?;

                                        ty = ty.map(|args| {
                                            let arg_range = match &args {
                                                Type::Tuple(value) => value.range,
                                                _ => panic!("Expected tuple."),
                                            };

                                            Type::Function(Arc::new(TypeFunction {
                                                ret_ty: Arc::new(ret_ty),
                                                args: Arc::new(args),
                                                range: arg_range.merge(range),
                                            }))
                                        });
                                        field_range = field_range.merge(range);
                                        last_range = range.end_as_range();
                                        InterfaceFieldParsing::ExpectingComma
                                    }
                                    InterfaceFieldParsing::ExpectingType => {
                                        let (t_ty, range) = Self::get_result_or_option(
                                            &mut curly_word_stream,
                                            Range::from_word_range(lq).as_position(),
                                        )?;

                                        ty = Some(t_ty);

                                        field_range = field_range.merge(range);
                                        last_range = range.end_as_range();
                                        InterfaceFieldParsing::ExpectingComma
                                    }
                                    _ => {
                                        return Err(InterfaceFieldError(
                                            InterfaceFieldErrorKind::InvalidBracketPlacement,
                                            last_range.merge(range),
                                        ))
                                    }
                                }
                            }
                            WordStream::LeftParenthesis(lh) => {
                                let position = Range::from_word_range(lh).as_position();
                                let range = position.as_range();

                                parsing = match parsing {
                                    InterfaceFieldParsing::ExpectingType => {
                                        InterfaceFieldParsing::Tuple
                                    }
                                    InterfaceFieldParsing::ExpectingAttribute => {
                                        return Err(InterfaceFieldError(
                                            InterfaceFieldErrorKind::MultipleIdentifier,
                                            last_range.merge(range),
                                        ))
                                    }
                                    InterfaceFieldParsing::ExpectingColon => {
                                        return Err(InterfaceFieldError(
                                            InterfaceFieldErrorKind::MissingColon,
                                            last_range.merge(range),
                                        ))
                                    }
                                    InterfaceFieldParsing::Tuple => {
                                        return Err(InterfaceFieldError(
                                            InterfaceFieldErrorKind::MissingArrowFunction,
                                            last_range.merge(range),
                                        ))
                                    }
                                    _ => {
                                        return Err(InterfaceFieldError(
                                            InterfaceFieldErrorKind::TypeDeclaration,
                                            last_range.merge(range),
                                        ))
                                    }
                                };
                                let (t_ty, t_range) =
                                    Self::push_type_tuple(&mut curly_word_stream, position)?;
                                field_range = field_range.merge(t_range);
                                last_range = range.end_as_range();
                                ty = Some(t_ty);
                            }
                            WordStream::NativeType(native_type) => {
                                let range = Range::from_word_range(native_type);
                                parsing = match parsing {
                                    InterfaceFieldParsing::ExpectingType => {
                                        ty = Some(Type::Native(Arc::new(TypeNative {
                                            ty: *native_type.get_word(),
                                            range,
                                        })));
                                        field_range = field_range.merge(range);
                                        last_range = range.end_as_range();
                                        InterfaceFieldParsing::Type
                                    }
                                    InterfaceFieldParsing::ExpectingReturnType => {
                                        let ret_ty = Type::Native(Arc::new(TypeNative {
                                            ty: *native_type.get_word(),
                                            range,
                                        }));
                                        ty = ty.map(|args| {
                                            let arg_range = match &args {
                                                Type::Tuple(value) => value.range,
                                                _ => panic!("Expected tuple."),
                                            };

                                            Type::Function(Arc::new(TypeFunction {
                                                ret_ty: Arc::new(ret_ty),
                                                args: Arc::new(args),
                                                range: arg_range.merge(range),
                                            }))
                                        });
                                        field_range = field_range.merge(range);
                                        last_range = range.end_as_range();
                                        InterfaceFieldParsing::ReturnType
                                    }
                                    InterfaceFieldParsing::ExpectingAttribute => {
                                        return Err(InterfaceFieldError(
                                            InterfaceFieldErrorKind::MultipleIdentifier,
                                            last_range.merge(range),
                                        ))
                                    }
                                    InterfaceFieldParsing::ExpectingColon => {
                                        return Err(InterfaceFieldError(
                                            InterfaceFieldErrorKind::MissingColon,
                                            last_range.merge(range),
                                        ))
                                    }
                                    InterfaceFieldParsing::Tuple => {
                                        return Err(InterfaceFieldError(
                                            InterfaceFieldErrorKind::MissingArrowFunction,
                                            last_range.merge(range),
                                        ))
                                    }
                                    _ => {
                                        return Err(InterfaceFieldError(
                                            InterfaceFieldErrorKind::TypeDeclaration,
                                            last_range.merge(range),
                                        ))
                                    }
                                };
                            }
                            WordStream::TypeName(type_name) => {
                                let range = Range::from_word_range(type_name);
                                parsing = match parsing {
                                    InterfaceFieldParsing::ExpectingType => {
                                        ty = Some(Type::Name(Arc::new(TypeName {
                                            ident: type_name.get_word().to_owned(),
                                            range,
                                        })));
                                        field_range = field_range.merge(range);
                                        last_range = range.end_as_range();
                                        InterfaceFieldParsing::Type
                                    }
                                    InterfaceFieldParsing::ExpectingReturnType => {
                                        let ret_ty = Type::Name(Arc::new(TypeName {
                                            ident: type_name.get_word().to_owned(),
                                            range,
                                        }));
                                        ty = ty.map(|args| {
                                            let arg_range = match &args {
                                                Type::Tuple(value) => value.range,
                                                _ => panic!("Expected tuple."),
                                            };

                                            Type::Function(Arc::new(TypeFunction {
                                                ret_ty: Arc::new(ret_ty),
                                                args: Arc::new(args),
                                                range: arg_range.merge(range),
                                            }))
                                        });
                                        field_range = field_range.merge(range);
                                        last_range = range.end_as_range();
                                        InterfaceFieldParsing::ReturnType
                                    }
                                    InterfaceFieldParsing::ExpectingAttribute => {
                                        return Err(InterfaceFieldError(
                                            InterfaceFieldErrorKind::MultipleIdentifier,
                                            last_range.merge(range),
                                        ))
                                    }
                                    InterfaceFieldParsing::ExpectingColon => {
                                        return Err(InterfaceFieldError(
                                            InterfaceFieldErrorKind::MissingColon,
                                            last_range.merge(range),
                                        ))
                                    }
                                    InterfaceFieldParsing::Tuple => {
                                        return Err(InterfaceFieldError(
                                            InterfaceFieldErrorKind::MissingArrowFunction,
                                            last_range.merge(range),
                                        ))
                                    }
                                    _ => {
                                        return Err(InterfaceFieldError(
                                            InterfaceFieldErrorKind::TypeDeclaration,
                                            last_range.merge(range),
                                        ))
                                    }
                                };
                            }
                            WordStream::Hyphen(hy) => {
                                let range = Range::from_word_range(hy);
                                match parsing {
                                    InterfaceFieldParsing::Tuple => {
                                        if let Some(g_stream) = curly_word_stream.next() {
                                            match g_stream {
                                                WordStream::GreaterThan(_) => {
                                                    field_range = field_range.merge(range);
                                                    last_range = range.end_as_range();
                                                    parsing =
                                                        InterfaceFieldParsing::ExpectingReturnType
                                                }
                                                _ => {
                                                    return Err(InterfaceFieldError(
                                                    InterfaceFieldErrorKind::MissingArrowFunction,
                                                    last_range.merge(range),
                                                ));
                                                }
                                            }
                                        } else {
                                            return Err(InterfaceFieldError(
                                                InterfaceFieldErrorKind::ExpectedFunctionSyntax,
                                                last_range.merge(range),
                                            ));
                                        }
                                    }
                                    InterfaceFieldParsing::ExpectingType => {
                                        return Err(InterfaceFieldError(
                                            InterfaceFieldErrorKind::ExpectedType("".to_owned()),
                                            last_range.merge(range),
                                        ));
                                    }
                                    _ => {
                                        return Err(InterfaceFieldError(
                                            InterfaceFieldErrorKind::ExpectedFunctionSyntax,
                                            last_range.merge(range),
                                        ))
                                    } // Not a function.
                                }
                            }
                            WordStream::NewLine(_) => {}
                            // New line after a comment is not relevant among fields.
                            WordStream::Comment(comment) => {
                                let range = Range::from_word_range(comment);
                                let comment_txt = comment.get_word().to_owned();
                                match parsing {
                                    InterfaceFieldParsing::ExpectingAttribute => {
                                        last_range = range.end_as_range();
                                        fields.push(InterfaceNode::Comment(comment_txt));
                                    }
                                    _ => {
                                        return Err(InterfaceFieldError(
                                            InterfaceFieldErrorKind::IncompleteField,
                                            last_range.merge(range),
                                        ))
                                    }
                                }
                            }
                            sw => {
                                return Err(InterfaceFieldError(
                                    InterfaceFieldErrorKind::InvalidSymbol,
                                    last_range.merge(Range::from_wrange(sw.get_range())),
                                ));
                            }
                        }
                    }
                }
                WordStream::RightCurlyBracket(rb) => {
                    let end_position = Range::from_word_range(rb).as_position();
                    let range = end_position.as_range();

                    match parsing {
                        // Only if at attributes, but when they are empty.
                        InterfaceFieldParsing::ExpectingAttribute => {
                            if fields.iter().any(|value| match value {
                                InterfaceNode::InterfaceField(_) => true,
                                _ => false,
                            }) {
                                return Ok((fields, end_position));
                            }
                            return Err(InterfaceFieldError(
                                InterfaceFieldErrorKind::EmptyBody,
                                last_range.merge(range),
                            ));
                        }
                        InterfaceFieldParsing::Type
                        | InterfaceFieldParsing::ReturnType
                        | InterfaceFieldParsing::Tuple
                        | InterfaceFieldParsing::ExpectingComma => {
                            fields.push(InterfaceNode::InterfaceField(Box::new(InterfaceField {
                                attributes,
                                ident: name.unwrap(),
                                ty: Arc::new(ty.take().unwrap()),
                                range: field_range,
                            })));
                            return Ok((fields, end_position));
                        }
                        InterfaceFieldParsing::ExpectingType => {
                            return Err(InterfaceFieldError(
                                InterfaceFieldErrorKind::ExpectedType("".to_owned()),
                                last_range.merge(range),
                            ));
                        }
                        _ => {
                            return Err(InterfaceFieldError(
                                InterfaceFieldErrorKind::IncompleteField,
                                last_range.merge(range),
                            ))
                        }
                    }
                }
                WordStream::NewLine(_) => {}
                sw => {
                    return Err(InterfaceFieldError(
                        InterfaceFieldErrorKind::MissingCurlyBracket,
                        last_range.merge(Range::from_wrange(sw.get_range())),
                    ))
                }
            }
        }

        Err(InterfaceFieldError(
            InterfaceFieldErrorKind::Undefined,
            last_range,
        ))
    }

    fn push_attribute<'a, I>(
        word_stream: &mut I,
        start_position: Position,
    ) -> Result<(Attribute, Range), AttributeError>
    where
        I: Iterator<Item = &'a WordStream>,
    {
        let mut fields = vec![];

        let mut attribute: Option<AttributeField> = None;
        let mut field_range = Range::default(); // TODO

        while let Some(wb_stream) = word_stream.next() {
            match wb_stream {
                WordStream::SquareBracketBody(sq_stream) => {
                    let mut s_stream = sq_stream.iter();
                    while let Some(w_stream) = s_stream.next() {
                        match w_stream {
                            WordStream::Comma(cm) => {
                                field_range = field_range.merge(Range::from_word_range(cm));
                                fields.push(attribute.take().ok_or_else(|| {
                                    AttributeError(
                                        AttributeErrorKind::EmptyField,
                                        Range::from_word_range(cm),
                                    )
                                })?);
                            }
                            WordStream::Identifier(ident) => {
                                if attribute.is_some() {
                                    return Err(AttributeError(
                                        AttributeErrorKind::MissingComma,
                                        Range::from_word_range(ident),
                                    ));
                                }
                                field_range = Range::from_word_range(ident);
                                attribute =
                                    Some(AttributeField::UnknownName(ident.get_word().to_owned()));
                            }
                            WordStream::Colon(cl) => {
                                return Err(AttributeError(
                                    AttributeErrorKind::InvalidField,
                                    Range::from_word_range(cl),
                                ));
                            }
                            WordStream::StringBody(str_body) => {
                                if attribute.is_some() {
                                    return Err(AttributeError(
                                        AttributeErrorKind::MissingComma,
                                        Range::from_word_range(str_body),
                                    ));
                                }
                                attribute = Some(AttributeField::StringField(
                                    str_body.get_word().to_owned(),
                                ));
                            }
                            WordStream::Attribute(att) => {
                                if attribute.is_some() {
                                    return Err(AttributeError(
                                        AttributeErrorKind::MissingComma,
                                        Range::from_word_range(att),
                                    ));
                                }
                                attribute = Some(AttributeField::Name(*att.get_word()))
                            }
                            sw => {
                                return Err(AttributeError(
                                    AttributeErrorKind::InvalidField,
                                    Range::from_wrange(sw.get_range()),
                                ));
                            }
                        }
                    }
                }
                WordStream::RightSquareBracket(rq) => {
                    if let Some(att) = attribute.take() {
                        fields.push(att);
                    } else if fields.is_empty() {
                        return Err(AttributeError(
                            AttributeErrorKind::Empty,
                            Range::from_word_range(rq),
                        ));
                    }
                    return Ok((
                        Attribute {
                            fields,
                            range: Range::default(),
                        },
                        Range {
                            start: start_position,
                            end: Range::from_word_range(rq).as_position(),
                        },
                    ));
                }
                sw => {
                    return Err(AttributeError(
                        AttributeErrorKind::MissingBracket,
                        Range::from_wrange(sw.get_range()),
                    ))
                }
            }
        }

        Err(AttributeError(
            AttributeErrorKind::Undefined,
            Range::default(),
        ))
    }

    fn get_array_or_map<'a, I>(
        word_stream: &mut I,
        ty: Arc<Type>,
        start_position: Position,
    ) -> Result<(Type, Range), TypeError>
    where
        I: Iterator<Item = &'a WordStream>,
    {
        let mut ar_range = Range::from_position(start_position);

        match &*ty {
            Type::Function(_) => {}
            _ => {
                while let Some(b_stream) = word_stream.next() {
                    match b_stream {
                        // Array.
                        WordStream::RightSquareBracket(rq) => {
                            if let Type::Map(_) = &*ty {
                                return Err(TypeError(
                                    TypeErrorKind::ArrayOfMapIsNotAllowed,
                                    Range::from_word_range(rq),
                                ));
                            }

                            match &*ty {
                                Type::Function(value) => {
                                    return Err(TypeError(
                                        TypeErrorKind::InvalidType(value.to_string()),
                                        Range::from_word_range(rq),
                                    ));
                                }
                                _ => {
                                    ar_range = ar_range.merge(Range::from_word_range(rq));
                                    let arr_ty = Type::Array(Arc::new(TypeArray {
                                        array_ty: ty.clone(),
                                        range: ar_range,
                                    }));

                                    return Ok((arr_ty, ar_range));
                                }
                            }
                        }
                        // Could only be a map.
                        WordStream::SquareBracketBody(sq_body) => {
                            let mut w_stream = sq_body.iter();

                            while let Some(bkt_stream) = w_stream.next() {
                                match bkt_stream {
                                    WordStream::NativeType(native_type) => {
                                        let range = Range::from_word_range(native_type);
                                        let n_type = native_type.get_word();
                                        match n_type {
                                            NativeTypes::String | NativeTypes::Int => match &*ty {
                                                Type::Array(_) => {
                                                    return Err(TypeError(
                                                        TypeErrorKind::MapOfArrayIsNotAllowed,
                                                        range,
                                                    ));
                                                }
                                                Type::Function(value) => {
                                                    return Err(TypeError(
                                                        TypeErrorKind::InvalidType(
                                                            value.to_string(),
                                                        ),
                                                        range,
                                                    ));
                                                }
                                                _ => {
                                                    if let Some(m_stream) = word_stream.next() {
                                                        if let WordStream::RightSquareBracket(rq) =
                                                            m_stream
                                                        {
                                                            ar_range = ar_range
                                                                .merge(Range::from_word_range(rq));

                                                            let index_ty = Type::Native(Arc::new(
                                                                TypeNative {
                                                                    ty: *native_type.get_word(),
                                                                    range,
                                                                },
                                                            ));

                                                            let map_ty =
                                                                Type::Map(Arc::new(TypeMap {
                                                                    index_ty: Arc::new(index_ty),
                                                                    m_ty: ty.clone(),
                                                                    range: ar_range,
                                                                }));

                                                            return Ok((map_ty, ar_range));
                                                        }
                                                    }

                                                    return Err(TypeError(
                                                        TypeErrorKind::InvalidMapDeclaration,
                                                        range,
                                                    ));
                                                }
                                            },
                                            sw => {
                                                return Err(TypeError(
                                                    TypeErrorKind::InvalidMapType(sw.to_string()),
                                                    range,
                                                ))
                                            }
                                        }
                                    }
                                    sw => {
                                        return Err(TypeError(
                                            TypeErrorKind::InvalidMapType(sw.to_string()),
                                            ar_range.merge(Range::from_wrange(sw.get_range())),
                                        ))
                                    } // Must be a native type inside the brackets.
                                }
                            }
                        }
                        sw => {
                            return Err(TypeError(
                                TypeErrorKind::Undefined,
                                ar_range.merge(Range::from_wrange(sw.get_range())),
                            ))
                        }
                    }
                }
            }
        }

        Err(TypeError(TypeErrorKind::Undefined, ar_range))
    }

    fn get_result_or_option<'a, I>(
        word_stream: &mut I,
        start_position: Position,
    ) -> Result<(Type, Range), TypeError>
    where
        I: Iterator<Item = &'a WordStream>,
    {
        let mut op_range = Range::from_position(start_position);
        let mut first_ty: Option<Type> = None;
        let mut second_ty: Option<Type> = None;
        let mut t_ty: Option<Type> = None;

        while let Some(b_stream) = word_stream.next() {
            match b_stream {
                WordStream::SquareBracketBody(sq_body) => {
                    let mut w_stream = sq_body.iter();

                    while let Some(bkt_stream) = w_stream.next() {
                        match bkt_stream {
                            WordStream::SemiColon(cm) => {
                                let range = Range::from_word_range(cm);
                                op_range = op_range.merge(range);
                                match t_ty.take() {
                                    Some(value) => {
                                        if first_ty.is_none() {
                                            first_ty = Some(value);
                                        } else {
                                            return Err(TypeError(
                                                TypeErrorKind::Undefined,
                                                op_range,
                                            ));
                                        }
                                    }
                                    None => {
                                        return Err(TypeError(TypeErrorKind::Undefined, op_range))
                                    }
                                }
                            }
                            WordStream::NativeType(native_type) => {
                                let range = Range::from_word_range(native_type);
                                op_range = op_range.merge(range);

                                if t_ty.is_none() {
                                    t_ty = Some(Type::Native(Arc::new(TypeNative {
                                        ty: *native_type.get_word(),
                                        range,
                                    })));
                                } else {
                                    return Err(TypeError(TypeErrorKind::Undefined, op_range));
                                }
                            }
                            WordStream::TypeName(type_name) => {
                                let range = Range::from_word_range(type_name);
                                op_range = op_range.merge(range);

                                if t_ty.is_none() {
                                    t_ty = Some(Type::Name(Arc::new(TypeName {
                                        ident: type_name.get_word().to_owned(),
                                        range,
                                    })));
                                } else {
                                    return Err(TypeError(TypeErrorKind::Undefined, op_range));
                                }
                            }
                            sw => {
                                return Err(TypeError(
                                    TypeErrorKind::InvalidType(sw.to_string()),
                                    op_range.merge(Range::from_wrange(sw.get_range())),
                                ))
                            }
                        }
                    }
                }
                WordStream::RightSquareBracket(rq) => {
                    let range = Range::from_word_range(rq);
                    op_range = op_range.merge(range);

                    if t_ty.is_none() {
                        return Err(TypeError(TypeErrorKind::WrongNumberOfTypes, op_range));
                    }

                    if first_ty.is_some() {
                        second_ty = t_ty.take();
                    } else {
                        first_ty = t_ty.take();
                    }

                    return if second_ty.is_some() {
                        Ok((
                            Type::Result(Arc::new(TypeResult {
                                ok_ty: Arc::new(first_ty.unwrap()),
                                err_ty: Arc::new(second_ty.unwrap()),
                                range: op_range,
                            })),
                            op_range,
                        ))
                    } else if first_ty.is_some() {
                        Ok((
                            Type::Option(Arc::new(TypeOption {
                                some_ty: Arc::new(first_ty.unwrap()),
                                range: op_range,
                            })),
                            op_range,
                        ))
                    } else {
                        return Err(TypeError(TypeErrorKind::WrongNumberOfTypes, op_range));
                    };
                }
                sw => {
                    return Err(TypeError(
                        TypeErrorKind::Undefined,
                        op_range.merge(Range::from_wrange(sw.get_range())),
                    ))
                }
            }
        }

        Err(TypeError(TypeErrorKind::Undefined, op_range))
    }

    fn push_type_tuple<'a, I>(
        word_stream: &mut I,
        start_position: Position,
    ) -> Result<(Type, Range), TypeTupleError>
    where
        I: Iterator<Item = &'a WordStream>,
    {
        let mut args = vec![];
        let mut parsing = TupleParsing::ExpectingFieldName;
        let mut ty: Option<Type> = None;
        let mut ty_name: Option<String> = None;
        let mut field_range = Range::default();
        let mut last_range = start_position.as_range();

        while let Some(wb_stream) = word_stream.next() {
            match wb_stream {
                WordStream::ParenthesisBody(sq_body) => {
                    let mut s_stream = sq_body.iter();

                    while let Some(w_stream) = s_stream.next() {
                        match w_stream {
                            WordStream::LeftSquareBracket(lq) => match parsing {
                                TupleParsing::Type => {
                                    let (arr_map, range) = Self::get_array_or_map(
                                        &mut s_stream,
                                        Arc::new(ty.unwrap()),
                                        Range::from_word_range(lq).as_position(),
                                    )?;
                                    ty = Some(arr_map);
                                    field_range = range.merge(range);
                                    last_range = range.end_as_range();
                                }
                                _ => {
                                    return Err(TypeTupleError(
                                        TypeTupleErrorKind::ExpectedType("".to_owned()),
                                        last_range.merge(Range::from_word_range(lq)),
                                    ))
                                }
                            },
                            WordStream::NativeType(native_type) => {
                                let range = Range::from_word_range(native_type);
                                parsing = match parsing {
                                    TupleParsing::ExpectingType => {
                                        ty = Some(Type::Native(Arc::new(TypeNative {
                                            ty: *native_type.get_word(),
                                            range,
                                        })));
                                        last_range = range.end_as_range();
                                        field_range =
                                            field_range.merge(Range::from_word_range(native_type));
                                        TupleParsing::Type
                                    }
                                    _ => {
                                        return Err(TypeTupleError(
                                            TypeTupleErrorKind::DuplicateType,
                                            last_range.merge(range),
                                        ))
                                    }
                                }
                            }
                            WordStream::TypeName(type_name) => {
                                let range = Range::from_word_range(type_name);
                                parsing = match parsing {
                                    TupleParsing::ExpectingType => {
                                        ty = Some(Type::Name(Arc::new(TypeName {
                                            ident: type_name.get_word().to_owned(),
                                            range,
                                        })));
                                        last_range = range.end_as_range();
                                        field_range =
                                            field_range.merge(Range::from_word_range(type_name));
                                        TupleParsing::Type
                                    }
                                    _ => {
                                        return Err(TypeTupleError(
                                            TypeTupleErrorKind::DuplicateType,
                                            last_range.merge(range),
                                        ))
                                    }
                                }
                            }
                            WordStream::Identifier(ident) => {
                                parsing = match parsing {
                                    TupleParsing::ExpectingFieldName => {
                                        ty_name = Some(ident.get_word().to_owned());
                                        field_range = Range::from_word_range(ident);
                                        last_range = field_range.end_as_range();
                                        TupleParsing::ExpectingColon
                                    }
                                    TupleParsing::ExpectingType => {
                                        return Err(TypeTupleError(
                                            TypeTupleErrorKind::ExpectedType(
                                                ident.get_word().to_owned(),
                                            ),
                                            last_range.merge(Range::from_word_range(ident)),
                                        ));
                                    }
                                    _ => {
                                        return Err(TypeTupleError(
                                            TypeTupleErrorKind::DuplicateIdentifier,
                                            last_range.merge(Range::from_word_range(ident)),
                                        ))
                                    }
                                }
                            }
                            WordStream::Colon(cl) => {
                                let range = Range::from_word_range(cl);
                                parsing = match parsing {
                                    TupleParsing::ExpectingColon => {
                                        field_range = field_range.merge(range);
                                        last_range = range.end_as_range();
                                        TupleParsing::ExpectingType
                                    }
                                    TupleParsing::ExpectingType => {
                                        return Err(TypeTupleError(
                                            TypeTupleErrorKind::ExpectedType("".to_owned()),
                                            last_range.merge(range),
                                        ));
                                    }
                                    _ => {
                                        return Err(TypeTupleError(
                                            TypeTupleErrorKind::MissingIdentifier,
                                            last_range.merge(range),
                                        ))
                                    }
                                }
                            }
                            WordStream::Comma(cm) => {
                                parsing = match parsing {
                                    TupleParsing::Type => {
                                        args.push(TupleEntry {
                                            ident: ty_name.take().unwrap(),
                                            ty: Arc::new(ty.take().unwrap()),
                                            range: field_range,
                                        });
                                        last_range = Range::from_word_range(cm).end_as_range();
                                        field_range = Range::default();
                                        TupleParsing::ExpectingFieldName
                                    }
                                    _ => {
                                        return Err(TypeTupleError(
                                            TypeTupleErrorKind::ExpectedType("".to_owned()),
                                            last_range.merge(Range::from_word_range(cm)),
                                        ));
                                    }
                                }
                            }
                            sw => {
                                return Err(TypeTupleError(
                                    TypeTupleErrorKind::Undefined,
                                    last_range.merge(Range::from_wrange(sw.get_range())),
                                ))
                            }
                        }
                    }
                }
                WordStream::RightParenthesis(rp) => {
                    last_range = last_range.merge(Range::from_word_range(rp));

                    match parsing {
                        TupleParsing::Type => {
                            args.push(TupleEntry {
                                ident: ty_name.take().unwrap(),
                                ty: Arc::new(ty.take().unwrap()),
                                range: field_range,
                            });
                            return Ok((
                                Type::Tuple(Arc::new(TypeTuple {
                                    ty_list: args,
                                    range: last_range,
                                })),
                                last_range,
                            ));
                        }
                        TupleParsing::ExpectingFieldName => {
                            return Ok((
                                Type::Tuple(Arc::new(TypeTuple {
                                    ty_list: args,
                                    range: last_range,
                                })),
                                last_range,
                            ));
                        }
                        TupleParsing::ExpectingColon => {
                            return Err(TypeTupleError(
                                TypeTupleErrorKind::MissingColon,
                                last_range.merge(Range::from_word_range(rp)),
                            ))
                        }
                        TupleParsing::ExpectingType => {
                            return Err(TypeTupleError(
                                TypeTupleErrorKind::ExpectedType("".to_owned()),
                                last_range,
                            ));
                        }
                    }
                }
                sw => {
                    return Err(TypeTupleError(
                        TypeTupleErrorKind::InvalidTupleDeclaration,
                        last_range.merge(Range::from_wrange(sw.get_range())),
                    ))
                }
            }
        }

        return Err(TypeTupleError(TypeTupleErrorKind::Undefined, last_range));
    }

    fn consume_stream<'a, I>(
        &mut self,
        word_stream: &mut I,
        start_position: Position,
    ) -> Result<(), StreamError>
    where
        I: Iterator<Item = &'a WordStream>,
    {
        let mut stream_name: Option<String> = None;
        let mut name_range = Range::default();
        let mut last_range = start_position.as_range();

        while let Some(w_stream) = word_stream.next() {
            match w_stream {
                WordStream::LeftCurlyBracket(lc) => match stream_name {
                    Some(ident) => {
                        // There's not much difference between structs and streams, yet.
                        let (fields, end_position) = Self::push_struct_fields(word_stream)?;

                        self.nodes.push(ParserNode::Stream(Stream {
                            ident: Arc::new(Type::Name(Arc::new(TypeName {
                                ident,
                                range: name_range,
                            }))),
                            fields,
                            range: Range {
                                start: start_position,
                                end: end_position,
                            },
                        }));

                        return Ok(());
                    }
                    None => {
                        return Err(StreamError(
                            StreamErrorKind::MissingTypeName,
                            last_range.merge(Range::from_word_range(lc)),
                        ))
                    }
                },
                WordStream::TypeName(type_name) => {
                    name_range = Range::from_word_range(type_name);
                    if stream_name.is_some() {
                        return Err(StreamError(StreamErrorKind::TypeDeclaration, name_range));
                    }

                    stream_name = Some(type_name.get_word().to_owned());
                    last_range = name_range.end_as_range();
                }
                sw => {
                    return Err(StreamError(
                        StreamErrorKind::TypeDeclaration,
                        last_range.merge(Range::from_wrange(sw.get_range())),
                    ))
                }
            }
        }

        Err(StreamError(StreamErrorKind::Undefined, last_range))
    }

    fn consume_struct<'a, I>(
        &mut self,
        word_stream: &mut I,
        start_position: Position,
    ) -> Result<(), StructError>
    where
        I: Iterator<Item = &'a WordStream>,
    {
        let mut struct_name: Option<String> = None;
        let mut name_range = Range::default();
        let mut last_range = start_position.as_range();

        while let Some(w_stream) = word_stream.next() {
            match w_stream {
                WordStream::LeftCurlyBracket(lc) => match struct_name {
                    Some(ident) => {
                        let (fields, end_position) = Self::push_struct_fields(word_stream)?;

                        self.nodes.push(ParserNode::Struct(Struct {
                            ident: Arc::new(Type::Name(Arc::new(TypeName {
                                ident,
                                range: name_range,
                            }))),
                            fields,
                            range: Range {
                                start: start_position,
                                end: end_position,
                            },
                        }));

                        return Ok(());
                    }
                    None => {
                        return Err(StructError(
                            StructErrorKind::MissingTypeName,
                            last_range.merge(Range::from_word_range(lc)),
                        ))
                    }
                },
                WordStream::TypeName(type_name) => {
                    name_range = Range::from_word_range(type_name);
                    if struct_name.is_some() {
                        return Err(StructError(StructErrorKind::TypeDeclaration, name_range));
                    }

                    struct_name = Some(type_name.get_word().to_owned());
                    last_range = name_range.end_as_range();
                }
                sw => {
                    return Err(StructError(
                        StructErrorKind::TypeDeclaration,
                        last_range.merge(Range::from_wrange(sw.get_range())),
                    ))
                }
            }
        }

        Err(StructError(StructErrorKind::Undefined, last_range))
    }

    fn push_struct_fields<'a, I>(
        word_stream: &mut I,
    ) -> Result<(Vec<StructNode>, Position), StructFieldError>
    where
        I: Iterator<Item = &'a WordStream>,
    {
        let mut fields = vec![];
        let mut parsing = StructFieldParsing::ExpectingIdentifier;
        let mut ty: Option<Type> = None;
        let mut field_name: Option<String> = None;
        let mut field_range = Range::default();
        let mut last_range = Range::default();

        while let Some(n_stream) = word_stream.next() {
            match n_stream {
                WordStream::CurlyBracketBody(curly_body) => {
                    let mut curly_word_stream = curly_body.iter();

                    while let Some(w_stream) = curly_word_stream.next() {
                        match w_stream {
                            WordStream::Comma(cm) => {
                                let range = Range::from_word_range(cm);
                                match parsing {
                                    StructFieldParsing::Type => {
                                        fields.push(StructNode::StructField(Box::new(
                                            StructField {
                                                ident: field_name.take().unwrap(),
                                                ty: Arc::new(ty.take().unwrap()),
                                                range: field_range,
                                            },
                                        )));

                                        field_range = Range::default();
                                        last_range = range.end_as_range();
                                        parsing = StructFieldParsing::ExpectingIdentifier;
                                    }
                                    StructFieldParsing::ExpectingType => {
                                        return Err(StructFieldError(
                                            StructFieldErrorKind::ExpectedType("".to_owned()),
                                            last_range.merge(range),
                                        ))
                                    }
                                    _ => {
                                        return Err(StructFieldError(
                                            StructFieldErrorKind::IncompleteField,
                                            last_range.merge(range),
                                        ))
                                    }
                                }
                            }
                            WordStream::LeftSquareBracket(lq) => {
                                let position = Range::from_word_range(lq).as_position();
                                let range = position.as_range();

                                match parsing {
                                    StructFieldParsing::Type => {
                                        let (arr_map, range) = Self::get_array_or_map(
                                            &mut curly_word_stream,
                                            Arc::new(ty.unwrap()),
                                            position,
                                        )?;
                                        field_range = field_range.merge(range);
                                        last_range = range.end_as_range();
                                        ty = Some(arr_map);
                                    }
                                    _ => {
                                        return Err(StructFieldError(
                                            StructFieldErrorKind::InvalidBracketPlacement,
                                            last_range.merge(range),
                                        ))
                                    }
                                }
                            }
                            WordStream::NativeType(native_type) => {
                                let range = Range::from_word_range(native_type);
                                parsing = match parsing {
                                    StructFieldParsing::ExpectingType => {
                                        ty = Some(Type::Native(Arc::new(TypeNative {
                                            ty: *native_type.get_word(),
                                            range,
                                        })));
                                        field_range = field_range.merge(range);
                                        last_range = range.end_as_range();
                                        StructFieldParsing::Type
                                    }
                                    StructFieldParsing::ExpectingColon => {
                                        return Err(StructFieldError(
                                            StructFieldErrorKind::MissingColon,
                                            last_range.merge(range),
                                        ))
                                    }
                                    _ => {
                                        return Err(StructFieldError(
                                            StructFieldErrorKind::TypeDeclaration,
                                            last_range.merge(range),
                                        ))
                                    }
                                };
                            }
                            WordStream::TypeName(type_name) => {
                                let range = Range::from_word_range(type_name);
                                parsing = match parsing {
                                    StructFieldParsing::ExpectingType => {
                                        ty = Some(Type::Name(Arc::new(TypeName {
                                            ident: type_name.get_word().to_owned(),
                                            range,
                                        })));
                                        field_range = field_range.merge(range);
                                        last_range = range.end_as_range();
                                        StructFieldParsing::Type
                                    }
                                    StructFieldParsing::ExpectingColon => {
                                        return Err(StructFieldError(
                                            StructFieldErrorKind::MissingColon,
                                            last_range.merge(range),
                                        ))
                                    }
                                    _ => {
                                        return Err(StructFieldError(
                                            StructFieldErrorKind::TypeDeclaration,
                                            range,
                                        ))
                                    }
                                };
                            }
                            WordStream::Identifier(ident) => {
                                let range = Range::from_word_range(ident);
                                parsing = match parsing {
                                    StructFieldParsing::ExpectingIdentifier => {
                                        field_name = Some(ident.get_word().to_owned());
                                        field_range = range;
                                        last_range = range.end_as_range();
                                        StructFieldParsing::ExpectingColon
                                    }
                                    StructFieldParsing::ExpectingColon => {
                                        return Err(StructFieldError(
                                            StructFieldErrorKind::MultipleIdentifier,
                                            last_range.merge(range),
                                        ))
                                    }
                                    StructFieldParsing::ExpectingType => {
                                        return Err(StructFieldError(
                                            StructFieldErrorKind::ExpectedType(
                                                ident.get_word().to_owned(),
                                            ),
                                            last_range.merge(range),
                                        ))
                                    }
                                    _ => {
                                        return Err(StructFieldError(
                                            StructFieldErrorKind::MissingComma,
                                            last_range.merge(range),
                                        ))
                                    }
                                }
                            }
                            WordStream::Colon(cl) => {
                                let range = Range::from_word_range(cl);
                                parsing = match parsing {
                                    StructFieldParsing::ExpectingColon => {
                                        field_range = field_range.merge(range);
                                        last_range = range.end_as_range();
                                        StructFieldParsing::ExpectingType
                                    }
                                    StructFieldParsing::ExpectingType => {
                                        return Err(StructFieldError(
                                            StructFieldErrorKind::ExpectedType("".to_owned()),
                                            last_range.merge(range),
                                        ));
                                    }
                                    _ => {
                                        return Err(StructFieldError(
                                            StructFieldErrorKind::MissingIdentifier,
                                            last_range.merge(range),
                                        ))
                                    }
                                }
                            }
                            WordStream::NewLine(_) => {}
                            WordStream::Comment(comment) => {
                                let range = Range::from_word_range(comment);
                                let comment_txt = comment.get_word().to_owned();
                                match parsing {
                                    StructFieldParsing::ExpectingIdentifier => {
                                        fields.push(StructNode::Comment(comment_txt));
                                        last_range = range.end_as_range();
                                    }
                                    _ => {
                                        return Err(StructFieldError(
                                            StructFieldErrorKind::IncompleteField,
                                            last_range.merge(range),
                                        ))
                                    }
                                }
                            }
                            sw => {
                                return Err(StructFieldError(
                                    StructFieldErrorKind::InvalidSymbol,
                                    last_range.merge(Range::from_wrange(sw.get_range())),
                                ));
                            }
                        }
                    }
                }
                WordStream::RightCurlyBracket(rc) => {
                    let end_position = Range::from_word_range(rc).as_position();
                    let range = end_position.as_range();
                    match parsing {
                        StructFieldParsing::Type => {
                            fields.push(StructNode::StructField(Box::new(StructField {
                                ident: field_name.unwrap(),
                                ty: Arc::new(ty.unwrap()),
                                range: field_range,
                            })));
                            return Ok((fields, end_position));
                        }
                        StructFieldParsing::ExpectingIdentifier => {
                            if fields.iter().any(|value| match value {
                                StructNode::StructField(_) => true,
                                _ => false,
                            }) {
                                return Ok((fields, end_position));
                            }
                            return Err(StructFieldError(
                                StructFieldErrorKind::EmptyBody,
                                last_range.merge(range),
                            ));
                        }
                        StructFieldParsing::ExpectingType => {
                            return Err(StructFieldError(
                                StructFieldErrorKind::ExpectedType("".to_owned()),
                                last_range.merge(range),
                            ));
                        }
                        _ => {
                            return Err(StructFieldError(
                                StructFieldErrorKind::IncompleteField,
                                last_range.merge(range),
                            ))
                        }
                    }
                }
                WordStream::NewLine(_) => {}
                sw => {
                    return Err(StructFieldError(
                        StructFieldErrorKind::MissingCurlyBracket,
                        last_range.merge(Range::from_wrange(sw.get_range())),
                    ))
                }
            }
        }

        Err(StructFieldError(
            StructFieldErrorKind::Undefined,
            last_range,
        ))
    }

    fn consume_enum<'a, I>(
        &mut self,
        word_stream: &mut I,
        start_position: Position,
    ) -> Result<(), EnumError>
    where
        I: Iterator<Item = &'a WordStream>,
    {
        let mut enum_name: Option<String> = None;
        let mut name_range = Range::default();
        let mut last_range = start_position.as_range();

        while let Some(w_stream) = word_stream.next() {
            match w_stream {
                WordStream::LeftCurlyBracket(lc) => match enum_name {
                    Some(ident) => {
                        let (fields, end_position) = Self::push_enum_fields(word_stream)?;

                        self.nodes.push(ParserNode::Enum(Enum {
                            ident: Arc::new(Type::Name(Arc::new(TypeName {
                                ident,
                                range: name_range,
                            }))),
                            fields,
                            range: Range {
                                start: start_position,
                                end: end_position,
                            },
                        }));

                        return Ok(());
                    }
                    None => {
                        return Err(EnumError(
                            EnumErrorKind::MissingTypeName,
                            last_range.merge(Range::from_word_range(lc)),
                        ))
                    }
                },
                WordStream::TypeName(type_name) => {
                    name_range = Range::from_word_range(type_name);
                    last_range = name_range.end_as_range();
                    if enum_name.is_some() {
                        return Err(EnumError(EnumErrorKind::TypeDeclaration, name_range));
                    }

                    enum_name = Some(type_name.get_word().to_owned());
                }
                sw => {
                    return Err(EnumError(
                        EnumErrorKind::TypeDeclaration,
                        last_range.merge(Range::from_wrange(sw.get_range())),
                    ))
                }
            }
        }

        Err(EnumError(EnumErrorKind::Undefined, last_range))
    }

    fn push_enum_fields<'a, I>(
        word_stream: &mut I,
    ) -> Result<(Vec<EnumNode>, Position), EnumFieldError>
    where
        I: Iterator<Item = &'a WordStream>,
    {
        let mut fields = vec![];
        let mut field_name: Option<String> = None;
        let mut field_range = Range::default();
        let mut last_range = Range::default();

        while let Some(n_stream) = word_stream.next() {
            match n_stream {
                WordStream::CurlyBracketBody(curly_body) => {
                    let mut curly_word_stream = curly_body.iter();

                    while let Some(w_stream) = curly_word_stream.next() {
                        match w_stream {
                            WordStream::Comma(cm) => {
                                let range = Range::from_word_range(cm);
                                match field_name.take() {
                                    Some(ident) => {
                                        fields.push(EnumNode::EnumField(Box::new(EnumField {
                                            ident,
                                            range: field_range,
                                        })));
                                        last_range = range.end_as_range();
                                        field_range = Range::default();
                                    }
                                    None => {
                                        return Err(EnumFieldError(
                                            EnumFieldErrorKind::MissingIdentifier,
                                            last_range.merge(range),
                                        ))
                                    }
                                }
                            }
                            WordStream::TypeName(ident) => {
                                let range = Range::from_word_range(ident);
                                if field_name.is_some() {
                                    return Err(EnumFieldError(
                                        EnumFieldErrorKind::MissingComma,
                                        last_range.merge(range),
                                    ));
                                }

                                field_range = range;
                                last_range = range.end_as_range();
                                field_name = Some(ident.get_word().to_owned());
                            }
                            WordStream::NewLine(_) => {}
                            WordStream::Comment(comment) => {
                                last_range = Range::from_word_range(comment);
                                let comment = comment.get_word().to_owned();
                                fields.push(EnumNode::Comment(comment));
                            }
                            sw => {
                                return Err(EnumFieldError(
                                    EnumFieldErrorKind::InvalidSymbol,
                                    last_range.merge(Range::from_wrange(sw.get_range())),
                                ));
                            }
                        }
                    }
                }
                WordStream::RightCurlyBracket(rc) => {
                    let end_position = Range::from_word_range(rc).as_position();
                    let range = end_position.as_range();

                    match field_name {
                        Some(ident) => {
                            fields.push(EnumNode::EnumField(Box::new(EnumField {
                                ident,
                                range: field_range,
                            })));
                            return Ok((fields, end_position));
                        }
                        None => {
                            if fields.iter().any(|value| match value {
                                EnumNode::EnumField(_) => true,
                                _ => false,
                            }) {
                                return Ok((fields, end_position));
                            }
                            return Err(EnumFieldError(
                                EnumFieldErrorKind::EmptyBody,
                                last_range.merge(range),
                            ));
                        }
                    }
                }
                WordStream::NewLine(_) => {}
                sw => {
                    return Err(EnumFieldError(
                        EnumFieldErrorKind::MissingCurlyBracket,
                        last_range.merge(Range::from_wrange(sw.get_range())),
                    ))
                }
            }
        }

        Err(EnumFieldError(EnumFieldErrorKind::Undefined, last_range))
    }

    fn consume_type<'a, I>(
        &mut self,
        word_stream: &mut I,
        start_position: Position,
    ) -> Result<(), TypeListError>
    where
        I: Iterator<Item = &'a WordStream>,
    {
        let mut type_list_name: Option<String> = None;
        let mut name_range = Range::default();
        let mut last_range = start_position.as_range();

        while let Some(w_stream) = word_stream.next() {
            match w_stream {
                WordStream::LeftCurlyBracket(lc) => match type_list_name {
                    Some(ident) => {
                        let (ty_list, end_position) = Self::push_type_list_fields(word_stream)?;

                        self.nodes.push(ParserNode::TypeList(TypeList {
                            ident: Arc::new(Type::Name(Arc::new(TypeName {
                                ident,
                                range: name_range,
                            }))),
                            ty_list,
                            range: Range {
                                start: start_position,
                                end: end_position,
                            },
                        }));

                        return Ok(());
                    }
                    None => {
                        return Err(TypeListError(
                            TypeListErrorKind::MissingTypeName,
                            last_range.merge(Range::from_word_range(lc)),
                        ))
                    }
                },
                WordStream::TypeName(type_name) => {
                    name_range = Range::from_word_range(type_name);

                    if type_list_name.is_some() {
                        return Err(TypeListError(
                            TypeListErrorKind::TypeDeclaration,
                            last_range.merge(name_range),
                        ));
                    }

                    last_range = name_range.end_as_range();
                    type_list_name = Some(type_name.get_word().to_owned());
                }
                sw => {
                    return Err(TypeListError(
                        TypeListErrorKind::TypeDeclaration,
                        last_range.merge(Range::from_wrange(sw.get_range())),
                    ))
                }
            }
        }

        Err(TypeListError(TypeListErrorKind::Undefined, last_range))
    }

    fn push_type_list_fields<'a, I>(
        word_stream: &mut I,
    ) -> Result<(Vec<TypeListNode>, Position), TypeListFieldError>
    where
        I: Iterator<Item = &'a WordStream>,
    {
        let mut fields = vec![];
        let mut parsing = TypeListFieldParsing::ExpectingIdentifier;
        let mut ty: Option<Type> = None;
        let mut ty_ident: Option<String> = None;
        let mut ty_range = Range::default();
        let mut last_range = Range::default();

        while let Some(n_stream) = word_stream.next() {
            match n_stream {
                WordStream::CurlyBracketBody(curly_body) => {
                    let mut curly_word_stream = curly_body.iter();

                    while let Some(w_stream) = curly_word_stream.next() {
                        match w_stream {
                            WordStream::Comma(cm) => {
                                let range = Range::from_word_range(cm);
                                parsing = match parsing {
                                    TypeListFieldParsing::Type => {
                                        fields.push(TypeListNode::TypeListField(TypeListField {
                                            ty: Arc::new(ty.take().unwrap()),
                                            ident: ty_ident.take().unwrap(),
                                            range: ty_range,
                                        }));

                                        ty_range = Range::default();
                                        last_range = range.end_as_range();
                                        TypeListFieldParsing::ExpectingIdentifier
                                    }
                                    TypeListFieldParsing::ExpectingType => {
                                        return Err(TypeListFieldError(
                                            TypeListFieldErrorKind::ExpectedType("".to_owned()),
                                            last_range.merge(range),
                                        ));
                                    }
                                    TypeListFieldParsing::ExpectingIdentifier => {
                                        return Err(TypeListFieldError(
                                            TypeListFieldErrorKind::MissingIdentifier,
                                            last_range.merge(range),
                                        ));
                                    }
                                    TypeListFieldParsing::ExpectingColon => {
                                        return Err(TypeListFieldError(
                                            TypeListFieldErrorKind::MissingColon,
                                            last_range.merge(range),
                                        ));
                                    }
                                }
                            }
                            WordStream::Colon(cl) => {
                                let range = Range::from_word_range(cl);
                                parsing = match parsing {
                                    TypeListFieldParsing::ExpectingColon => {
                                        last_range = range.end_as_range();
                                        ty_range = ty_range.merge(range);
                                        TypeListFieldParsing::ExpectingType
                                    }
                                    TypeListFieldParsing::ExpectingIdentifier => {
                                        return Err(TypeListFieldError(
                                            TypeListFieldErrorKind::MissingIdentifier,
                                            last_range.merge(range),
                                        ));
                                    }
                                    TypeListFieldParsing::ExpectingType => {
                                        return Err(TypeListFieldError(
                                            TypeListFieldErrorKind::ExpectedType("".to_owned()),
                                            last_range.merge(range),
                                        ));
                                    }
                                    _ => {
                                        return Err(TypeListFieldError(
                                            TypeListFieldErrorKind::Undefined,
                                            last_range.merge(range),
                                        ))
                                    }
                                }
                            }
                            WordStream::LeftSquareBracket(lq) => match parsing {
                                TypeListFieldParsing::Type => {
                                    let (arr_map, range) = Self::get_array_or_map(
                                        &mut curly_word_stream,
                                        Arc::new(ty.unwrap()),
                                        Range::from_word_range(lq).as_position(),
                                    )?;

                                    ty_range = ty_range.merge(range);
                                    last_range = range.end_as_range();
                                    ty = Some(arr_map);
                                }
                                _ => {
                                    return Err(TypeListFieldError(
                                        TypeListFieldErrorKind::InvalidBracketPlacement,
                                        last_range.merge(Range::from_word_range(lq)),
                                    ))
                                }
                            },
                            WordStream::NativeType(native_type) => {
                                let range = Range::from_word_range(native_type);
                                parsing = match parsing {
                                    TypeListFieldParsing::ExpectingType => {
                                        ty = Some(Type::Native(Arc::new(TypeNative {
                                            ty: *native_type.get_word(),
                                            range,
                                        })));
                                        last_range = range.end_as_range();
                                        ty_range = ty_range.merge(range);
                                        TypeListFieldParsing::Type
                                    }
                                    _ => {
                                        return Err(TypeListFieldError(
                                            TypeListFieldErrorKind::TypeDeclaration,
                                            last_range.merge(range),
                                        ))
                                    }
                                };
                            }
                            WordStream::TypeName(type_name) => {
                                let range = Range::from_word_range(type_name);
                                parsing = match parsing {
                                    TypeListFieldParsing::ExpectingType => {
                                        ty = Some(Type::Name(Arc::new(TypeName {
                                            ident: type_name.get_word().to_owned(),
                                            range,
                                        })));
                                        last_range = range.end_as_range();
                                        ty_range = ty_range.merge(range);
                                        TypeListFieldParsing::Type
                                    }
                                    TypeListFieldParsing::ExpectingIdentifier => {
                                        ty_ident = Some(type_name.get_word().to_owned());
                                        ty_range = range;
                                        last_range = range.end_as_range();
                                        TypeListFieldParsing::ExpectingColon
                                    }
                                    _ => {
                                        return Err(TypeListFieldError(
                                            TypeListFieldErrorKind::TypeDeclaration,
                                            last_range.merge(range),
                                        ))
                                    }
                                };
                            }
                            WordStream::NewLine(_) => {}
                            WordStream::Comment(comment) => {
                                let range = Range::from_word_range(comment);
                                let comment_txt = comment.get_word().to_owned();
                                match parsing {
                                    TypeListFieldParsing::ExpectingIdentifier => {
                                        fields.push(TypeListNode::Comment(comment_txt));
                                        last_range = range.end_as_range();
                                    }
                                    _ => {
                                        return Err(TypeListFieldError(
                                            TypeListFieldErrorKind::Undefined,
                                            last_range.merge(range),
                                        ))
                                    }
                                }
                            }
                            sw => {
                                return Err(TypeListFieldError(
                                    TypeListFieldErrorKind::InvalidSymbol,
                                    last_range.merge(Range::from_wrange(sw.get_range())),
                                ));
                            }
                        }
                    }
                }
                WordStream::RightCurlyBracket(rc) => {
                    let end_position = Range::from_word_range(rc).as_position();
                    let range = end_position.as_range();

                    match parsing {
                        TypeListFieldParsing::Type => {
                            fields.push(TypeListNode::TypeListField(TypeListField {
                                ty: Arc::new(ty.unwrap()),
                                ident: ty_ident.unwrap(),
                                range: ty_range,
                            }));

                            return Ok((fields, end_position));
                        }
                        TypeListFieldParsing::ExpectingIdentifier => {
                            if fields.iter().any(|value| match value {
                                TypeListNode::TypeListField(_) => true,
                                _ => false,
                            }) {
                                return Ok((fields, end_position));
                            }
                            return Err(TypeListFieldError(
                                TypeListFieldErrorKind::EmptyBody,
                                last_range.merge(range),
                            ));
                        }
                        TypeListFieldParsing::ExpectingType => {
                            return Err(TypeListFieldError(
                                TypeListFieldErrorKind::ExpectedType("".to_owned()),
                                last_range.merge(range),
                            ));
                        }
                        _ => {
                            return Err(TypeListFieldError(
                                TypeListFieldErrorKind::Undefined,
                                last_range.merge(range),
                            ))
                        }
                    }
                }
                WordStream::NewLine(_) => {}
                sw => {
                    return Err(TypeListFieldError(
                        TypeListFieldErrorKind::MissingCurlyBracket,
                        last_range.merge(Range::from_wrange(sw.get_range())),
                    ))
                }
            }
        }

        Err(TypeListFieldError(
            TypeListFieldErrorKind::Undefined,
            last_range,
        ))
    }

    fn consume_const<'a, I>(
        &mut self,
        word_stream: &mut I,
        start_position: Position,
    ) -> Result<(), ConstError>
    where
        I: Iterator<Item = &'a WordStream>,
    {
        let mut const_name: Option<String> = None;
        let mut name_range = Range::default();
        let mut last_range = start_position.as_range();

        while let Some(w_stream) = word_stream.next() {
            match w_stream {
                WordStream::LeftCurlyBracket(lc) => match const_name {
                    Some(ident) => {
                        let (const_type, fields, end_position) =
                            Self::push_const_fields(word_stream)?;
                        self.nodes.push(ParserNode::Const(Const {
                            ident: Arc::new(Type::Name(Arc::new(TypeName {
                                ident,
                                range: name_range,
                            }))),
                            const_type,
                            fields,
                            range: Range {
                                start: start_position,
                                end: end_position,
                            },
                        }));

                        return Ok(());
                    }
                    None => {
                        return Err(ConstError(
                            ConstErrorKind::MissingTypeName,
                            last_range.merge(Range::from_word_range(lc)),
                        ))
                    }
                },
                WordStream::TypeName(type_name) => {
                    name_range = Range::from_word_range(type_name);
                    last_range = name_range.end_as_range();

                    if const_name.is_some() {
                        return Err(ConstError(ConstErrorKind::TypeDeclaration, name_range));
                    }

                    const_name = Some(type_name.get_word().to_owned());
                }
                sw => {
                    return Err(ConstError(
                        ConstErrorKind::MissingTypeName,
                        last_range.merge(Range::from_wrange(sw.get_range())),
                    ))
                }
            }
        }

        Err(ConstError(ConstErrorKind::Undefined, last_range))
    }

    fn push_const_fields<'a, I>(
        word_stream: &mut I,
    ) -> Result<(ConstType, Vec<ConstNode>, Position), ConstFieldError>
    where
        I: Iterator<Item = &'a WordStream>,
    {
        let mut fields = vec![];
        let mut parsing = ConstFieldParsing::ExpectingIdentifier;
        let mut field_value: Option<String> = None;
        let mut field_name: Option<String> = None;
        let mut const_type: Option<ConstType> = None;
        let mut field_range = Range::default();
        let mut last_range = Range::default();

        while let Some(n_stream) = word_stream.next() {
            match n_stream {
                WordStream::CurlyBracketBody(curly_body) => {
                    let mut curly_word_stream = curly_body.iter();

                    while let Some(w_stream) = curly_word_stream.next() {
                        match w_stream {
                            WordStream::Comma(cm) => {
                                let range = Range::from_word_range(cm);

                                match parsing {
                                    ConstFieldParsing::Value => {
                                        fields.push(ConstNode::ConstField(ConstField {
                                            ident: field_name.take().unwrap(),
                                            value: field_value.take().unwrap(),
                                            const_type: const_type.unwrap(),
                                            range: field_range,
                                        }));

                                        field_range = Range::default();
                                        last_range = range.end_as_range();
                                        parsing = ConstFieldParsing::ExpectingIdentifier;
                                    }
                                    _ => {
                                        return Err(ConstFieldError(
                                            ConstFieldErrorKind::IncompleteField,
                                            last_range.merge(range),
                                        ))
                                    }
                                }
                            }
                            WordStream::Identifier(ident) => {
                                parsing = match parsing {
                                    ConstFieldParsing::ExpectingIdentifier => {
                                        field_name = Some(ident.get_word().to_owned());
                                        field_range = Range::from_word_range(ident);
                                        last_range = field_range.end_as_range();
                                        ConstFieldParsing::ExpectingColon
                                    }
                                    ConstFieldParsing::ExpectingColon => {
                                        return Err(ConstFieldError(
                                            ConstFieldErrorKind::MultipleIdentifier,
                                            Range::from_word_range(ident),
                                        ))
                                    }
                                    _ => {
                                        return Err(ConstFieldError(
                                            ConstFieldErrorKind::MissingComma,
                                            last_range.merge(Range::from_word_range(ident)),
                                        ))
                                    }
                                }
                            }
                            WordStream::Colon(ag) => {
                                let range = Range::from_word_range(ag);
                                parsing = match parsing {
                                    ConstFieldParsing::ExpectingColon => {
                                        field_range = field_range.merge(range);
                                        last_range = field_range.end_as_range();
                                        ConstFieldParsing::ExpectingValue
                                    }
                                    _ => {
                                        return Err(ConstFieldError(
                                            ConstFieldErrorKind::MissingIdentifier,
                                            last_range.merge(range),
                                        ))
                                    }
                                }
                            }
                            WordStream::StringBody(s_body)
                            | WordStream::FloatValue(s_body)
                            | WordStream::IntegerValue(s_body) => {
                                let range = Range::from_word_range(s_body);

                                const_type = match w_stream {
                                    WordStream::StringBody(_) => match const_type {
                                        Some(ConstType::String) | None => Some(ConstType::String),
                                        _ => {
                                            return Err(ConstFieldError(
                                                ConstFieldErrorKind::ConstTypeMustBeUnique,
                                                range,
                                            ))
                                        }
                                    },
                                    WordStream::FloatValue(_) => match const_type {
                                        Some(ConstType::Float) | None => Some(ConstType::Float),
                                        _ => {
                                            return Err(ConstFieldError(
                                                ConstFieldErrorKind::ConstTypeMustBeUnique,
                                                range,
                                            ))
                                        }
                                    },
                                    WordStream::IntegerValue(_) => match const_type {
                                        Some(ConstType::Int) | None => Some(ConstType::Int),
                                        _ => {
                                            return Err(ConstFieldError(
                                                ConstFieldErrorKind::ConstTypeMustBeUnique,
                                                range,
                                            ))
                                        }
                                    },
                                    _ => None,
                                };

                                parsing = match parsing {
                                    ConstFieldParsing::ExpectingValue => {
                                        field_value = Some(s_body.get_word().to_owned());
                                        field_range = field_range.merge(range);
                                        last_range = range.end_as_range();

                                        ConstFieldParsing::Value
                                    }
                                    ConstFieldParsing::Value => {
                                        return Err(ConstFieldError(
                                            ConstFieldErrorKind::IncompleteField,
                                            last_range.merge(range),
                                        ));
                                    }
                                    ConstFieldParsing::ExpectingIdentifier => {
                                        return Err(ConstFieldError(
                                            ConstFieldErrorKind::MissingIdentifier,
                                            last_range.merge(range),
                                        ));
                                    }
                                    ConstFieldParsing::ExpectingColon => {
                                        return Err(ConstFieldError(
                                            ConstFieldErrorKind::MissingAssignment,
                                            last_range.merge(range),
                                        ));
                                    }
                                }
                            }
                            WordStream::NewLine(_) => {}
                            WordStream::Comment(comment) => {
                                let range = Range::from_word_range(comment);
                                let comment_txt = comment.get_word().to_owned();
                                match parsing {
                                    ConstFieldParsing::ExpectingIdentifier => {
                                        fields.push(ConstNode::Comment(comment_txt));
                                    }
                                    _ => {
                                        return Err(ConstFieldError(
                                            ConstFieldErrorKind::IncompleteField,
                                            last_range.merge(range),
                                        ))
                                    }
                                }
                            }
                            sw => {
                                return Err(ConstFieldError(
                                    ConstFieldErrorKind::InvalidSymbol,
                                    last_range.merge(Range::from_wrange(sw.get_range())),
                                ));
                            }
                        }
                    }
                }
                WordStream::RightCurlyBracket(rc) => {
                    let end_position = Range::from_word_range(rc).as_position();
                    let range = end_position.as_range();

                    match parsing {
                        ConstFieldParsing::Value => {
                            fields.push(ConstNode::ConstField(ConstField {
                                ident: field_name.unwrap(),
                                value: field_value.unwrap(),
                                const_type: const_type.unwrap(),
                                range: field_range,
                            }));
                            return Ok((const_type.unwrap(), fields, end_position));
                        }
                        ConstFieldParsing::ExpectingIdentifier => {
                            if fields.iter().any(|value| match value {
                                ConstNode::ConstField(_) => true,
                                _ => false,
                            }) {
                                return Ok((const_type.unwrap(), fields, end_position));
                            }
                            return Err(ConstFieldError(
                                ConstFieldErrorKind::EmptyBody,
                                last_range.merge(range),
                            ));
                        }
                        _ => {
                            return Err(ConstFieldError(
                                ConstFieldErrorKind::IncompleteField,
                                last_range.merge(range),
                            ))
                        }
                    }
                }
                WordStream::NewLine(_) => {}
                sw => {
                    return Err(ConstFieldError(
                        ConstFieldErrorKind::MissingCurlyBracket,
                        last_range.merge(Range::from_wrange(sw.get_range())),
                    ))
                }
            }
        }

        Err(ConstFieldError(ConstFieldErrorKind::Undefined, last_range))
    }
}
