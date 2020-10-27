use clap::{App, Arg};
use cmd::{formatter, generator};
use idl::parser::Parser;

fn main() {
    let mut app = App::new("native_idl")
        .version("0.1")
        .author("Adriano Souza <adriano.souza113@gmail.com>")
        .about("Native Idl language")
        .subcommand(generator::create_command())
        .subcommand(formatter::create_command());

    let matches = app.get_matches_mut();

    if let Err(err) = match matches.subcommand() {
        Some(("format", value)) => formatter::parse(value),
        Some(("gen", value)) => generator::parse(value),
        _ => Err(anyhow::anyhow!("{}", app.generate_usage())),
    } {
        println!("{}", err);
    }
}