mod analyzer {
    static IDL_FIRST: &str = r#"
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
    Tokens: pair[string, bytes][][],
    Mpp: map[string, bool][],
}

struct TypeInterface {
    ident: string,
    fields: bytes[],
    tokens: pair[string, bytes],
    mp: map[string, bool],
    name: option[string],
}

const Never {
    ents: "pf",
    vcxmxkv: "43242340"
}

interface GetName {
    names: (thef: stream[map[Never, string]]),
    ship: option[int],
    numbers: result[bool, int],
    tokens: pair[string, bytes][],
    mp: map[string, bool][],
}
"#;

    use idl::analyzer;
    use idl::parser;

    #[test]
    fn try_this() {
        match parser::Parser::parse(IDL_FIRST) {
            Ok(parser) => match analyzer::Analyzer::resolve(&parser, None) {
                Ok(value) => {}
                Err(err) => println!("Error: {}", err),
            },
            Err(err) => {
                panic!("{:?} at {:?}", err.1, err.1.get_range());
            }
        }
    }
}
