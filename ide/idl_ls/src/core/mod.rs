pub(crate) mod completion;
pub(crate) mod formatting;
pub(crate) mod symbol;
pub(crate) mod session;

use native_idl::parser;
use tower_lsp::lsp_types::*;

pub(crate) fn get_range_from_parser(range: parser::Range) -> Range {
    Range {
        start: Position {
            line: range.start.line as u64,
            character: range.start.index as u64,
        },
        end: Position {
            line: range.end.line as u64,
            character: range.end.index as u64,
        },
    }
}

pub(crate) fn create_parser_range(range: Range) -> parser::Range {
    parser::Range {
        start: parser::Position {
            line: range.start.line as usize,
            index: range.start.character as usize,
        }, 
        end: parser::Position {
            line: range.end.line as usize,
            index: range.end.character as usize,
        }
    }
}

pub(crate) fn create_parser_position(pos: Position) -> parser::Position {
    parser::Position {
        line: pos.line as usize,
        index: pos.character as usize,
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
