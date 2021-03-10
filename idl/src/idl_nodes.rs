use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub enum Keywords {
    Enum,
    Struct,
    Interface,
    Import,
    Library,
    Type,
    Const,
    Stream,
    Static,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub enum Types {
    NatInt,
    NatFloat,
    NatString,
    NatBytes,
    NatBool,
    NatUUID,
    NatNone,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub enum IdlNode {
    LibraryName(String),
    Imports(Vec<String>),
    Comment(Vec<String>),
    InterfaceComment(Vec<String>),
    StructComment(Vec<String>),
    EnumComment(Vec<String>),
    ConstComment(Vec<String>),
    TypeListComment(Vec<String>),
    TypeStruct(Box<TypeStruct>),
    TypeEnum(Box<TypeEnum>),
    TypeList(Box<TypeList>),
    TypeConst(Box<TypeConst>),
    TypeInterface(Box<TypeInterface>),
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct TypeInterface {
    pub ident: String,
    pub fields: Vec<InterfaceNode>,
    pub hash: Box<[u8]>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub enum InterfaceNode {
    InterfaceField(Box<InterfaceField>),
    Comment(Vec<String>),
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct InterfaceField {
    pub attributes: Vec<Attributes>,
    pub ident: String,
    pub is_static: bool,
    pub ty: TypeName,
    pub hash: Box<[u8]>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct TypeStruct {
    pub ident: String,
    pub fields: Vec<StructNode>,
    pub hash: Box<[u8]>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub enum StructNode {
    StructField(Box<StructField>),
    Comment(Vec<String>),
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct StructField {
    pub ident: String,
    pub ty: TypeName,
    pub hash: Box<[u8]>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct TypeList {
    pub ident: String,
    pub fields: Vec<TypeListNode>,
    pub hash: Box<[u8]>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub enum TypeListNode {
    TypeListField(Box<TypeListField>),
    Comment(Vec<String>),
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct TypeListField {
    pub ident: String,
    pub ty: TypeName,
    pub hash: Box<[u8]>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct TypeEnum {
    pub ident: String,
    pub fields: Vec<EnumNode>,
    pub hash: Box<[u8]>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub enum EnumNode {
    EnumField(Box<EnumField>),
    Comment(Vec<String>),
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct EnumField {
    pub ident: String,
    pub hash: Box<[u8]>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct TypeConst {
    pub ident: String,
    pub fields: Vec<ConstNode>,
    pub const_type: ConstTypes,
    pub hash: Box<[u8]>,
}

#[derive(Debug, Copy, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub enum ConstTypes {
    NatInt,
    NatFloat,
    NatString,
    NatUuid,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub enum ConstNode {
    ConstField(Box<ConstField>),
    Comment(Vec<String>),
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct ConstField {
    pub ident: String,
    pub value: String,
    pub hash: Box<[u8]>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct Attributes {
    pub field: Vec<AttributeNode>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub enum AttributeNode {
    Name(AttributeNames),
    UnknownName(String),
    StringField(String),
}

#[derive(Debug, Copy, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub enum AttributeNames {
    DeprecatedName,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub enum TypeName {
    Types(Types),
    TypeFunction(Box<TypeFunction>),
    TypeTuple(Box<TypeTuple>),
    TypeArray(Box<TypeArray>),
    TypeMap(Box<TypeMap>),
    TypeOption(Box<TypeOption>),
    TypeResult(Box<TypeResult>),
    TypePair(Box<TypePair>),
    TypeStream(Box<TypeStream>),
    ListTypeName(String),
    EnumTypeName(String),
    StructTypeName(String),
    InterfaceTypeName(String),
    ConstTypeName(String),
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct TypeFunction {
    pub args: TypeName,
    pub return_ty: TypeName,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct TypeArray {
    pub ty: TypeName,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct TypeMap {
    pub map_ty: TypeName,
    pub index_ty: TypeName,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct TypePair {
    pub first_ty: TypeName,
    pub second_ty: TypeName,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct TypeTuple {
    pub fields: Vec<TupleEntry>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct TypeStream {
    pub s_ty: TypeName,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct TupleEntry {
    pub ident: String,
    pub ty: TypeName,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct TypeResult {
    pub ok_ty: TypeName,
    pub err_ty: TypeName,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct TypeOption {
    pub some_ty: TypeName,
}