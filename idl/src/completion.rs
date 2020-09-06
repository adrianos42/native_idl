use std::sync::Arc;

use crate::idl_types;
use crate::parser;
use crate::parser::Range;

#[derive(Debug)]
enum CompletionInterface {
    FieldTypeName,
    Empty,
}

impl From<CompletionInterface> for CompletionType {
    fn from(value: CompletionInterface) -> Self {
        CompletionType::Interface(value)
    }
}

#[derive(Debug)]
enum CompletionFactory {
    FieldTypeName,
    Empty,
}

impl From<CompletionFactory> for CompletionType {
    fn from(value: CompletionFactory) -> Self {
        CompletionType::Factory(value)
    }
}

#[derive(Debug)]
enum CompletionStruct {
    FieldTypeName,
    Empty,
}

impl From<CompletionStruct> for CompletionType {
    fn from(value: CompletionStruct) -> Self {
        CompletionType::Struct(value)
    }
}

#[derive(Debug)]
enum CompletionStream {
    FieldTypeName,
    Empty,
}

impl From<CompletionStream> for CompletionType {
    fn from(value: CompletionStream) -> Self {
        CompletionType::Stream(value)
    }
}

#[derive(Debug)]
enum CompletionType {
    Empty,
    Body,
    Interface(CompletionInterface),
    Struct(CompletionStruct),
    Stream(CompletionStream),
    Factory(CompletionFactory),
    MapType,
    Keyword,
}

pub enum CompletionNode {
    Type(Arc<parser::Type>),
    StructName(String),
    NativeName(String),
    EnumName(String),
    ConstName(String),
    ListName(String),
    FactoryName(String),
    StreamName(String),
    InterfaceName(String),
    MapType,
    Keyword,
}

pub struct Completion {
    pub nodes: Vec<CompletionNode>,
}

impl Completion {
    pub fn try_complete(
        text: &str,
        pos: parser::Position,
        module: &super::module::Module,
    ) -> Option<Completion> {
        let completions = Self::get_location_in_text(text, pos.as_range());
        let mut nodes = vec![];

        for (completion_type, reference_name) in completions {
            match completion_type {
                CompletionType::Factory(value) => match value {
                    CompletionFactory::FieldTypeName => {
                        let sts = module.get_all_interface_refs();
                        for interface_ref in sts {
                            if interface_ref.starts_with(reference_name.as_str()) {
                                nodes.push(CompletionNode::InterfaceName(interface_ref));
                            }
                        }
                    }
                    _ => {}
                },
                CompletionType::Interface(value) => match value {
                    CompletionInterface::FieldTypeName => {
                        let sts = module.get_all_struct_refs();
                        for struct_ref in sts {
                            if struct_ref.starts_with(reference_name.as_str()) {
                                nodes.push(CompletionNode::StructName(struct_ref));
                            }
                        }

                        let nats = module.get_all_native_refs();
                        for nat_ref in nats {
                            if nat_ref.starts_with(reference_name.as_str()) {
                                nodes.push(CompletionNode::NativeName(nat_ref));
                            }
                        }

                        let types = module.get_all_type_refs();
                        for ty_ref in types {
                            if ty_ref.starts_with(reference_name.as_str()) {
                                nodes.push(CompletionNode::ListName(ty_ref));
                            }
                        }

                        let consts = module.get_all_const_refs();
                        for cons_ref in consts {
                            if cons_ref.starts_with(reference_name.as_str()) {
                                nodes.push(CompletionNode::ConstName(cons_ref));
                            }
                        }

                        let enums = module.get_all_enum_refs();
                        for enum_ref in enums {
                            if enum_ref.starts_with(reference_name.as_str()) {
                                nodes.push(CompletionNode::EnumName(enum_ref));
                            }
                        }

                        let streams = module.get_all_stream_refs();
                        for stream_ref in streams {
                            if stream_ref.starts_with(reference_name.as_str()) {
                                nodes.push(CompletionNode::StreamName(stream_ref));
                            }
                        }
                    }
                    _ => {}
                },
                CompletionType::Struct(value) => match value {
                    CompletionStruct::FieldTypeName => {
                        let sts = module.get_all_struct_refs();
                        for struct_ref in sts {
                            if struct_ref.starts_with(reference_name.as_str()) {
                                nodes.push(CompletionNode::StructName(struct_ref));
                            }
                        }

                        let nats = module.get_all_native_refs();
                        for nat_ref in nats {
                            if nat_ref.starts_with(reference_name.as_str()) {
                                nodes.push(CompletionNode::NativeName(nat_ref));
                            }
                        }

                        let types = module.get_all_type_refs();
                        for ty_ref in types {
                            if ty_ref.starts_with(reference_name.as_str()) {
                                nodes.push(CompletionNode::ListName(ty_ref));
                            }
                        }

                        let consts = module.get_all_const_refs();
                        for cons_ref in consts {
                            if cons_ref.starts_with(reference_name.as_str()) {
                                nodes.push(CompletionNode::ConstName(cons_ref));
                            }
                        }

                        let enums = module.get_all_enum_refs();
                        for enum_ref in enums {
                            if enum_ref.starts_with(reference_name.as_str()) {
                                nodes.push(CompletionNode::EnumName(enum_ref));
                            }
                        }
                    }
                    _ => {}
                },
                CompletionType::Stream(value) => match value {
                    CompletionStream::FieldTypeName => {
                        let sts = module.get_all_struct_refs();
                        for struct_ref in sts {
                            if struct_ref.starts_with(reference_name.as_str()) {
                                nodes.push(CompletionNode::StructName(struct_ref));
                            }
                        }

                        let nats = module.get_all_native_refs();
                        for nat_ref in nats {
                            if nat_ref.starts_with(reference_name.as_str()) {
                                nodes.push(CompletionNode::NativeName(nat_ref));
                            }
                        }

                        let types = module.get_all_type_refs();
                        for ty_ref in types {
                            if ty_ref.starts_with(reference_name.as_str()) {
                                nodes.push(CompletionNode::ListName(ty_ref));
                            }
                        }

                        let consts = module.get_all_const_refs();
                        for cons_ref in consts {
                            if cons_ref.starts_with(reference_name.as_str()) {
                                nodes.push(CompletionNode::ConstName(cons_ref));
                            }
                        }

                        let enums = module.get_all_enum_refs();
                        for enum_ref in enums {
                            if enum_ref.starts_with(reference_name.as_str()) {
                                nodes.push(CompletionNode::EnumName(enum_ref));
                            }
                        }
                    }
                    _ => {}
                },
                CompletionType::MapType => nodes.push(CompletionNode::MapType),
                CompletionType::Keyword => nodes.push(CompletionNode::Keyword),
                _ => {}
            }
        }

        return Some(Completion { nodes });
    }

