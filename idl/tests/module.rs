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

static IDL_FIRST: &str = r#"
library programmer

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

use ansi_term::Color;
use anyhow::{format_err, Result};
use idl::module::Module;

#[test]
fn try_this() -> Result<()> {
    let mut module = Module::new();

    module.replace_ids_document("idl_nodes.ids", IDS_FIRST);
    module.replace_idl_document("idl_nodes.idl", IDL_FIRST);

    module.update()?;

    Ok(())
}