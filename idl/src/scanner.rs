use regex::Regex;
use std::fmt;

use super::range::{Position, Range};

static KEYWORDS: &'static [&str] = &[
    "enum",
    "struct",
    "interface",
    "import",
    "library",
    "static",
    "stream",
    "type",
    "const",
    "layer",
    "server",
    "client",
    "map",
    "result",
    "option",
];

static BOOLEAN_VALUES: &'static [&str] = &["false", "true"];

static NATIVE_TYPES: &'static [&str] = &["int", "float", "bool", "string", "bytes", "none"];

static ATTRIBUTES_NAMES: &'static [&str] = &["deprecated"];

#[derive(Debug, Copy, Clone)]
pub enum Keywords {
    Enum,
    Struct,
    Interface,
    Import,
    Library,
    Type,
    Const,
    Static,
    Stream,
    Layer,
    Server,
    Client,
    Map,
    Result,
    Option,
    Set,
}

impl From<&str> for Keywords {
    fn from(value: &str) -> Self {
        match value {
            "enum" => Keywords::Enum,
            "struct" => Keywords::Struct,
            "interface" => Keywords::Interface,
            "import" => Keywords::Import,
            "library" => Keywords::Library,
            "static" => Keywords::Static,
            "stream" => Keywords::Stream,
            "type" => Keywords::Type,
            "const" => Keywords::Const,
            "layer" => Keywords::Layer,
            "server" => Keywords::Server,
            "client" => Keywords::Client,
            "map" => Keywords::Map,
            "result" => Keywords::Result,
            "option" => Keywords::Option,
            "set" => Keywords::Set,
            _ => panic!(),
        }
    }
}

impl fmt::Display for Keywords {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Keywords::Enum => "enum",
            Keywords::Struct => "struct",
            Keywords::Interface => "interface",
            Keywords::Import => "import",
            Keywords::Library => "library",
            Keywords::Type => "type",
            Keywords::Const => "const",
            Keywords::Static => "static",
            Keywords::Stream => "stream",
            Keywords::Layer => "layer",
            Keywords::Server => "server",
            Keywords::Client => "client",
            Keywords::Result => "result",
            Keywords::Option => "option",
            Keywords::Map => "map",
            Keywords::Set => "set",
        };

        write!(f, "{}", name)
    }
}

#[derive(Debug, Copy, Clone)]
pub enum NativeTypes {
    Int,
    Float,
    String,
    Bytes,
    Bool,
    None,
}

impl From<&str> for NativeTypes {
    fn from(value: &str) -> Self {
        match value {
            "int" => NativeTypes::Int,
            "float"  => NativeTypes::Float,
            "string"  => NativeTypes::String,
            "bytes" => NativeTypes::Bytes,
            "bool" => NativeTypes::Bool,
            "none" => NativeTypes::None,
            _ => panic!(),
        }
    }
}

impl fmt::Display for NativeTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            NativeTypes::Int => "int",
            NativeTypes::Float => "float",
            NativeTypes::String => "string",
            NativeTypes::Bytes => "bytes",
            NativeTypes::Bool => "bool",
            NativeTypes::None => "none",
        };

        write!(f, "{}", name)
    }
}

#[derive(Debug, Copy, Clone)]
pub enum AttributeNames {
    Deprecated,
}

impl fmt::Display for AttributeNames {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            AttributeNames::Deprecated => "deprecated",
        };

        write!(f, "{}", name)
    }
}

#[derive(Debug)]
pub(super) struct WordRange<T> {
    pub(super) range: Range,
    word: T,
}

impl Range {
    fn new_with_length(line: usize, index: usize, length: usize) -> Self {
        Self {
            start: Position {
                line,
                column: index,
            },
            end: Position {
                line,
                column: index + length,
            },
        }
    }
}

impl<T> WordRange<T> {
    pub(super) fn get_word(&self) -> &T {
        &self.word
    }
}

