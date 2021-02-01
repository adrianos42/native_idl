mod analyzer {
    static IDS_FIRST: &str = r#"
package idl_nodes {
    version: 1.0,
    idl_version: 0.1,
    libs: [programmer],
}

server Main {
    layers: [FFI],
}

client Maine {
    description: "Client generator.",
    layers: [FFI],
}

layer FFI {
    description: "This is supposed to be the translation layer.",
}
"#;

    use ansi_term::Color;
    use anyhow::{format_err, Result};
    use idl::ids::analyzer;
    use idl::ids::parser;

    #[test]
    fn try_this() -> Result<()> {
        match parser::Parser::parse(IDS_FIRST) {
            Ok(parser) => match analyzer::Analyzer::resolve(&parser) {
                Ok(_) => return Ok(()),
                Err(err) => println!("Error: {}", Color::Red.paint(format!("{}", err))),
            },
            Err(err) => {
                println!(
                    "{} at {:?}",
                    Color::Red.paint(err.1.to_string()),
                    err.1.get_range()
                );
            }
        }

        Err(format_err!(""))
    }
}
