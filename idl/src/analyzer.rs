use super::idl_types;
use super::module;
use super::parser::{self, Range};
use super::reserved::{
    field_name_is_valid, is_reserved_type, is_reserved_word, type_name_is_valid, NameError,
};
use super::scanner;
use std::convert::From;
use std::fmt;
use std::sync::Arc;

#[derive(Debug)]
pub enum ReferenceErrorKind {
    Invalid,
    ReferencesInterface,
    ReferencesStream,
    StructRecursiveReference,
    ReferencesResult,
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
            ReferenceErrorKind::ReferencesStream => format!("References stream `{}`.", name),
            ReferenceErrorKind::ReferencesResult => format!("References result `{}`.", name),
            ReferenceErrorKind::StructRecursiveReference => {
                format!("Recursive reference in struct `{}`.", name)
            }
            ReferenceErrorKind::UndefinedType => format!("Undefined type `{}`.", name),
        };

        write!(f, "{}", errstr)
    }
}

impl From<NameError> for AnalyzerError {
    fn from(value: NameError) -> Self {
        AnalyzerError::NameError(value)
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
            AnalyzerError::ReferenceError(ref_err) => ref_err.to_string(),
        };

        write!(f, "{}", errstr)
    }
}

#[derive(Debug, Default)]
pub struct Analyzer {
    pub nodes: Vec<idl_types::TypeNode>,
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

    pub fn get_library_name(&self) -> String {
        for node in &self.nodes {
            match node {
                idl_types::TypeNode::LibraryName(name) => return name.to_owned(),
                _ => {}
            }
        }

        panic!("Does not have a library name");
    }

    pub fn find_ty_struct(&self, name: &str) -> Option<&Box<idl_types::TypeStruct>> {
        for node in &self.nodes {
            match node {
                idl_types::TypeNode::TypeStruct(value) => {
                    if value.ident == name {
                        return Some(value);
                    }
                }
                _ => {}
            }
        }

        None
    }

    pub fn find_ty_interface(&self, name: &str) -> Option<&Box<idl_types::TypeInterface>> {
        for node in &self.nodes {
            match node {
                idl_types::TypeNode::TypeInterface(value) => {
                    if value.ident == name {
                        return Some(value);
                    }
                }
                _ => {}
            }
        }

        None
    }

    pub fn find_ty_enum(&self, name: &str) -> Option<&Box<idl_types::TypeEnum>> {
        for node in &self.nodes {
            match node {
                idl_types::TypeNode::TypeEnum(value) => {
                    if value.ident == name {
                        return Some(value);
                    }
                }
                _ => {}
            }
        }

        None
    }

    pub fn find_ty_const(&self, name: &str) -> Option<&Box<idl_types::TypeConst>> {
        for node in &self.nodes {
            match node {
                idl_types::TypeNode::TypeConst(value) => {
                    if value.ident == name {
                        return Some(value);
                    }
                }
                _ => {}
            }
        }

        None
    }

    pub fn find_ty_list(&self, name: &str) -> Option<&Box<idl_types::TypeList>> {
        for node in &self.nodes {
            match node {
                idl_types::TypeNode::TypeList(value) => {
                    if value.ident == name {
                        return Some(value);
                    }
                }
                _ => {}
            }
        }

        None
    }

    pub fn interface_has_static_field(name: &idl_types::TypeInterface) -> bool {
        if name.fields.iter().any(|node| {
            if let idl_types::InterfaceNode::InterfaceField(field) = node {
                field.is_static
            } else {
                false
            }
        }) {
            return true;
        }

        false
    }

    pub fn interface_has_non_static_field(name: &idl_types::TypeInterface) -> bool {
        if name.fields.iter().any(|node| {
            if let idl_types::InterfaceNode::InterfaceField(field) = node {
                !field.is_static
            } else {
                false
            }
        }) {
            return true;
        }

        false
    }

    fn returns_interface(ty_name: &idl_types::TypeName) -> bool {
        match ty_name {
            // Result cannot be returned as an error
            idl_types::TypeName::InterfaceTypeName(_) => true,
            idl_types::TypeName::TypeFunction(value) => Self::returns_interface(&value.return_ty),
            idl_types::TypeName::TypeArray(value) => Self::returns_interface(&value.ty),
            idl_types::TypeName::TypeMap(value) => Self::returns_interface(&value.map_ty),
            idl_types::TypeName::TypeOption(value) => Self::returns_interface(&value.some_ty),
            idl_types::TypeName::TypeResult(value) => Self::returns_interface(&value.ok_ty), 
            idl_types::TypeName::TypeStream(value) => Self::returns_interface(&value.s_ty),
            _ => false,
        }
    }

