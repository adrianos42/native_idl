use super::idl_types;
use super::module;
use super::parser::{self, Range};
use super::scanner;

use regex::Regex;
use std::convert::From;
use std::fmt;
use std::sync::Arc;

#[derive(Debug)]
pub enum ReferenceErrorKind {
    Invalid,
    ReferencesInterface,
    ReferencesFactory,
    ReferencesStream,
    StructRecursiveReference,
    UndefinedType,
}

impl From<ReferenceError> for AnalyzerError {
    fn from(value: ReferenceError) -> Self {
        AnalyzerError::ReferenceError(value)
    }
}

#[derive(Debug)]
pub struct ReferenceError(ReferenceErrorKind, Range, String);

impl fmt::Display for ReferenceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = self.2.as_str();

        let errstr = match &self.0 {
            ReferenceErrorKind::Invalid => format!("Invalid type name `{}`.", name),
            ReferenceErrorKind::ReferencesInterface => format!("References interface `{}`.", name),
            ReferenceErrorKind::ReferencesFactory => format!("References factory `{}`.", name),
            ReferenceErrorKind::ReferencesStream => format!("References stream `{}`.", name),
            ReferenceErrorKind::StructRecursiveReference => {
                format!("Recursive reference in struct `{}`.", name)
            }
            ReferenceErrorKind::UndefinedType => format!("Undefined type `{}`.", name),
        };

        write!(f, "{}", errstr)
    }
}

#[derive(Debug)]
pub enum FactoryErrorKind {
    DoesNotReturnInterface,
}

impl From<FactoryError> for AnalyzerError {
    fn from(value: FactoryError) -> Self {
        AnalyzerError::FactoryError(value)
    }
}

#[derive(Debug)]
pub struct FactoryError(FactoryErrorKind, Range);

impl fmt::Display for FactoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let errstr = match &self.0 {
            FactoryErrorKind::DoesNotReturnInterface => "Field does not return an interface.",
        };

        write!(f, "{}", errstr)
    }
}

#[derive(Debug)]
pub struct NameError(NameErrorKind, Range, String);

impl From<NameError> for AnalyzerError {
    fn from(value: NameError) -> Self {
        AnalyzerError::NameError(value)
    }
}

#[derive(Debug)]
pub enum NameErrorKind {
    InvalidFieldName,
    InvalidTypeName,
}

impl fmt::Display for NameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let errstr = match &self.0 {
            NameErrorKind::InvalidFieldName => format!("Invalid field name `{}`", self.2),
            NameErrorKind::InvalidTypeName => format!("Invalid type name `{}`", self.2),
        };

        write!(f, "{}", errstr)
    }
}

#[derive(Debug)]
pub struct DuplicateFieldNameError(String, Range);

impl From<DuplicateFieldNameError> for AnalyzerError {
    fn from(value: DuplicateFieldNameError) -> Self {
        AnalyzerError::DuplicateFieldNameError(value)
    }
}

impl fmt::Display for DuplicateFieldNameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Duplicate field `{}`.", self.0)
    }
}

#[derive(Debug)]
pub enum AnalyzerError {
    Undefined,
    Closed,
    ImportDefinition,
    LibraryDefinition,
    MissingLibraryDefinition,
    DuplicateFieldNameError(DuplicateFieldNameError),
    NameError(NameError),
    FactoryError(FactoryError),
    ReferenceError(ReferenceError),
}

impl AnalyzerError {
    pub fn get_message_with_range(&self) -> (String, Range) {
        match self {
            AnalyzerError::Undefined
            | AnalyzerError::Closed
            | AnalyzerError::ImportDefinition
            | AnalyzerError::LibraryDefinition
            | AnalyzerError::MissingLibraryDefinition => (self.to_string(), Range::default()),
            AnalyzerError::DuplicateFieldNameError(value) => (value.to_string(), value.1),
            AnalyzerError::FactoryError(value) => (value.to_string(), value.1),
            AnalyzerError::ReferenceError(value) => (value.to_string(), value.1),
            AnalyzerError::NameError(value) => (value.to_string(), value.1),
        }
    }
}

impl fmt::Display for AnalyzerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let errstr = match self {
            AnalyzerError::Undefined => "Analyzer error.".to_owned(),
            AnalyzerError::Closed => "Analyzer closed.".to_owned(),
            AnalyzerError::ImportDefinition => "Import redefinition.".to_owned(),
            AnalyzerError::LibraryDefinition => "Library redefinition.".to_owned(),
            AnalyzerError::MissingLibraryDefinition => "Missing library definition.".to_owned(),
            AnalyzerError::DuplicateFieldNameError(dup_err) => dup_err.to_string(),
            AnalyzerError::NameError(name_err) => name_err.to_string(),
            AnalyzerError::FactoryError(fac_err) => fac_err.to_string(),
            AnalyzerError::ReferenceError(ref_err) => ref_err.to_string(),
        };

        write!(f, "{}", errstr)
    }
}

#[derive(Debug, Default)]
pub struct Analyzer {
    pub(super) nodes: Vec<idl_types::TypeNode>,
}

