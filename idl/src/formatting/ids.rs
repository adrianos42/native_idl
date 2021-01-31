use super::*;
use crate::ids::parser::*;
use crate::scanner::Keywords;

// TODO Split lines with more than 80 characters.

// The only stuff that this formatter reorders are the the library name and imports,
// since they must appear first. Anything else, only the comments, spaces and indentantion are fixed.
pub fn format_document(parser: &Parser) -> Option<String> {
    let mut body = String::new();
    let mut library_body = String::new();

    let mut is_comment = false;

    for node in &parser.nodes {
        match node {
            ParserNode::Package(value)
            | ParserNode::Layer(value)
            | ParserNode::Server(value)
            | ParserNode::Client(value) => {
                let keyword = match node {
                    ParserNode::Package(_) => Keywords::Package,
                    ParserNode::Layer(_) => Keywords::Layer,
                    ParserNode::Server(_) => Keywords::Server,
                    ParserNode::Client(_) => Keywords::Client,
                };

                let body = match node {
                    ParserNode::Package(_) => &mut library_body,
                    _ => &mut body,
                };

                let name_ident = match &value.ident {
                    ItemIdent::TypeName(name) => &name.ident,
                    ItemIdent::Identifier(name) => &name.ident,
                };


                let type_body = format!("{} {}{}", keyword, name_ident, OPEN_NEW_LINE);
                body.push_str(type_body.as_str());

                for node in &value.nodes {
                    match node {
                        ItemNode::ItemField(field) => {
                            body.push_str(INDENT);
                            body.push_str(field.to_string().as_str());
                            body.push_str(",\n");
                        }
                        ItemNode::Comment(_) => {}
                    }
                }

                body.push_str(CLOSE_NEW_LINE);
            }
        }
    }

    library_body += body.as_str();

    Some(library_body.trim().to_string())
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