    pub fn interface_has_constructor_field(name: &idl_types::TypeInterface) -> bool {
        for node in &name.fields {
            match node {
                idl_types::InterfaceNode::InterfaceField(field) => {
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
                                let is_static = interface_field.is_static;

                                field_name_is_valid(field_ident.as_str(), interface_field.range)?;
                                is_reserved_type(field_ident.as_str(), interface_field.range)?;
                                is_reserved_word(field_ident.as_str(), interface_field.range)?;

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

                                let interface_field = Box::new(idl_types::InterfaceField {
                                    attributes: vec![],
                                    ident: field_ident,
                                    is_static,
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

                                field_name_is_valid(field_ident.as_str(), struct_field.range)?;
                                is_reserved_type(field_ident.as_str(), struct_field.range)?;
                                is_reserved_word(field_ident.as_str(), struct_field.range)?;

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

                                type_name_is_valid(ident.as_str(), value.range)?;
                                is_reserved_type(ident.to_lowercase().as_str(), value.range)?;
                                is_reserved_word(ident.to_lowercase().as_str(), value.range)?;

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

                                field_name_is_valid(field_ident.as_str(), const_field.range)?;
                                is_reserved_type(field_ident.as_str(), const_field.range)?;
                                is_reserved_word(field_ident.as_str(), const_field.range)?;

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
                        parser::ConstType::Int => idl_types::ConstTypes::NatInt,
                        parser::ConstType::String => idl_types::ConstTypes::NatString,
                        parser::ConstType::Float => idl_types::ConstTypes::NatFloat,
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

                    let mut fields = vec![];

                    for field in value.fields.iter() {
                        match field {
                            parser::TypeListNode::Comment(comment) => {
                                fields.push(idl_types::TypeListNode::Comment(comment.to_owned()))
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
                                    if let idl_types::TypeListNode::TypeListField(in_field) = v {
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

                                fields.push(idl_types::TypeListNode::TypeListField(Box::new(
                                    idl_types::TypeListField { ty, ident },
                                )));
                            }
                        }
                    }

                    analyzer.nodes.push(idl_types::TypeNode::TypeList(Box::new(
                        idl_types::TypeList { ident, fields },
                    )));
                }
                parser::ParserNode::Library(_) | parser::ParserNode::Imports(_) => {}
            }
        }

        Self::references_invalid_type(&analyzer.nodes, parsers)?;
        Self::struct_has_recursive_reference(&analyzer.nodes, parsers)?;
        Self::tuple_has_duplicate_fields(&analyzer.nodes, parsers)?;
        Self::tuple_has_result_type(&analyzer.nodes, parsers)?;
        Self::tuple_has_incorrect_stream_type(&analyzer.nodes, parsers)?;

        Ok(analyzer)
    }

    fn tuple_has_result_type(
        nodes: &[idl_types::TypeNode],
        parsers: &parser::Parser,
    ) -> Result<(), ReferenceError> {
        let refences_result = |ty: &idl_types::TypeName, range: Range| {
            let tuple = match ty {
                idl_types::TypeName::TypeFunction(value) => {
                    if let idl_types::TypeName::TypeTuple(tul) = &value.args {
                        Some(tul)
                    } else {
                        None
                    }
                }
                idl_types::TypeName::TypeTuple(value) => Some(value),
                _ => None,
            };

            if let Some(tuple) = tuple {
                for t_ty in &tuple.fields {
                    if let idl_types::TypeName::TypeResult(_) = &t_ty.ty {
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

        for node in nodes {
            match node {
                idl_types::TypeNode::TypeInterface(value) => {
                    for interface_node in &value.fields {
                        if let idl_types::InterfaceNode::InterfaceField(field) = interface_node {
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
        nodes: &[idl_types::TypeNode],
        parsers: &parser::Parser,
    ) -> Result<(), ReferenceError> {
        let refences_stream = |ty: &idl_types::TypeName, range: Range| {
            let tuple = match ty {
                idl_types::TypeName::TypeFunction(value) => {
                    if let idl_types::TypeName::TypeTuple(tul) = &value.args {
                        Some(tul)
                    } else {
                        None
                    }
                }
                idl_types::TypeName::TypeTuple(value) => Some(value),
                _ => None,
            };

            if let Some(tuple) = tuple {
                for t_ty in &tuple.fields[..tuple.fields.len() - 1] {
                    if let idl_types::TypeName::TypeStream(st) = &t_ty.ty {
                        return Err(ReferenceError(
                            ReferenceErrorKind::ReferencesStream,
                            range,
                            format!("{:?}", st), // TODO
                        ));
                    }
                }
            }

            Ok(())
        };

        for node in nodes {
            match node {
                idl_types::TypeNode::TypeInterface(value) => {
                    for interface_node in &value.fields {
                        if let idl_types::InterfaceNode::InterfaceField(field) = interface_node {
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
        nodes: &[idl_types::TypeNode],
        parsers: &parser::Parser,
    ) -> Result<(), DuplicateFieldNameError> {
        let has_duplicate_in_tuple = |ty: &idl_types::TypeName| {
            let tuple = match ty {
                idl_types::TypeName::TypeFunction(value) => {
                    if let idl_types::TypeName::TypeTuple(tul) = &value.args {
                        Some(tul)
                    } else {
                        None
                    }
                }
                idl_types::TypeName::TypeTuple(value) => Some(value),
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

        for node in nodes {
            match node {
                idl_types::TypeNode::TypeInterface(value) => {
                    for interface_node in &value.fields {
                        if let idl_types::InterfaceNode::InterfaceField(field) = interface_node {
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

    fn references_invalid_type(
        nodes: &[idl_types::TypeNode],
        parsers: &parser::Parser,
    ) -> Result<(), ReferenceError> {
        for node in nodes {
            match node {
                idl_types::TypeNode::TypeInterface(value) => {
                    for interface_node in value.fields.iter() {
                        if let idl_types::InterfaceNode::InterfaceField(interface_field) =
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
                idl_types::TypeNode::TypeStruct(value) => {
                    for struct_node in value.fields.iter() {
                        if let idl_types::StructNode::StructField(struct_field) = struct_node {
                            let range = parsers.get_range_from_field_name(
                                value.ident.as_str(),
                                struct_field.ident.as_str(),
                            );

                            Self::references_interface(&struct_field.ty, range)?;
                        }
                    }
                }
                idl_types::TypeNode::TypeList(value) => {
                    for type_list_node in value.fields.iter() {
                        if let idl_types::TypeListNode::TypeListField(type_list_field) =
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

    fn references_interface(ty: &idl_types::TypeName, range: Range) -> Result<(), ReferenceError> {
        match ty {
            idl_types::TypeName::InterfaceTypeName(_) => Err(ReferenceError(
                ReferenceErrorKind::ReferencesInterface,
                range,
                ty.get_type_reference(),
            )),
            idl_types::TypeName::TypeFunction(function) => {
                Self::references_interface(&function.return_ty, range)?;
                Self::references_interface(&function.args, range)
            }
            idl_types::TypeName::TypeTuple(tuple) => {
                for tuple_ty in tuple.fields.iter() {
                    Self::references_interface(&tuple_ty.ty, range)?;
                }
                Ok(())
            }
            idl_types::TypeName::TypeMap(map) => Self::references_interface(&map.map_ty, range),
            idl_types::TypeName::TypeArray(array) => Self::references_interface(&array.ty, range),
            idl_types::TypeName::TypeOption(option) => {
                Self::references_interface(&option.some_ty, range)
            }
            idl_types::TypeName::TypeResult(result) => {
                Self::references_interface(&result.ok_ty, range)?;
                Self::references_interface(&result.err_ty, range)
            }
            idl_types::TypeName::TypeStream(stream) => {
                Self::references_interface(&stream.s_ty, range)
            }
            _ => Ok(()),
        }
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
                            for ty_node in value.fields.iter() {
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
                for ty in tuple.fields.iter() {
                    if Self::has_type_reference(nodes, target, &ty.ty) {
                        return true;
                    }
                }

                false
            }
            _ => false,
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

                for t_list in tuple.fields.iter() {
                    tuple_list.push(idl_types::TupleEntry {
                        ident: t_list.ident.to_owned(),
                        ty: self.get_type(t_list.ty.clone())?,
                    })
                }

                Ok(idl_types::TypeName::TypeTuple(Box::new(
                    idl_types::TypeTuple { fields: tuple_list },
                )))
            }
            parser::Type::Function(function) => Ok(idl_types::TypeName::TypeFunction(Box::new(
                idl_types::TypeFunction {
                    args: self.get_type(function.args.clone())?,
                    return_ty: self.get_type(function.ret_ty.clone())?,
                },
            ))),
            parser::Type::Native(native) => Ok(idl_types::TypeName::Types(match native.ty {
                scanner::NativeTypes::Bool => idl_types::Types::NatBool,
                scanner::NativeTypes::Int => idl_types::Types::NatInt,
                scanner::NativeTypes::Bytes => idl_types::Types::NatBytes,
                scanner::NativeTypes::String => idl_types::Types::NatString,
                scanner::NativeTypes::None => idl_types::Types::NatNone,
                scanner::NativeTypes::Float => idl_types::Types::NatFloat,
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
            parser::Type::Result(result) => Ok(idl_types::TypeName::TypeResult(Box::new(
                idl_types::TypeResult {
                    ok_ty: self.get_type(result.ok_ty.clone())?,
                    err_ty: self.get_type(result.err_ty.clone())?,
                },
            ))),
            parser::Type::Stream(stream) => Ok(idl_types::TypeName::TypeStream(Box::new(
                idl_types::TypeStream {
                    s_ty: self.get_type(stream.s_ty.clone())?,
                },
            ))),
            parser::Type::Option(option) => Ok(idl_types::TypeName::TypeOption(Box::new(
                idl_types::TypeOption {
                    some_ty: self.get_type(option.some_ty.clone())?,
                },
            ))),
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
            | idl_types::TypeName::ListTypeName(value) => value.to_owned(),
            _ => "".to_owned(),
        }
    }
}
