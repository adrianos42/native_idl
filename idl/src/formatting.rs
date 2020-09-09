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
                if is_comment {
                    body += "\n";
                }

                for comment in value {
                    let list_comment = format!("{}{}\n", COMMENT_START, comment.as_str());
                    body += list_comment.as_str();
                }
                is_comment = true;
            }
            nodw => {
                if is_comment {
                    body += "\n";
                    is_comment = false;
                }
                match nodw {
                    ParserNode::ConstComment(comment) => push_comment(&mut body, comment),
                    ParserNode::Const(value) => {
                        let type_body =
                            format!("{} {}{}", Keywords::Const, value.ident, OPEN_NEW_LINE);
                        body += type_body.as_str();

                        for const_node in value.fields.iter() {
                            match const_node {
                                ConstNode::Comment(comment) => {
                                    push_field_comment(&mut body, comment)
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
                    ParserNode::EnumComment(comment) => push_comment(&mut body, comment),
                    ParserNode::Enum(value) => {
                        let type_body =
                            format!("{} {}{}", Keywords::Enum, value.ident, OPEN_NEW_LINE);
                        body += type_body.as_str();

                        for enum_node in value.fields.iter() {
                            match enum_node {
                                EnumNode::Comment(comment) => {
                                    push_field_comment(&mut body, comment)
                                }
                                EnumNode::EnumField(field) => {
                                    let field_str = format!("{}{},\n", INDENT, field);
                                    body += field_str.as_str();
                                }
                            }
                        }
                        body += CLOSE_NEW_LINE;
                    }
                    ParserNode::InterfaceComment(comment) => push_comment(&mut body, comment),
                    ParserNode::Interface(value) => {
                        let type_body =
                            format!("{} {}{}", Keywords::Interface, value.ident, OPEN_NEW_LINE);
                        body += type_body.as_str();

                        for interface_node in value.fields.iter() {
                            match interface_node {
                                InterfaceNode::Comment(comment) => {
                                    push_field_comment(&mut body, comment)
                                }
                                InterfaceNode::InterfaceField(field) => {
                                    let field_str = split_interface_field(field);
                                    body += field_str.as_str();
                                }
                            }
                        }
                        body += CLOSE_NEW_LINE;
                    }
                    ParserNode::StructComment(comment) => push_comment(&mut body, comment),
                    ParserNode::Struct(value) => {
                        let type_body =
                            format!("{} {}{}", Keywords::Struct, value.ident, OPEN_NEW_LINE);
                        body += type_body.as_str();

                        for struct_node in value.fields.iter() {
                            match struct_node {
                                StructNode::Comment(comment) => {
                                    push_field_comment(&mut body, comment)
                                }
                                StructNode::StructField(field) => {
                                    let field_str = format!("{}{},\n", INDENT, field);
                                    body += field_str.as_str();
                                }
                            }
                        }
                        body += CLOSE_NEW_LINE;
                    }
                    ParserNode::TypeListComment(comment) => push_comment(&mut body, comment),
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
                                        push_field_comment(&mut body, comment)
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
                    ParserNode::FactoryComment(comment) => push_comment(&mut body, comment),
                    ParserNode::Factory(value) => {
                        let type_body =
                            format!("{} {}{}", Keywords::Factory, value.ident, OPEN_NEW_LINE);
                        body += type_body.as_str();

                        for factory_node in value.fields.iter() {
                            match factory_node {
                                InterfaceNode::Comment(comment) => {
                                    push_field_comment(&mut body, comment)
                                }
                                InterfaceNode::InterfaceField(field) => {
                                    let field_str = split_interface_field(field);
                                    body += field_str.as_str();
                                }
                            }
                        }
                        body += CLOSE_NEW_LINE;
                    }
                    ParserNode::StreamComment(comment) => push_comment(&mut body, comment),
                    ParserNode::Stream(value) => {
                        let type_body =
                            format!("{} {}{}", Keywords::Stream, value.ident, OPEN_NEW_LINE);
                        body += type_body.as_str();

                        for struct_node in value.fields.iter() {
                            match struct_node {
                                StructNode::Comment(comment) => {
                                    push_field_comment(&mut body, comment)
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

fn push_field_comment(body: &mut String, comments: &[String]) {
    for comment in comments {
        let comment_str = format!("{}{}{}\n", INDENT, COMMENT_START, comment.as_str());
        body.push_str(comment_str.as_str());
    }
}

fn push_comment(body: &mut String, comments: &[String]) {
    for comment in comments {
        let list_comment = format!("{}{}\n", COMMENT_START, comment.as_str());
        body.push_str(list_comment.as_str());
    }
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
    }

    result
}

fn split_interface_field(interface_field: &InterfaceField) -> String {
    // TODO
    let split_tuple = |indent: usize, ty: &[TupleEntry]| {
        let mut result = String::new();

        if ty.is_empty() {
            return "()".to_owned();
        }

        for (index, t) in ty.into_iter().enumerate() {
            let st = format!(
                "{}{}: {}{}",
                if index == 0 {
                    "(".to_owned()
                } else {
                    " ".repeat(indent)
                },
                t.ident,
                t.ty,
                if index != ty.len() - 1 { ",\n" } else { ")" }
            );
            result += st.as_str()
        }

        result
    };

    let tryln = format!(
        "{}{}: {},",
        INDENT,
        interface_field.ident.to_owned(),
        interface_field.ty.to_string()
    );

    if tryln.len() > MAX_LENGTH {
        let indent_len = format!("{}{}: (", INDENT, interface_field.ident.to_owned()).len();
        let ty_field = match &*interface_field.ty {
            Type::Tuple(tuple) => split_tuple(indent_len, &tuple.ty_list),
            Type::Function(function) => {
                let tt = match &*function.args {
                    Type::Tuple(value) => value.clone(),
                    _ => panic!("Not a tuple."),
                };

                format!(
                    "{} -> {}",
                    split_tuple(indent_len, &*tt.ty_list),
                    function.ret_ty.to_string()
                )
            }
            ty => ty.to_string(),
        };

        format!(
            "{}{}: {},\n",
            INDENT,
            interface_field.ident.to_owned(),
            ty_field
        )
    } else {
        tryln + "\n"
    }
}
