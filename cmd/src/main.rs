use idl::parser::Parser;

fn main() {
    let text = std::fs::read_to_string("idl_exm.idl").expect("Could not read file");
    let mut modl = Parser::parse(&text);
    println!("{:?}", modl);
}