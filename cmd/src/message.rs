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
        if !messages.is_empty() && messages.iter().all(|v| !v.is_empty()) {
            println!("{}: ", name);
            for message in messages {
                println!("{}", message);
            }
        }

        Ok(())
    }

    pub fn info(message: &str) -> anyhow::Result<()> {
        if !message.is_empty() {
            let mut terminal = term::stdout().unwrap();
            terminal.fg(term::color::BRIGHT_BLUE)?;
            println!("{}", message);
            terminal.reset()?;
        }

        Ok(())
    }
}