#[derive(Debug)]
pub(super) enum WordStream {
    LeftSquareBracket(WordRange<char>),
    RightSquareBracket(WordRange<char>),
    SquareBracketBody(Vec<WordStream>),
    LeftCurlyBracket(WordRange<char>),
    RightCurlyBracket(WordRange<char>),
    CurlyBracketBody(Vec<WordStream>),
    LeftParenthesis(WordRange<char>),
    RightParenthesis(WordRange<char>),
    ParenthesisBody(Vec<WordStream>),
    Comma(WordRange<char>),
    Assignment(WordRange<char>),
    Colon(WordRange<char>),
    Hyphen(WordRange<char>),
    GreaterThan(WordRange<char>),
    NewLine(WordRange<char>),
    SemiColon(WordRange<char>),
    Identifier(WordRange<String>),
    Keyword(WordRange<Keywords>),
    NativeType(WordRange<NativeTypes>),
    Attribute(WordRange<AttributeNames>),
    TypeName(WordRange<String>),
    Comment(WordRange<String>),
    Literal(WordRange<String>),
    FloatValue(WordRange<String>),
    IntegerValue(WordRange<String>),
    BooleanValue(WordRange<String>),
}

impl WordStream {
    pub(super) fn get_range(&self) -> Range {
        match self {
            WordStream::LeftCurlyBracket(value)
            | WordStream::RightSquareBracket(value)
            | WordStream::LeftSquareBracket(value)
            | WordStream::RightCurlyBracket(value)
            | WordStream::LeftParenthesis(value)
            | WordStream::RightParenthesis(value)
            | WordStream::Comma(value)
            | WordStream::Assignment(value)
            | WordStream::Colon(value)
            | WordStream::NewLine(value)
            | WordStream::Hyphen(value)
            | WordStream::GreaterThan(value)
            | WordStream::SemiColon(value) => value.range,
            WordStream::Identifier(value)
            | WordStream::TypeName(value)
            | WordStream::Comment(value)
            | WordStream::Literal(value)
            | WordStream::FloatValue(value)
            | WordStream::BooleanValue(value)
            | WordStream::IntegerValue(value) => value.range,
            WordStream::Keyword(value) => value.range,
            WordStream::NativeType(value) => value.range,
            WordStream::Attribute(value) => value.range,
            WordStream::CurlyBracketBody(w_streams)
            | WordStream::ParenthesisBody(w_streams)
            | WordStream::SquareBracketBody(w_streams) => match w_streams.first() {
                Some(value) => value.get_range(),
                None => Range::default(),
            },
        }
    }
}

impl fmt::Display for WordStream {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let strname = match self {
            WordStream::LeftCurlyBracket(name)
            | WordStream::RightSquareBracket(name)
            | WordStream::LeftSquareBracket(name)
            | WordStream::RightCurlyBracket(name)
            | WordStream::LeftParenthesis(name)
            | WordStream::RightParenthesis(name)
            | WordStream::Comma(name)
            | WordStream::Assignment(name)
            | WordStream::Colon(name)
            | WordStream::Hyphen(name)
            | WordStream::GreaterThan(name)
            | WordStream::SemiColon(name) => name.get_word().to_string(),
            WordStream::Identifier(name)
            | WordStream::TypeName(name)
            | WordStream::Comment(name)
            | WordStream::Literal(name)
            | WordStream::FloatValue(name)
            | WordStream::BooleanValue(name)
            | WordStream::IntegerValue(name) => name.get_word().to_owned(),
            WordStream::Keyword(name) => name.get_word().to_string(),
            WordStream::NativeType(name) => name.get_word().to_string(),
            WordStream::Attribute(name) => name.get_word().to_string(),
            WordStream::NewLine(_) => "".to_owned(),
            WordStream::CurlyBracketBody(_)
            | WordStream::ParenthesisBody(_)
            | WordStream::SquareBracketBody(_) => "".to_owned(),
        };