    fn get_location_in_text(text: &str, range: Range) -> Vec<(CompletionType, String)> {
        match parser::Parser::parse(text) {
            Ok(parse) => {
                if let Some(compl) = Self::add_from_parser(&parse, range) {
                    return vec![compl];
                }
            }
            Err((_, err)) => match err {
                parser::ParserError::Enum(value) => {
                    if range.in_range(value.1) {
                        return vec![(CompletionType::Empty, "".to_owned())];
                    }
                }
                parser::ParserError::Interface(value) => {
                    if range.in_range(value.1) {
                        let mut completions = vec![];

                        match value.0 {
                            parser::InterfaceErrorKind::InterfaceField(field) => match field {
                                parser::InterfaceFieldErrorKind::ExpectedType(ident) => {
                                    completions
                                        .push((CompletionInterface::FieldTypeName.into(), ident));
                                }
                                parser::InterfaceFieldErrorKind::TypeTuple(tu_error) => {
                                    match tu_error {
                                        parser::TypeTupleErrorKind::ExpectedType(ex_ty) => {
                                            completions.push((
                                                CompletionInterface::FieldTypeName.into(),
                                                ex_ty.to_owned(),
                                            ));
                                        }
                                        _ => {}
                                    }
                                }
                                parser::InterfaceFieldErrorKind::Type(ty_error) => match ty_error {
                                    parser::TypeErrorKind::InvalidMapType(name) => {
                                        completions.push((CompletionType::MapType, name.to_owned()))
                                    }
                                    _ => {}
                                },
                                _ => {}
                            },
                            _ => {}
                        }

                        if completions.is_empty() {
                            completions.push((CompletionInterface::Empty.into(), "".to_owned()));
                        }

                        return completions;
                    }
                }
                parser::ParserError::Struct(value) => {
                    if range.in_range(value.1) {
                        let mut completions = vec![];

                        match value.0 {
                            parser::StructErrorKind::StructField(field) => match field {
                                parser::StructFieldErrorKind::ExpectedType(ident) => {
                                    completions
                                        .push((CompletionStruct::FieldTypeName.into(), ident));
                                }
                                parser::StructFieldErrorKind::Type(ty_error) => match ty_error {
                                    parser::TypeErrorKind::InvalidMapType(name) => {
                                        completions.push((CompletionType::MapType, name.to_owned()))
                                    }
                                    _ => {}
                                },
                                _ => {}
                            },
                            _ => {}
                        }

                        if completions.is_empty() {
                            completions.push((CompletionStruct::Empty.into(), "".to_owned()));
                        }

                        return completions;
                    }
                }
                parser::ParserError::Stream(value) => {
                    if range.in_range(value.1) {
                        let mut completions = vec![];

                        match value.0 {
                            parser::StreamErrorKind::StructField(field) => match field {
                                parser::StructFieldErrorKind::ExpectedType(ident) => {
                                    completions
                                        .push((CompletionStruct::FieldTypeName.into(), ident));
                                }
                                parser::StructFieldErrorKind::Type(ty_error) => match ty_error {
                                    parser::TypeErrorKind::InvalidMapType(name) => {
                                        completions.push((CompletionType::MapType, name.to_owned()))
                                    }
                                    _ => {}
                                },
                                _ => {}
                            },
                            _ => {}
                        }

                        if completions.is_empty() {
                            completions.push((CompletionStruct::Empty.into(), "".to_owned()));
                        }

                        return completions;
                    }
                }
                parser::ParserError::Library(value) => {}
                parser::ParserError::Imports(value) => {}
                parser::ParserError::TypeList(value) => {
                    if range.in_range(value.1) {
                        let mut completions = vec![];

                        if completions.is_empty() {
                            completions.push((CompletionStruct::Empty.into(), "".to_owned()));
                        }

                        return completions;
                    }
                }
                parser::ParserError::Const(value) => {
                    if range.in_range(value.1) {
                        return vec![(CompletionType::Empty, "".to_owned())];
                    }
                }
                parser::ParserError::Factory(value) => {
                    // TODO
                    if range.in_range(value.1) {
                        let mut completions = vec![];

                        match value.0 {
                            parser::FactoryErrorKind::InterfaceField(field) => match field {
                                parser::InterfaceFieldErrorKind::ExpectedType(ident) => {
                                    completions
                                        .push((CompletionFactory::FieldTypeName.into(), ident));
                                }
                                parser::InterfaceFieldErrorKind::TypeTuple(tu_error) => {
                                    match tu_error {
                                        parser::TypeTupleErrorKind::ExpectedType(ex_ty) => {
                                            completions.push((
                                                CompletionFactory::FieldTypeName.into(),
                                                ex_ty.to_owned(),
                                            ));
                                        }
                                        _ => {}
                                    }
                                }
                                parser::InterfaceFieldErrorKind::Type(ty_error) => match ty_error {
                                    parser::TypeErrorKind::InvalidMapType(name) => {
                                        completions.push((CompletionType::MapType, name.to_owned()))
                                    }
                                    _ => {}
                                },
                                _ => {}
                            },
                            _ => {}
                        }

                        if completions.is_empty() {
                            completions.push((CompletionFactory::Empty.into(), "".to_owned()));
                        }

                        return completions;
                    }
                }
                _ => {
                    return vec![(CompletionType::Keyword, "".to_owned())];
                }
            },
        }

        vec![]
    }

