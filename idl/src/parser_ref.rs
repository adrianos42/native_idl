use super::parser::*;
use std::sync::Arc;

impl Parser {
    pub fn get_range_from_type_name(&self, name: &str) -> Range {
        match self.create_type_from_reference(name).unwrap().as_ref() {
            Type::Name(name) => name.range,
            _ => panic!("Not a type name."),
        }
    }

    pub fn get_range_from_field_name(&self, name: &str, field_name: &str) -> Range {
        for node in &self.nodes {
            match node {
                ParserNode::Interface(value) => {
                    if value.ident.to_string() == name {
                        for field_node in value.fields.iter() {
                            if let InterfaceNode::InterfaceField(field) = field_node {
                                if field.ident == field_name {
                                    return field.range;
                                }
                            }
                        }
                    }
                }
                ParserNode::Struct(value) => {
                    if value.ident.to_string() == name {
                        for field_node in value.fields.iter() {
                            if let StructNode::StructField(field) = field_node {
                                if field.ident == field_name {
                                    return field.range;
                                }
                            }
                        }
                    }
                }
                ParserNode::Enum(value) => {
                    if value.ident.to_string() == name {
                        for field_node in value.fields.iter() {
                            if let EnumNode::EnumField(field) = field_node {
                                if field.ident == field_name {
                                    return field.range;
                                }
                            }
                        }
                    }
                }
                ParserNode::TypeList(value) => {
                    if value.ident.to_string() == name {
                        for field_node in value.ty_list.iter() {
                            if let TypeListNode::TypeListField(field) = field_node {
                                if field.ident == field_name {
                                    return field.range;
                                }
                            }
                        }
                    }
                }
                ParserNode::Const(value) => {
                    if value.ident.to_string() == name {
                        for field_node in value.fields.iter() {
                            if let ConstNode::ConstField(field) = field_node {
                                if field.ident == field_name {
                                    return field.range;
                                }
                            }
                        }
                    }
                }
                ParserNode::Stream(value) => {
                    if value.ident.to_string() == name {
                        for field_node in value.fields.iter() {
                            if let StructNode::StructField(field) = field_node {
                                if field.ident == field_name {
                                    return field.range;
                                }
                            }
                        }
                    }
                }
                ParserNode::Factory(value) => {
                    if value.ident.to_string() == name {
                        for field_node in value.fields.iter() {
                            if let InterfaceNode::InterfaceField(field) = field_node {
                                if field.ident == field_name {
                                    return field.range;
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        panic!("Field `{}` in `{}` not found.", name, field_name)
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
