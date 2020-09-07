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
pub enum InterfaceAttributes {}

#[derive(Debug)]
pub enum TypeNode {
    LibraryName(String),
    Imports(Vec<String>),
    Comment(String),
    InterfaceComment(String),
    TypeInterface(Box<TypeInterface>),
    StructComment(StructComment),
    TypeStruct(Box<TypeStruct>),
    EnumComment(String),
    TypeEnum(Box<TypeEnum>),
    TypeListComment(String),
    TypeList(Box<TypeList>),
    ConstComment(String),
    TypeConst(Box<TypeConst>),
    StreamComment(String),
    TypeStream(Box<TypeStream>),
    FactoryComment(String),
    TypeFactory(Box<TypeFactory>),
}

#[derive(Debug)]
pub struct TypeFactory {
    pub ident: String,
    pub fields: Vec<FactoryNode>,
}

#[derive(Debug)]
pub enum FactoryNode {
    FactoryField(Box<FactoryField>),
    Comment(String),
}

#[derive(Debug)]
pub struct FactoryField {
    pub attributes: Vec<Attribute>,
    pub ident: String,
    pub ty: TypeName,
}

#[derive(Debug)]
pub struct TypeInterface {
    pub ident: String,
    pub fields: Vec<InterfaceNode>,
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
    Comment(String),
}

#[derive(Debug)]
pub struct StructField {
    pub ident: String,
    pub ty: TypeName,
}

#[derive(Debug)]
pub struct TypeStream {
    pub ident: String,
    pub fields: Vec<StreamNode>,
}

#[derive(Debug)]
pub enum StreamNode {
    StreamField(Box<StreamField>),
    Comment(String),
}

#[derive(Debug)]
pub struct StreamField {
    pub ident: String,
    pub ty: TypeName,
}

#[derive(Debug)]
pub struct TypeList {
    pub ident: String,
    pub ty_list: Vec<TypeListNode>,
}

#[derive(Debug)]
pub enum TypeListNode {
    TypeListField(Box<TypeListField>),
    Comment(String),
}

#[derive(Debug)]
pub struct TypeListField {
    pub ident: String,
    pub ty: TypeName,
}

#[derive(Debug)]
pub struct TypeEnum {
    pub ident: String,
    pub fields: Vec<EnumNode>,
}

#[derive(Debug)]
pub enum EnumNode {
    EnumField(Box<EnumField>),
    Comment(String),
}

#[derive(Debug)]
pub struct EnumField {
    pub ident: String,
}

#[derive(Debug)]
pub struct TypeConst {
    pub ident: String,
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
    Comment(String),
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
    TypeResult(Box<TypeResult>),
    TypeOption(Box<TypeOption>),
    ListTypeName(String),
    EnumTypeName(String),
    StructTypeName(StructTypeName),
    InterfaceTypeName(String),
    ConstTypeName(String),
    FactoryTypeName(String),
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

#[derive(Debug)]
pub struct TypeResult {
    pub ok_ty: TypeName,
    pub err_ty: TypeName,
}

#[derive(Debug)]
pub struct TypeOption {
    pub some_ty: TypeName,
}