#[derive(Debug, Default)]
struct AnalyzerItems {
    interfaces: Vec<String>,
    structs: Vec<String>,
    enums: Vec<String>,
    consts: Vec<String>,
    type_lists: Vec<String>,
    factories: Vec<String>,
    streams: Vec<String>,
    library_name: Option<idl_types::LibraryName>,
    imports: Option<idl_types::Imports>,
}

impl Analyzer {
    pub fn closed() -> Result<Self, AnalyzerError> {
        Err(AnalyzerError::Closed)
    }

    pub fn resolve(
        parsers: &parser::Parser,
        parent_module: &module::Module,
    ) -> Result<Self, AnalyzerError> {
        let mut items = AnalyzerItems::default();
        let mut analyzer = Self::default();

        for p_value in parsers.nodes.iter() {
            match p_value {
                parser::ParserNode::Imports(value) => {
                    if items.imports.is_some() {
                        return Err(AnalyzerError::ImportDefinition);
                    }
                    items.imports = Some(value.to_vec());
                }
                parser::ParserNode::Library(value) => {
                    if items.library_name.is_some() {
                        return Err(AnalyzerError::LibraryDefinition);
                    }
                    items.library_name = Some(value.to_owned());
                }
                parser::ParserNode::Interface(value) => {
                    let ident = match &*value.ident.clone() {
                        parser::Type::Name(name) => name.ident.to_owned(),
                        _ => {
                            return Err(AnalyzerError::ReferenceError(ReferenceError(
                                ReferenceErrorKind::Invalid,
                                value.range,
                                value.ident.to_string(),
                            )))
                        }
                    };
                    Self::type_name_is_valid(ident.as_str(), value.range)?;
                    items.interfaces.push(ident);
                }
                parser::ParserNode::Struct(value) => {
                    let ident = match &*value.ident.clone() {
                        parser::Type::Name(name) => name.ident.to_owned(),
                        _ => {
                            return Err(AnalyzerError::ReferenceError(ReferenceError(
                                ReferenceErrorKind::Invalid,
                                value.range,
                                value.ident.to_string(),
                            )))
                        }
                    };
                    Self::type_name_is_valid(ident.as_str(), value.range)?;
                    items.structs.push(ident);
                }
                parser::ParserNode::Enum(value) => {
                    let ident = match &*value.ident.clone() {
                        parser::Type::Name(name) => name.ident.to_owned(),
                        _ => {
                            return Err(AnalyzerError::ReferenceError(ReferenceError(
                                ReferenceErrorKind::Invalid,
                                value.range,
                                value.ident.to_string(),
                            )))
                        }
                    };
                    Self::type_name_is_valid(ident.as_str(), value.range)?;
                    items.enums.push(ident);
                }
                parser::ParserNode::Const(value) => {
                    let ident = match &*value.ident.clone() {
                        parser::Type::Name(name) => name.ident.to_owned(),
                        _ => {
                            return Err(AnalyzerError::ReferenceError(ReferenceError(
                                ReferenceErrorKind::Invalid,
                                value.range,
                                value.ident.to_string(),
                            )))
                        }
                    };
                    Self::type_name_is_valid(ident.as_str(), value.range)?;
                    items.consts.push(ident);
                }
                parser::ParserNode::TypeList(value) => {
                    let ident = match &*value.ident.clone() {
                        parser::Type::Name(name) => name.ident.to_owned(),
                        _ => {
                            return Err(AnalyzerError::ReferenceError(ReferenceError(
                                ReferenceErrorKind::Invalid,
                                value.range,
                                value.ident.to_string(),
                            )))
                        }
                    };
                    Self::type_name_is_valid(ident.as_str(), value.range)?;
                    items.type_lists.push(ident);
                }
                parser::ParserNode::Factory(value) => {
                    let ident = match &*value.ident.clone() {
                        parser::Type::Name(name) => name.ident.to_owned(),
                        _ => {
                            return Err(AnalyzerError::ReferenceError(ReferenceError(
                                ReferenceErrorKind::Invalid,
                                value.range,
                                value.ident.to_string(),
                            )))
                        }
                    };
                    Self::type_name_is_valid(ident.as_str(), value.range)?;
                    items.factories.push(ident);
                }
                parser::ParserNode::Stream(value) => {
                    let ident = match &*value.ident.clone() {
                        parser::Type::Name(name) => name.ident.to_owned(),
                        _ => {
                            return Err(AnalyzerError::ReferenceError(ReferenceError(
                                ReferenceErrorKind::Invalid,
                                value.range,
                                value.ident.to_string(),
                            )))
                        }
                    };
                    Self::type_name_is_valid(ident.as_str(), value.range)?;
                    items.streams.push(ident);
                }
                _ => {}
            }
        }

        if let Some(value) = &items.imports {
            analyzer
                .nodes
                .push(idl_types::TypeNode::Imports(value.to_owned()));
        }

        match &items.library_name {
            Some(value) => analyzer
                .nodes
                .push(idl_types::TypeNode::LibraryName(value.to_owned())),
            None => return Err(AnalyzerError::MissingLibraryDefinition),
        }

        for p_value in parsers.nodes.iter() {
            match p_value {
                parser::ParserNode::Comment(value) => {
                    analyzer
                        .nodes
                        .push(idl_types::TypeNode::Comment(value.to_owned()));
                }
                parser::ParserNode::EnumComment(value) => {
                    analyzer
                        .nodes
                        .push(idl_types::TypeNode::EnumComment(value.to_owned()));
                }
                parser::ParserNode::FactoryComment(value) => {
                    analyzer
                        .nodes
                        .push(idl_types::TypeNode::FactoryComment(value.to_owned()));
                }
                parser::ParserNode::StreamComment(value) => {
                    analyzer
                        .nodes
                        .push(idl_types::TypeNode::StreamComment(value.to_owned()));
                }
                parser::ParserNode::InterfaceComment(value) => {
                    analyzer
                        .nodes
                        .push(idl_types::TypeNode::InterfaceComment(value.to_owned()));
                }
                parser::ParserNode::StructComment(value) => {
                    analyzer
                        .nodes
                        .push(idl_types::TypeNode::StructComment(value.to_owned()));
                }
                parser::ParserNode::ConstComment(value) => {
                    analyzer
                        .nodes
                        .push(idl_types::TypeNode::ConstComment(value.to_owned()));
                }
                parser::ParserNode::TypeListComment(value) => {
                    analyzer
                        .nodes
                        .push(idl_types::TypeNode::TypeListComment(value.to_owned()));
                }
                parser::ParserNode::Interface(value) => {
                    let ident = match items.get_type(value.ident.clone()) {
                        Ok(value) => match value {
                            idl_types::TypeName::InterfaceTypeName(value) => value,
                            _ => return Err(AnalyzerError::Undefined),
                        },
                        Err(err) => return Err(err.into()),
                    };

                    let mut fields = vec![];

                    for field in value.fields.iter() {
                        match field {
                            parser::InterfaceNode::Comment(comment) => {
                                fields.push(idl_types::InterfaceNode::Comment(comment.to_owned()))
                            }
                            parser::InterfaceNode::InterfaceField(interface_field) => {
                                let ty = match items.get_type(interface_field.ty.clone()) {
                                    Ok(value) => value,
                                    Err(err) => return Err(err.into()),
                                };

                                let field_ident = interface_field.ident.to_owned();

                                if fields.iter().any(|v| {
                                    if let idl_types::InterfaceNode::InterfaceField(in_field) = v {
                                        if in_field.ident.as_str() == field_ident.as_str() {
                                            return true;
                                        }
                                    }
                                    false
                                }) {
                                    return Err(DuplicateFieldNameError(
                                        field_ident,
                                        interface_field.range,
                                    )
                                    .into());
                                }

                                Self::field_name_is_valid(
                                    field_ident.as_str(),
                                    interface_field.range,
                                )?;
                                let interface_field = Box::new(idl_types::InterfaceField {
                                    attributes: vec![],
                                    ident: field_ident,
                                    ty,
                                });

                                fields.push(idl_types::InterfaceNode::InterfaceField(
                                    interface_field,
                                ));
                            }
                        }
                    }

                    analyzer
                        .nodes
                        .push(idl_types::TypeNode::TypeInterface(Box::new(
                            idl_types::TypeInterface { ident, fields },
                        )));
                }
                parser::ParserNode::Struct(value) => {
                    let ident = match items.get_type(value.ident.clone()) {
                        Ok(value) => match value {
                            idl_types::TypeName::StructTypeName(value) => value,
                            _ => return Err(AnalyzerError::Undefined),
                        },
                        Err(err) => return Err(err.into()),
                    };

                    let mut fields = vec![];

                    for field in value.fields.iter() {
                        match field {
                            parser::StructNode::Comment(comment) => {
                                fields.push(idl_types::StructNode::Comment(comment.to_owned()))
                            }
                            parser::StructNode::StructField(struct_field) => {
                                let ty = match items.get_type(struct_field.ty.clone()) {
                                    Ok(value) => value,
                                    Err(err) => return Err(err.into()),
                                };

                                let field_ident = struct_field.ident.to_owned();

                                if fields.iter().any(|v| {
                                    if let idl_types::StructNode::StructField(in_field) = v {
                                        if in_field.ident.as_str() == field_ident.as_str() {
                                            return true;
                                        }
                                    }
                                    false
                                }) {
                                    return Err(DuplicateFieldNameError(
                                        field_ident,
                                        struct_field.range,
                                    )
                                    .into());
                                }

                                Self::field_name_is_valid(
                                    field_ident.as_str(),
                                    struct_field.range,
                                )?;
                                fields.push(idl_types::StructNode::StructField(Box::new(
                                    idl_types::StructField {
                                        ident: field_ident,
                                        ty,
                                    },
                                )));
                            }
                        }
                    }

                    analyzer
                        .nodes
                        .push(idl_types::TypeNode::TypeStruct(Box::new(
                            idl_types::TypeStruct { ident, fields },
                        )));
                }
                parser::ParserNode::Enum(value) => {
                    let ident = match items.get_type(value.ident.clone()) {
                        Ok(value) => match value {
                            idl_types::TypeName::EnumTypeName(value) => value,
                            _ => return Err(AnalyzerError::Undefined),
                        },
                        Err(err) => return Err(err.into()),
                    };

                    let mut fields = vec![];

                    for field in value.fields.iter() {
                        match field {
                            parser::EnumNode::Comment(comment) => {
                                fields.push(idl_types::EnumNode::Comment(comment.to_owned()))
                            }
                            parser::EnumNode::EnumField(enum_field) => {
                                let field_ident = enum_field.ident.to_owned();

                                if fields.iter().any(|v| {
                                    if let idl_types::EnumNode::EnumField(in_field) = v {
                                        if in_field.ident.as_str() == field_ident.as_str() {
                                            return true;
                                        }
                                    }
                                    false
                                }) {
                                    return Err(DuplicateFieldNameError(
                                        field_ident,
                                        enum_field.range,
                                    )
                                    .into());
                                }

                                Self::type_name_is_valid(field_ident.as_str(), enum_field.range)?;
                                fields.push(idl_types::EnumNode::EnumField(Box::new(
                                    idl_types::EnumField { ident: field_ident },
                                )))
                            }
                        }
                    }

                    analyzer.nodes.push(idl_types::TypeNode::TypeEnum(Box::new(
                        idl_types::TypeEnum { ident, fields },
                    )));
                }
                parser::ParserNode::Const(value) => {
                    let ident = match items.get_type(value.ident.clone()) {
                        Ok(value) => match value {
                            idl_types::TypeName::ConstTypeName(value) => value,
                            _ => return Err(AnalyzerError::Undefined),
                        },
                        Err(err) => return Err(err.into()),
                    };

                    let mut fields = vec![];

                    for field in value.fields.iter() {
                        match field {
                            parser::ConstNode::Comment(comment) => {
                                fields.push(idl_types::ConstNode::Comment(comment.to_owned()))
                            }
                            parser::ConstNode::ConstField(const_field) => {
                                let field_ident = const_field.ident.to_owned();

                                if fields.iter().any(|v| {
                                    if let idl_types::ConstNode::ConstField(in_field) = v {
                                        if in_field.ident.as_str() == field_ident.as_str() {
                                            return true;
                                        }
                                    }
                                    false
                                }) {
                                    return Err(DuplicateFieldNameError(
                                        field_ident,
                                        const_field.range,
                                    )
                                    .into());
                                }

                                Self::field_name_is_valid(field_ident.as_str(), const_field.range)?;
                                fields.push(idl_types::ConstNode::ConstField(Box::new(
                                    idl_types::ConstField {
                                        ident: field_ident,
                                        value: const_field.value.to_owned(),
                                    },
                                )))
                            }
                        }
                    }

                    let const_type = match value.const_type {
                        parser::ConstType::Int => idl_types::ConstTypes::NativeInt,
                        parser::ConstType::String => idl_types::ConstTypes::NativeString,
                        parser::ConstType::Float => idl_types::ConstTypes::NativeFloat,
                    };

                    analyzer.nodes.push(idl_types::TypeNode::TypeConst(Box::new(
                        idl_types::TypeConst {
                            ident,
                            fields,
                            const_type,
                        },
                    )));
                }
                parser::ParserNode::TypeList(value) => {
                    let ident = match items.get_type(value.ident.clone()) {
                        Ok(value) => match value {
                            idl_types::TypeName::ListTypeName(value) => value,
                            _ => return Err(AnalyzerError::Undefined),
                        },
                        Err(err) => return Err(err.into()),
                    };

                    let mut ty_list = vec![];

                    for field in value.ty_list.iter() {
                        match field {
                            parser::TypeListNode::Comment(comment) => {
                                ty_list.push(idl_types::TypeListNode::Comment(comment.to_owned()))
                            }
                            parser::TypeListNode::TypeListField(ty_list_field) => {
                                let ty = match items.get_type(ty_list_field.ty.clone()) {
                                    Ok(value) => value,
                                    Err(err) => return Err(err.into()),
                                };

                                if ty_list.iter().any(|v| {
                                    if let idl_types::TypeListNode::TypeListField(in_field) = v {
                                        if in_field.ty.get_type_reference()
                                            == ty.get_type_reference().as_str()
                                        {
                                            return true;
                                        }
                                    }
                                    false
                                }) {
                                    return Err(DuplicateFieldNameError(
                                        ty.get_type_reference(),
                                        ty_list_field.range,
                                    )
                                    .into());
                                }

                                ty_list.push(idl_types::TypeListNode::TypeListField(Box::new(
                                    idl_types::TypeListField { ty },
                                )));
                            }
                        }
                    }

                    analyzer.nodes.push(idl_types::TypeNode::TypeList(Box::new(
                        idl_types::TypeList { ident, ty_list },
                    )));
                }
                parser::ParserNode::Stream(value) => {
                    let ident = match items.get_type(value.ident.clone()) {
                        Ok(value) => match value {
                            idl_types::TypeName::StreamTypeName(value) => value,
                            _ => return Err(AnalyzerError::Undefined),
                        },
                        Err(err) => return Err(err.into()),
                    };

                    let mut fields = vec![];

                    for field in value.fields.iter() {
                        match field {
                            parser::StructNode::Comment(comment) => {
                                fields.push(idl_types::StreamNode::Comment(comment.to_owned()))
                            }
                            parser::StructNode::StructField(stream_field) => {
                                let ty = match items.get_type(stream_field.ty.clone()) {
                                    Ok(value) => value,
                                    Err(err) => return Err(err.into()),
                                };

                                let field_ident = stream_field.ident.to_owned();

                                if fields.iter().any(|v| {
                                    if let idl_types::StreamNode::StreamField(in_field) = v {
                                        if in_field.ident.as_str() == field_ident.as_str() {
                                            return true;
                                        }
                                    }
                                    false
                                }) {
                                    return Err(DuplicateFieldNameError(
                                        field_ident,
                                        stream_field.range,
                                    )
                                    .into());
                                }

                                Self::field_name_is_valid(
                                    field_ident.as_str(),
                                    stream_field.range,
                                )?;
                                fields.push(idl_types::StreamNode::StreamField(Box::new(
                                    idl_types::StreamField {
                                        ident: field_ident,
                                        ty,
                                    },
                                )));
                            }
                        }
                    }

                    analyzer
                        .nodes
                        .push(idl_types::TypeNode::TypeStream(Box::new(
                            idl_types::TypeStream { ident, fields },
                        )))
                }
                parser::ParserNode::Factory(value) => {
                    let ident = match items.get_type(value.ident.clone()) {
                        Ok(value) => match value {
                            idl_types::TypeName::FactoryTypeName(value) => value,
                            _ => return Err(AnalyzerError::Undefined),
                        },
                        Err(err) => return Err(err.into()),
                    };

                    let mut fields = vec![];

                    for field in value.fields.iter() {
                        match field {
                            parser::InterfaceNode::Comment(comment) => {
                                fields.push(idl_types::FactoryNode::Comment(comment.to_owned()))
                            }
                            parser::InterfaceNode::InterfaceField(factory_field) => {
                                let ty = match items.get_type(factory_field.ty.clone()) {
                                    Ok(value) => value,
                                    Err(err) => return Err(err.into()),
                                };

                                let field_ident = factory_field.ident.to_owned();

                                if fields.iter().any(|v| {
                                    if let idl_types::FactoryNode::FactoryField(in_field) = v {
                                        if in_field.ident.as_str() == field_ident.as_str() {
                                            return true;
                                        }
                                    }
                                    false
                                }) {
                                    return Err(DuplicateFieldNameError(
                                        field_ident,
                                        factory_field.range,
                                    )
                                    .into());
                                }

                                Self::field_name_is_valid(
                                    field_ident.as_str(),
                                    factory_field.range,
                                )?;
                                let factory_field = Box::new(idl_types::FactoryField {
                                    attributes: vec![],
                                    ident: field_ident,
                                    ty,
                                });

                                fields.push(idl_types::FactoryNode::FactoryField(factory_field));
                            }
                        }
                    }

                    analyzer
                        .nodes
                        .push(idl_types::TypeNode::TypeFactory(Box::new(
                            idl_types::TypeFactory { ident, fields },
                        )));
                }
                parser::ParserNode::Library(_) | parser::ParserNode::Imports(_) => {}
            }
        }

        Self::references_invalid_type(&analyzer.nodes, parsers)?;
        Self::struct_has_recursive_reference(&analyzer.nodes, parsers)?;
        Self::factory_returns_invalid_type(&analyzer.nodes, parsers)?;

        Ok(analyzer)
    }