        write!(f, "{}", strname)
    }
}

#[derive(Debug)]
pub(super) enum ScError {
    SymbolMissing(WordStream),
    Name(WordStream),
    InvalidCharacter(WordRange<char>),
    InvalidComment(WordRange<String>),
    InvalidString(WordRange<String>),
}

#[derive(Default)]
pub(super) struct ContextStream {
    cur_line: usize,
    cur_char: usize,
}

#[derive(Debug)]
pub(super) struct SourceStream {
    pub(super) word_streams: Vec<WordStream>,
}

impl ContextStream {
    fn new() -> Self {
        Self::default()
    }

    pub(super) fn scan_text(lines: &[&str]) -> Result<SourceStream, ScError> {
        let mut context = Self::new();
        let mut brackets_word_stream = vec![];

        while let Some(line_text) = lines.get(context.cur_line) {
            match line_text.chars().nth(context.cur_char) {
                Some(ch) => {
                    let line_str = line_text;

                    // At this point, the character can only be a whitespace, start of a comment,
                    // which means the entire line, an identifier, or a body.
                    match ch {
                        c if c.is_whitespace() => context.cur_char += 1,
                        c if c.is_ascii_alphabetic() => {
                            context.push_identifier(line_str, &mut brackets_word_stream)?
                        }
                        '-' => {
                            if context
                                .consume_comment(line_str, &mut brackets_word_stream)
                                .is_err()
                            {
                                // Parsing a number in case it's not a number.
                                // But finding a number here? Maybe.
                                context.push_numeric_string(line_str, &mut brackets_word_stream)?;
                            }
                        }
                        ';' => context.push_semi_colon(&mut brackets_word_stream),
                        ',' => context.push_comma(&mut brackets_word_stream),
                        ':' => context.push_colon(&mut brackets_word_stream),
                        '{' => context.consume_body(lines, &mut brackets_word_stream)?,
                        sw => {
                            return Err(ScError::InvalidCharacter(WordRange {
                                range: Range::new_with_length(
                                    context.cur_line,
                                    context.cur_char,
                                    1,
                                ),
                                word: sw,
                            }))
                        }
                    }
                }
                None => context.next_line(&mut brackets_word_stream),
            }
        }

        Ok(SourceStream {
            word_streams: brackets_word_stream,
        })
    }

