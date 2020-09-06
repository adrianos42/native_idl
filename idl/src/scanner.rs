use regex::Regex;
use std::fmt;

static KEYWORDS: &'static [&str] = &[
    "enum",
    "struct",
    "interface",
    "import",
    "library",
    "factory",
    "stream",
    "type",
    "const",
];

static RESERVED: &'static [&str] = &[
    "abstract",
    "as",
    "assert",
    "async",
    "attribute",
    "await",
    "become",
    "box",
    "break",
    "case",
    "catch",
    "class",
    "continue",
    "covariant",
    "crate",
    "default",
    "deferred",
    "do",
    "dyn",
    "dynamic",
    "else",
    "export",
    "extends",
    "extension",
    "external",
    "false",
    "final",
    "finally",
    "fn",
    "for",
    "get",
    "hashcode",
    "hide",
    "if",
    "impl",
    "implements",
    "in",
    "instance",
    "is",
    "let",
    "loop",
    "macro",
    "match",
    "mixin",
    "mod",
    "move",
    "mut",
    "new",
    "nosuchfunction",
    "null",
    "on",
    "operator",
    "override",
    "part",
    "priv",
    "private",
    "pub",
    "public",
    "ref",
    "rethrow",
    "return",
    "runtimetype",
    "self",
    "service",
    "set",
    "show",
    "static",
    "structure",
    "super",
    "switch",
    "this",
    "throw",
    "time",
    "tostring",
    "trait",
    "true",
    "try",
    "typedef",
    "typeof",
    "unsafe",
    "unsized",
    "use",
    "value",
    "var",
    "version",
    "virtual",
    "void",
    "where",
    "while",
    "with",
    "yield",
];

static NATIVE_TYPES: &'static [&str] = &["int", "float", "bool", "string", "bytes", "none"];

static RESERVED_TYPES: &'static [&str] = &[
    "uint", "i8", "i16", "i32", "i64", "i128", "u8", "u16", "u32", "u64", "u128", "f32", "f64",
    "double", "void", "date", "time", "char", "decimal",
];

static ATTRIBUTES_NAMES: &'static [&str] = &["deprecated"];

static RESERVED_ATTRIBUTES_NAMES: &'static [&str] = &[
    "abstract",
    "assert",
    "attribute",
    "const",
    "default",
    "deferred",
    "dynamic",
    "export",
    "extends",
    "extension",
    "external",
    "hide",
    "implements",
    "in",
    "is",
    "new",
    "null",
    "on",
    "operator",
    "private",
    "public",
    "return",
    "service",
    "static",
    "time",
    "value",
    "version",
    "void",
];

#[derive(Debug, Copy, Clone)]
pub enum Keywords {
    Enum,
    Struct,
    Interface,
    Import,
    Library,
    Type,
    Const,
    Factory,
    Stream,
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
            Keywords::Factory => "factory",
            Keywords::Stream => "stream",
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
    Get,
    Set,
    Stream,
    Factory,
    Sync,
    Async,
}

impl fmt::Display for AttributeNames {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            AttributeNames::Async => "async",
            AttributeNames::Sync => "sync",
            AttributeNames::Stream => "stream",
            AttributeNames::Set => "set",
            AttributeNames::Get => "get",
            AttributeNames::Factory => "factory",
        };

        write!(f, "{}", name)
    }
}

#[derive(Debug)]
pub(super) struct WordRange<T> {
    pub(super) line: usize,
    pub(super) index: usize,
    pub(super) length: usize,
    word: T,
}

#[derive(Debug, Default, Copy, Clone)]
pub(super) struct WRange {
    pub(super) line: usize,
    pub(super) index: usize,
    pub(super) length: usize,
}

impl<T> WordRange<T> {
    pub(super) fn get_word(&self) -> &T {
        &self.word
    }
}

impl<T: fmt::Display> fmt::Display for WordRange<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // On the editor, lines and columns start at index 1 :\
        write!(
            f,
            "WordRange {{Word: {}, Line {}, Column {}, Length {}}}",
            self.word,
            self.line + 1,
            self.index + 1,
            self.length
        )
    }
}

