use super::idl_nodes;
use super::parser;
use crate::range::Range;
use crate::reserved::{
    field_name_is_valid, is_reserved_type, is_reserved_word, type_name_is_valid, NameError,
};
use crate::scanner;
use std::{fmt, path::Path};
use std::sync::Arc;
use idl_nodes::TypePair;
use thiserror::Error;

#[derive(Debug, Clone, Copy)]
pub enum ReferenceErrorKind {
    Invalid,
    ReferencesInterface,
    ReferencesStream,
    StructRecursiveReference,
    ReferencesResult,
    UndefinedType,
}

#[derive(Error, Debug, Clone)]
pub struct ReferenceError(ReferenceErrorKind, Range, String);

impl fmt::Display for ReferenceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = self.2.as_str();

        let errstr = match &self.0 {
            ReferenceErrorKind::Invalid => format!("Invalid type name `{}`", name),
            ReferenceErrorKind::ReferencesInterface => format!("References interface `{}`", name),
            ReferenceErrorKind::ReferencesStream => format!("References stream `{}`", name),
            ReferenceErrorKind::ReferencesResult => format!("References result `{}`", name),
            ReferenceErrorKind::StructRecursiveReference => {
                format!("Recursive reference in struct `{}`", name)
            }
            ReferenceErrorKind::UndefinedType => format!("Undefined type `{}`", name),
        };

        write!(f, "{}", errstr)
    }
}

#[derive(Error, Debug, Clone)]
pub struct DuplicateNameError(String);

impl fmt::Display for DuplicateNameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Error, Debug, Clone)]
pub struct DuplicateFieldNameError(String, Range);

