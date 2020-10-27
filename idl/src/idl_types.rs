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
    NatNone,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum TypeNode {
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

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TypeInterface {
    pub ident: String,
    pub fields: Vec<InterfaceNode>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum InterfaceNode {
    InterfaceField(Box<InterfaceField>),
    Comment(Vec<String>),
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct InterfaceField {
    pub attributes: Vec<Attributes>,
    pub ident: String,
    pub is_static: bool,
    pub ty: TypeName,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TypeStruct {
    pub ident: String,
    pub fields: Vec<StructNode>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum StructNode {
    StructField(Box<StructField>),
    Comment(Vec<String>),
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct StructField {
    pub ident: String,
    pub ty: TypeName,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TypeList {
    pub ident: String,
    pub fields: Vec<TypeListNode>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum TypeListNode {
    TypeListField(Box<TypeListField>),
    Comment(Vec<String>),
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TypeListField {
    pub ident: String,
    pub ty: TypeName,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TypeEnum {
    pub ident: String,
    pub fields: Vec<EnumNode>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum EnumNode {
    EnumField(Box<EnumField>),
    Comment(Vec<String>),
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct EnumField {
    pub ident: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TypeConst {
    pub ident: String,
    pub fields: Vec<ConstNode>,
    pub const_type: ConstTypes,
}

#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
pub enum ConstTypes {
    NatInt,
    NatFloat,
    NatString,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum ConstNode {
    ConstField(Box<ConstField>),
    Comment(Vec<String>),
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ConstField {
    pub ident: String,
    pub value: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Attributes {
    pub field: Vec<AttributeNode>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum AttributeNode {
    Name(AttributeNames),
    UnknownName(String),
    StringField(String),
}

#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
pub enum AttributeNames {
    DeprecatedName,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum TypeName {
    Types(Types),
    TypeFunction(Box<TypeFunction>),
    TypeTuple(Box<TypeTuple>),
    TypeArray(Box<TypeArray>),
    TypeMap(Box<TypeMap>),
    TypeOption(Box<TypeOption>),
    TypeResult(Box<TypeResult>),
    TypeStream(Box<TypeStream>),
    ListTypeName(String),
    EnumTypeName(String),
    StructTypeName(String),
    InterfaceTypeName(String),
    ConstTypeName(String),
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TypeFunction {
    pub args: TypeName,
    pub return_ty: TypeName,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TypeArray {
    pub ty: TypeName,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TypeMap {
    pub map_ty: TypeName,
    pub index_ty: TypeName,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TypeTuple {
    pub fields: Vec<TupleEntry>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TypeStream {
    pub s_ty: TypeName,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TupleEntry {
    pub ident: String,
    pub ty: TypeName,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TypeResult {
    pub ok_ty: TypeName,
    pub err_ty: TypeName,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TypeOption {
    pub some_ty: TypeName,
}