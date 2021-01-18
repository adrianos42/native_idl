mod analyzer {
    static IDS_FIRST: &str = r#"
library idl_nodes

enum Types {
    NatInt,
    NatFloat,
    NatString,
    NatBytes,
    NatBool,
    NatNone,
}

type IdlNode {
    LibraryName: string,
    Imports: string[],
    Comment: string[],
    InterfaceComment: string[],
    StructComment: string[],
    EnumComment: string[],
    ConstComment: string[],
    StreamComment: string[],
    TypeListComment: string[],
    TypeInterface: TypeInterface,
}

struct TypeInterface {
    ident: string,
    fields: bytes[],
}

const Never {
    ents: "pf",
    vcxmxkv: "43242340"
}


interface GetName {
    names: (thef: stream[map[Never, string]]),
    ship: option[int],
    numbers: result[bool, int],
}
"#;

    use idl::analyzer;
    use idl::parser;

    #[test]
    fn try_this() {
        match parser::Parser::parse(IDS_FIRST) {
            Ok(parser) => match analyzer::Analyzer::resolve(&parser, None) {
                Ok(value) => {}
                Err(err) => println!("Error: {}", err),
            },
            Err(err) => {
                panic!("{} at {:?}", err.1, err.1.get_range());
            }
        }
    }
}
