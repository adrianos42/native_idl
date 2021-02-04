use clap::{App, Arg};
use cmd::{formatter, client, server, analyze, clean, message};

fn main() -> anyhow::Result<()> {
    let mut app = App::new("native_idl")
        .version("0.1")
        .author("Adriano Souza <adriano.souza113@gmail.com>")
        .about("Native Idl language")
        .subcommand(analyze::create_command())
        .subcommand(client::create_command())
        .subcommand(server::create_command())
        .subcommand(clean::create_command())
        .subcommand(formatter::create_command());

    let matches = app.get_matches_mut();

    if let Err(err) = match matches.subcommand() {
        Some(("format", value)) => formatter::parse(value),
        Some(("server", value)) => server::parse(value),
        Some(("client", value)) => client::parse(value),
        Some(("clean", value)) => clean::parse(value),
        Some(("analyze", value)) => analyze::parse(value),
        _ => Err(anyhow::anyhow!("{}", app.generate_usage())),
    } {
        message::Message::error("", err.to_string())?;
    }

    Ok(())
}