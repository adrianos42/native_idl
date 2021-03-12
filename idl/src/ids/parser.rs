use crate::range::{Position, Range};
use crate::scanner::{ContextStream, WordStream};
use std::{fmt, sync::Arc};
use thiserror::Error;

pub use crate::parser::{Identifier, Keywords, ScannerError, TypeName};

#[derive(Debug)]
pub enum ParserNode {
    Package(Arc<Item>),
    Server(Arc<Item>),
    Client(Arc<Item>),
}

#[derive(Debug)]
pub enum ItemIdent {
    TypeName(TypeName),     // Type declaration
    Identifier(Identifier), // Usually, only the library declarion
}

impl fmt::Display for ItemIdent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ItemIdent::TypeName(name) => &name.ident,
                ItemIdent::Identifier(name) => &name.ident,
            }
        )
    }
}

#[derive(Debug)]
pub struct Item {
    pub ident: ItemIdent,
    pub nodes: Vec<ItemNode>,
    pub range: Range,
}

#[derive(Debug)]
pub enum ItemNode {
    ItemField(ItemField),
    Comment(Vec<String>),
}

#[derive(Debug)]
pub enum ItemType {
    Int(String),
    Float(String),
    String(String),
    Boolean(String),
    TypeName(TypeName),
    Identifier(Identifier),
    Values(Vec<ItemType>),
}

#[derive(Debug)]
pub struct ItemField {
    pub ident: String,
    pub value: ItemType,
    pub range: Range,
}

impl fmt::Display for ItemField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.ident.as_str(), self.value,)
    }
}

impl fmt::Display for ItemType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ItemType::TypeName(value) => value.ident.to_owned(),
                ItemType::String(value) => format!("\"{}\"", value.as_str()),
                ItemType::Int(value) | ItemType::Float(value) | ItemType::Boolean(value) =>
                    value.to_owned(),
                ItemType::Values(values) => format!(
                    "[{}]",
                    values.iter().fold("".to_owned(), |p, v| if p.is_empty() {
                        p
                    } else {
                        p + ", "
                    } + &v.to_string())
                ),
                ItemType::Identifier(value) => value.ident.to_owned(),
            }
        )
    }
}

#[derive(Error, Debug, Clone)]
pub struct ItemError(pub ItemErrorKind, pub Range);

impl From<ItemFieldError> for ItemError {
    fn from(value: ItemFieldError) -> Self {
        ItemError(ItemErrorKind::ItemField(value.0), value.1)
    }
}

impl fmt::Display for ItemError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for ItemErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let errstr = match self {
            ItemErrorKind::Undefined => "Item error".to_owned(),
            ItemErrorKind::ItemField(field) => field.to_string(),
            ItemErrorKind::TypeDeclaration => "Type declaration".to_owned(),
            ItemErrorKind::MissingTypeName => "Missing type name".to_owned(),
        };

        write!(f, "{}", errstr)
    }
}

#[derive(Debug, Clone)]
pub enum ItemErrorKind {
    Undefined,
    MissingTypeName,
    TypeDeclaration,
    ItemField(ItemFieldErrorKind),
}

#[derive(Debug, Eq, PartialEq)]
enum ItemFieldParsing {
    ExpectingIdentifier,
    ExpectingColon,
    ExpectingValue,
    Value,
}

#[derive(Error, Debug, Clone)]
pub struct ItemFieldError(pub ItemFieldErrorKind, pub Range);

impl fmt::Display for ItemFieldError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for ItemFieldErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let errstr = match self {
            ItemFieldErrorKind::Undefined => "Item field error",
            ItemFieldErrorKind::ItemTypeMustBeUnique => "Const type must be unique",
            ItemFieldErrorKind::EmptyBody => "Empty body",
            ItemFieldErrorKind::IncompleteField => "Incomplete field",
            ItemFieldErrorKind::InvalidSymbol => "Invalid symbol",
            ItemFieldErrorKind::MissingAssignment => "Missing `=`",
            ItemFieldErrorKind::MissingComma => "Missing `,`",
            ItemFieldErrorKind::MissingCurlyBracket => "Missing `}`",
            ItemFieldErrorKind::MissingIdentifier => "Missing identifier",
            ItemFieldErrorKind::MultipleIdentifiers => "Multiple identifier",
            ItemFieldErrorKind::MissingSquareBracket => "Missing `]`",
        };

        write!(f, "{}", errstr)
    }
}