    fn add_from_parser(parse: &parser::Parser, range: Range) -> Option<(CompletionType, String)> {
        for node in &parse.nodes {
            match node {
                parser::ParserNode::Interface(value) => {
                    if range.in_range(value.range) {
                        let mut completion = None;

                        for field_node in &value.fields {
                            if let parser::InterfaceNode::InterfaceField(field) = field_node {
                                if let Some((name, t_range)) = match &*field.ty {
                                    parser::Type::Name(name) => {
                                        Some((name.ident.to_owned(), name.range))
                                    }
                                    parser::Type::Native(native) => {
                                        Some((native.to_string(), native.range))
                                    }
                                    _ => None,
                                } {
                                    if range.in_range(t_range) {
                                        completion =
                                            Some((CompletionInterface::FieldTypeName.into(), name));

                                        return completion;
                                    }
                                }

                                match &*field.ty {
                                    parser::Type::Function(function) => {
                                        if let parser::Type::Tuple(tuple) = &*function.args {
                                            for tuple_field in &tuple.ty_list {
                                                let t_range = tuple_field.ty.get_range();
                                                if range.in_range(t_range) {
                                                    completion = Some((
                                                        CompletionInterface::FieldTypeName.into(),
                                                        tuple_field.ty.to_string(),
                                                    ));

                                                    return completion;
                                                }
                                            }
                                        }

                                        if range.in_range(function.ret_ty.get_range()) {
                                            return Some((
                                                CompletionInterface::FieldTypeName.into(),
                                                function.ret_ty.to_string(),
                                            ));
                                        }
                                    }
                                    parser::Type::Tuple(tuple) => {
                                        for tuple_field in &tuple.ty_list {
                                            let t_range = tuple_field.ty.get_range();
                                            if range.in_range(t_range) {
                                                completion = Some((
                                                    CompletionInterface::FieldTypeName.into(),
                                                    tuple_field.ty.to_string(),
                                                ));

                                                return completion;
                                            }
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }

                        if completion.is_none() {
                            completion = Some((
                                CompletionInterface::Empty.into(),
                                "".to_owned(),
                            ));
                        }

                        return completion;
                    }
                }
                parser::ParserNode::Struct(value) => {
                    if range.in_range(value.range) {
                        let mut completion = None;

                        for field_node in &value.fields {
                            if let parser::StructNode::StructField(field) = field_node {
                                if let Some((name, t_range)) = match &*field.ty {
                                    parser::Type::Name(name) => {
                                        Some((name.ident.to_owned(), name.range))
                                    }
                                    parser::Type::Native(native) => {
                                        Some((native.to_string(), native.range))
                                    }
                                    _ => None,
                                } {
                                    if range.in_range(t_range) {
                                        completion =
                                            Some((CompletionStruct::FieldTypeName.into(), name));
                                    }
                                }
                            }
                        }

                        if completion.is_none() {
                            completion = Some((CompletionStruct::Empty.into(), "".to_owned()));
                        }

                        return completion;
                    }
                }
                parser::ParserNode::Stream(value) => {
                    if range.in_range(value.range) {
                        let mut completion = None;

                        for field_node in &value.fields {
                            if let parser::StructNode::StructField(field) = field_node {
                                if let Some((name, t_range)) = match &*field.ty {
                                    parser::Type::Name(name) => {
                                        Some((name.ident.to_owned(), name.range))
                                    }
                                    parser::Type::Native(native) => {
                                        Some((native.to_string(), native.range))
                                    }
                                    _ => None,
                                } {
                                    if range.in_range(t_range) {
                                        completion =
                                            Some((CompletionStream::FieldTypeName.into(), name));
                                    }
                                }
                            }
                        }

                        if completion.is_none() {
                            completion = Some((CompletionStream::Empty.into(), "".to_owned()));
                        }

                        return completion;
                    }
                }
                parser::ParserNode::Enum(value) => {
                    if range.in_range(value.range) {
                        return Some((CompletionType::Empty, "".to_owned()));
                    }
                }
                parser::ParserNode::TypeList(value) => {
                    if range.in_range(value.range) {
                        return Some((CompletionType::Empty, "".to_owned()));
                    }
                }
                parser::ParserNode::Const(value) => {
                    if range.in_range(value.range) {
                        return Some((CompletionType::Empty, "".to_owned()));
                    }
                }
                parser::ParserNode::Factory(value) => {
                    if range.in_range(value.range) {
                        if range.in_range(value.range) {
                            let mut completion = None;

                            for field_node in &value.fields {
                                if let parser::InterfaceNode::InterfaceField(field) = field_node {
                                    if let Some((name, t_range)) = match &*field.ty {
                                        parser::Type::Name(name) => {
                                            Some((name.ident.to_owned(), name.range))
                                        }
                                        parser::Type::Native(native) => {
                                            Some((native.to_string(), native.range))
                                        }
                                        _ => None,
                                    } {
                                        if range.in_range(t_range) {
                                            completion = Some((
                                                CompletionFactory::FieldTypeName.into(),
                                                name,
                                            ));
                                        }
                                    }
                                }
                            }

                            if completion.is_none() {
                                completion = Some((CompletionFactory::Empty.into(), "".to_owned()));
                            }

                            return completion;
                        }
                    }
                }
                parser::ParserNode::Stream(value) => {
                    if range.in_range(value.range) {
                        return Some((CompletionType::Empty, "".to_owned()));
                    }
                }
                parser::ParserNode::Library(_) => {
                    // TODO Range in library.
                }
                parser::ParserNode::Imports(_) => {
                    // TODO Range in imports.
                }
                parser::ParserNode::Comment(value)
                | parser::ParserNode::InterfaceComment(value)
                | parser::ParserNode::TypeListComment(value)
                | parser::ParserNode::StructComment(value)
                | parser::ParserNode::ConstComment(value)
                | parser::ParserNode::EnumComment(value)
                | parser::ParserNode::FactoryComment(value)
                | parser::ParserNode::StreamComment(value) => {
                    // TODO Range in comments.
                }
            }
        }

        Some((CompletionType::Keyword, "".to_owned()))
    }
}
