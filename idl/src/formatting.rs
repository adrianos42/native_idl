use crate::parser::*;
use crate::scanner::Keywords;

// TODO Split lines with more than 80 characters.

pub const INDENT: &str = "    ";
pub const MAX_LENGTH: usize = 100;
pub const COMMENT_START: &str = "--";

const NEW_LINE: &str = "\n\n";
const CLOSE_NEW_LINE: &str = "}\n\n";
const OPEN_NEW_LINE: &str = " {\n";

// The only stuff that this formatter reorders are the the library name and imports,
// since they must appear first. Anything else, only the comments, spaces and indentantion are fixed.
pub fn format_document(parser: &Parser) -> Option<String> {
    let mut body = String::new();
    let mut library = String::new();
    let mut imports = String::new();

    let mut is_comment = false;

    for node in &parser.nodes {
        match node {
            ParserNode::Imports(value) => {
                imports = format!("{} {{", Keywords::Import);
                // Only imports can span an entire line without being separated by a new line.
                imports +=
                    split_if_longer_than(value.as_slice(), MAX_LENGTH - imports.len() - 1).as_str();
                imports += CLOSE_NEW_LINE;
            }
            ParserNode::Library(value) => {
                library = format!("{} ", Keywords::Library);
                library += value.as_str();
                library += NEW_LINE;
            }
            ParserNode::Comment(value) => {
                let comment = format!("{}{}\n", COMMENT_START, value.as_str());
                body += comment.as_str();
                is_comment = true;
            }
            nodw => {
                if is_comment {
                    body += "\n";
                    is_comment = false;
                }
                match nodw {
                    ParserNode::ConstComment(value) => {
                        let comment = format!("{}{}\n", COMMENT_START, value.as_str());
                        body += comment.as_str();
                    }
                    ParserNode::Const(value) => {
                        let type_body =
                            format!("{} {}{}", Keywords::Const, value.ident, OPEN_NEW_LINE);
                        body += type_body.as_str();

                        for const_node in value.fields.iter() {
                            match const_node {
                                ConstNode::Comment(comment) => {
                                    let comment_str = format!(
                                        "{}{}{}\n",
                                        INDENT,
                                        COMMENT_START,
                                        comment.as_str()
                                    );
                                    body += comment_str.as_str();
                                }
                                ConstNode::ConstField(field) => {
                                    body += INDENT;
                                    body += field.to_string().as_str();
                                    body += ",\n";
                                }
                            }
                        }
                        body += CLOSE_NEW_LINE;
                    }
                    ParserNode::EnumComment(value) => {
                        let comment = format!("{}{}\n", COMMENT_START, value.as_str());
                        body += comment.as_str();
                    }
                    ParserNode::Enum(value) => {
                        let type_body =
                            format!("{} {}{}", Keywords::Enum, value.ident, OPEN_NEW_LINE);
                        body += type_body.as_str();

                        for enum_node in value.fields.iter() {
                            match enum_node {
                                EnumNode::Comment(comment) => {
                                    let comment_str = format!(
                                        "{}{}{}\n",
                                        INDENT,
                                        COMMENT_START,
                                        comment.as_str()
                                    );
                                    body += comment_str.as_str();
                                }
                                EnumNode::EnumField(field) => {
                                    let field_str = format!("{}{},\n", INDENT, field);
                                    body += field_str.as_str();
                                }
                            }
                        }
                        body += CLOSE_NEW_LINE;
                    }
                    ParserNode::InterfaceComment(value) => {
                        let comment = format!("{}{}\n", COMMENT_START, value.as_str());
                        body += comment.as_str();
                    }
                    ParserNode::Interface(value) => {
                        let type_body =
                            format!("{} {}{}", Keywords::Interface, value.ident, OPEN_NEW_LINE);
                        body += type_body.as_str();

                        for interface_node in value.fields.iter() {
                            match interface_node {
                                InterfaceNode::Comment(comment) => {
                                    let comment_str = format!(
                                        "{}{}{}\n",
                                        INDENT,
                                        COMMENT_START,
                                        comment.as_str()
                                    );
                                    body += comment_str.as_str();
                                }
                                InterfaceNode::InterfaceField(field) => {
                                    let field_str = format!("{}{},\n", INDENT, field);
                                    body += field_str.as_str();
                                }
                            }
                        }
                        body += CLOSE_NEW_LINE;
                    }
                    ParserNode::StructComment(value) => {
                        let comment = format!("{}{}\n", COMMENT_START, value.as_str());
                        body += comment.as_str();
                    }
                    ParserNode::Struct(value) => {
                        let type_body =
                            format!("{} {}{}", Keywords::Struct, value.ident, OPEN_NEW_LINE);
                        body += type_body.as_str();

                        for struct_node in value.fields.iter() {
                            match struct_node {
                                StructNode::Comment(comment) => {
                                    let comment_str = format!(
                                        "{}{}{}\n",
                                        INDENT,
                                        COMMENT_START,
                                        comment.as_str()
                                    );
                                    body += comment_str.as_str();
                                }
                                StructNode::StructField(field) => {
                                    let field_str = format!("{}{},\n", INDENT, field);
                                    body += field_str.as_str();
                                }
                            }
                        }
                        body += CLOSE_NEW_LINE;
                    }
                    ParserNode::TypeListComment(value) => {
                        let list_comment = format!("{}{}\n", COMMENT_START, value.as_str());
                        body += list_comment.as_str();
                    }
                    ParserNode::TypeList(value) => {
                        if value.ty_list.len() == 1 {
                            let type_body = format!("{} {} {{", Keywords::Type, value.ident);
                            body += type_body.as_str();

                            match value.ty_list.first().unwrap() {
                                TypeListNode::TypeListField(field) => {
                                    let field_str = format!(" {} ", field.ty);
                                    body += field_str.as_str();
                                }
                                _ => return None,
                            }
                        } else {
                            let type_body =
                                format!("{} {}{}", Keywords::Type, value.ident, OPEN_NEW_LINE);
                            body += type_body.as_str();

                            for type_node in value.ty_list.iter() {
                                match type_node {
                                    TypeListNode::Comment(comment) => {
                                        let comment_str = format!(
                                            "{}{}{}\n",
                                            INDENT,
                                            COMMENT_START,
                                            comment.as_str()
                                        );
                                        body += comment_str.as_str();
                                    }
                                    TypeListNode::TypeListField(field) => {
                                        let field_str = format!("{}{},\n", INDENT, field);
                                        body += field_str.as_str();
                                    }
                                }
                            }
                        }

                        body += CLOSE_NEW_LINE;
                    }
                    ParserNode::FactoryComment(value) => {
                        let list_comment = format!("{}{}\n", COMMENT_START, value.as_str());
                        body += list_comment.as_str();
                    }
                    ParserNode::Factory(value) => {
                        let type_body =
                            format!("{} {}{}", Keywords::Factory, value.ident, OPEN_NEW_LINE);
                        body += type_body.as_str();

                        for factory_node in value.fields.iter() {
                            match factory_node {
                                InterfaceNode::Comment(comment) => {
                                    let comment_str = format!(
                                        "{}{}{}\n",
                                        INDENT,
                                        COMMENT_START,
                                        comment.as_str()
                                    );
                                    body += comment_str.as_str();
                                }
                                InterfaceNode::InterfaceField(field) => {
                                    let field_str = format!("{}{},\n", INDENT, field);
                                    body += field_str.as_str();
                                }
                            }
                        }
                        body += CLOSE_NEW_LINE;
                    }
                    ParserNode::StreamComment(value) => {
                        let list_comment = format!("{}{}\n", COMMENT_START, value.as_str());
                        body += list_comment.as_str();
                    }
                    ParserNode::Stream(value) => {
                        let type_body =
                            format!("{} {}{}", Keywords::Stream, value.ident, OPEN_NEW_LINE);
                        body += type_body.as_str();

                        for struct_node in value.fields.iter() {
                            match struct_node {
                                StructNode::Comment(comment) => {
                                    let comment_str = format!(
                                        "{}{}{}\n",
                                        INDENT,
                                        COMMENT_START,
                                        comment.as_str()
                                    );
                                    body += comment_str.as_str();
                                }
                                StructNode::StructField(field) => {
                                    let field_str = format!("{}{},\n", INDENT, field);
                                    body += field_str.as_str();
                                }
                            }
                        }
                        body += CLOSE_NEW_LINE;
                    }
                    _ => {}
                }
            }
        }
    }

    library += imports.as_str();
    library += body.as_str();

    Some(library.trim().to_string())
}

// If it fits in a line, a list with space in both sides,
// otherwise in each line, separated by a comma.
fn split_if_longer_than(txt: &[String], max_length: usize) -> String {
    let mut result = String::from(" ");

    for value in txt.iter() {
        if result != " " {
            result += ", ";
        }
        result += value;
    }

    result += " ";

    if result.len() > max_length {
        result = String::from("\n");

        for value in txt.iter() {
            let r = format!("{}{},\n", INDENT, value);
            result += r.as_str();
        }

        result += "\n";
    }

    result
}
