use super::parser::*;
use std::sync::Arc;

impl Parser {
    pub fn get_range_from_type_name(&self, name: &str) -> Range {
        match self.create_type_from_reference(name).unwrap().as_ref() {
            Type::Name(name) => name.range,
            _ => panic!("Not a type name."),
        }
    }

    // Only returns the name from user defined types.
    pub fn create_type_from_reference(&self, name: &str) -> Option<Arc<Type>> {
        for node in &self.nodes {
            match node {
                ParserNode::Interface(value) => {
                    if value.ident.to_string() == name {
                        return Some(value.ident.clone());
                    }
                }
                ParserNode::Struct(value) => {
                    if value.ident.to_string() == name {
                        return Some(value.ident.clone());
                    }
                }
                ParserNode::Enum(value) => {
                    if value.ident.to_string() == name {
                        return Some(value.ident.clone());
                    }
                }
                ParserNode::TypeList(value) => {
                    if value.ident.to_string() == name {
                        return Some(value.ident.clone());
                    }
                }
                ParserNode::Const(value) => {
                    if value.ident.to_string() == name {
                        return Some(value.ident.clone());
                    }
                }
                ParserNode::Stream(value) => {
                    if value.ident.to_string() == name {
                        return Some(value.ident.clone());
                    }
                }
                ParserNode::Factory(value) => {
                    if value.ident.to_string() == name {
                        return Some(value.ident.clone());
                    }
                }
                _ => {}
            }
        }

        None
    }
}