#[derive(Debug, Clone)]
pub enum ItemFieldErrorKind {
    Undefined,
    EmptyBody,
    IncompleteField,
    ItemTypeMustBeUnique,
    MultipleIdentifiers,
    InvalidSymbol,
    MissingIdentifier,
    MissingAssignment,
    MissingComma,
    MissingCurlyBracket,
    MissingSquareBracket,
}

#[derive(Error, Debug, Clone)]
pub enum ParserError {
    #[error("Undefined `{0}`")]
    Undefined(String, Range),
    #[error("Closed")]
    Closed,
    #[error(transparent)]
    Text(#[from] ScannerError),
    #[error(transparent)]
    Item(#[from] ItemError),
}

impl ParserError {
    pub fn get_message_with_range(&self) -> (String, Range) {
        match self {
            ParserError::Closed => (self.to_string(), Range::default()),
            ParserError::Undefined(_, range) => (self.to_string(), *range),
            ParserError::Text(ScannerError(kind, range)) => (kind.to_string(), *range),
            ParserError::Item(ItemError(kind, range)) => (kind.to_string(), *range),
        }
    }

    pub fn get_range(&self) -> Range {
        match self {
            ParserError::Closed => Range::default(),
            ParserError::Undefined(_, range) => *range,
            ParserError::Text(ScannerError(_, range)) => *range,
            ParserError::Item(ItemError(_, range)) => *range,
        }
    }
}

#[derive(Debug, Default)]
pub struct Parser {
    pub nodes: Vec<ParserNode>,
}

impl Parser {
    pub fn closed() -> Result<Self, (Self, ParserError)> {
        Err((Self::default(), ParserError::Closed))
    }

    pub fn package_name(&self) -> Option<String> {
        for node in &self.nodes {
            match node {
                ParserNode::Package(name) => match &name.ident {
                    ItemIdent::Identifier(value) => return Some(value.ident.to_owned()),
                    _ => {}
                },
                _ => {}
            }
        }
        None
    }

    pub fn parse(text: &str) -> Result<Self, (Self, ParserError)> {
        if text.is_empty() {
            return Self::closed();
        }

        let lines: Vec<&str> = text.lines().collect();

        let mut context = Self::default();

        let source_stream = match ContextStream::scan_text(&lines) {
            Ok(value) => value,
            Err(err) => return Err((context, ScannerError::from(err).into())),
        };

        let mut word_stream = source_stream.word_streams.iter();
        //let mut comments = vec![];

        while let Some(w_stream) = word_stream.next() {
            match w_stream {
                WordStream::Keyword(keyword) => {
                    let start_position = keyword.range.as_position();

                    match &keyword.get_word() {
                        Keywords::Server => {
                            match Self::consume_item(&mut word_stream, start_position) {
                                Ok(value) => {
                                    context.nodes.push(ParserNode::Server(Arc::new(value)))
                                }
                                Err(err) => return Err((context, err.into())),
                            }
                        }
                        Keywords::Package => {
                            match Self::consume_item(&mut word_stream, start_position) {
                                Ok(value) => {
                                    context.nodes.push(ParserNode::Package(Arc::new(value)))
                                }
                                Err(err) => return Err((context, err.into())),
                            }
                        }
                        Keywords::Client => {
                            match Self::consume_item(&mut word_stream, start_position) {
                                Ok(value) => {
                                    context.nodes.push(ParserNode::Client(Arc::new(value)))
                                }
                                Err(err) => return Err((context, err.into())),
                            }
                        }
                        _ => {
                            return Err((
                                context,
                                ParserError::Undefined(
                                    keyword.get_word().to_string(),
                                    keyword.range,
                                ),
                            ))
                        }
                    }
                }
                WordStream::Comment(_comment) => {}
                WordStream::NewLine(_) => {}
                sw => {
                    return Err((
                        context,
                        ParserError::Undefined(sw.to_string(), sw.get_range()),
                    ))
                }
            }
        }

        Ok(context)
    }

    fn consume_item<'a, I>(word_stream: &mut I, start_position: Position) -> Result<Item, ItemError>
    where
        I: Iterator<Item = &'a WordStream>,
    {
        let mut last_range = start_position.as_range();
        let mut item_name = None;

        while let Some(w_stream) = word_stream.next() {
            match w_stream {
                WordStream::LeftCurlyBracket(lc) => match item_name {
                    Some(ident) => {
                        let (nodes, end_position) = Self::push_item_fields(word_stream)?;

                        return Ok(Item {
                            ident,
                            nodes,
                            range: Range {
                                start: start_position,
                                end: end_position,
                            },
                        });
                    }
                    None => {
                        return Err(ItemError(
                            ItemErrorKind::TypeDeclaration,
                            last_range.merge(lc.range),
                        ))
                    }
                },
                WordStream::TypeName(name) => {
                    let name_range = name.range;
                    last_range = name_range.end_as_range();

                    if item_name.is_some() {
                        return Err(ItemError(ItemErrorKind::TypeDeclaration, name_range));
                    }

                    item_name = Some(ItemIdent::TypeName(TypeName {
                        ident: name.get_word().to_owned(),
                        range: name_range,
                    }));
                }
                WordStream::Identifier(name) => {
                    let name_range = name.range;
                    last_range = name_range.end_as_range();

                    if item_name.is_some() {
                        return Err(ItemError(ItemErrorKind::TypeDeclaration, name_range));
                    }

                    item_name = Some(ItemIdent::Identifier(Identifier {
                        ident: name.get_word().to_owned(),
                        range: name_range,
                    }));
                }
                sw => {
                    return Err(ItemError(
                        ItemErrorKind::MissingTypeName,
                        last_range.merge(sw.get_range()),
                    ))
                }
            }
        }

        Err(ItemError(ItemErrorKind::Undefined, last_range))
    }

    fn push_item_fields<'a, I>(
        word_stream: &mut I,
    ) -> Result<(Vec<ItemNode>, Position), ItemFieldError>
    where
        I: Iterator<Item = &'a WordStream>,
    {
        let mut fields = vec![];
        let mut parsing = ItemFieldParsing::ExpectingIdentifier;
        let mut field_name: Option<String> = None;
        let mut item_type: Option<ItemType> = None;
        let mut field_range = Range::default();
        let mut last_range = Range::default();
        let mut comments = vec![];

        while let Some(n_stream) = word_stream.next() {
            match n_stream {
                WordStream::CurlyBracketBody(curly_body) => {
                    let mut curly_word_stream = curly_body.iter();

                    while let Some(w_stream) = curly_word_stream.next() {
                        if let WordStream::Comment(comment) = w_stream {
                            let range = comment.range;
                            let comment_txt = comment.get_word().to_owned();
                            match parsing {
                                ItemFieldParsing::ExpectingIdentifier => {
                                    last_range = range.end_as_range();
                                    comments.push(comment_txt);

                                    if let Some(c_stream) = word_stream.next() {
                                        match c_stream {
                                            WordStream::NewLine(_) => {}
                                            _ => {
                                                return Err(ItemFieldError(
                                                    ItemFieldErrorKind::Undefined,
                                                    last_range,
                                                ))
                                            }
                                        }
                                    }
                                }
                                _ => {
                                    return Err(ItemFieldError(
                                        ItemFieldErrorKind::IncompleteField,
                                        last_range.merge(range),
                                    ))
                                }
                            }

                            continue;
                        }

                        if !comments.is_empty() {
                            fields.push(ItemNode::Comment(comments));
                            comments = vec![];
                        }

                        match w_stream {
                            WordStream::Comma(cm) => {
                                let range = cm.range;

                                match parsing {
                                    ItemFieldParsing::Value => {
                                        fields.push(ItemNode::ItemField(ItemField {
                                            ident: field_name.take().unwrap(),
                                            value: item_type.take().unwrap(),
                                            range: field_range,
                                        }));

                                        field_range = Range::default();
                                        last_range = range.end_as_range();
                                        parsing = ItemFieldParsing::ExpectingIdentifier;
                                    }
                                    _ => {
                                        return Err(ItemFieldError(
                                            ItemFieldErrorKind::IncompleteField,
                                            last_range.merge(range),
                                        ))
                                    }
                                }
                            }
                            WordStream::Identifier(ident) => {
                                let range = ident.range;

                                parsing = match parsing {
                                    ItemFieldParsing::ExpectingValue => {
                                        item_type = Some(ItemType::Identifier(Identifier {
                                            ident: ident.get_word().to_owned(),
                                            range,
                                        }));
                                        field_range = field_range.merge(range);
                                        last_range = range.end_as_range();
                                        ItemFieldParsing::Value
                                    }
                                    ItemFieldParsing::ExpectingIdentifier => {
                                        field_name = Some(ident.get_word().to_owned());
                                        field_range = range;
                                        last_range = field_range.end_as_range();
                                        ItemFieldParsing::ExpectingColon
                                    }
                                    ItemFieldParsing::ExpectingColon => {
                                        return Err(ItemFieldError(
                                            ItemFieldErrorKind::MultipleIdentifiers,
                                            ident.range,
                                        ))
                                    }
                                    _ => {
                                        return Err(ItemFieldError(
                                            ItemFieldErrorKind::MissingComma,
                                            last_range.merge(ident.range),
                                        ))
                                    }
                                }
                            }
                            WordStream::Colon(ag) => {
                                let range = ag.range;
                                parsing = match parsing {
                                    ItemFieldParsing::ExpectingColon => {
                                        field_range = field_range.merge(range);
                                        last_range = field_range.end_as_range();
                                        ItemFieldParsing::ExpectingValue
                                    }
                                    _ => {
                                        return Err(ItemFieldError(
                                            ItemFieldErrorKind::MissingIdentifier,
                                            last_range.merge(range),
                                        ))
                                    }
                                }
                            }
                            WordStream::LeftSquareBracket(lq) => {
                                let range = lq.range;

                                parsing = match parsing {
                                    ItemFieldParsing::ExpectingValue => {
                                        let position = range.as_position();
                                        let (items, range) = Self::push_item_field_values(
                                            &mut curly_word_stream,
                                            position,
                                        )?;

                                        item_type = Some(items);
                                        field_range = field_range.merge(range);
                                        last_range = range.end_as_range();

                                        ItemFieldParsing::Value
                                    }
                                    ItemFieldParsing::Value => {
                                        return Err(ItemFieldError(
                                            ItemFieldErrorKind::IncompleteField,
                                            last_range.merge(range),
                                        ));
                                    }
                                    ItemFieldParsing::ExpectingIdentifier => {
                                        return Err(ItemFieldError(
                                            ItemFieldErrorKind::MissingIdentifier,
                                            last_range.merge(range),
                                        ));
                                    }
                                    ItemFieldParsing::ExpectingColon => {
                                        return Err(ItemFieldError(
                                            ItemFieldErrorKind::MissingAssignment,
                                            last_range.merge(range),
                                        ));
                                    }
                                }
                            }
                            WordStream::TypeName(type_name) => {
                                let range = type_name.range;

                                parsing = match parsing {
                                    ItemFieldParsing::ExpectingValue => {
                                        item_type = Some(ItemType::TypeName(TypeName {
                                            ident: type_name.get_word().to_owned(),
                                            range,
                                        }));
                                        field_range = field_range.merge(range);
                                        last_range = range.end_as_range();

                                        ItemFieldParsing::Value
                                    }
                                    ItemFieldParsing::Value => {
                                        return Err(ItemFieldError(
                                            ItemFieldErrorKind::IncompleteField,
                                            last_range.merge(range),
                                        ));
                                    }
                                    ItemFieldParsing::ExpectingIdentifier => {
                                        return Err(ItemFieldError(
                                            ItemFieldErrorKind::MissingIdentifier,
                                            last_range.merge(range),
                                        ));
                                    }
                                    ItemFieldParsing::ExpectingColon => {
                                        return Err(ItemFieldError(
                                            ItemFieldErrorKind::MissingAssignment,
                                            last_range.merge(range),
                                        ));
                                    }
                                }
                            }
                            WordStream::Literal(s_body)
                            | WordStream::FloatValue(s_body)
                            | WordStream::IntegerValue(s_body)
                            | WordStream::BooleanValue(s_body) => {
                                let f_value = s_body.get_word().to_owned();
                                let range = s_body.range;

                                item_type = match w_stream {
                                    WordStream::Literal(_) => Some(ItemType::String(f_value)),
                                    WordStream::FloatValue(_) => Some(ItemType::Float(f_value)),
                                    WordStream::IntegerValue(_) => Some(ItemType::Int(f_value)),
                                    WordStream::BooleanValue(_) => Some(ItemType::Boolean(f_value)),
                                    _ => {
                                        return Err(ItemFieldError(
                                            ItemFieldErrorKind::Undefined,
                                            last_range.merge(range),
                                        ))
                                    }
                                };

                                parsing = match parsing {
                                    ItemFieldParsing::ExpectingValue => {
                                        field_range = field_range.merge(range);
                                        last_range = range.end_as_range();

                                        ItemFieldParsing::Value
                                    }
                                    ItemFieldParsing::Value => {
                                        return Err(ItemFieldError(
                                            ItemFieldErrorKind::IncompleteField,
                                            last_range.merge(range),
                                        ));
                                    }
                                    ItemFieldParsing::ExpectingIdentifier => {
                                        return Err(ItemFieldError(
                                            ItemFieldErrorKind::MissingIdentifier,
                                            last_range.merge(range),
                                        ));
                                    }
                                    ItemFieldParsing::ExpectingColon => {
                                        return Err(ItemFieldError(
                                            ItemFieldErrorKind::MissingAssignment,
                                            last_range.merge(range),
                                        ));
                                    }
                                }
                            }
                            WordStream::NewLine(_) => {}
                            sw => {
                                return Err(ItemFieldError(
                                    ItemFieldErrorKind::InvalidSymbol,
                                    last_range.merge(sw.get_range()),
                                ));
                            }
                        }
                    }
                }
                WordStream::RightCurlyBracket(rc) => {
                    let end_position = rc.range.as_position();
                    let range = end_position.as_range();

                    match parsing {
                        ItemFieldParsing::Value => {
                            fields.push(ItemNode::ItemField(ItemField {
                                ident: field_name.unwrap(),
                                value: item_type.unwrap(),
                                range: field_range,
                            }));
                            return Ok((fields, end_position));
                        }
                        ItemFieldParsing::ExpectingIdentifier => {
                            if fields.iter().any(|value| match value {
                                ItemNode::ItemField(_) => true,
                                _ => false,
                            }) {
                                return Ok((fields, end_position));
                            }
                            return Err(ItemFieldError(
                                ItemFieldErrorKind::EmptyBody,
                                last_range.merge(range),
                            ));
                        }
                        _ => {
                            return Err(ItemFieldError(
                                ItemFieldErrorKind::IncompleteField,
                                last_range.merge(range),
                            ))
                        }
                    }
                }
                WordStream::NewLine(_) => {}
                sw => {
                    return Err(ItemFieldError(
                        ItemFieldErrorKind::MissingCurlyBracket,
                        last_range.merge(sw.get_range()),
                    ))
                }
            }
        }

        Err(ItemFieldError(ItemFieldErrorKind::Undefined, last_range))
    }

    fn push_item_field_values<'a, I>(
        word_stream: &mut I,
        start_position: Position,
    ) -> Result<(ItemType, Range), ItemFieldError>
    where
        I: Iterator<Item = &'a WordStream>,
    {
        let mut fields = vec![];

        let mut last_range = Range::from_position(start_position);
        let mut expecting_value = true;
        let mut item_type: Option<ItemType> = None;

        while let Some(wb_stream) = word_stream.next() {
            match wb_stream {
                WordStream::SquareBracketBody(sq_stream) => {
                    let mut s_stream = sq_stream.iter();
                    while let Some(w_stream) = s_stream.next() {
                        match w_stream {
                            WordStream::Comma(cm) => {
                                last_range = last_range.merge(cm.range);
                                if expecting_value {
                                    return Err(ItemFieldError(
                                        ItemFieldErrorKind::InvalidSymbol,
                                        last_range.merge(cm.range),
                                    ));
                                }
                                fields.push(item_type.take().unwrap());
                                expecting_value = true;
                            }
                            WordStream::TypeName(type_name) => {
                                let range = type_name.range;

                                if !expecting_value {
                                    return Err(ItemFieldError(
                                        ItemFieldErrorKind::MultipleIdentifiers,
                                        last_range.merge(range),
                                    ));
                                }

                                item_type = Some(ItemType::TypeName(TypeName {
                                    ident: type_name.get_word().to_owned(),
                                    range,
                                }));

                                expecting_value = false;
                            }
                            WordStream::Identifier(ident) => {
                                let range = ident.range;

                                if !expecting_value {
                                    return Err(ItemFieldError(
                                        ItemFieldErrorKind::MultipleIdentifiers,
                                        last_range.merge(range),
                                    ));
                                }

                                item_type = Some(ItemType::Identifier(Identifier {
                                    ident: ident.get_word().to_owned(),
                                    range,
                                }));

                                expecting_value = false;
                            }
                            WordStream::Literal(s_body)
                            | WordStream::FloatValue(s_body)
                            | WordStream::IntegerValue(s_body)
                            | WordStream::BooleanValue(s_body) => {
                                let range = s_body.range;

                                if !expecting_value {
                                    return Err(ItemFieldError(
                                        ItemFieldErrorKind::MultipleIdentifiers,
                                        last_range.merge(range),
                                    ));
                                }

                                let f_value = s_body.get_word().to_owned();

                                item_type = match w_stream {
                                    WordStream::Literal(_) => Some(ItemType::String(f_value)),
                                    WordStream::FloatValue(_) => Some(ItemType::Float(f_value)),
                                    WordStream::IntegerValue(_) => Some(ItemType::Int(f_value)),
                                    WordStream::BooleanValue(_) => Some(ItemType::Boolean(f_value)),
                                    _ => {
                                        return Err(ItemFieldError(
                                            ItemFieldErrorKind::Undefined,
                                            last_range.merge(range),
                                        ))
                                    }
                                };

                                expecting_value = false;
                            }
                            sw => {
                                return Err(ItemFieldError(
                                    ItemFieldErrorKind::InvalidSymbol,
                                    sw.get_range(),
                                ));
                            }
                        }
                    }
                }
                WordStream::RightSquareBracket(rq) => {
                    if let Some(item) = item_type.take() {
                        fields.push(item);
                    }

                    return Ok((
                        ItemType::Values(fields),
                        Range {
                            start: start_position,
                            end: rq.range.as_position(),
                        },
                    ));
                }
                sw => {
                    return Err(ItemFieldError(
                        ItemFieldErrorKind::MissingSquareBracket,
                        sw.get_range(),
                    ))
                }
            }
        }

        Err(ItemFieldError(
            ItemFieldErrorKind::Undefined,
            Range::default(),
        ))
    }
}
