mod api;
mod capabilities;
pub mod server;

pub(crate) mod completion;
pub(crate) mod formatting;
pub(crate) mod symbol;
pub(crate) mod session;


use idl::idl::parser;
use tower_lsp::lsp_types::*;

pub(crate) fn get_range_from_parser(range: idl::range::Range) -> Range {
    Range {
        start: Position {
            line: range.start.line as u64,
            character: range.start.column as u64,
        },
        end: Position {
            line: range.end.line as u64,
            character: range.end.column as u64,
        },
    }
}

pub(crate) fn create_parser_range(range: Range) -> idl::range::Range {
    idl::range::Range {
        start: idl::range::Position {
            line: range.start.line as usize,
            column: range.start.character as usize,
        }, 
        end: idl::range::Position {
            line: range.end.line as usize,
            column: range.end.character as usize,
        }
    }
}

pub(crate) fn create_parser_position(pos: Position) -> idl::range::Position {
    idl::range::Position {
        line: pos.line as usize,
        column: pos.character as usize,
    }
}

pub(crate) fn range_from_position(pos: Position) -> Range {
    Range {
        start: pos,
        end: pos,
    }
}

pub(crate) fn get_range_from_text(txt: &str) -> Range {
    let lines = txt.lines().count();
    let chars = txt.lines().last().unwrap_or_default().len();
    Range {
        start: Position::default(),
        end: Position {
            line: if lines > 0 { lines - 1 } else { 0 } as u64,
            character: chars as u64,
        },
    }
}

pub(crate) fn get_range_from_text_inclusive(txt: &str) -> Range {
    let lines = txt.lines().count();
    Range {
        start: Position::default(),
        end: Position {
            line: lines as u64,
            character: 0 as u64,
        },
    }
}

pub(crate) fn create_error_diagnostic(message: String, range: Range) -> Diagnostic {
    Diagnostic {
        code: None,
        message,
        range,
        related_information: None,
        severity: Some(DiagnosticSeverity::Error),
        source: None,
        tags: None,
    }
}

pub(crate) fn file_name_from_uri(uri: &Url) -> Option<String> {
    match uri.to_file_path() {
        Ok(file_path) => {
            let mut result = None;
            let path = file_path.as_path();

            if let Some(file_name_os) = path.file_name() {
                if let Some(file_name) = file_name_os.to_str() {
                    result = Some(file_name.to_owned())
                }
            }

            result
        }
        Err(_) => None,
    }
}