impl fmt::Display for DuplicateFieldNameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Error, Debug, Clone)]
pub enum AnalyzerError {
    #[error("Undefined error")]
    Undefined,
    #[error("Analyzer closed")]
    Closed,
    #[error("Import redefinition")]
    ImportDefinition,
    #[error("Library redefinition")]
    LibraryDefinition,
    #[error("Missing library definition")]
    MissingLibraryDefinition,
    #[error("Duplicate name `{0}`")]
    DuplicateName(#[from] DuplicateNameError),
    #[error("Duplicate name field `{0}`")]
    DuplicateFieldNameError(#[from] DuplicateFieldNameError),
    #[error("Name `{0}`")]
    NameError(#[from] NameError),
    #[error("Reference `{0}`")]
    ReferenceError(#[from] ReferenceError),
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
            AnalyzerError::DuplicateName(value) => (value.to_string(), Range::default()),
            AnalyzerError::ReferenceError(value) => (value.to_string(), value.1),
            AnalyzerError::NameError(value) => (value.to_string(), value.1),
        }
    }
}

#[derive(Debug, Default)]
pub struct Analyzer {
    pub nodes: Vec<idl_nodes::IdlNode>,
}

#[derive(Debug, Default)]
struct AnalyzerItems {
    interfaces: Vec<String>,
    structs: Vec<String>,
    enums: Vec<String>,
    consts: Vec<String>,
    type_lists: Vec<String>,
    library_name: Option<String>,
    imports: Option<Vec<String>>,
}

impl Analyzer {
    pub fn closed() -> Result<Self, AnalyzerError> {
        Err(AnalyzerError::Closed)
    }

    pub fn from_nodes(nodes: Vec<idl_nodes::IdlNode>) -> Self {
        Self { nodes }
    }

    pub fn library_name(&self) -> String {
        for node in &self.nodes {
            match node {
                idl_nodes::IdlNode::LibraryName(name) => return name.to_owned(),
                _ => {}
            }
        }

        panic!("Does not have a library name");
    }

    pub fn find_ty_struct(&self, name: &str) -> Option<&Box<idl_nodes::TypeStruct>> {
        for node in &self.nodes {
            match node {
                idl_nodes::IdlNode::TypeStruct(value) => {
                    if value.ident == name {
                        return Some(value);
                    }
                }
                _ => {}
            }
        }

        None
    }

    pub fn find_ty_interface(&self, name: &str) -> Option<&Box<idl_nodes::TypeInterface>> {
        for node in &self.nodes {
            match node {
                idl_nodes::IdlNode::TypeInterface(value) => {
                    if value.ident == name {
                        return Some(value);
                    }
                }
                _ => {}
            }
        }

        None
    }

    pub fn find_ty_enum(&self, name: &str) -> Option<&Box<idl_nodes::TypeEnum>> {
        for node in &self.nodes {
            match node {
                idl_nodes::IdlNode::TypeEnum(value) => {
                    if value.ident == name {
                        return Some(value);
                    }
                }
                _ => {}
            }
        }

        None
    }

    pub fn find_ty_const(&self, name: &str) -> Option<&Box<idl_nodes::TypeConst>> {
        for node in &self.nodes {
            match node {
                idl_nodes::IdlNode::TypeConst(value) => {
                    if value.ident == name {
                        return Some(value);
                    }
                }
                _ => {}
            }
        }

        None
    }

    pub fn find_ty_list(&self, name: &str) -> Option<&Box<idl_nodes::TypeList>> {
        for node in &self.nodes {
            match node {
                idl_nodes::IdlNode::TypeList(value) => {
                    if value.ident == name {
                        return Some(value);
                    }
                }
                _ => {}
            }
        }

        None
    }

    pub fn any_interface_field_returns_stream(&self, is_static: Option<bool>) -> bool {
        self.nodes
            .iter()
            .filter_map(|node| match node {
                idl_nodes::IdlNode::TypeInterface(ty_interface) => Some(ty_interface),
                _ => None,
            })
            .any(|interface| {
                interface.fields.iter().any(|node| match node {
                    idl_nodes::InterfaceNode::InterfaceField(field) => {
                        (is_static.is_none() || is_static.unwrap() == field.is_static)
                            && Self::field_returns_stream(field)
                    }
                    _ => false,
                })
            })
    }

    pub fn any_interface_field_sends_stream(&self, is_static: Option<bool>) -> bool {
        self.nodes
            .iter()
            .filter_map(|node| match node {
                idl_nodes::IdlNode::TypeInterface(ty_interface) => Some(ty_interface),
                _ => None,
            })
            .any(|interface| {
                interface.fields.iter().any(|node| match node {
                    idl_nodes::InterfaceNode::InterfaceField(field) => {
                        (is_static.is_none() || is_static.unwrap() == field.is_static)
                            && Self::field_sends_stream(field)
                    }
                    _ => false,
                })
            })
    }

    pub fn interface_has_static_field(name: &idl_nodes::TypeInterface) -> bool {
        name.fields.iter().any(|node| match node {
            idl_nodes::InterfaceNode::InterfaceField(field) => field.is_static,
            _ => false,
        })
    }

    pub fn interface_has_non_static_field(name: &idl_nodes::TypeInterface) -> bool {
        name.fields.iter().any(|node| match node {
            idl_nodes::InterfaceNode::InterfaceField(field) => !field.is_static,
            _ => false,
        })
    }

    pub fn interface_returns_stream(
        name: &idl_nodes::TypeInterface,
        is_static: Option<bool>,
    ) -> bool {
        name.fields.iter().any(|node| match node {
            idl_nodes::InterfaceNode::InterfaceField(field) => {
                (is_static.is_none() || is_static.unwrap() == field.is_static)
                    && Self::field_returns_stream(field)
            }
            _ => false,
        })
    }

    pub fn interface_sends_stream(
        name: &idl_nodes::TypeInterface,
        is_static: Option<bool>,
    ) -> bool {
        name.fields.iter().any(|node| match node {
            idl_nodes::InterfaceNode::InterfaceField(field) => {
                (is_static.is_none() || is_static.unwrap() == field.is_static)
                    && Self::field_sends_stream(field)
            }
            _ => false,
        })
    }

    pub fn field_returns_stream(field: &idl_nodes::InterfaceField) -> bool {
        Self::field_stream_return_ty(field).is_some()
    }

    pub fn field_stream_return_ty(
        field: &idl_nodes::InterfaceField,
    ) -> Option<&idl_nodes::TypeName> {
        match &field.ty {
            idl_nodes::TypeName::TypeStream(value) => Some(&value.s_ty),
            idl_nodes::TypeName::TypeFunction(value) => match &value.return_ty {
                idl_nodes::TypeName::TypeStream(value) => Some(&value.s_ty),
                _ => None,
            },
            _ => None,
        }
    }

    pub fn field_sends_stream(field: &idl_nodes::InterfaceField) -> bool {
        Self::field_stream_send_ty(field).is_some()
    }

    pub fn field_stream_send_ty(field: &idl_nodes::InterfaceField) -> Option<&idl_nodes::TypeName> {
        let args = match &field.ty {
            idl_nodes::TypeName::TypeFunction(value) => match &value.args {
                idl_nodes::TypeName::TypeTuple(value) => &value.fields,
                _ => return None,
            },
            idl_nodes::TypeName::TypeTuple(value) => &value.fields,
            _ => return None,
        };

        if let Some(value) = args.iter().last() {
            if let idl_nodes::TypeName::TypeStream(value) = &value.ty {
                return Some(&value.s_ty);
            }
        }

        None
    }

    pub fn interface_has_constructor_field(name: &idl_nodes::TypeInterface) -> bool {
        for node in &name.fields {
            match node {
                idl_nodes::InterfaceNode::InterfaceField(field) => {
                    if Self::returns_interface(&field.ty) {
                        return true;
                    }
                }
                _ => {}
            }
        }

        false
    }

    pub fn resolve(
        parsers: &parser::Parser,
        package: Option<&crate::mod_package::Package>, // TODO
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

                    is_reserved_type(value.to_lowercase().as_str(), Range::default())?;
                    is_reserved_word(value.to_lowercase().as_str(), Range::default())?;
                    items.library_name = Some(value.to_owned());
                }
                parser::ParserNode::Interface(value) => {
                    // TODO verify is duplicate???
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

                    type_name_is_valid(ident.as_str(), value.range)?;
                    is_reserved_type(ident.to_lowercase().as_str(), value.range)?;
                    is_reserved_word(ident.to_lowercase().as_str(), value.range)?;
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

                    type_name_is_valid(ident.as_str(), value.range)?;
                    is_reserved_type(ident.to_lowercase().as_str(), value.range)?;
                    is_reserved_word(ident.to_lowercase().as_str(), value.range)?;
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

                    type_name_is_valid(ident.as_str(), value.range)?;
                    is_reserved_type(ident.to_lowercase().as_str(), value.range)?;
                    is_reserved_word(ident.to_lowercase().as_str(), value.range)?;
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

                    type_name_is_valid(ident.as_str(), value.range)?;
                    is_reserved_type(ident.to_lowercase().as_str(), value.range)?;
                    is_reserved_word(ident.to_lowercase().as_str(), value.range)?;
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

                    type_name_is_valid(ident.as_str(), value.range)?;
                    is_reserved_type(ident.to_lowercase().as_str(), value.range)?;
                    is_reserved_word(ident.to_lowercase().as_str(), value.range)?;
                    items.type_lists.push(ident);
                }
                _ => {}
            }
        }

        // See if there's any duplicate type name
        Self::has_duplicate_names(&items)?;

