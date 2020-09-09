#[derive(Debug, Copy, Clone)]
pub enum Types {
    NatInt,
    NatFloat,
    NatString,
    NatBytes,
    NatBool,
    NatNone,
}

#[derive(Debug)]
pub enum TypeNode {
    LibraryName(String),
    Imports(Vec<String>),
    Comment(Vec<String>),
    InterfaceComment(Vec<String>),
    StructComment(Vec<String>),
    EnumComment(Vec<String>),
    ConstComment(Vec<String>),
    FactoryComment(Vec<String>),
    StreamComment(Vec<String>),
    TypeListComment(Vec<String>),
    TypeStruct(Box<TypeStruct>),
    TypeStream(Box<TypeStream>),
    TypeFactory(Box<TypeFactory>),
    TypeEnum(Box<TypeEnum>),
    TypeList(Box<TypeList>),
    TypeConst(Box<TypeConst>),
    TypeInterface(Box<TypeInterface>),
}

#[derive(Debug)]
pub struct TypeFactory {
    pub ident: String,
    pub fields: Vec<FactoryNode>,
}

#[derive(Debug)]
pub enum FactoryNode {
    FactoryField(Box<FactoryField>),
    Comment(Vec<String>),
}

#[derive(Debug)]
pub struct FactoryField {
    pub attributes: Vec<Attributes>,
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
    Comment(Vec<String>),
}

#[derive(Debug)]
pub struct InterfaceField {
    pub attributes: Vec<Attributes>,
    pub ident: String,
    pub ty: TypeName,
}

#[derive(Debug)]
pub struct TypeStruct {
    pub ident: String,
    pub fields: Vec<StructNode>,
}

#[derive(Debug)]
pub enum StructNode {
    StructField(Box<StructField>),
    Comment(Vec<String>),
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
    Comment(Vec<String>),
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
    Comment(Vec<String>),
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
    Comment(Vec<String>),
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

#[derive(Debug, Copy, Clone)]
pub enum ConstTypes {
    NatInt,
    NatFloat,
    NatString,
}

#[derive(Debug)]
pub enum ConstNode {
    ConstField(Box<ConstField>),
    Comment(Vec<String>),
}

#[derive(Debug)]
pub struct ConstField {
    pub ident: String,
    pub value: String,
}

#[derive(Debug)]
pub struct Attributes {
    pub field: Vec<AttributeNode>,
}

#[derive(Debug)]
pub enum AttributeNode {
    Name(AttributeNames),
    UnknownName(String),
    StringField(String),
}

#[derive(Debug, Copy, Clone)]
pub enum AttributeNames {
    DeprecatedName,
}

#[derive(Debug)]
pub enum TypeName {
    Types(Types),
    TypeFunction(Box<TypeFunction>),
    TypeTuple(Box<TypeTuple>),
    TypeArray(Box<TypeArray>),
    TypeMap(Box<TypeMap>),
    TypeOption(Box<TypeOption>),
    TypeResult(Box<TypeResult>),
    ListTypeName(String),
    EnumTypeName(String),
    StructTypeName(String),
    InterfaceTypeName(String),
    ConstTypeName(String),
    FactoryTypeName(String),
    StreamTypeName(String),
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
