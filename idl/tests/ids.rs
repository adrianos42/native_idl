mod analyzer {
    static IDS_FIRST: &str = r#"
library idl_nodes {
    version: "1.0",
    idl_version: "0.1",
}

server Main {
    layers: [FFI],
}

client Maine {
    description: "Client generator.",
    layers: [FFI],
}

layer FFI {
    description: "This is supposed to be the tranlation layer.",
}
"#;

    use idl::ids::analyzer;
    use idl::ids::parser;

    #[test]
    fn try_this() {
        match parser::Parser::parse(IDS_FIRST) {
            Ok(parser) => {
                match analyzer::Analyzer::resolve(&parser, None) {
                    Ok(value) => {}
                    Err(err) => println!("Error: {}", err),
                }
            }
            Err(err) => {
                println!("{} at {:?}", err.1, err.1.get_range());
            }
        }
    }
}