        if let Some(value) = &items.imports {
            analyzer
                .nodes
                .push(idl_nodes::IdlNode::Imports(value.to_owned()));
        }

        match &items.library_name {
            Some(value) => analyzer
                .nodes
                .push(idl_nodes::IdlNode::LibraryName(value.to_owned())),
            None => return Err(AnalyzerError::MissingLibraryDefinition),
        }

        for p_value in parsers.nodes.iter() {
            match p_value {
                parser::ParserNode::Comment(value) => {
                    analyzer
                        .nodes
                        .push(idl_nodes::IdlNode::Comment(value.to_owned()));
                }
                parser::ParserNode::EnumComment(value) => {
                    analyzer
                        .nodes
                        .push(idl_nodes::IdlNode::EnumComment(value.to_owned()));
                }
                parser::ParserNode::InterfaceComment(value) => {
                    analyzer
                        .nodes
                        .push(idl_nodes::IdlNode::InterfaceComment(value.to_owned()));
                }
                parser::ParserNode::StructComment(value) => {
                    analyzer
                        .nodes
                        .push(idl_nodes::IdlNode::StructComment(value.to_owned()));
                }
                parser::ParserNode::ConstComment(value) => {
                    analyzer
                        .nodes
                        .push(idl_nodes::IdlNode::ConstComment(value.to_owned()));
                }
                parser::ParserNode::TypeListComment(value) => {
                    analyzer
                        .nodes
                        .push(idl_nodes::IdlNode::TypeListComment(value.to_owned()));
                }
                parser::ParserNode::Interface(value) => {
                    let ident = match items.get_type(value.ident.clone()) {
                        Ok(value) => match value {
                            idl_nodes::TypeName::InterfaceTypeName(value) => value,
                            _ => return Err(AnalyzerError::Undefined),
                        },
                        Err(err) => return Err(err.into()),
                    };

                    let mut fields = vec![];

                    for field in value.fields.iter() {
                        match field {
                            parser::InterfaceNode::Comment(comment) => {
                                fields.push(idl_nodes::InterfaceNode::Comment(comment.to_owned()))
                            }
                            parser::InterfaceNode::InterfaceField(interface_field) => {
                                let ty = match items.get_type(interface_field.ty.clone()) {
                                    Ok(value) => value,
                                    Err(err) => return Err(err.into()),
                                };

                                let field_ident = interface_field.ident.to_owned();
                                let is_static = interface_field.is_static;

                                field_name_is_valid(field_ident.as_str(), interface_field.range)?;
                                is_reserved_type(field_ident.as_str(), interface_field.range)?;
                                is_reserved_word(field_ident.as_str(), interface_field.range)?;

                                if fields.iter().any(|v| {
                                    if let idl_nodes::InterfaceNode::InterfaceField(in_field) = v {
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

                                let interface_field = Box::new(idl_nodes::InterfaceField {
                                    attributes: vec![],
                                    ident: field_ident,
                                    is_static,
                                    ty,
                                });

                                fields.push(idl_nodes::InterfaceNode::InterfaceField(
                                    interface_field,
                                ));
                            }
                        }
                    }

                    analyzer
                        .nodes
                        .push(idl_nodes::IdlNode::TypeInterface(Box::new(
                            idl_nodes::TypeInterface { ident, fields },
                        )));
                }
                parser::ParserNode::Struct(value) => {
                    let ident = match items.get_type(value.ident.clone()) {
                        Ok(value) => match value {
                            idl_nodes::TypeName::StructTypeName(value) => value,
                            _ => return Err(AnalyzerError::Undefined),
                        },
                        Err(err) => return Err(err.into()),
                    };

                    let mut fields = vec![];

                    for field in value.fields.iter() {
                        match field {
                            parser::StructNode::Comment(comment) => {
                                fields.push(idl_nodes::StructNode::Comment(comment.to_owned()))
                            }
                            parser::StructNode::StructField(struct_field) => {
                                let ty = match items.get_type(struct_field.ty.clone()) {
                                    Ok(value) => value,
                                    Err(err) => return Err(err.into()),
                                };

                                let field_ident = struct_field.ident.to_owned();

                                field_name_is_valid(field_ident.as_str(), struct_field.range)?;
                                is_reserved_type(field_ident.as_str(), struct_field.range)?;
                                is_reserved_word(field_ident.as_str(), struct_field.range)?;

                                if fields.iter().any(|v| {
                                    if let idl_nodes::StructNode::StructField(in_field) = v {
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

                                fields.push(idl_nodes::StructNode::StructField(Box::new(
                                    idl_nodes::StructField {
                                        ident: field_ident,
                                        ty,
                                    },
                                )));
                            }
                        }
                    }

                    analyzer.nodes.push(idl_nodes::IdlNode::TypeStruct(Box::new(
                        idl_nodes::TypeStruct { ident, fields },
                    )));
                }
                parser::ParserNode::Enum(value) => {
                    let ident = match items.get_type(value.ident.clone()) {
                        Ok(value) => match value {
                            idl_nodes::TypeName::EnumTypeName(value) => value,
                            _ => return Err(AnalyzerError::Undefined),
                        },
                        Err(err) => return Err(err.into()),
                    };

                    let mut fields = vec![];

                    for field in value.fields.iter() {
                        match field {
                            parser::EnumNode::Comment(comment) => {
                                fields.push(idl_nodes::EnumNode::Comment(comment.to_owned()))
                            }
                            parser::EnumNode::EnumField(enum_field) => {
                                let field_ident = enum_field.ident.to_owned();

                                type_name_is_valid(field_ident.as_str(), value.range)?;
                                is_reserved_type(field_ident.to_lowercase().as_str(), value.range)?;
                                is_reserved_word(field_ident.to_lowercase().as_str(), value.range)?;

                                if fields.iter().any(|v| {
                                    if let idl_nodes::EnumNode::EnumField(in_field) = v {
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

                                fields.push(idl_nodes::EnumNode::EnumField(Box::new(
                                    idl_nodes::EnumField { ident: field_ident },
                                )))
                            }
                        }
                    }

                    analyzer.nodes.push(idl_nodes::IdlNode::TypeEnum(Box::new(
                        idl_nodes::TypeEnum { ident, fields },
                    )));
                }
                parser::ParserNode::Const(value) => {
                    let ident = match items.get_type(value.ident.clone()) {
                        Ok(value) => match value {
                            idl_nodes::TypeName::ConstTypeName(value) => value,
                            _ => return Err(AnalyzerError::Undefined),
                        },
                        Err(err) => return Err(err.into()),
                    };

                    let mut fields = vec![];

                    for field in value.fields.iter() {
                        match field {
                            parser::ConstNode::Comment(comment) => {
                                fields.push(idl_nodes::ConstNode::Comment(comment.to_owned()))
                            }
                            parser::ConstNode::ConstField(const_field) => {
                                let field_ident = const_field.ident.to_owned();

                                field_name_is_valid(field_ident.as_str(), const_field.range)?;
                                is_reserved_type(field_ident.as_str(), const_field.range)?;
                                is_reserved_word(field_ident.as_str(), const_field.range)?;

                                if fields.iter().any(|v| {
                                    if let idl_nodes::ConstNode::ConstField(in_field) = v {
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

                                fields.push(idl_nodes::ConstNode::ConstField(Box::new(
                                    idl_nodes::ConstField {
                                        ident: field_ident,
                                        value: const_field.value.to_owned(),
                                    },
                                )))
                            }
                        }
                    }

                    let const_type = match value.const_type {
                        parser::ConstType::Int => idl_nodes::ConstTypes::NatInt,
                        parser::ConstType::String => idl_nodes::ConstTypes::NatString,
                        parser::ConstType::Float => idl_nodes::ConstTypes::NatFloat,
                    };

                    analyzer.nodes.push(idl_nodes::IdlNode::TypeConst(Box::new(
                        idl_nodes::TypeConst {
                            ident,
                            fields,
                            const_type,
                        },
                    )));
                }
                parser::ParserNode::TypeList(value) => {
                    let ident = match items.get_type(value.ident.clone()) {
                        Ok(value) => match value {
                            idl_nodes::TypeName::ListTypeName(value) => value,
                            _ => return Err(AnalyzerError::Undefined),
                        },
                        Err(err) => return Err(err.into()),
                    };

                    let mut fields = vec![];

                    for field in value.fields.iter() {
                        match field {
                            parser::TypeListNode::Comment(comment) => {
                                fields.push(idl_nodes::TypeListNode::Comment(comment.to_owned()))
                            }
                            parser::TypeListNode::TypeListField(ty_list_field) => {
                                let ty = match items.get_type(ty_list_field.ty.clone()) {
                                    Ok(value) => value,
                                    Err(err) => return Err(err.into()),
                                };

                                let ident = ty_list_field.ident.to_owned();

                                type_name_is_valid(ident.as_str(), value.range)?;
                                is_reserved_type(ident.to_lowercase().as_str(), value.range)?;
                                is_reserved_word(ident.to_lowercase().as_str(), value.range)?;

                                if fields.iter().any(|v| {
                                    if let idl_nodes::TypeListNode::TypeListField(in_field) = v {
                                        if in_field.ident.as_str() == ident {
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

                                fields.push(idl_nodes::TypeListNode::TypeListField(Box::new(
                                    idl_nodes::TypeListField { ty, ident },
                                )));
                            }
                        }
                    }

                    analyzer.nodes.push(idl_nodes::IdlNode::TypeList(Box::new(
                        idl_nodes::TypeList { ident, fields },
                    )));
                }
                parser::ParserNode::Library(_) | parser::ParserNode::Imports(_) => {}
            }
        }

        analyzer.references_invalid_type(parsers)?;
        analyzer.struct_has_recursive_reference(parsers)?;
        analyzer.tuple_has_duplicate_fields(parsers)?;
        analyzer.tuple_has_result_type(parsers)?;
        analyzer.tuple_has_incorrect_stream_type(parsers)?;
        analyzer.map_has_incorrect_types(parsers)?;
        analyzer.array_has_incorrect_types(parsers)?;

        Ok(analyzer)
    }

   
    fn returns_interface(ty_name: &idl_nodes::TypeName) -> bool {
        match ty_name {
            // Result cannot be returned as an error
            idl_nodes::TypeName::InterfaceTypeName(_) => true,
            idl_nodes::TypeName::TypeFunction(value) => Self::returns_interface(&value.return_ty),
            idl_nodes::TypeName::TypeArray(value) => Self::returns_interface(&value.ty),
            idl_nodes::TypeName::TypeMap(value) => Self::returns_interface(&value.map_ty),
            idl_nodes::TypeName::TypeOption(value) => Self::returns_interface(&value.some_ty),
            idl_nodes::TypeName::TypeResult(value) => Self::returns_interface(&value.ok_ty),
            idl_nodes::TypeName::TypeStream(value) => Self::returns_interface(&value.s_ty),
            _ => false,
        }
    }

    // Iterates every interface, struct, and typelist to verify every map value type
    fn map_has_incorrect_types(&self, parsers: &parser::Parser) -> Result<(), ReferenceError> {
        for (ty, range) in self.get_all_types_recursive(parsers) {
            if let idl_nodes::TypeName::TypeMap(map) = ty {
                let index_ref = map.index_ty.get_type_reference(); // TODO get the real name of the type
                let ty_ref = map.map_ty.get_type_reference();

                let index_invalid = match &map.index_ty {
                    idl_nodes::TypeName::Types(types) => match types {
                        idl_nodes::Types::NatInt | idl_nodes::Types::NatString => None,
                        _ => Some(index_ref),
                    },
                    idl_nodes::TypeName::EnumTypeName(_) => None, // Since it's a integer, can be used as an index.
                    idl_nodes::TypeName::ConstTypeName(value) => {
                        let const_ty = self.find_ty_const(value).unwrap();
                        match const_ty.const_type {
                            idl_nodes::ConstTypes::NatInt | idl_nodes::ConstTypes::NatString => {
                                None
                            }
                            idl_nodes::ConstTypes::NatFloat => Some(index_ref),
                        }
                    }
                    _ => Some(index_ref),
                };

                if let Some(ty) = index_invalid {
                    return Err(ReferenceError(
                        ReferenceErrorKind::Invalid,
                        range,
                        format!("map index `{}`", ty),
                    ));
                }

                // Types that are allowed.
                let ty_invalid = match &map.map_ty {
                    idl_nodes::TypeName::Types(types) => match types {
                        idl_nodes::Types::NatNone => Some(ty_ref),
                        _ => None,
                    },
                    idl_nodes::TypeName::ListTypeName(_)
                    | idl_nodes::TypeName::EnumTypeName(_)
                    | idl_nodes::TypeName::StructTypeName(_)
                    | idl_nodes::TypeName::TypePair(_)
                    | idl_nodes::TypeName::ConstTypeName(_) => None,
                    _ => Some(ty_ref),
                };

                if let Some(ty) = ty_invalid {
                    return Err(ReferenceError(
                        ReferenceErrorKind::Invalid,
                        range,
                        format!("map value type `{}`", ty),
                    ));
                }
            };
        }

        Ok(())
    }

    // Iterates every interface, struct, and typelist to verify every array type
    fn array_has_incorrect_types(&self, parsers: &parser::Parser) -> Result<(), ReferenceError> {
        for (ty, range) in self.get_all_types_recursive(parsers) {
            if let idl_nodes::TypeName::TypeArray(array) = ty {
                let ty_ref = array.ty.get_type_reference(); // TODO get the real name of the type

                let ty_invalid = match &array.ty {
                    idl_nodes::TypeName::Types(types) => match types {
                        idl_nodes::Types::NatNone => Some(ty_ref),
                        _ => None,
                    },
                    idl_nodes::TypeName::ListTypeName(_)
                    | idl_nodes::TypeName::EnumTypeName(_)
                    | idl_nodes::TypeName::StructTypeName(_)
                    | idl_nodes::TypeName::TypeMap(_)
                    | idl_nodes::TypeName::TypePair(_)
                    | idl_nodes::TypeName::ConstTypeName(_) => None,
                    _ => Some(ty_ref),
                };

                if let Some(ty) = ty_invalid {
                    return Err(ReferenceError(
                        ReferenceErrorKind::Invalid,
                        range,
                        format!("array type `{}`", ty),
                    ));
                }
            };
        }

        Ok(())
    }

    fn add_types_for_recursive<'a>(
        &self,
        ty: &'a idl_nodes::TypeName,
        range: Range,
    ) -> Vec<(&'a idl_nodes::TypeName, Range)> {
        let mut types = vec![];

        let add_from_tuples = |tuple: &'a idl_nodes::TypeTuple| {
            let mut result = vec![];
            for field in &tuple.fields {
                result.append(&mut self.add_types_for_recursive(&field.ty, range));
            }
            result
        };

        types.push((ty, range));

        match ty {
            idl_nodes::TypeName::TypeFunction(value) => {
                types.append(&mut self.add_types_for_recursive(&value.return_ty, range));
                if let idl_nodes::TypeName::TypeTuple(tuple) = &value.args {
                    types.append(&mut add_from_tuples(tuple));
                }
            }
            idl_nodes::TypeName::TypeTuple(tuple) => types.append(&mut add_from_tuples(tuple)),
            idl_nodes::TypeName::TypeArray(value) => {
                types.append(&mut self.add_types_for_recursive(&value.ty, range))
            }
            idl_nodes::TypeName::TypeMap(value) => {
                types.append(&mut self.add_types_for_recursive(&value.index_ty, range));
                types.append(&mut self.add_types_for_recursive(&value.map_ty, range));
            }
            idl_nodes::TypeName::TypeOption(value) => {
                types.append(&mut self.add_types_for_recursive(&value.some_ty, range))
            }
            idl_nodes::TypeName::TypeResult(value) => {
                types.append(&mut self.add_types_for_recursive(&value.ok_ty, range));
                types.append(&mut self.add_types_for_recursive(&value.err_ty, range));
            }
            idl_nodes::TypeName::TypeStream(value) => {
                types.append(&mut self.add_types_for_recursive(&value.s_ty, range))
            }
            _ => {}
        };

        types
    }

    fn get_all_types_recursive<'a>(
        &'a self,
        parsers: &parser::Parser,
    ) -> Vec<(&'a idl_nodes::TypeName, Range)> {
        let mut result = vec![];

        for node in &self.nodes {
            match node {
                idl_nodes::IdlNode::TypeInterface(value) => {
                    for t_node in &value.fields {
                        if let idl_nodes::InterfaceNode::InterfaceField(field) = t_node {
                            let range = parsers.get_range_from_field_name(
                                value.ident.as_str(),
                                field.ident.as_str(),
                            );
                            result.append(&mut self.add_types_for_recursive(&field.ty, range))
                        }
                    }
                }
                idl_nodes::IdlNode::TypeStruct(value) => {
                    for t_node in &value.fields {
                        if let idl_nodes::StructNode::StructField(field) = t_node {
                            let range = parsers.get_range_from_field_name(
                                value.ident.as_str(),
                                field.ident.as_str(),
                            );
                            result.append(&mut self.add_types_for_recursive(&field.ty, range))
                        }
                    }
                }
                idl_nodes::IdlNode::TypeList(value) => {
                    for t_node in &value.fields {
                        if let idl_nodes::TypeListNode::TypeListField(field) = t_node {
                            let range = parsers.get_range_from_field_name(
                                value.ident.as_str(),
                                field.ident.as_str(),
                            );
                            result.append(&mut self.add_types_for_recursive(&field.ty, range))
                        }
                    }
                }
                _ => {}
            }
        }

        result
    }

    fn tuple_has_result_type(&self, parsers: &parser::Parser) -> Result<(), ReferenceError> {
        let refences_result = |ty: &idl_nodes::TypeName, range: Range| {
            let tuple = match ty {
                idl_nodes::TypeName::TypeFunction(value) => {
                    if let idl_nodes::TypeName::TypeTuple(tul) = &value.args {
                        Some(tul)
                    } else {
                        None
                    }
                }
                idl_nodes::TypeName::TypeTuple(value) => Some(value),
                _ => None,
            };

            if let Some(tuple) = tuple {
                for t_ty in &tuple.fields {
                    if let idl_nodes::TypeName::TypeResult(_) = &t_ty.ty {
                        return Err(ReferenceError(
                            ReferenceErrorKind::ReferencesResult,
                            range,
                            t_ty.ty.get_type_reference(),
                        ));
                    }
                }
            }

            Ok(())
        };

        for node in &self.nodes {
            match node {
                idl_nodes::IdlNode::TypeInterface(value) => {
                    for interface_node in &value.fields {
                        if let idl_nodes::InterfaceNode::InterfaceField(field) = interface_node {
                            let range = parsers.get_range_from_field_name(
                                value.ident.as_str(),
                                field.ident.as_str(),
                            );
                            refences_result(&field.ty, range)?;
                        }
                    }
                }
                _ => {}
            }
        }

        Ok(())
    }

    fn tuple_has_incorrect_stream_type(
        &self,
        parsers: &parser::Parser,
    ) -> Result<(), ReferenceError> {
        let refences_stream = |ty: &idl_nodes::TypeName, range: Range| {
            let tuple = match ty {
                idl_nodes::TypeName::TypeFunction(value) => {
                    if let idl_nodes::TypeName::TypeTuple(tul) = &value.args {
                        Some(tul)
                    } else {
                        None
                    }
                }
                idl_nodes::TypeName::TypeTuple(value) => Some(value),
                _ => None,
            };

            if let Some(tuple) = tuple {
                if let Some(sl) = tuple.fields.get(..tuple.fields.len() - 1) {
                    for t_ty in sl {
                        if let idl_nodes::TypeName::TypeStream(st) = &t_ty.ty {
                            return Err(ReferenceError(
                                ReferenceErrorKind::ReferencesStream,
                                range,
                                format!("{:?}", st), // TODO
                            ));
                        }
                    }
                }
            }

            Ok(())
        };

        for node in &self.nodes {
            match node {
                idl_nodes::IdlNode::TypeInterface(value) => {
                    for interface_node in &value.fields {
                        if let idl_nodes::InterfaceNode::InterfaceField(field) = interface_node {
                            let range = parsers.get_range_from_field_name(
                                value.ident.as_str(),
                                field.ident.as_str(),
                            );
                            refences_stream(&field.ty, range)?;
                        }
                    }
                }
                _ => {}
            }
        }

        Ok(())
    }

    fn tuple_has_duplicate_fields(
        &self,
        parsers: &parser::Parser,
    ) -> Result<(), DuplicateFieldNameError> {
        let has_duplicate_in_tuple = |ty: &idl_nodes::TypeName| {
            let tuple = match ty {
                idl_nodes::TypeName::TypeFunction(value) => {
                    if let idl_nodes::TypeName::TypeTuple(tul) = &value.args {
                        Some(tul)
                    } else {
                        None
                    }
                }
                idl_nodes::TypeName::TypeTuple(value) => Some(value),
                _ => None,
            };

            if let Some(tuple) = tuple {
                !tuple.fields.iter().all(|entry| {
                    tuple
                        .fields
                        .iter()
                        .filter(|v| &v.ident == &entry.ident)
                        .count()
                        == 1
                })
            } else {
                false
            }
        };

        for node in &self.nodes {
            match node {
                idl_nodes::IdlNode::TypeInterface(value) => {
                    for interface_node in &value.fields {
                        if let idl_nodes::InterfaceNode::InterfaceField(field) = interface_node {
                            if has_duplicate_in_tuple(&field.ty) {
                                let range = parsers.get_range_from_field_name(
                                    value.ident.as_str(),
                                    field.ident.as_str(),
                                );

                                return Err(DuplicateFieldNameError(field.ident.to_owned(), range));
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        Ok(())
    }

    fn references_invalid_type(&self, parsers: &parser::Parser) -> Result<(), ReferenceError> {
        for node in &self.nodes {
            match node {
                idl_nodes::IdlNode::TypeInterface(value) => {
                    for interface_node in value.fields.iter() {
                        if let idl_nodes::InterfaceNode::InterfaceField(interface_field) =
                            interface_node
                        {
                            let range = parsers.get_range_from_field_name(
                                value.ident.as_str(),
                                interface_field.ident.as_str(),
                            );

                            if !interface_field.is_static {
                                Self::references_interface(&interface_field.ty, range)?;
                            }
                        }
                    }
                }
                idl_nodes::IdlNode::TypeStruct(value) => {
                    for struct_node in value.fields.iter() {
                        if let idl_nodes::StructNode::StructField(struct_field) = struct_node {
                            let range = parsers.get_range_from_field_name(
                                value.ident.as_str(),
                                struct_field.ident.as_str(),
                            );

                            Self::references_interface(&struct_field.ty, range)?;
                        }
                    }
                }
                idl_nodes::IdlNode::TypeList(value) => {
                    for type_list_node in value.fields.iter() {
                        if let idl_nodes::TypeListNode::TypeListField(type_list_field) =
                            type_list_node
                        {
                            let range = parsers.get_range_from_field_name(
                                value.ident.as_str(),
                                type_list_field.ident.as_str(),
                            );

                            Self::references_interface(&type_list_field.ty, range)?;
                        }
                    }
                }
                _ => {}
            }
        }

        Ok(())
    }

    fn references_interface(ty: &idl_nodes::TypeName, range: Range) -> Result<(), ReferenceError> {
        match ty {
            idl_nodes::TypeName::InterfaceTypeName(_) => Err(ReferenceError(
                ReferenceErrorKind::ReferencesInterface,
                range,
                ty.get_type_reference(),
            )),
            idl_nodes::TypeName::TypeFunction(function) => {
                Self::references_interface(&function.return_ty, range)?;
                Self::references_interface(&function.args, range)
            }
            idl_nodes::TypeName::TypeTuple(tuple) => {
                for tuple_ty in tuple.fields.iter() {
                    Self::references_interface(&tuple_ty.ty, range)?;
                }
                Ok(())
            }
            idl_nodes::TypeName::TypeMap(map) => Self::references_interface(&map.map_ty, range),
            idl_nodes::TypeName::TypeArray(array) => Self::references_interface(&array.ty, range),
            idl_nodes::TypeName::TypeOption(option) => {
                Self::references_interface(&option.some_ty, range)
            }
            idl_nodes::TypeName::TypeResult(result) => {
                Self::references_interface(&result.ok_ty, range)?;
                Self::references_interface(&result.err_ty, range)
            }
            idl_nodes::TypeName::TypeStream(stream) => {
                Self::references_interface(&stream.s_ty, range)
            }
            _ => Ok(()),
        }
    }

    // Given a type name, find if any other type inside references it.
    fn struct_has_recursive_reference(
        &self,
        parsers: &parser::Parser,
    ) -> Result<(), ReferenceError> {
        for node in &self.nodes {
            match node {
                idl_nodes::IdlNode::TypeStruct(value) => {
                    let range = parsers.get_range_from_type_name(value.ident.as_str());

                    for struct_node in value.fields.iter() {
                        if let idl_nodes::StructNode::StructField(struct_field) = struct_node {
                            if self.has_type_reference_in_struct(
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
    fn has_type_reference_in_struct(&self, target: &str, refer: &idl_nodes::TypeName) -> bool {
        match refer {
            idl_nodes::TypeName::StructTypeName(struct_ident) => {
                if target == struct_ident {
                    return true;
                }

                for node in &self.nodes {
                    if let idl_nodes::IdlNode::TypeStruct(value) = node {
                        if value.ident.as_str() == struct_ident {
                            for struct_node in value.fields.iter() {
                                if let idl_nodes::StructNode::StructField(field) = struct_node {
                                    if self.has_type_reference_in_struct(target, &field.ty) {
                                        return true;
                                    }
                                }
                            }
                        }
                    }
                }

                false
            }
            idl_nodes::TypeName::TypeArray(array) => {
                self.has_type_reference_in_struct(target, &array.ty)
            }
            idl_nodes::TypeName::TypeMap(map) => {
                self.has_type_reference_in_struct(target, &map.map_ty)
                    || self.has_type_reference_in_struct(target, &map.map_ty)
            }
            _ => false,
        }
    }

    // Finds if any type has the reference target.
    fn has_type_reference(&self, target: &str, refer: &idl_nodes::TypeName) -> bool {
        match refer {
            idl_nodes::TypeName::StructTypeName(struct_ident) => {
                if target == struct_ident {
                    return true;
                }

                for node in &self.nodes {
                    if let idl_nodes::IdlNode::TypeStruct(value) = node {
                        if value.ident.as_str() == struct_ident {
                            for struct_node in value.fields.iter() {
                                if let idl_nodes::StructNode::StructField(field) = struct_node {
                                    if self.has_type_reference(target, &field.ty) {
                                        return true;
                                    }
                                }
                            }
                        }
                    }
                }

                false
            }
            idl_nodes::TypeName::InterfaceTypeName(interface_ident) => {
                if target == interface_ident {
                    return true;
                }

                for node in &self.nodes {
                    if let idl_nodes::IdlNode::TypeInterface(value) = node {
                        if value.ident.as_str() == interface_ident {
                            for interface_node in value.fields.iter() {
                                if let idl_nodes::InterfaceNode::InterfaceField(field) =
                                    interface_node
                                {
                                    if self.has_type_reference(target, &field.ty) {
                                        return true;
                                    }
                                }
                            }
                        }
                    }
                }

                false
            }
            idl_nodes::TypeName::ListTypeName(list_ident) => {
                if target == list_ident {
                    return true;
                }

                for node in &self.nodes {
                    if let idl_nodes::IdlNode::TypeList(value) = node {
                        if value.ident.as_str() == list_ident {
                            for ty_node in value.fields.iter() {
                                if let idl_nodes::TypeListNode::TypeListField(field) = ty_node {
                                    if self.has_type_reference(target, &field.ty) {
                                        return true;
                                    }
                                }
                            }
                        }
                    }
                }

                false
            }
            idl_nodes::TypeName::TypeArray(array) => self.has_type_reference(target, &array.ty),
            idl_nodes::TypeName::TypeMap(map) => {
                self.has_type_reference(target, &map.map_ty)
                    || self.has_type_reference(target, &map.map_ty)
            }
            idl_nodes::TypeName::TypeFunction(function) => {
                self.has_type_reference(target, &function.return_ty)
                    || self.has_type_reference(target, &function.args)
            }
            idl_nodes::TypeName::TypeTuple(tuple) => {
                for ty in tuple.fields.iter() {
                    if self.has_type_reference(target, &ty.ty) {
                        return true;
                    }
                }

                false
            }
            _ => false,
        }
    }

    fn has_duplicate_names(items: &AnalyzerItems) -> Result<(), DuplicateNameError> {
        let names = items
            .interfaces
            .iter()
            .chain(items.structs.iter())
            .chain(items.type_lists.iter())
            .chain(items.consts.iter())
            .chain(items.enums.iter());

        if let Some(value) = names
            .clone()
            .find(|v| names.clone().filter(|f| f == v).count() != 1)
        {
            return Err(DuplicateNameError(value.to_owned()));
        }

        Ok(())
    }
}

impl AnalyzerItems {
    fn get_type(&self, ty: Arc<parser::Type>) -> Result<idl_nodes::TypeName, ReferenceError> {
        match &*ty {
            parser::Type::Name(name) => {
                if self.interfaces.contains(&name.ident) {
                    Ok(idl_nodes::TypeName::InterfaceTypeName(
                        name.ident.to_owned(),
                    ))
                } else if self.structs.contains(&name.ident) {
                    Ok(idl_nodes::TypeName::StructTypeName(name.ident.to_owned()))
                } else if self.enums.contains(&name.ident) {
                    Ok(idl_nodes::TypeName::EnumTypeName(name.ident.to_owned()))
                } else if self.consts.contains(&name.ident) {
                    Ok(idl_nodes::TypeName::ConstTypeName(name.ident.to_owned()))
                } else if self.type_lists.contains(&name.ident) {
                    Ok(idl_nodes::TypeName::ListTypeName(name.ident.to_owned()))
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

                for t_list in tuple.fields.iter() {
                    tuple_list.push(idl_nodes::TupleEntry {
                        ident: t_list.ident.to_owned(),
                        ty: self.get_type(t_list.ty.clone())?,
                    })
                }

                Ok(idl_nodes::TypeName::TypeTuple(Box::new(
                    idl_nodes::TypeTuple { fields: tuple_list },
                )))
            }
            parser::Type::Function(function) => Ok(idl_nodes::TypeName::TypeFunction(Box::new(
                idl_nodes::TypeFunction {
                    args: self.get_type(function.args.clone())?,
                    return_ty: self.get_type(function.ret_ty.clone())?,
                },
            ))),
            parser::Type::Native(native) => Ok(idl_nodes::TypeName::Types(match native.ty {
                scanner::NativeTypes::Bool => idl_nodes::Types::NatBool,
                scanner::NativeTypes::Int => idl_nodes::Types::NatInt,
                scanner::NativeTypes::Bytes => idl_nodes::Types::NatBytes,
                scanner::NativeTypes::String => idl_nodes::Types::NatString,
                scanner::NativeTypes::None => idl_nodes::Types::NatNone,
                scanner::NativeTypes::Float => idl_nodes::Types::NatFloat,
            })),
            parser::Type::Array(array) => Ok(idl_nodes::TypeName::TypeArray(Box::new(
                idl_nodes::TypeArray {
                    ty: self.get_type(array.array_ty.clone())?,
                },
            ))),
            parser::Type::Map(map) => {
                Ok(idl_nodes::TypeName::TypeMap(Box::new(idl_nodes::TypeMap {
                    index_ty: self.get_type(map.index_ty.clone())?,
                    map_ty: self.get_type(map.m_ty.clone())?,
                })))
            }
            parser::Type::Result(result) => Ok(idl_nodes::TypeName::TypeResult(Box::new(
                idl_nodes::TypeResult {
                    ok_ty: self.get_type(result.ok_ty.clone())?,
                    err_ty: self.get_type(result.err_ty.clone())?,
                },
            ))),
            parser::Type::Pair(pair) => Ok(idl_nodes::TypeName::TypePair(Box::new(
                idl_nodes::TypePair {
                    first_ty: self.get_type(pair.first_ty.clone())?,
                    second_ty: self.get_type(pair.second_ty.clone())?,
                }
            ))),
            parser::Type::Stream(stream) => Ok(idl_nodes::TypeName::TypeStream(Box::new(
                idl_nodes::TypeStream {
                    s_ty: self.get_type(stream.s_ty.clone())?,
                },
            ))),
            parser::Type::Option(option) => Ok(idl_nodes::TypeName::TypeOption(Box::new(
                idl_nodes::TypeOption {
                    some_ty: self.get_type(option.some_ty.clone())?,
                },
            ))),
        }
    }
}

impl idl_nodes::TypeName {
    fn get_type_reference(&self) -> String {
        match self {
            idl_nodes::TypeName::ConstTypeName(value)
            | idl_nodes::TypeName::InterfaceTypeName(value)
            | idl_nodes::TypeName::StructTypeName(value)
            | idl_nodes::TypeName::EnumTypeName(value)
            | idl_nodes::TypeName::ListTypeName(value) => value.to_owned(),
            _ => "".to_owned(),
        }
    }
}
