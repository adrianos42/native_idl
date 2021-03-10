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

    fn to_hex_string(bytes: &[u8]) -> String {
        let strs: Vec<String> = bytes.iter().map(|b| format!("{:02x}", b)).collect();
        strs.join("")
    }

    #[test]
    fn try_this() {
        match parser::Parser::parse(IDL_FIRST) {
            Ok(parser) => match analyzer::Analyzer::resolve(&parser, None) {
                Ok(analyzer) => {
                    let hex_value = analyzer.library_hash();
                    //println!("library disgest: {}", to_hex_string(&hex_value));
                    for node in &analyzer.nodes {
                        match node {
                            idl::idl_nodes::IdlNode::TypeStruct(_) => {}
                            idl::idl_nodes::IdlNode::TypeEnum(_) => {}
                            idl::idl_nodes::IdlNode::TypeList(_) => {}
                            idl::idl_nodes::IdlNode::TypeConst(value) => {
                                match value.hash[..] {
                                    [0x3b, 0x18, 0x04, 0x2f, 0x06, 0x99, 0x50, 0xae, 0x49, 0xa4, 0x7b, 0xd5, 0xbc, 0x97, 0xe6, 0x36] =>
                                        {
                                            println!("first")
                                        }
                                    [0xdd, 0xaf, 0xa4, 0x0e, 0xfc, 0xb1, 0xd9, 0x92, 0xa0, 0xf0, 0x75, 0x42, 0x7f, 0x50, 0xa1, 0x77] =>
                                        {
                                            println!("second")
                                        }
                                    _ => {}
                                }
                                println!("{} disgest: {}", value.ident, to_hex_string(&hex_value));
                            }
                            idl::idl_nodes::IdlNode::TypeInterface(value) => {
                                let hex_value = value.hash.clone();
                                //println!("{} disgest: {}", value.ident, to_hex_string(&hex_value));
                            }
                            _ => {}
                        }
                    }
                }
                Err(err) => println!("Error: {}", err),
            },
            Err(err) => {
                panic!("{:?} at {:?}", err.1, err.1.get_range());
            }
        }
    }
}
