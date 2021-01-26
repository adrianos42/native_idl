use term;

pub struct Message;

impl Message {
    pub fn error(name: &str, message: &str) -> anyhow::Result<()> {
        if !message.is_empty() {
            let mut terminal = term::stdout().unwrap();
            println!("{}: ", name);
            terminal.fg(term::color::RED)?;
            println!("{}", message);
            terminal.reset()?;
        }

        Ok(())
    }

    pub fn normal(name: &str, messages: Vec<String>) -> anyhow::Result<()> {
        if !messages.is_empty() && messages.iter().any(|v| !v.is_empty()) {
            println!("{}: ", name);
            let mut terminal = term::stdout().unwrap();
            terminal.fg(term::color::BRIGHT_BLUE)?;
            for message in messages {
                println!("{}", message);
            }
            terminal.reset()?;
        }

        Ok(())
    }

    pub fn info(message: &str) -> anyhow::Result<()> {
        if !message.is_empty() {
            println!("{}", message);
        }

        Ok(())
    }
}