    fn consume_comment(
        &mut self,
        line_text: &str,
        word_stream: &mut Vec<WordStream>,
    ) -> Result<(), ScError> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"\A(?:--)(.*)").unwrap();
        }

        let text = line_text[self.cur_char..].to_string();
        match RE.captures(&text) {
            Some(caps) => match caps.get(1) {
                Some(m) => {
                    let comment = m.as_str().to_owned();
                    word_stream.push(WordStream::Comment(WordRange {
                        range: Range::new_with_length(self.cur_line, self.cur_char, comment.len()),
                        word: comment,
                    }));
                    self.next_line(word_stream);
                    return Ok(());
                }
                None => {
                    return Err(ScError::InvalidComment(WordRange {
                        range: Range::new_with_length(self.cur_line, self.cur_char, text.len()),
                        word: text,
                    }));
                }
            },
            None => {
                return Err(ScError::InvalidComment(WordRange {
                    range: Range::new_with_length(self.cur_line, self.cur_char, text.len()),
                    word: text,
                }));
            }
        }
    }

    fn consume_string(
        &mut self,
        line_text: &str,
        word_stream: &mut Vec<WordStream>,
    ) -> Result<(), ScError> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r#"\A"(.*?)""#).unwrap();
        }

        let text = line_text[self.cur_char..].to_string();
        match RE.captures(&text) {
            Some(caps) => {
                let match_len = caps.get(0).unwrap().as_str().len();

                match caps.get(1) {
                    Some(m) => word_stream.push(WordStream::Literal(WordRange {
                        range: Range::new_with_length(self.cur_line, self.cur_char, match_len),
                        word: m.as_str().to_owned(),
                    })),
                    None => {
                        return Err(ScError::InvalidString(WordRange {
                            range: Range::new_with_length(self.cur_line, self.cur_char, text.len()),
                            word: text,
                        }));
                    }
                }
                self.cur_char += match_len;
            }
            None => {
                return Err(ScError::InvalidString(WordRange {
                    range: Range::new_with_length(self.cur_line, self.cur_char, text.len()),
                    word: text,
                }));
            }
        }

        Ok(())
    }

    fn consume_brackets(
        &mut self,
        lines: &[&str],
        word_stream: &mut Vec<WordStream>,
    ) -> Result<(), ScError> {
        let mut brackets_word_stream = vec![];
        self.push_left_square_bracket(word_stream);

        while let Some(line_text) = lines.get(self.cur_line) {
            match line_text.chars().nth(self.cur_char) {
                Some(ch) => {
                    let line_str = line_text;

                    match ch {
                        c if c.is_ascii_alphabetic() => {
                            self.push_identifier(line_str, &mut brackets_word_stream)?
                        }
                        c if c.is_whitespace() => self.cur_char += 1,
                        '[' => self.consume_brackets(lines, &mut brackets_word_stream)?,
                        ']' => {
                            word_stream.push(WordStream::SquareBracketBody(brackets_word_stream));
                            self.push_right_square_bracket(word_stream);
                            return Ok(());
                        }
                        '(' => self.consume_parenthesis(lines, &mut brackets_word_stream)?,
                        ';' => self.push_semi_colon(&mut brackets_word_stream),
                        '"' => self.consume_string(line_str, &mut brackets_word_stream)?,
                        '-' => {
                            if self
                                .consume_comment(line_str, &mut brackets_word_stream)
                                .is_err()
                            {
                                if self
                                    .push_numeric_string(line_str, &mut brackets_word_stream)
                                    .is_err()
                                {
                                    self.push_hyphen(&mut brackets_word_stream);
                                }
                            }
                        }
                        '>' => {
                            self.push_greater_than(&mut brackets_word_stream);
                        }
                        ',' => self.push_comma(&mut brackets_word_stream),
                        sw => {
                            return Err(ScError::InvalidCharacter(WordRange {
                                range: Range::new_with_length(self.cur_line, self.cur_char, 1),
                                word: sw,
                            }));
                        }
                    }
                }
                None => self.next_line(&mut brackets_word_stream),
            }
        }

        Err(ScError::SymbolMissing(WordStream::RightSquareBracket(
            WordRange {
                range: Range::new_with_length(self.cur_line, self.cur_char, 1),
                word: ']',
            },
        )))
    }

    fn consume_parenthesis(
        &mut self,
        lines: &[&str],
        word_stream: &mut Vec<WordStream>,
    ) -> Result<(), ScError> {
        let mut parens_word_stream = vec![];
        self.push_left_paren(word_stream);

        while let Some(line_text) = lines.get(self.cur_line) {
            match line_text.chars().nth(self.cur_char) {
                Some(ch) => {
                    let line_str = line_text;

                    match ch {
                        c if c.is_ascii_alphabetic() => {
                            self.push_identifier(line_str, &mut parens_word_stream)?
                        }
                        c if c.is_whitespace() => self.cur_char += 1,
                        '(' => self.consume_parenthesis(lines, &mut parens_word_stream)?,
                        ')' => {
                            word_stream.push(WordStream::ParenthesisBody(parens_word_stream));
                            self.push_right_paren(word_stream);
                            return Ok(());
                        }
                        '[' => self.consume_brackets(lines, &mut parens_word_stream)?,
                        '"' => self.consume_string(line_str, &mut parens_word_stream)?,
                        ';' => self.push_semi_colon(&mut parens_word_stream),
                        ':' => self.push_colon(&mut parens_word_stream),
                        '-' => {
                            if self
                                .consume_comment(line_str, &mut parens_word_stream)
                                .is_err()
                            {
                                if self
                                    .push_numeric_string(line_str, &mut parens_word_stream)
                                    .is_err()
                                {
                                    self.push_hyphen(&mut parens_word_stream);
                                }
                            }
                        }
                        '>' => {
                            self.push_greater_than(&mut parens_word_stream);
                        }
                        ',' => self.push_comma(&mut parens_word_stream),
                        sw => {
                            return Err(ScError::InvalidCharacter(WordRange {
                                range: Range::new_with_length(self.cur_line, self.cur_char, 1),
                                word: sw,
                            }));
                        }
                    }
                }
                None => self.next_line(&mut parens_word_stream),
            }
        }

        Err(ScError::SymbolMissing(WordStream::RightParenthesis(
            WordRange {
                range: Range::new_with_length(self.cur_line, self.cur_char, 1),
                word: ')',
            },
        )))
    }

    fn consume_body(
        &mut self,
        lines: &[&str],
        word_stream: &mut Vec<WordStream>,
    ) -> Result<(), ScError> {
        let mut brackets_word_stream = vec![];
        self.push_left_curly_bracket(word_stream);

        while let Some(line_text) = lines.get(self.cur_line) {
            match line_text.chars().nth(self.cur_char) {
                Some(ch) => {
                    let line_str = line_text;

                    match ch {
                        c if c.is_ascii_alphabetic() => {
                            self.push_identifier(line_str, &mut brackets_word_stream)?
                        }
                        c if c.is_ascii_digit() => {
                            self.push_numeric_string(line_str, &mut brackets_word_stream)?
                        }
                        c if c.is_whitespace() => self.cur_char += 1,
                        '{' => self.consume_body(lines, &mut brackets_word_stream)?,
                        '}' => {
                            word_stream.push(WordStream::CurlyBracketBody(brackets_word_stream));
                            self.push_right_curly_bracket(word_stream);
                            return Ok(());
                        }
                        '[' => self.consume_brackets(lines, &mut brackets_word_stream)?,
                        '(' => self.consume_parenthesis(lines, &mut brackets_word_stream)?,
                        ',' => self.push_comma(&mut brackets_word_stream),
                        ':' => self.push_colon(&mut brackets_word_stream),
                        ';' => self.push_semi_colon(&mut brackets_word_stream),
                        '=' => self.push_assignment(&mut brackets_word_stream),
                        '"' => self.consume_string(line_str, &mut brackets_word_stream)?,
                        '-' => {
                            if self
                                .consume_comment(line_str, &mut brackets_word_stream)
                                .is_err()
                            {
                                if self
                                    .push_numeric_string(line_str, &mut brackets_word_stream)
                                    .is_err()
                                {
                                    self.push_hyphen(&mut brackets_word_stream);
                                }
                            }
                        }
                        '>' => {
                            self.push_greater_than(&mut brackets_word_stream);
                        }
                        sw => {
                            return Err(ScError::InvalidCharacter(WordRange {
                                range: Range::new_with_length(self.cur_line, self.cur_char, 1),
                                word: sw,
                            }));
                        }
                    }
                }
                None => self.next_line(&mut brackets_word_stream),
            }
        }

        Err(ScError::SymbolMissing(WordStream::RightCurlyBracket(
            WordRange {
                range: Range::new_with_length(self.cur_line, self.cur_char, 1),
                word: '}',
            },
        )))
    }

    fn push_identifier(
        &mut self,
        line_text: &str,
        word_stream: &mut Vec<WordStream>,
    ) -> Result<(), ScError> {
        let mut ident = String::new();
        let index = self.cur_char;

        while let Some(ch) = line_text.chars().nth(self.cur_char) {
            // Looks for a correct word, until it finds a whitespace.
            if ch.is_ascii_alphanumeric() || ch == '_' {
                ident.push(ch);
                self.cur_char += 1;
            } else {
                break;
            }
        }

        if KEYWORDS.contains(&ident.as_str()) {
            let keyword = Keywords::from(ident.as_str());
            let word_range = WordRange {
                range: Range::new_with_length(self.cur_line, index, ident.len()),
                word: keyword,
            };

            word_stream.push(WordStream::Keyword(word_range));
        } else if NATIVE_TYPES.contains(&ident.as_str()) {
            let native_type = NativeTypes::from(ident.as_str());
            let word_range = WordRange {
                range: Range::new_with_length(self.cur_line, index, ident.len()),
                word: native_type,
            };

            word_stream.push(WordStream::NativeType(word_range));
        } else if BOOLEAN_VALUES.contains(&ident.as_str()) {
            let word_range = WordRange {
                range: Range::new_with_length(self.cur_line, index, ident.len()),
                word: ident.to_owned(),
            };

            word_stream.push(WordStream::BooleanValue(word_range));
        } else if ATTRIBUTES_NAMES.contains(&ident.as_str()) {
            let attribute_name = match ident.as_str() {
                "deprecated" => AttributeNames::Deprecated,
                _ => panic!("Not an attribute name"),
            };

            let word_range = WordRange {
                range: Range::new_with_length(self.cur_line, index, ident.len()),
                word: attribute_name,
            };

            word_stream.push(WordStream::Attribute(word_range));
        } else {
            let word_range = WordRange {
                range: Range::new_with_length(self.cur_line, index, ident.len()),
                word: ident.to_owned(),
            };

            lazy_static! {
                static ref RE: Regex =
                    Regex::new(r"^(?:([A-Z]+[A-Za-z0-9]*)|((?:[a-z]+[0-9]*)+(?:_[a-z0-9]+)*))$")
                        .unwrap();
            }

            match RE.captures(ident.as_str()) {
                Some(caps) => {
                    if caps.get(1).is_some() {
                        // Pascal case, user type name.
                        word_stream.push(WordStream::TypeName(word_range));
                    } else if caps.get(2).is_some() {
                        // All lower case, field name or library name.
                        word_stream.push(WordStream::Identifier(word_range));
                    } else {
                        return Err(ScError::Name(WordStream::Identifier(word_range)));
                    }
                }
                None => {
                    return Err(ScError::Name(WordStream::Identifier(word_range)));
                }
            }
        }
        Ok(())
    }

    fn push_numeric_string(
        &mut self,
        line_text: &str,
        word_stream: &mut Vec<WordStream>,
    ) -> Result<(), ScError> {
        let mut ident = String::new();
        let mut cur_char = self.cur_char;

        while let Some(ch) = line_text.chars().nth(cur_char) {
            match ch {
                c if c.is_ascii_hexdigit() => {
                    cur_char += 1;
                    ident.push(ch);
                }
                'x' | '.' | '-' | 'e' | 'E' => {
                    cur_char += 1;
                    ident.push(ch);
                }
                _ => break,
            }
        }

        // TODO Use basic floating point notation.
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^(?:(-?(?:0x[0-9A-F]+|0x[0-9a-f]+|[0-9]+))|(-?(?:[0-9]+\.)+[0-9]+))$")
                    .unwrap();
        }

        let word_range = WordRange {
            range: Range::new_with_length(self.cur_line, self.cur_char, ident.len()), // TODO
            word: ident.to_owned(),
        };

        if let Some(caps) = RE.captures(ident.as_str()) {
            if caps.get(1).is_some() {
                word_stream.push(WordStream::IntegerValue(word_range));
                self.cur_char = cur_char;
                return Ok(());
            } else if caps.get(2).is_some() {
                word_stream.push(WordStream::FloatValue(word_range));
                self.cur_char = cur_char;
                return Ok(());
            }
        }

        return Err(ScError::InvalidString(WordRange {
            range: Range::new_with_length(self.cur_line, self.cur_char, ident.len()),
            word: ident,
        }));
    }

    fn push_comma(&mut self, word_stream: &mut Vec<WordStream>) {
        word_stream.push(WordStream::Comma(WordRange {
            range: Range::new_with_length(self.cur_line, self.cur_char, 1),
            word: ',',
        }));
        self.cur_char += 1;
    }

    fn push_hyphen(&mut self, word_stream: &mut Vec<WordStream>) {
        word_stream.push(WordStream::Hyphen(WordRange {
            range: Range::new_with_length(self.cur_line, self.cur_char, 1),
            word: '-',
        }));
        self.cur_char += 1;
    }

    fn push_greater_than(&mut self, word_stream: &mut Vec<WordStream>) {
        word_stream.push(WordStream::GreaterThan(WordRange {
            range: Range::new_with_length(self.cur_line, self.cur_char, 1),
            word: '>',
        }));
        self.cur_char += 1;
    }

    fn push_left_square_bracket(&mut self, word_stream: &mut Vec<WordStream>) {
        word_stream.push(WordStream::LeftSquareBracket(WordRange {
            range: Range::new_with_length(self.cur_line, self.cur_char, 1),
            word: '[',
        }));
        self.cur_char += 1;
    }

    fn push_right_square_bracket(&mut self, word_stream: &mut Vec<WordStream>) {
        word_stream.push(WordStream::RightSquareBracket(WordRange {
            range: Range::new_with_length(self.cur_line, self.cur_char, 1),
            word: ']',
        }));
        self.cur_char += 1;
    }

    fn push_left_curly_bracket(&mut self, word_stream: &mut Vec<WordStream>) {
        word_stream.push(WordStream::LeftCurlyBracket(WordRange {
            range: Range::new_with_length(self.cur_line, self.cur_char, 1),
            word: '{',
        }));
        self.cur_char += 1;
    }

    fn push_right_curly_bracket(&mut self, word_stream: &mut Vec<WordStream>) {
        word_stream.push(WordStream::RightCurlyBracket(WordRange {
            range: Range::new_with_length(self.cur_line, self.cur_char, 1),
            word: '}',
        }));
        self.cur_char += 1;
    }

    fn push_left_paren(&mut self, word_stream: &mut Vec<WordStream>) {
        word_stream.push(WordStream::LeftParenthesis(WordRange {
            range: Range::new_with_length(self.cur_line, self.cur_char, 1),
            word: '(',
        }));
        self.cur_char += 1;
    }

    fn push_right_paren(&mut self, word_stream: &mut Vec<WordStream>) {
        word_stream.push(WordStream::RightParenthesis(WordRange {
            range: Range::new_with_length(self.cur_line, self.cur_char, 1),
            word: ')',
        }));
        self.cur_char += 1;
    }

    fn push_colon(&mut self, word_stream: &mut Vec<WordStream>) {
        word_stream.push(WordStream::Colon(WordRange {
            range: Range::new_with_length(self.cur_line, self.cur_char, 1),
            word: ':',
        }));
        self.cur_char += 1;
    }

    fn push_assignment(&mut self, word_stream: &mut Vec<WordStream>) {
        word_stream.push(WordStream::Assignment(WordRange {
            range: Range::new_with_length(self.cur_line, self.cur_char, 1),
            word: '=',
        }));
        self.cur_char += 1;
    }

    fn push_semi_colon(&mut self, word_stream: &mut Vec<WordStream>) {
        word_stream.push(WordStream::SemiColon(WordRange {
            range: Range::new_with_length(self.cur_line, self.cur_char, 1),
            word: ';',
        }));
        self.cur_char += 1;
    }

    fn next_line(&mut self, word_stream: &mut Vec<WordStream>) {
        word_stream.push(WordStream::NewLine(WordRange {
            range: Range::new_with_length(self.cur_line, self.cur_char, 1),
            word: '\n',
        }));
        self.cur_line += 1;
        self.cur_char = 0;
    }
}