// impl fmt::Display for WRange {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{{Line {}, Column {}}}", self. + 1, self.1 + 1)
//     }
// }

#[derive(Debug)]
pub(super) enum WordStream {
    LeftSquareBracket(WRange),
    RightSquareBracket(WRange),
    SquareBracketBody(Vec<WordStream>),
    LeftCurlyBracket(WRange),
    RightCurlyBracket(WRange),
    CurlyBracketBody(Vec<WordStream>),
    LeftParenthesis(WRange),
    RightParenthesis(WRange),
    ParenthesisBody(Vec<WordStream>),
    Comma(WRange),
    Assignment(WRange),
    Colon(WRange),
    Hyphen(WRange),
    GreaterThan(WRange),
    NewLine(WRange),
    SemiColon(WRange),
    Identifier(WordRange<String>),
    Keyword(WordRange<Keywords>),
    NativeType(WordRange<NativeTypes>),
    Attribute(WordRange<AttributeNames>),
    ReservedType(WordRange<String>),
    ReservedAttribute(WordRange<String>),
    TypeName(WordRange<String>),
    Comment(WordRange<String>),
    StringBody(WordRange<String>),
    FloatValue(WordRange<String>),
    IntegerValue(WordRange<String>),
}

impl WordStream {
    pub(super) fn get_range(&self) -> WRange {
        match self {
            WordStream::LeftCurlyBracket(i_source)
            | WordStream::RightSquareBracket(i_source)
            | WordStream::LeftSquareBracket(i_source)
            | WordStream::RightCurlyBracket(i_source)
            | WordStream::LeftParenthesis(i_source)
            | WordStream::RightParenthesis(i_source)
            | WordStream::Comma(i_source)
            | WordStream::Assignment(i_source)
            | WordStream::Colon(i_source)
            | WordStream::NewLine(i_source)
            | WordStream::Hyphen(i_source)
            | WordStream::GreaterThan(i_source)
            | WordStream::SemiColon(i_source) => *i_source,
            WordStream::Identifier(w_range)
            | WordStream::ReservedType(w_range)
            | WordStream::ReservedAttribute(w_range)
            | WordStream::TypeName(w_range)
            | WordStream::Comment(w_range)
            | WordStream::StringBody(w_range)
            | WordStream::FloatValue(w_range)
            | WordStream::IntegerValue(w_range) => WRange {
                index: w_range.index,
                line: w_range.line,
                length: w_range.length,
            },
            WordStream::Keyword(w_range) => WRange {
                index: w_range.index,
                line: w_range.line,
                length: w_range.length,
            },
            WordStream::NativeType(w_range) => WRange {
                index: w_range.index,
                line: w_range.line,
                length: w_range.length,
            },
            WordStream::Attribute(w_range) => WRange {
                index: w_range.index,
                line: w_range.line,
                length: w_range.length,
            },
            WordStream::CurlyBracketBody(w_streams)
            | WordStream::ParenthesisBody(w_streams)
            | WordStream::SquareBracketBody(w_streams) => match w_streams.first() {
                Some(w_range) => w_range.get_range(),
                None => WRange::default(),
            },
        }
    }
}

impl fmt::Display for WordStream {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let strname = match self {
            WordStream::LeftCurlyBracket(_) => "{".to_owned(),
            WordStream::RightSquareBracket(_) => "]".to_owned(),
            WordStream::LeftSquareBracket(_) => "[".to_owned(),
            WordStream::RightCurlyBracket(_) => "}".to_owned(),
            WordStream::LeftParenthesis(_) => "(".to_owned(),
            WordStream::RightParenthesis(_) => ")".to_owned(),
            WordStream::Comma(_) => ",".to_owned(),
            WordStream::Assignment(_) => "=".to_owned(),
            WordStream::Colon(_) => ":".to_owned(),
            WordStream::NewLine(_) => "\n".to_owned(),
            WordStream::Hyphen(_) => "-".to_owned(),
            WordStream::GreaterThan(_) => ">".to_owned(),
            WordStream::SemiColon(_) => ";".to_owned(),
            WordStream::Identifier(name)
            | WordStream::ReservedType(name)
            | WordStream::ReservedAttribute(name)
            | WordStream::TypeName(name)
            | WordStream::Comment(name)
            | WordStream::StringBody(name)
            | WordStream::FloatValue(name)
            | WordStream::IntegerValue(name) => name.get_word().to_owned(),
            WordStream::Keyword(name) => name.get_word().to_string().to_owned(),
            WordStream::NativeType(name) => name.get_word().to_string().to_owned(),
            WordStream::Attribute(name) => name.get_word().to_string().to_owned(),
            WordStream::CurlyBracketBody(_)
            | WordStream::ParenthesisBody(_)
            | WordStream::SquareBracketBody(_) => "".to_owned(),
        };

