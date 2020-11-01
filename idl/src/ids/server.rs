use super::analyzer;
use super::ids_nodes::*;
use super::parser;

pub(super) fn get_node(
    item: &parser::Item,
    analyzer_items: &analyzer::AnalyzerItems,
) -> Result<IdsNode, analyzer::AnalyzerError> {
    let mut nodes = vec![];

    let create_field_node = |ident: &str, value| {
        ServerNode::ServerField(Box::new(ServerField {
            value: Box::new(value),
            ident: ident.to_owned(),
        }))
    };

    for node in item.nodes.iter() {
        match node {
            parser::ItemNode::ItemField(field) => {
                let value = analyzer_items.get_ids_node(&field.value)?;
                match field.ident.as_str() {
                    "layers" => {
                        if match &value {
                            ItemType::LayerTypeName(_) => false,
                            ItemType::Values(values) => values.iter().any(|v| !v.is_layer()),
                            _ => true,
                        } {
                            return Err(analyzer::ReferenceError(
                                analyzer::ReferenceErrorKind::NotLayerTypeName,
                                field.range,
                                field.ident.to_owned(),
                            )
                            .into());
                        }
                    }
                    "description" => {
                        if !value.is_string() {
                            return Err(analyzer::ReferenceError(
                                analyzer::ReferenceErrorKind::NotString,
                                field.range,
                                field.ident.to_owned(),
                            )
                            .into());
                        }
                    }
                    _ => {
                        return Err(analyzer::ReferenceError(
                            analyzer::ReferenceErrorKind::UnknownField,
                            field.range,
                            field.ident.to_owned(),
                        )
                        .into())
                    }
                }
                nodes.push(create_field_node(&field.ident, value));
            }
            parser::ItemNode::Comment(_) => {}
        }
    }

    let ident = match &item.ident {
        parser::ItemIdent::Identifier(_) => return Err(analyzer::AnalyzerError::Undefined),
        parser::ItemIdent::TypeName(value) => value.ident.to_owned(),
    };

    Ok(IdsNode::Server(Box::new(Server { ident, nodes })))
}
