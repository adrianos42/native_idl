use codespan_reporting::{
    diagnostic::{Diagnostic, Label},
    files::SimpleFiles,
    term,
    term::termcolor::{ColorChoice, StandardStream},
};
use idl::{self, module};
use std::ops::Range;

pub fn diagnostic(module: &module::Module) -> anyhow::Result<bool> {
    let files = files::Files::from(module);

    let idl_parser_errors = module.get_idl_parser_errors();
    let idl_analyze_errors = module.get_idl_analyze_errors();
    let ids_parser_errors = module.get_ids_parser_errors();
    let ids_analyze_errors = module.get_ids_analyze_errors();
    let mut messages = vec![];

    for (name, err) in idl_parser_errors {
        if let Some(id) = files.get_id(&name) {
            let range = err
                .get_range()
                .get_byte_range(&module.get_idl_document_text(&name).unwrap())
                .unwrap_or_default();

            messages.push(Message::IdlParser { id, err, range });
        }
    }

    for (name, err) in idl_analyze_errors {
        if let Some(id) = files.get_id(&name) {
            let range = 0..1;
            messages.push(Message::IdlAnalyzer { id, err, range });
        }
    }

    for (name, err) in ids_parser_errors {
        if let Some(id) = files.get_id(&name) {
            let range = err
                .get_range()
                .get_byte_range(&module.get_idl_document_text(&name).unwrap())
                .unwrap_or_default();

            messages.push(Message::IdsParser { id, err, range });
        }
    }

    for (name, err) in ids_analyze_errors {
        if let Some(id) = files.get_id(&name) {
            let range = 0..1;
            messages.push(Message::IdsAnalyzer { id, err, range });
        }
    }

    let has_error = !messages.is_empty();

    let writer = StandardStream::stderr(ColorChoice::Always);
    let config = term::Config::default();
    for message in messages {
        let writer = &mut writer.lock();
        term::emit(writer, &config, &files, &message.to_diagnostic())?;
    }

    Ok(has_error)
}

pub fn diagnostic_generic(name: &str, message: &str) -> anyhow::Result<()> {
    let mut writer = StandardStream::stderr(ColorChoice::Always);
    let config = term::Config::default();

    let mut files = SimpleFiles::new();
    let id = files.add(name, message);
    let diagnostic = Diagnostic::error().with_labels(vec![Label::primary(id, 0..message.len())]);

    term::emit(&mut writer, &config, &files, &diagnostic)?;

    Ok(())
}

mod files {
    use codespan_reporting::files;
    use std::{collections::HashMap, ops::Range};

    #[derive(Debug, Clone)]
    struct File {
        name: String,
        source: String,
        line_starts: Vec<usize>,
    }

    impl File {
        fn line_start(&self, line_index: usize) -> Option<usize> {
            use std::cmp::Ordering;

            match line_index.cmp(&self.line_starts.len()) {
                Ordering::Less => Some(self.line_starts.get(line_index).expect("failed").clone()),
                Ordering::Equal => Some(self.source.len()),
                Ordering::Greater => None,
            }
        }
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct FileId(usize);

    #[derive(Debug, Clone)]
    pub struct Files {
        files: Vec<File>,
        names: HashMap<String, FileId>,
    }

    impl From<&idl::module::Module> for Files {
        fn from(value: &idl::module::Module) -> Self {
            let mut result = Self::new();

            for name in value.get_document_names() {
                if let Some(source) = value.get_idl_document_text(&name) {
                    result.add(name, source);
                }
            }

            result
        }
    }

    impl Files {
        pub fn get_id(&self, name: &str) -> Option<FileId> {
            self.names.get(name).map(|v| *v)
        }

        fn new() -> Files {
            Files {
                files: Vec::new(),
                names: HashMap::new(),
            }
        }

        fn add(&mut self, name: impl Into<String>, source: impl Into<String>) -> Option<FileId> {
            use std::convert::TryFrom;

            let file_id = FileId(usize::try_from(self.files.len()).ok()?);
            let name = name.into();
            let source = source.into();
            let line_starts = files::line_starts(&source).collect();

            if self.names.insert(name.to_owned(), file_id).is_some() {
                panic!("same name")
            }

            self.files.push(File {
                name,
                line_starts,
                source,
            });

            Some(file_id)
        }

        fn get(&self, file_id: FileId) -> Option<&File> {
            self.files.get(file_id.0 as usize)
        }
    }

    impl<'files> files::Files<'files> for Files {
        type FileId = FileId;
        type Name = &'files str;
        type Source = &'files str;

        fn name(&self, file_id: FileId) -> Option<&str> {
            Some(self.get(file_id).unwrap().name.as_ref())
        }

        fn source(&self, file_id: Self::FileId) -> Option<&str> {
            Some(&self.get(file_id).unwrap().source)
        }

        fn line_index(&self, file_id: Self::FileId, byte_index: usize) -> Option<usize> {
            Some(
                self.get(file_id)?
                    .line_starts
                    .binary_search(&byte_index)
                    .unwrap_or_else(|next_line| next_line - 1),
            )
        }

        fn line_range(&self, file_id: FileId, line_index: usize) -> Option<Range<usize>> {
            let file = self.get(file_id)?;
            let line_start = file.line_start(line_index)?;
            let next_line_start = file.line_start(line_index + 1)?;

            Some(line_start..next_line_start)
        }
    }
}

pub enum Message {
    // Spec {
    //     id: files::FileId,
    //     range: Range<usize>,
    //     err: spec::SpecError,
    // },
    IdlAnalyzer {
        id: files::FileId,
        range: Range<usize>,
        err: idl::analyzer::AnalyzerError,
    },
    IdlParser {
        id: files::FileId,
        range: Range<usize>,
        err: idl::parser::ParserError,
    },
    IdsAnalyzer {
        id: files::FileId,
        range: Range<usize>,
        err: idl::ids::analyzer::AnalyzerError,
    },
    IdsParser {
        id: files::FileId,
        range: Range<usize>,
        err: idl::ids::parser::ParserError,
    },
}

impl Message {
    fn to_diagnostic(&self) -> Diagnostic<files::FileId> {
        match self {
            //Message::Spec { id, range, err } => Diagnostic::error().with_message("spec error"),
            Message::IdlParser { id, range, err } => {
                let (message, e_range) = err.get_message_with_range();

                Diagnostic::error()
                    .with_message("parser error")
                    .with_labels(vec![
                        Label::primary(*id, range.clone()).with_message(message)
                    ])
            }
            Message::IdlAnalyzer { id, range, err } => {
                let (message, e_range) = err.get_message_with_range();

                Diagnostic::error()
                    .with_message("analyzer error")
                    .with_labels(vec![
                        Label::primary(*id, range.clone()).with_message(message)
                    ])
            }
            Message::IdsParser { id, range, err } => {
                let (message, e_range) = err.get_message_with_range();

                Diagnostic::error()
                    .with_message("parser error")
                    .with_labels(vec![
                        Label::primary(*id, range.clone()).with_message(message)
                    ])
            }
            Message::IdsAnalyzer { id, range, err } => {
                let (message, e_range) = err.get_message_with_range();

                Diagnostic::error()
                    .with_message("analyzer error")
                    .with_labels(vec![
                        Label::primary(*id, range.clone()).with_message(message)
                    ])
            }
        }
    }
}