    fn references_invalid_type(
        nodes: &[idl_types::TypeNode],
        parsers: &parser::Parser,
    ) -> Result<(), ReferenceError> {
        for node in nodes {
            match node {
                idl_types::TypeNode::TypeInterface(value) => {
                    let range = parsers.get_range_from_type_name(value.ident.as_str());

                    for interface_node in value.fields.iter() {
                        if let idl_types::InterfaceNode::InterfaceField(interface_field) =
                            interface_node
                        {
                            if Self::has_idl_interface(&interface_field.ty) {
                                return Err(ReferenceError(
                                    ReferenceErrorKind::ReferencesInterface,
                                    range,
                                    interface_field.ty.get_type_reference(),
                                ));
                            } else if Self::has_idl_factory(&interface_field.ty) {
                                return Err(ReferenceError(
                                    ReferenceErrorKind::ReferencesFactory,
                                    range,
                                    interface_field.ty.get_type_reference(),
                                ));
                            } else {
                                let ty_ref = interface_field.ty.get_type_reference();
                                match &interface_field.ty {
                                    idl_types::TypeName::TypeMap(map) => {
                                        if Self::has_idl_stream(&map.map_ty) {
                                            return Err(ReferenceError(
                                                ReferenceErrorKind::ReferencesStream,
                                                range,
                                                ty_ref,
                                            ));
                                        }
                                    }
                                    idl_types::TypeName::TypeArray(array) => {
                                        if Self::has_idl_stream(&array.ty) {
                                            return Err(ReferenceError(
                                                ReferenceErrorKind::ReferencesStream,
                                                range,
                                                ty_ref,
                                            ));
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                }
                idl_types::TypeNode::TypeFactory(value) => {
                    let range = parsers.get_range_from_type_name(value.ident.as_str());

                    for interface_node in value.fields.iter() {
                        if let idl_types::FactoryNode::FactoryField(factory_field) = interface_node
                        {
                            let ty_ref = factory_field.ty.get_type_reference();

                            if Self::has_idl_factory(&factory_field.ty) {
                                return Err(ReferenceError(
                                    ReferenceErrorKind::ReferencesFactory,
                                    range,
                                    ty_ref,
                                ));
                            } else if Self::has_idl_stream(&factory_field.ty) {
                                return Err(ReferenceError(
                                    ReferenceErrorKind::ReferencesStream,
                                    range,
                                    ty_ref,
                                ));
                            }
                        }
                    }
                }
                idl_types::TypeNode::TypeStruct(value) => {
                    let range = parsers.get_range_from_type_name(value.ident.as_str());

                    for struct_node in value.fields.iter() {
                        if let idl_types::StructNode::StructField(struct_field) = struct_node {
                            let ty_ref = struct_field.ty.get_type_reference();

                            if Self::has_idl_interface(&struct_field.ty) {
                                return Err(ReferenceError(
                                    ReferenceErrorKind::ReferencesInterface,
                                    range,
                                    ty_ref,
                                ));
                            } else if Self::has_idl_factory(&struct_field.ty) {
                                return Err(ReferenceError(
                                    ReferenceErrorKind::ReferencesFactory,
                                    range,
                                    ty_ref,
                                ));
                            } else if Self::has_idl_stream(&struct_field.ty) {
                                return Err(ReferenceError(
                                    ReferenceErrorKind::ReferencesStream,
                                    range,
                                    ty_ref,
                                ));
                            }
                        }
                    }
                }
                idl_types::TypeNode::TypeList(value) => {
                    let range = parsers.get_range_from_type_name(value.ident.as_str());

                    for type_list_node in value.ty_list.iter() {
                        if let idl_types::TypeListNode::TypeListField(type_list_field) =
                            type_list_node
                        {
                            let ty_ref = type_list_field.ty.get_type_reference();

                            if Self::has_idl_interface(&type_list_field.ty) {
                                return Err(ReferenceError(
                                    ReferenceErrorKind::ReferencesInterface,
                                    range,
                                    ty_ref,
                                ));
                            } else if Self::has_idl_factory(&type_list_field.ty) {
                                return Err(ReferenceError(
                                    ReferenceErrorKind::ReferencesFactory,
                                    range,
                                    ty_ref,
                                ));
                            } else if Self::has_idl_stream(&type_list_field.ty) {
                                return Err(ReferenceError(
                                    ReferenceErrorKind::ReferencesStream,
                                    range,
                                    ty_ref,
                                ));
                            }
                        }
                    }
                }
                idl_types::TypeNode::TypeStream(value) => {
                    let range = parsers.get_range_from_type_name(value.ident.as_str());

                    for stream_node in value.fields.iter() {
                        if let idl_types::StreamNode::StreamField(stream_field) = stream_node {
                            let ty_ref = stream_field.ty.get_type_reference();

                            if Self::has_idl_interface(&stream_field.ty) {
                                return Err(ReferenceError(
                                    ReferenceErrorKind::ReferencesInterface,
                                    range,
                                    ty_ref,
                                ));
                            } else if Self::has_idl_factory(&stream_field.ty) {
                                return Err(ReferenceError(
                                    ReferenceErrorKind::ReferencesFactory,
                                    range,
                                    ty_ref,
                                ));
                            } else if Self::has_idl_stream(&stream_field.ty) {
                                return Err(ReferenceError(
                                    ReferenceErrorKind::ReferencesStream,
                                    range,
                                    ty_ref,
                                ));
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        Ok(())
    }

    fn has_idl_interface(ty: &idl_types::TypeName) -> bool {
        match ty {
            idl_types::TypeName::InterfaceTypeName(_) => true,
            idl_types::TypeName::TypeFunction(function) => {
                Self::has_idl_interface(&function.return_ty)
                    || Self::has_idl_interface(&function.args)
            }
            idl_types::TypeName::TypeTuple(tuple) => {
                for tuple_ty in tuple.ty_list.iter() {
                    if Self::has_idl_interface(&tuple_ty.ty) {
                        return true;
                    }
                }
                false
            }
            idl_types::TypeName::TypeMap(map) => Self::has_idl_interface(&map.map_ty),
            idl_types::TypeName::TypeArray(array) => Self::has_idl_interface(&array.ty),
            _ => false,
        }
    }

    fn has_idl_factory(ty: &idl_types::TypeName) -> bool {
        match ty {
            idl_types::TypeName::FactoryTypeName(_) => true,
            idl_types::TypeName::TypeFunction(function) => {
                Self::has_idl_factory(&function.return_ty) || Self::has_idl_factory(&function.args)
            }
            idl_types::TypeName::TypeTuple(tuple) => {
                for tuple_ty in tuple.ty_list.iter() {
                    if Self::has_idl_factory(&tuple_ty.ty) {
                        return true;
                    }
                }
                false
            }
            idl_types::TypeName::TypeMap(map) => Self::has_idl_factory(&map.map_ty),
            idl_types::TypeName::TypeArray(array) => Self::has_idl_factory(&array.ty),
            _ => false,
        }
    }

    fn has_idl_stream(ty: &idl_types::TypeName) -> bool {
        match ty {
            idl_types::TypeName::StreamTypeName(_) => true,
            idl_types::TypeName::TypeFunction(function) => {
                Self::has_idl_stream(&function.return_ty) || Self::has_idl_stream(&function.args)
            }
            idl_types::TypeName::TypeTuple(tuple) => {
                for tuple_ty in tuple.ty_list.iter() {
                    if Self::has_idl_stream(&tuple_ty.ty) {
                        return true;
                    }
                }
                false
            }
            idl_types::TypeName::TypeMap(map) => Self::has_idl_stream(&map.map_ty),
            idl_types::TypeName::TypeArray(array) => Self::has_idl_stream(&array.ty),
            _ => false,
        }
    }

    fn factory_returns_invalid_type(
        nodes: &[idl_types::TypeNode],
        parsers: &parser::Parser,
    ) -> Result<(), FactoryError> {
        for node in nodes {
            match node {
                idl_types::TypeNode::TypeFactory(value) => {
                    let range = parsers.get_range_from_type_name(value.ident.as_str());

                    for factory_node in &value.fields {
                        if let idl_types::FactoryNode::FactoryField(field) = factory_node {
                            let ty = &field.ty;

                            let has_interface_return = match ty {
                                idl_types::TypeName::TypeFunction(function) => {
                                    Self::has_idl_interface(&function.return_ty)
                                }
                                idl_types::TypeName::TypeArray(array) => {
                                    Self::has_idl_interface(&array.ty)
                                }
                                idl_types::TypeName::InterfaceTypeName(_) => true,
                                _ => false,
                            };

                            if !has_interface_return {
                                return Err(FactoryError(
                                    FactoryErrorKind::DoesNotReturnInterface,
                                    range,
                                ));
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        Ok(())
    }

    // Given a type name, find if any other type inside references it.
    fn struct_has_recursive_reference(
        nodes: &[idl_types::TypeNode],
        parsers: &parser::Parser,
    ) -> Result<(), ReferenceError> {
        for node in nodes {
            match node {
                idl_types::TypeNode::TypeStruct(value) => {
                    let range = parsers.get_range_from_type_name(value.ident.as_str());

                    for struct_node in value.fields.iter() {
                        if let idl_types::StructNode::StructField(struct_field) = struct_node {
                            if Self::has_type_reference_in_struct(
                                nodes,
                                value.ident.as_str(),
                                &struct_field.ty,
                            ) {
                                return Err(ReferenceError(
                                    ReferenceErrorKind::StructRecursiveReference,
                                    range,
                                    struct_field.ty.get_type_reference(),
                                ));
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        Ok(())
    }

    // Finds if any type has the reference target inside a struct.
    fn has_type_reference_in_struct(
        nodes: &[idl_types::TypeNode],
        target: &str,
        refer: &idl_types::TypeName,
    ) -> bool {
        match refer {
            idl_types::TypeName::StructTypeName(struct_ident) => {
                if target == struct_ident {
                    return true;
                }

                for node in nodes {
                    if let idl_types::TypeNode::TypeStruct(value) = node {
                        if value.ident.as_str() == struct_ident {
                            for struct_node in value.fields.iter() {
                                if let idl_types::StructNode::StructField(field) = struct_node {
                                    if Self::has_type_reference_in_struct(nodes, target, &field.ty)
                                    {
                                        return true;
                                    }
                                }
                            }
                        }
                    }
                }

                false
            }
            idl_types::TypeName::TypeArray(array) => {
                Self::has_type_reference_in_struct(nodes, target, &array.ty)
            }
            idl_types::TypeName::TypeMap(map) => {
                Self::has_type_reference_in_struct(nodes, target, &map.map_ty)
                    || Self::has_type_reference_in_struct(nodes, target, &map.map_ty)
            }
            _ => false,
        }
    }

    // Finds if any type has the reference target.
    fn has_type_reference(
        nodes: &[idl_types::TypeNode],
        target: &str,
        refer: &idl_types::TypeName,
    ) -> bool {
        match refer {
            idl_types::TypeName::StructTypeName(struct_ident) => {
                if target == struct_ident {
                    return true;
                }

                for node in nodes {
                    if let idl_types::TypeNode::TypeStruct(value) = node {
                        if value.ident.as_str() == struct_ident {
                            for struct_node in value.fields.iter() {
                                if let idl_types::StructNode::StructField(field) = struct_node {
                                    if Self::has_type_reference(nodes, target, &field.ty) {
                                        return true;
                                    }
                                }
                            }
                        }
                    }
                }

                false
            }
            idl_types::TypeName::InterfaceTypeName(interface_ident) => {
                if target == interface_ident {
                    return true;
                }

                for node in nodes {
                    if let idl_types::TypeNode::TypeInterface(value) = node {
                        if value.ident.as_str() == interface_ident {
                            for interface_node in value.fields.iter() {
                                if let idl_types::InterfaceNode::InterfaceField(field) =
                                    interface_node
                                {
                                    if Self::has_type_reference(nodes, target, &field.ty) {
                                        return true;
                                    }
                                }
                            }
                        }
                    }
                }

                false
            }
            idl_types::TypeName::ListTypeName(list_ident) => {
                if target == list_ident {
                    return true;
                }

                for node in nodes {
                    if let idl_types::TypeNode::TypeList(value) = node {
                        if value.ident.as_str() == list_ident {
                            for ty_node in value.ty_list.iter() {
                                if let idl_types::TypeListNode::TypeListField(field) = ty_node {
                                    if Self::has_type_reference(nodes, target, &field.ty) {
                                        return true;
                                    }
                                }
                            }
                        }
                    }
                }

                false
            }
            idl_types::TypeName::TypeArray(array) => {
                Self::has_type_reference(nodes, target, &array.ty)
            }
            idl_types::TypeName::TypeMap(map) => {
                Self::has_type_reference(nodes, target, &map.map_ty)
                    || Self::has_type_reference(nodes, target, &map.map_ty)
            }
            idl_types::TypeName::TypeFunction(function) => {
                Self::has_type_reference(nodes, target, &function.return_ty)
                    || Self::has_type_reference(nodes, target, &function.args)
            }
            idl_types::TypeName::TypeTuple(tuple) => {
                for ty in tuple.ty_list.iter() {
                    if Self::has_type_reference(nodes, target, &ty.ty) {
                        return true;
                    }
                }

                false
            }
            _ => false,
        }
    }

    fn type_name_is_valid(name: &str, range: Range) -> Result<(), NameError> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^[A-Z](?:[a-z0-9][A-Z]?)+$").unwrap();
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

    fn field_name_is_valid(name: &str, range: Range) -> Result<(), NameError> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^([a-z]+(?:_[a-z0-9]+)*)$").unwrap();
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
}

impl AnalyzerItems {
    fn get_type(&self, ty: Arc<parser::Type>) -> Result<idl_types::TypeName, ReferenceError> {
        match &*ty {
            parser::Type::Name(name) => {
                if self.interfaces.contains(&name.ident) {
                    Ok(idl_types::TypeName::InterfaceTypeName(
                        name.ident.to_owned(),
                    ))
                } else if self.factories.contains(&name.ident) {
                    Ok(idl_types::TypeName::FactoryTypeName(name.ident.to_owned()))
                } else if self.streams.contains(&name.ident) {
                    Ok(idl_types::TypeName::StreamTypeName(name.ident.to_owned()))
                } else if self.structs.contains(&name.ident) {
                    Ok(idl_types::TypeName::StructTypeName(name.ident.to_owned()))
                } else if self.enums.contains(&name.ident) {
                    Ok(idl_types::TypeName::EnumTypeName(name.ident.to_owned()))
                } else if self.consts.contains(&name.ident) {
                    Ok(idl_types::TypeName::ConstTypeName(name.ident.to_owned()))
                } else if self.type_lists.contains(&name.ident) {
                    Ok(idl_types::TypeName::ListTypeName(name.ident.to_owned()))
                } else {
                    Err(ReferenceError(
                        ReferenceErrorKind::UndefinedType,
                        Range::default(),
                        ty.to_string(),
                    ))
                }
            }
            parser::Type::Tuple(tuple) => {
                let mut tuple_list = vec![];

                for t_list in tuple.ty_list.iter() {
                    tuple_list.push(idl_types::TupleEntry {
                        ident: t_list.ident.to_owned(),
                        ty: self.get_type(t_list.ty.clone())?,
                    })
                }

                Ok(idl_types::TypeName::TypeTuple(Box::new(
                    idl_types::TypeTuple {
                        ty_list: tuple_list,
                    },
                )))
            }
            parser::Type::Function(function) => Ok(idl_types::TypeName::TypeFunction(Box::new(
                idl_types::TypeFunction {
                    args: self.get_type(function.args.clone())?,
                    return_ty: self.get_type(function.ret_ty.clone())?,
                },
            ))),
            parser::Type::Native(native) => Ok(idl_types::TypeName::Types(match native.ty {
                scanner::NativeTypes::Bool => idl_types::Types::NativeBool,
                scanner::NativeTypes::Int => idl_types::Types::NativeInt,
                scanner::NativeTypes::Bytes => idl_types::Types::NativeBytes,
                scanner::NativeTypes::String => idl_types::Types::NativeString,
                scanner::NativeTypes::None => idl_types::Types::NativeNone,
                scanner::NativeTypes::Float => idl_types::Types::NativeFloat,
            })),
            parser::Type::Array(array) => Ok(idl_types::TypeName::TypeArray(Box::new(
                idl_types::TypeArray {
                    ty: self.get_type(array.array_ty.clone())?,
                },
            ))),
            parser::Type::Map(map) => {
                Ok(idl_types::TypeName::TypeMap(Box::new(idl_types::TypeMap {
                    index_ty: self.get_type(map.index_ty.clone())?,
                    map_ty: self.get_type(map.m_ty.clone())?,
                })))
            }
        }
    }
}

impl idl_types::TypeName {
    fn get_type_reference(&self) -> String {
        match self {
            idl_types::TypeName::ConstTypeName(value)
            | idl_types::TypeName::InterfaceTypeName(value)
            | idl_types::TypeName::StructTypeName(value)
            | idl_types::TypeName::EnumTypeName(value)
            | idl_types::TypeName::ListTypeName(value)
            | idl_types::TypeName::FactoryTypeName(value)
            | idl_types::TypeName::StreamTypeName(value) => value.to_owned(),
            _ => "".to_owned(),
        }
    }
}
