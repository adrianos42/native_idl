#[derive(Debug, Copy, Clone)]
pub enum Keywords {
    Enum,
    Struct,
    Interface,
    Import,
    Library,
    Type,
    Const,
    Stream,
    Factory,
}

#[derive(Debug, Copy, Clone)]
pub enum Types {
    NativeInt,
    NativeFloat,
    NativeString,
    NativeBytes,
    NativeBool,
    NativeNone,
}

#[derive(Debug, Copy, Clone)]
pub enum InterfaceAttributes {
  
}

#[derive(Debug)]
pub enum TypeNode {
    LibraryName(LibraryName),
    Imports(Imports),
    Comment(Comment),
    InterfaceComment(InterfaceComment),
    TypeInterface(Box<TypeInterface>),
    StructComment(StructComment),
    TypeStruct(Box<TypeStruct>),
    EnumComment(EnumComment),
    TypeEnum(Box<TypeEnum>),
    TypeListComment(TypeListComment),
    TypeList(Box<TypeList>),
    ConstComment(ConstComment),
    TypeConst(Box<TypeConst>),
    StreamComment(StreamComment),
    TypeStream(Box<TypeStream>),
    FactoryComment(FactoryComment),
    TypeFactory(Box<TypeFactory>),
}

pub type LibraryName = String;
pub type Imports = Vec<String>;
pub type Comment = String;

pub type FactoryComment = String;
pub type FactoryTypeName = String;

#[derive(Debug)]
pub struct TypeFactory {
    pub ident: FactoryTypeName,
    pub fields: Vec<FactoryNode>,
}

#[derive(Debug)]
pub enum FactoryNode {
    FactoryField(Box<FactoryField>),
    Comment(Comment),
}

#[derive(Debug)]
pub struct FactoryField {
    pub attributes: Vec<Attribute>,
    pub ident: String,
    pub ty: TypeName,
}

pub type InterfaceComment = String;
pub type InterfaceTypeName = String;

#[derive(Debug)]
pub struct TypeInterface {
    pub ident: InterfaceTypeName,
    pub fields: Vec<InterfaceNode>,
}

#[derive(Debug)]
pub enum InterfaceNode {
    InterfaceField(Box<InterfaceField>),
    Comment(Comment),
}

#[derive(Debug)]
pub struct InterfaceField {
    pub attributes: Vec<Attribute>,
    pub ident: String,
    pub ty: TypeName,
}

pub type StructComment = String;
pub type StructTypeName = String;

#[derive(Debug)]
pub struct TypeStruct {
    pub ident: StructTypeName,
    pub fields: Vec<StructNode>,
}

#[derive(Debug)]
pub enum StructNode {
    StructField(Box<StructField>),
    Comment(Comment),
}

#[derive(Debug)]
pub struct StructField {
    pub ident: String,
    pub ty: TypeName,
}

pub type StreamComment = String;
pub type StreamTypeName = String;

#[derive(Debug)]
pub struct TypeStream {
    pub ident: StreamTypeName,
    pub fields: Vec<StreamNode>,
}

#[derive(Debug)]
pub enum StreamNode {
    StreamField(Box<StreamField>),
    Comment(Comment),
}

#[derive(Debug)]
pub struct StreamField {
    pub ident: String,
    pub ty: TypeName,
}

pub type TypeListComment = String;
pub type ListTypeName = String;

#[derive(Debug)]
pub struct TypeList {
    pub ident: ListTypeName,
    pub ty_list: Vec<TypeListNode>,
}

#[derive(Debug)]
pub enum TypeListNode {
    TypeListField(Box<TypeListField>),
    Comment(Comment),
}

#[derive(Debug)]
pub struct TypeListField {
    pub ty: TypeName,
}

pub type EnumComment = String;
pub type EnumTypeName = String;

#[derive(Debug)]
pub struct TypeEnum {
    pub ident: EnumTypeName,
    pub fields: Vec<EnumNode>,
}

#[derive(Debug)]
pub enum EnumNode {
    EnumField(Box<EnumField>),
    Comment(Comment),
}

#[derive(Debug)]
pub struct EnumField {
    pub ident: String,
}

pub type ConstComment = String;
pub type ConstTypeName = String;

#[derive(Debug)]
pub struct TypeConst {
    pub ident: ConstTypeName,
    pub fields: Vec<ConstNode>,
    pub const_type: ConstTypes,
}

#[derive(Debug)]
pub enum ConstTypes {
    NativeInt,
    NativeFloat,
    NativeString,
}

#[derive(Debug)]
pub enum ConstNode {
    ConstField(Box<ConstField>),
    Comment(Comment),
}

#[derive(Debug)]
pub struct ConstField {
    pub ident: String,
    pub value: String,
}

#[derive(Debug)]
pub struct Attribute {
    pub fields: Vec<AttributeField>,
}

#[derive(Debug)]
pub enum AttributeField {
    Name(InterfaceAttributes),
    UnknownName(String),
    StringField(String),
}

#[derive(Debug)]
pub enum TypeName {
    Types(Types),
    TypeFunction(Box<TypeFunction>),
    TypeTuple(Box<TypeTuple>),
    TypeArray(Box<TypeArray>),
    TypeMap(Box<TypeMap>),
    ListTypeName(ListTypeName),
    EnumTypeName(EnumTypeName),
    StructTypeName(StructTypeName),
    InterfaceTypeName(InterfaceTypeName),
    ConstTypeName(ConstTypeName),
    FactoryTypeName(FactoryTypeName),
    StreamTypeName(StructTypeName),
}

#[derive(Debug)]
pub struct TypeFunction {
    pub args: TypeName,
    pub return_ty: TypeName,
}

#[derive(Debug)]
pub struct TypeArray {
    pub ty: TypeName,
}

#[derive(Debug)]
pub struct TypeMap {
    pub map_ty: TypeName,
    pub index_ty: TypeName,
}

#[derive(Debug)]
pub struct TypeTuple {
    pub ty_list: Vec<TupleEntry>,
}

#[derive(Debug)]
pub struct TupleEntry {
    pub ident: String,
    pub ty: TypeName,
}