        // On the editor, lines and columns start at index 1 :\
        write!(f, "{}", strname)
    }
}

pub(super) enum ScError {
    SymbolMissing(WordStream),
    ReservedWord(WordRange<String>),
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
                                // Just try parsing a number in case it's not a number.
                                // But find a number here? Maybe.
                                context.push_numeric_string(line_str, &mut brackets_word_stream)?;
                            }
                        }
                        ';' => context.push_semi_colon(&mut brackets_word_stream),
                        ',' => context.push_comma(&mut brackets_word_stream),
                        ':' => context.push_colon(&mut brackets_word_stream),
                        '{' => context.consume_body(lines, &mut brackets_word_stream)?,
                        sw => {
                            return Err(ScError::InvalidCharacter(WordRange {
                                line: context.cur_line,
                                index: context.cur_char,
                                length: 1,
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
            // static ref RE: Regex = Regex::new(r"\A(?=--)-*(.*)").unwrap(); // DOES NOT SUPPORT LOOKAROUND????
            static ref RE: Regex = Regex::new(r"\A(?:--)(.*)").unwrap();
        }

        let text = line_text[self.cur_char..].to_string();
        match RE.captures(&text) {
            Some(caps) => match caps.get(1) {
                Some(m) => {
                    let comment = m.as_str().to_owned();
                    word_stream.push(WordStream::Comment(WordRange {
                        index: self.cur_char,
                        length: comment.len(),
                        line: self.cur_line,
                        word: comment,
                    }));
                    self.next_line(word_stream);
                    return Ok(());
                }
                None => {
                    return Err(ScError::InvalidComment(WordRange {
                        line: self.cur_line,
                        index: self.cur_char,
                        length: text.len(),
                        word: text,
                    }));
                }
            },
            None => {
                return Err(ScError::InvalidComment(WordRange {
                    line: self.cur_line,
                    index: self.cur_char,
                    length: text.len(),
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
                    Some(m) => word_stream.push(WordStream::StringBody(WordRange {
                        index: self.cur_char,
                        length: match_len,
                        line: self.cur_line,
                        word: m.as_str().to_owned(),
                    })),
                    None => {
                        return Err(ScError::InvalidString(WordRange {
                            line: self.cur_line,
                            index: self.cur_char,
                            length: text.len(),
                            word: text,
                        }));
                    }
                }
                self.cur_char += match_len;
            }
            None => {
                return Err(ScError::InvalidString(WordRange {
                    line: self.cur_line,
                    index: self.cur_char,
                    length: text.len(),
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
                                line: self.cur_line,
                                index: self.cur_char,
                                length: 1,
                                word: sw,
                            }));
                        }
                    }
                }
                None => self.next_line(&mut brackets_word_stream),
            }
        }

        Err(ScError::SymbolMissing(WordStream::RightSquareBracket(
            WRange {
                line: self.cur_line,
                index: self.cur_char,
                length: 1,
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
                            let text = sw.to_string();
                            return Err(ScError::InvalidCharacter(WordRange {
                                line: self.cur_line,
                                index: self.cur_char,
                                length: 1,
                                word: sw,
                            }));
                        }
                    }
                }
                None => self.next_line(&mut parens_word_stream),
            }
        }

        Err(ScError::SymbolMissing(WordStream::RightParenthesis(
            WRange {
                line: self.cur_line,
                index: self.cur_char,
                length: 1,
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
                                line: self.cur_line,
                                index: self.cur_char,
                                length: 1,
                                word: sw,
                            }));
                        }
                    }
                }
                None => self.next_line(&mut brackets_word_stream),
            }
        }

        Err(ScError::SymbolMissing(WordStream::RightCurlyBracket(
            WRange {
                line: self.cur_line,
                index: self.cur_char,
                length: 1,
            },
        )))
    }

    fn push_identifier(
        &mut self,
        line_text: &str,
        word_stream: &mut Vec<WordStream>,
    ) -> Result<(), ScError> {
        let mut ident = String::new();

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
            let keyword = match ident.as_str() {
                "interface" => Keywords::Interface,
                "struct" => Keywords::Struct,
                "enum" => Keywords::Enum,
                "library" => Keywords::Library,
                "import" => Keywords::Import,
                "type" => Keywords::Type,
                "const" => Keywords::Const,
                "stream" => Keywords::Stream,
                "factory" => Keywords::Factory,
                _ => panic!("Not a keyword."),
            };

            let word_range = WordRange {
                index: self.cur_char - ident.len(),
                length: ident.len(),
                line: self.cur_line,
                word: keyword,
            };

            word_stream.push(WordStream::Keyword(word_range));
        } else if NATIVE_TYPES.contains(&ident.as_str()) {
            let native_type = match ident.as_str() {
                "int" => NativeTypes::Int,
                "float" => NativeTypes::Float,
                "bool" => NativeTypes::Bool,
                "string" => NativeTypes::String,
                "bytes" => NativeTypes::Bytes,
                "none" => NativeTypes::None,
                _ => panic!("Not a native type."),
            };

            let word_range = WordRange {
                index: self.cur_char - ident.len(),
                length: ident.len(),
                line: self.cur_line,
                word: native_type,
            };

            word_stream.push(WordStream::NativeType(word_range));
        } else if ATTRIBUTES_NAMES.contains(&ident.as_str()) {
            let attribute_name = match ident.as_str() {
                "get" => AttributeNames::Get,
                "set" => AttributeNames::Set,
                "stream" => AttributeNames::Stream,
                "factory" => AttributeNames::Factory,
                "sync" => AttributeNames::Sync,
                "async" => AttributeNames::Async,
                _ => panic!("Not an attribute name."),
            };

            let word_range = WordRange {
                index: self.cur_char - ident.len(),
                length: ident.len(),
                line: self.cur_line,
                word: attribute_name,
            };

            word_stream.push(WordStream::Attribute(word_range));
        } else {
            let word_range = WordRange {
                index: self.cur_char - ident.len(),
                length: ident.len(),
                line: self.cur_line,
                word: ident.to_owned(),
            };

            ///if RESERVED_TYPES.contains(&ident.as_str()) {
            // word_stream.push(WordStream::ReservedType(word_range));
            //} //else if RESERVED_ATTRIBUTES_NAMES.contains(&ident.as_str()) {
            //word_stream.push(WordStream::ReservedAttribute(word_range));
            //} else
            {
                // let cmp_ident = ident.to_owned().to_lowercase();
                // if RESERVED.contains(&cmp_ident.as_str()) {
                //     return Err(ScError::ReservedWord(word_range));
                // } else if KEYWORDS.contains(&cmp_ident.as_str())
                //     || NATIVE_TYPES.contains(&cmp_ident.as_str())
                //     || RESERVED_TYPES.contains(&cmp_ident.as_str())
                //     || ATTRIBUTES_NAMES.contains(&cmp_ident.as_str())
                //     || RESERVED_ATTRIBUTES_NAMES.contains(&cmp_ident.as_str())
                // {
                //     return Err(ScError::Name(WordStream::Identifier(word_range)));
                // }

                lazy_static! {
                    static ref RE: Regex =
                        Regex::new(r"^(?:([A-Z](?:[a-z0-9][A-Z]?)*)|([a-z]+(?:_[a-z0-9]+)*))$")
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

        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^(?:(-?(?:0x[0-9A-F]+|0x[0-9a-f]+|[0-9]+))|(-?(?:[0-9]+\.[0-9]+(?:[Ee][+-]?[0-9]+)?|[0-9]+[Ee][+-]?[0-9]+)))$")
                    .unwrap();
        }

        let word_range = WordRange {
            index: cur_char - ident.len(),
            length: ident.len(),
            line: self.cur_line,
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
            length: ident.len(),
            line: self.cur_line,
            index: self.cur_char,
            word: ident,
        }));
    }

    fn push_comma(&mut self, word_stream: &mut Vec<WordStream>) {
        word_stream.push(WordStream::Comma(WRange {
            line: self.cur_line,
            index: self.cur_char,
            length: 1,
        }));
        self.cur_char += 1;
    }

    fn push_hyphen(&mut self, word_stream: &mut Vec<WordStream>) {
        word_stream.push(WordStream::Hyphen(WRange {
            line: self.cur_line,
            index: self.cur_char,
            length: 1,
        }));
        self.cur_char += 1;
    }

    fn push_greater_than(&mut self, word_stream: &mut Vec<WordStream>) {
        word_stream.push(WordStream::GreaterThan(WRange {
            line: self.cur_line,
            index: self.cur_char,
            length: 1,
        }));
        self.cur_char += 1;
    }

    fn push_left_square_bracket(&mut self, word_stream: &mut Vec<WordStream>) {
        word_stream.push(WordStream::LeftSquareBracket(WRange {
            line: self.cur_line,
            index: self.cur_char,
            length: 1,
        }));
        self.cur_char += 1;
    }

    fn push_right_square_bracket(&mut self, word_stream: &mut Vec<WordStream>) {
        word_stream.push(WordStream::RightSquareBracket(WRange {
            line: self.cur_line,
            index: self.cur_char,
            length: 1,
        }));
        self.cur_char += 1;
    }

    fn push_left_curly_bracket(&mut self, word_stream: &mut Vec<WordStream>) {
        word_stream.push(WordStream::LeftCurlyBracket(WRange {
            line: self.cur_line,
            index: self.cur_char,
            length: 1,
        }));
        self.cur_char += 1;
    }

    fn push_right_curly_bracket(&mut self, word_stream: &mut Vec<WordStream>) {
        word_stream.push(WordStream::RightCurlyBracket(WRange {
            line: self.cur_line,
            index: self.cur_char,
            length: 1,
        }));
        self.cur_char += 1;
    }

    fn push_left_paren(&mut self, word_stream: &mut Vec<WordStream>) {
        word_stream.push(WordStream::LeftParenthesis(WRange {
            line: self.cur_line,
            index: self.cur_char,
            length: 1,
        }));
        self.cur_char += 1;
    }

    fn push_right_paren(&mut self, word_stream: &mut Vec<WordStream>) {
        word_stream.push(WordStream::RightParenthesis(WRange {
            line: self.cur_line,
            index: self.cur_char,
            length: 1,
        }));
        self.cur_char += 1;
    }

    fn push_colon(&mut self, word_stream: &mut Vec<WordStream>) {
        word_stream.push(WordStream::Colon(WRange {
            line: self.cur_line,
            index: self.cur_char,
            length: 1,
        }));
        self.cur_char += 1;
    }

    fn push_assignment(&mut self, word_stream: &mut Vec<WordStream>) {
        word_stream.push(WordStream::Assignment(WRange {
            line: self.cur_line,
            index: self.cur_char,
            length: 1,
        }));
        self.cur_char += 1;
    }

    fn push_semi_colon(&mut self, word_stream: &mut Vec<WordStream>) {
        word_stream.push(WordStream::SemiColon(WRange {
            line: self.cur_line,
            index: self.cur_char,
            length: 1,
        }));
        self.cur_char += 1;
    }

    fn next_line(&mut self, word_stream: &mut Vec<WordStream>) {
        word_stream.push(WordStream::NewLine(WRange {
            line: self.cur_line,
            index: self.cur_char,
            length: 1,
        }));
        self.cur_line += 1;
        self.cur_char = 0;
    }
}
